//! Session subscription persistence, UNSUBSCRIBE, and request-stream responses.

use std::sync::Arc;
use uuid::Uuid;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::*;
use fig_core::session::{Session, SessionStore, SessionSubscription};
use fig_core::transport::FigConnection;

use crate::account_state::{parse_account_subscription, AccountSubscriptionKind};
use crate::broker_api::{handle_account_subscribe, handle_market_subscribe};
use crate::market_data::parse_md_subscription;
use crate::server::{make_error_frame, schema_id, ExchangeState};

pub const REQUEST_STREAM_THRESHOLD: usize = 50;
pub const REQUEST_STREAM_CHUNK: usize = 25;

fn cap(path: &str, pattern: CapabilityPathPattern, auth_required: bool) -> CapabilityPath {
    CapabilityPath {
        path: path.to_string(),
        pattern,
        auth_required,
    }
}

/// SPEC §9.1 catalog. Dual-pattern paths (pub/sub + GET) emit both rows.
pub fn capabilities_response() -> CapabilitiesResponse {
    use CapabilityPathPattern::{PubSub, RequestResponse, RequestStream};
    CapabilitiesResponse {
        schema_ids: vec![0x01, 0x02, 0x03],
        paths: vec![
            cap("marketdata/{symbol}/book", PubSub, false),
            cap("marketdata/{symbol}/book", RequestResponse, false),
            cap("marketdata/{symbol}/quotes", PubSub, false),
            cap("marketdata/{symbol}/quotes", RequestResponse, false),
            cap("marketdata/{symbol}/bbo", PubSub, false),
            cap("marketdata/{symbol}/trades", PubSub, false),
            cap("marketdata/{symbol}/trades", RequestResponse, false),
            cap("marketdata/{symbol}/aggtrades", PubSub, false),
            cap("marketdata/{symbol}/aggtrades", RequestResponse, false),
            cap("marketdata/{symbol}/candles/{interval}", PubSub, false),
            cap(
                "marketdata/{symbol}/candles/{interval}",
                RequestResponse,
                false,
            ),
            cap("marketdata/{symbol}/ticker", PubSub, false),
            cap("marketdata/{symbol}/ticker", RequestResponse, false),
            cap("marketdata/ticker/all", PubSub, false),
            cap("marketdata/ticker/all", RequestResponse, false),
            cap("marketdata/{symbol}/mark", PubSub, false),
            cap("marketdata/{symbol}/mark", RequestResponse, false),
            cap("marketdata/liquidations", PubSub, false),
            cap("trading/accounts/{account}/executions", PubSub, true),
            cap(
                "trading/accounts/{account}/orders/open",
                RequestResponse,
                true,
            ),
            cap("trading/accounts/{account}/orders", RequestStream, true),
            cap("accounts/{account}", RequestResponse, true),
            cap("accounts/{account}/balances", PubSub, true),
            cap("accounts/{account}/positions", PubSub, true),
            cap("accounts/{account}/margin", PubSub, true),
            cap("accounts/{account}/margin", RequestResponse, true),
            cap("accounts/{account}/fills", RequestResponse, true),
            cap("accounts/{account}/funding", PubSub, true),
            cap("accounts/{account}/funding", RequestResponse, true),
            cap("accounts/{account}/ledger", PubSub, true),
            cap("accounts/{account}/ledger", RequestResponse, true),
            cap("accounts/{account}/liquidations", PubSub, true),
            cap("trading/accounts/{account}/orderlists", PubSub, true),
            cap(".well-known/capabilities", RequestResponse, false),
            cap(".well-known/instruments", RequestResponse, false),
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
    conn: Option<Arc<FigConnection>>,
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
            handle_account_subscribe(frame.clone(), state, conn.clone()).await
        } else {
            handle_market_subscribe(frame.clone(), state, conn.clone()).await
        };
        frames.append(&mut part);
    }
    frames
}

