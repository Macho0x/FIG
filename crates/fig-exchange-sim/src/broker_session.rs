//! Session subscription persistence, UNSUBSCRIBE, and request-stream responses.

use std::sync::Arc;
use uuid::Uuid;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::*;
use fig_core::session::{Session, SessionStore, SessionSubscription};

use crate::account_state::{parse_account_subscription, AccountSubscriptionKind};
use crate::broker_api::{handle_account_subscribe, handle_market_subscribe};
use crate::market_data::parse_md_subscription;
use crate::server::{make_error_frame, schema_id, ExchangeState};

pub const REQUEST_STREAM_THRESHOLD: usize = 50;
pub const REQUEST_STREAM_CHUNK: usize = 25;

pub fn capabilities_response() -> CapabilitiesResponse {
    CapabilitiesResponse {
        schema_ids: vec![0x01, 0x02, 0x03],
        paths: vec![
            CapabilityPath {
                path: "marketdata/{symbol}/candles/{interval}".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: false,
            },
            CapabilityPath {
                path: "marketdata/{symbol}/trades".to_string(),
                pattern: CapabilityPathPattern::RequestResponse,
                auth_required: false,
            },
            CapabilityPath {
                path: "trading/accounts/{account}/orders/open".to_string(),
                pattern: CapabilityPathPattern::RequestResponse,
                auth_required: true,
            },
            CapabilityPath {
                path: "trading/accounts/{account}/orders".to_string(),
                pattern: CapabilityPathPattern::RequestStream,
                auth_required: true,
            },
            CapabilityPath {
                path: "marketdata/{symbol}/aggtrades".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: false,
            },
            CapabilityPath {
                path: "marketdata/ticker/all".to_string(),
                pattern: CapabilityPathPattern::RequestResponse,
                auth_required: false,
            },
            CapabilityPath {
                path: "marketdata/{symbol}/mark".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: false,
            },
            CapabilityPath {
                path: "marketdata/liquidations".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: false,
            },
            CapabilityPath {
                path: "accounts/{account}/margin".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: true,
            },
            CapabilityPath {
                path: "accounts/{account}/liquidations".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: true,
            },
            CapabilityPath {
                path: "trading/accounts/{account}/orderlists".to_string(),
                pattern: CapabilityPathPattern::PubSub,
                auth_required: true,
            },
            CapabilityPath {
                path: ".well-known/capabilities".to_string(),
                pattern: CapabilityPathPattern::RequestResponse,
                auth_required: false,
            },
        ],
        symbols: vec![
            "AAPL".to_string(),
            "BTC".to_string(),
            "ETH".to_string(),
            "BTCUSDT".to_string(),
        ],
        intervals: vec!["1m".to_string(), "5m".to_string()],
    }
}

pub fn instrument_catalog_response() -> InstrumentCatalogResponse {
    InstrumentCatalogResponse {
        instruments: vec![
            InstrumentMetadata {
                instrument_id: "BTC-PERP".to_string(),
                symbol: "BTC".to_string(),
                product_kind: "perp".to_string(),
                margin_asset: Some("USDC".to_string()),
                display_name: Some("Bitcoin Perpetual".to_string()),
                tick_size: Some(0.1),
                lot_size: Some(0.001),
            },
            InstrumentMetadata {
                instrument_id: "ETH-PERP".to_string(),
                symbol: "ETH".to_string(),
                product_kind: "perp".to_string(),
                margin_asset: Some("USDC".to_string()),
                display_name: Some("Ethereum Perpetual".to_string()),
                tick_size: Some(0.01),
                lot_size: Some(0.01),
            },
        ],
    }
}

pub fn session_id_from_frame(frame: &Frame) -> Option<Uuid> {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::SessionId)
        .and_then(|e| {
            let bytes = e.value.as_bytes();
            if bytes.len() == 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&bytes);
                Some(Uuid::from_bytes(arr))
            } else {
                None
            }
        })
}

pub fn persist_subscription(state: &Arc<ExchangeState>, session_id: Uuid, frame: &Frame) {
    let channel_path = extension_text(frame, ExtensionTag::ChannelPath);
    let routing_key = extension_text(frame, ExtensionTag::RoutingKey);
    let kind = if let Some((account, ak)) = parse_account_subscription(&routing_key, &channel_path)
    {
        let tag = match ak {
            AccountSubscriptionKind::Executions => "executions",
            AccountSubscriptionKind::Balances => "balances",
            AccountSubscriptionKind::Positions => "positions",
            AccountSubscriptionKind::Margin => "margin",
            AccountSubscriptionKind::Funding => "funding",
            AccountSubscriptionKind::Ledger => "ledger",
            AccountSubscriptionKind::Liquidations => "liquidations",
            AccountSubscriptionKind::OrderLists => "orderlists",
            AccountSubscriptionKind::OpenOrders => "orders/open",
        };
        format!("acct:{account}:{tag}")
    } else if parse_md_subscription(&routing_key, &channel_path).is_some() {
        format!("md:{channel_path}")
    } else {
        "unknown".to_string()
    };
    let account = parse_account_subscription(&routing_key, &channel_path).map(|(a, _)| a);
    let sub = SessionSubscription {
        channel_path,
        routing_key,
        kind,
        account,
    };
    if let Ok(Some(mut session)) = state.file_sessions.get(&session_id) {
        session.add_subscription(sub);
        session.add_channel(frame.channel_id);
        if state.file_sessions.update(&session).is_err() {
            let _ = state.file_sessions.put(&session);
        }
    } else {
        let mut session = Session::new();
        session.session_id = session_id;
        session.add_subscription(sub);
        session.add_channel(frame.channel_id);
        let _ = state.file_sessions.put(&session);
    }
}

pub async fn restore_session_subscriptions(
    state: &Arc<ExchangeState>,
    session_id: Uuid,
    channel_id: u16,
) -> Vec<Frame> {
    let Some(session) = state.file_sessions.get(&session_id).ok().flatten() else {
        return Vec::new();
    };
    let mut frames = Vec::new();
    for sub in &session.subscriptions {
        let frame = Frame::new(FrameType::Subscribe, channel_id)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                &sub.channel_path,
            ))
            .with_extension(Extension::text(ExtensionTag::RoutingKey, &sub.routing_key));
        let mut part = if sub.kind.starts_with("acct:") {
            handle_account_subscribe(frame.clone(), state).await
        } else {
            handle_market_subscribe(frame.clone(), state).await
        };
        frames.append(&mut part);
    }
    frames
}

pub async fn handle_unsubscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);
    let routing_key = extension_text(&frame, ExtensionTag::RoutingKey);

    state.subscriptions.lock().await.retain(|s| {
        !(s.channel_id == frame.channel_id
            && (channel_path.is_empty() || s.routing_key.contains(&channel_path)))
    });
    state.account_subscriptions.lock().await.retain(|s| {
        !(s.channel_id == frame.channel_id
            && (channel_path.is_empty() || s.routing_key.contains(&channel_path)))
    });

    if let Some(session_id) = session_id_from_frame(&frame) {
        if let Ok(Some(mut session)) = state.file_sessions.get(&session_id) {
            session.remove_subscription(&channel_path, &routing_key);
            let _ = state.file_sessions.update(&session);
        }
    }

    vec![
        Frame::new(FrameType::Response, frame.channel_id)
            .with_seq(frame.stream_seq)
            .with_extension(Extension::u16(ExtensionTag::StatusCode, 200)),
        Frame::new(FrameType::StreamClose, frame.channel_id).with_seq(frame.stream_seq),
    ]
}

pub fn respond_cbor<T: serde::Serialize>(
    frame: Frame,
    payload: &T,
    stream_items: Option<Vec<Vec<u8>>>,
) -> Vec<Frame> {
    if let Some(items) = stream_items {
        return stream_response(frame, items);
    }
    match codec::encode_response_payload(&frame, payload) {
        Ok(bytes) => vec![response_frame(&frame, bytes)],
        Err(_) => vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "ENCODE_ERROR",
        )],
    }
}

fn response_frame(frame: &Frame, payload: Vec<u8>) -> Frame {
    Frame::new(FrameType::Response, frame.channel_id)
        .with_seq(frame.stream_seq)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            codec::frame_content_type(frame),
        ))
        .with_payload(payload)
}