pub async fn handle_unsubscribe(
    frame: Frame,
    state: &Arc<ExchangeState>,
    conn: Option<Arc<FigConnection>>,
) -> Vec<Frame> {
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);
    let routing_key = extension_text(&frame, ExtensionTag::RoutingKey);

    let same_conn = |sub_conn: &Option<Arc<FigConnection>>| match (&conn, sub_conn) {
        (Some(want), Some(have)) => Arc::ptr_eq(want, have),
        (None, _) => true,
        (Some(_), None) => false,
    };

    state.subscriptions.lock().await.retain(|s| {
        !(s.channel_id == frame.channel_id
            && (channel_path.is_empty() || s.routing_key.contains(&channel_path))
            && same_conn(&s.conn))
    });
    state.account_subscriptions.lock().await.retain(|s| {
        !(s.channel_id == frame.channel_id
            && (channel_path.is_empty() || s.routing_key.contains(&channel_path))
            && same_conn(&s.conn))
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

pub fn parse_instruments_path(path: &str) -> bool {
    path == ".well-known/instruments" || path == "instruments"
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

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::messages::CapabilityPathPattern::{PubSub, RequestResponse, RequestStream};

    #[test]
    fn capabilities_match_spec_9_1() {
        let caps = capabilities_response();
        let expected: &[(&str, CapabilityPathPattern, bool)] = &[
            ("marketdata/{symbol}/book", PubSub, false),
            ("marketdata/{symbol}/book", RequestResponse, false),
            ("marketdata/{symbol}/quotes", PubSub, false),
            ("marketdata/{symbol}/quotes", RequestResponse, false),
            ("marketdata/{symbol}/bbo", PubSub, false),
            ("marketdata/{symbol}/trades", PubSub, false),
            ("marketdata/{symbol}/trades", RequestResponse, false),
            ("marketdata/{symbol}/aggtrades", PubSub, false),
            ("marketdata/{symbol}/aggtrades", RequestResponse, false),
            ("marketdata/{symbol}/candles/{interval}", PubSub, false),
            (
                "marketdata/{symbol}/candles/{interval}",
                RequestResponse,
                false,
            ),
            ("marketdata/{symbol}/ticker", PubSub, false),
            ("marketdata/{symbol}/ticker", RequestResponse, false),
            ("marketdata/ticker/all", PubSub, false),
            ("marketdata/ticker/all", RequestResponse, false),
            ("marketdata/{symbol}/mark", PubSub, false),
            ("marketdata/{symbol}/mark", RequestResponse, false),
            ("marketdata/liquidations", PubSub, false),
            ("trading/accounts/{account}/executions", PubSub, true),
            (
                "trading/accounts/{account}/orders/open",
                RequestResponse,
                true,
            ),
            ("trading/accounts/{account}/orders", RequestStream, true),
            ("accounts/{account}", RequestResponse, true),
            ("accounts/{account}/balances", PubSub, true),
            ("accounts/{account}/positions", PubSub, true),
            ("accounts/{account}/margin", PubSub, true),
            ("accounts/{account}/margin", RequestResponse, true),
            ("accounts/{account}/fills", RequestResponse, true),
            ("accounts/{account}/funding", PubSub, true),
            ("accounts/{account}/funding", RequestResponse, true),
            ("accounts/{account}/ledger", PubSub, true),
            ("accounts/{account}/ledger", RequestResponse, true),
            ("accounts/{account}/liquidations", PubSub, true),
            ("trading/accounts/{account}/orderlists", PubSub, true),
            (".well-known/capabilities", RequestResponse, false),
            (".well-known/instruments", RequestResponse, false),
        ];
        assert_eq!(caps.paths.len(), expected.len());
        for (path, pattern, auth) in expected {
            assert!(
                caps.paths
                    .iter()
                    .any(|p| p.path == *path && p.pattern == *pattern && p.auth_required == *auth),
                "missing catalog row {path} {pattern:?} auth={auth}"
            );
        }
    }
}