/// Stream a large paginated batch as `STREAM_ITEM` chunks + `STREAM_CLOSE`.
pub fn stream_paginated_batch<T, F>(
    frame: Frame,
    items: &[T],
    _has_more: bool,
    encode: F,
) -> Vec<Frame>
where
    F: Fn(&[T]) -> Option<Vec<u8>>,
{
    if items.len() <= REQUEST_STREAM_THRESHOLD {
        return encode(items)
            .map(|bytes| vec![response_frame(&frame, bytes)])
            .unwrap_or_else(|| {
                vec![make_error_frame(
                    frame.channel_id,
                    frame.stream_seq,
                    "ENCODE_ERROR",
                )]
            });
    }
    let chunks: Vec<Vec<u8>> = items
        .chunks(REQUEST_STREAM_CHUNK)
        .filter_map(encode)
        .collect();
    stream_response(frame, chunks)
}

pub fn stream_candle_batch(frame: Frame, batch: &CandleBarBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.bars, batch.has_more, |chunk| {
        codec::encode_cbor(&CandleBarBatch {
            symbol: batch.symbol.clone(),
            interval: batch.interval.clone(),
            bars: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_order_history(frame: Frame, batch: &OrderHistoryBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.orders, batch.has_more, |chunk| {
        codec::encode_cbor(&OrderHistoryBatch {
            account: batch.account.clone(),
            orders: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_fill_history(frame: Frame, batch: &FillHistoryBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.fills, batch.has_more, |chunk| {
        codec::encode_cbor(&FillHistoryBatch {
            account: batch.account.clone(),
            fills: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_public_trade_batch(frame: Frame, batch: &PublicTradeBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.trades, batch.has_more, |chunk| {
        codec::encode_cbor(&PublicTradeBatch {
            symbol: batch.symbol.clone(),
            trades: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_agg_trade_batch(frame: Frame, batch: &AggregateTradeBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.trades, batch.has_more, |chunk| {
        codec::encode_cbor(&AggregateTradeBatch {
            symbol: batch.symbol.clone(),
            trades: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_funding_batch(frame: Frame, batch: &FundingHistoryBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.payments, batch.has_more, |chunk| {
        codec::encode_cbor(&FundingHistoryBatch {
            account: batch.account.clone(),
            payments: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

pub fn stream_ledger_batch(frame: Frame, batch: &LedgerHistoryBatch) -> Vec<Frame> {
    stream_paginated_batch(frame, &batch.entries, batch.has_more, |chunk| {
        codec::encode_cbor(&LedgerHistoryBatch {
            account: batch.account.clone(),
            entries: chunk.to_vec(),
            has_more: batch.has_more,
            next_cursor: batch.next_cursor.clone(),
        })
        .ok()
    })
}

fn stream_response(frame: Frame, payloads: Vec<Vec<u8>>) -> Vec<Frame> {
    let content_type = codec::frame_content_type(&frame);
    let mut frames: Vec<Frame> = payloads
        .into_iter()
        .map(|payload| {
            Frame::new(FrameType::StreamItem, frame.channel_id)
                .with_seq(frame.stream_seq)
                .with_schema_id(schema_id::TRADING_ORDERS)
                .with_extension(Extension::text(ExtensionTag::ContentType, content_type))
                .with_payload(payload)
        })
        .collect();
    frames.push(Frame::new(FrameType::StreamClose, frame.channel_id).with_seq(frame.stream_seq));
    frames
}

fn extension_text(frame: &Frame, tag: ExtensionTag) -> String {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == tag)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string()
}

pub fn parse_open_orders_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 5
        && parts[0] == "trading"
        && parts[1] == "accounts"
        && parts[3] == "orders"
        && parts[4] == "open"
    {
        return Some(parts[2].to_string());
    }
    None
}

pub fn parse_order_history_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 4 && parts[0] == "trading" && parts[1] == "accounts" && parts[3] == "orders" {
        return Some(parts[2].to_string());
    }
    None
}

pub fn parse_capabilities_path(path: &str) -> bool {
    path == ".well-known/capabilities" || path == "capabilities"
}

pub fn parse_order_book_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" {
        match parts.get(2).copied() {
            Some("book") | Some("quotes") => Some(parts[1].to_string()),
            _ => None,
        }
    } else {
        None
    }
}

pub fn parse_position_query_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 3 && parts[0] == "accounts" && parts[2] == "positions" {
        return Some(parts[1].to_string());
    }
    None
}
