//! §17 broker API: subscribe routing, query handlers, post-fill fan-out.

use std::sync::Arc;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::*;

use crate::account_state::{
    parse_account_subscription, AccountSubscription, AccountSubscriptionKind,
};
use crate::auth::{account_from_private_path, authorize_private};
use crate::market_data::{
    parse_candle_query_path, parse_md_subscription, parse_ticker_query_path,
    parse_trade_query_path, StreamSubscription, SubscriptionKind,
};
use crate::matching::Fill;
use crate::server::{make_error_frame, schema_id, ExchangeState};

pub async fn handle_query_request(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);
    let method = extension_text(&frame, ExtensionTag::Method);
    let method = if method.is_empty() { "GET".to_string() } else { method };

    if let Some(account) = account_from_private_path(&channel_path) {
        if let Some(err) = authorize_private(&frame, &account) {
            return vec![err];
        }
    }

    if let Some(symbol) = parse_ticker_query_path(&channel_path) {
        let md = state.market_data.lock().await;
        let ticker = md.ticker_snapshot(&symbol);
        drop(md);
        return ok_response(frame, &ticker);
    }

    if method != "GET" && !frame.payload.is_empty() {
        // POST with query payload is allowed
    }

    if let Some((symbol, interval)) = parse_candle_query_path(&channel_path) {
        let req = decode_or(
            &frame.payload,
            CandleBarRequest {
                symbol: symbol.clone(),
                interval: interval.clone(),
                start_time: None,
                end_time: None,
                limit: Some(500),
            },
        );
        let md = state.market_data.lock().await;
        let batch = md.query_candles(
            &req.symbol,
            &req.interval,
            req.start_time,
            req.end_time,
            req.limit,
        );
        drop(md);
        return ok_response(frame, &batch);
    }

    if let Some(symbol) = parse_trade_query_path(&channel_path) {
        let req = decode_or(
            &frame.payload,
            TradeHistoryRequest {
                symbol: symbol.clone(),
                start_time: None,
                end_time: None,
                limit: Some(500),
            },
        );
        let md = state.market_data.lock().await;
        let batch = md.query_trades(
            &req.symbol,
            req.start_time,
            req.end_time,
            req.limit,
        );
        drop(md);
        return ok_response(frame, &batch);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/fills") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame.payload,
            FillHistoryRequest {
                account: account.to_string(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(500),
            },
        );
        let accounts = state.accounts.lock().await;
        let acct = accounts
            .get(&req.account)
            .cloned()
            .unwrap_or_else(|| crate::account_state::SimAccount::demo(&req.account));
        drop(accounts);
        let batch = acct.query_fills(
            req.symbol.as_deref(),
            req.start_time,
            req.end_time,
            req.limit,
        );
        return ok_response(frame, &batch);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/funding") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame.payload,
            FundingHistoryRequest {
                account: account.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(500),
            },
        );
        let accounts = state.accounts.lock().await;
        let acct = accounts
            .get(&req.account)
            .cloned()
            .unwrap_or_else(|| crate::account_state::SimAccount::demo(&req.account));
        drop(accounts);
        return ok_response(frame, &acct.query_funding(req.start_time, req.end_time, req.limit));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/ledger") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame.payload,
            LedgerHistoryRequest {
                account: account.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(500),
            },
        );
        let accounts = state.accounts.lock().await;
        let acct = accounts
            .get(&req.account)
            .cloned()
            .unwrap_or_else(|| crate::account_state::SimAccount::demo(&req.account));
        drop(accounts);
        return ok_response(frame, &acct.query_ledger(req.start_time, req.end_time, req.limit));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/margin") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let mut accounts = state.accounts.lock().await;
        let summary = accounts.get_or_create(account).margin_summary();
        drop(accounts);
        return ok_response(frame, &summary);
    }

    if channel_path.starts_with("accounts/") {
        let parts: Vec<&str> = channel_path.split('/').collect();
        if parts.len() == 2 {
            let account = parts[1];
            let mut accounts = state.accounts.lock().await;
            let summary = accounts.get_or_create(account).account_summary();
            drop(accounts);
            return ok_response(frame, &summary);
        }
    }

    vec![make_error_frame(
        frame.channel_id,
        frame.stream_seq,
        "UNKNOWN_QUERY_PATH",
    )]
}

fn decode_or<T: serde::de::DeserializeOwned + Clone>(payload: &[u8], default: T) -> T {
    if payload.is_empty() {
        default
    } else {
        codec::decode_cbor(payload).unwrap_or(default)
    }
}

fn ok_response<T: serde::Serialize>(frame: Frame, payload: &T) -> Vec<Frame> {
    match codec::encode_cbor(payload) {
        Ok(bytes) => vec![
            Frame::new(FrameType::Response, frame.channel_id)
                .with_seq(frame.stream_seq)
                .with_schema_id(schema_id::TRADING_ORDERS)
                .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
                .with_extension(Extension::text(
                    ExtensionTag::ContentType,
                    "application/cbor",
                ))
                .with_payload(bytes),
        ],
        Err(_) => vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "ENCODE_ERROR",
        )],
    }
}

pub async fn handle_market_subscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let routing_key = extension_text(&frame, ExtensionTag::RoutingKey);
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);

    let Some(kind) = parse_md_subscription(&routing_key, &channel_path) else {
        return vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "UNKNOWN_MD_SUBSCRIPTION",
        )];
    };

    state.subscriptions.lock().await.push(StreamSubscription {
        channel_id: frame.channel_id,
        routing_key: if routing_key.is_empty() {
            channel_path.clone()
        } else {
            routing_key.clone()
        },
        kind: kind.clone(),
    });

    match kind {
        SubscriptionKind::Quotes { symbol } => {
            let mut frames: Vec<Frame> = crate::server::build_market_data_push(state, &symbol)
                .await
                .into_iter()
                .filter(|f| f.channel_id == frame.channel_id)
                .collect();
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
        SubscriptionKind::Candles { symbol, interval } => {
            let md = state.market_data.lock().await;
            let mut frames = Vec::new();
            if let Some(mut partial) = md.partial_candle(&symbol, &interval) {
                partial.is_snapshot = Some(true);
                if let Ok(payload) = codec::encode_cbor(&CandleBarEvent { bar: partial }) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/candles",
                    ));
                }
            }
            drop(md);
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
        SubscriptionKind::Trades { symbol } => {
            let md = state.market_data.lock().await;
            let batch = md.query_trades(&symbol, None, None, Some(50));
            drop(md);
            let mut frames = Vec::new();
            for trade in batch.trades {
                if let Ok(payload) = codec::encode_cbor(&PublicTradeEvent { trade }) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/trades",
                    ));
                }
            }
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
        SubscriptionKind::Bbo { symbol } => {
            let engine = state.engine.lock().await;
            let bbo_frame = if let Some(book) = engine.get_book(&symbol) {
                let bid = book.best_bid().map(|(p, q)| (p.0, q.0));
                let ask = book.best_ask().map(|(p, q)| (p.0, q.0));
                drop(engine);
                let mut md = state.market_data.lock().await;
                md.update_bbo(&symbol, bid, ask);
                md.last_bbo(&symbol).map(|mut bbo| {
                    bbo.is_snapshot = Some(true);
                    bbo
                })
            } else {
                None
            };
            if let Some(bbo) = bbo_frame {
                if let Ok(payload) = codec::encode_cbor(&bbo) {
                    return vec![stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/bbo",
                    )];
                }
            }
            vec![ack_subscribe(frame)]
        }
        SubscriptionKind::Ticker { symbol } => {
            let md = state.market_data.lock().await;
            let mut ticker = md.ticker_snapshot(&symbol);
            ticker.is_snapshot = Some(true);
            drop(md);
            if let Ok(payload) = codec::encode_cbor(&ticker) {
                return vec![stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "marketdata/ticker",
                )];
            }
            vec![ack_subscribe(frame)]
        }
    }
}

pub async fn handle_account_subscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let routing_key = extension_text(&frame, ExtensionTag::RoutingKey);
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);

    let Some((account, kind)) = parse_account_subscription(&routing_key, &channel_path) else {
        return vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "UNKNOWN_ACCOUNT_SUBSCRIPTION",
        )];
    };

    if let Some(err) = authorize_private(&frame, &account) {
        return vec![err];
    }

    state
        .account_subscriptions
        .lock()
        .await
        .push(AccountSubscription {
            channel_id: frame.channel_id,
            routing_key: if routing_key.is_empty() {
                channel_path.clone()
            } else {
                routing_key.clone()
            },
            account: account.clone(),
            kind: kind.clone(),
        });

    let mut accounts = state.accounts.lock().await;
    let acct = accounts.get_or_create(&account);
    let mut frames = Vec::new();
    match kind {
        AccountSubscriptionKind::Executions => {
            for fill in &acct.fills {
                if let Ok(payload) = codec::encode_cbor(fill) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "trading/executions",
                    ));
                }
            }
        }
        AccountSubscriptionKind::Balances => {
            let snap = acct.balance_snapshot(true);
            if let Ok(payload) = codec::encode_cbor(&snap) {
                frames.push(stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "accounts/balances",
                ));
            }
        }
        AccountSubscriptionKind::Positions => {
            let snap = acct.position_snapshot(true);
            if let Ok(payload) = codec::encode_cbor(&snap) {
                frames.push(stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "accounts/positions",
                ));
            }
        }
        AccountSubscriptionKind::Funding => {
            for payment in &acct.funding {
                if let Ok(payload) = codec::encode_cbor(payment) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "accounts/funding",
                    ));
                }
            }
        }
        AccountSubscriptionKind::Ledger => {
            for entry in &acct.ledger {
                if let Ok(payload) = codec::encode_cbor(entry) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "accounts/ledger",
                    ));
                }
            }
        }
    }
    drop(accounts);
    if frames.is_empty() {
        frames.push(ack_subscribe(frame));
    }
    frames
}

pub async fn post_fill_updates(
    state: &Arc<ExchangeState>,
    symbol: &str,
    account: &str,
    fills: &[Fill],
    reports: &[ExecutionReport],
) -> Vec<Frame> {
    let mut frames = Vec::new();
    {
        let mut md = state.market_data.lock().await;
        for fill in fills {
            md.on_trade(symbol, fill.fill_price.0, fill.fill_qty.0, fill.side);
        }
    }

    {
        let mut accounts = state.accounts.lock().await;
        let acct = accounts.get_or_create(account);
        let subs: Vec<AccountSubscription> =
            state.account_subscriptions.lock().await.clone();
        for report in reports {
            let balance_update = acct.record_fill(report.clone());
            for sub in subs.iter().filter(|s| s.account == account) {
                match sub.kind {
                    AccountSubscriptionKind::Executions => {
                        if let Ok(payload) = codec::encode_cbor(report) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "trading/executions",
                            ));
                        }
                    }
                    AccountSubscriptionKind::Balances => {
                        if let Ok(payload) = codec::encode_cbor(&balance_update) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "accounts/balances",
                            ));
                        }
                    }
                    AccountSubscriptionKind::Positions => {}
                    AccountSubscriptionKind::Funding => {}
                    AccountSubscriptionKind::Ledger => {}
                }
            }
            let ledger = LedgerUpdate {
                account: account.to_string(),
                asset: "USD".to_string(),
                delta: balance_update.delta,
                kind: LedgerUpdateKind::Fee,
                timestamp: report.transact_time,
                reference_id: Some(report.exec_id.clone()),
            };
            acct.record_ledger(ledger.clone());
            for sub in subs.iter().filter(|s| s.account == account) {
                if sub.kind == AccountSubscriptionKind::Ledger {
                    if let Ok(payload) = codec::encode_cbor(&ledger) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "accounts/ledger",
                        ));
                    }
                }
            }
        }
    }

    let subs: Vec<StreamSubscription> = state.subscriptions.lock().await.clone();
    for sub in subs {
        match &sub.kind {
            SubscriptionKind::Candles { symbol: sym, interval } if sym == symbol => {
                let md = state.market_data.lock().await;
                if let Some(bar) = md.partial_candle(sym, interval) {
                    if let Ok(payload) = codec::encode_cbor(&CandleBarEvent { bar }) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "marketdata/candles",
                        ));
                    }
                }
            }
            SubscriptionKind::Trades { symbol: sym } if sym == symbol => {
                let md = state.market_data.lock().await;
                if let Some(trade) = md.last_trade(symbol) {
                    if let Ok(payload) = codec::encode_cbor(&PublicTradeEvent { trade }) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "marketdata/trades",
                        ));
                    }
                }
            }
            SubscriptionKind::Bbo { symbol: sym } if sym == symbol => {
                let bid_ask = {
                    let engine = state.engine.lock().await;
                    engine.get_book(&symbol.to_string()).map(|book| {
                        (
                            book.best_bid().map(|(p, q)| (p.0, q.0)),
                            book.best_ask().map(|(p, q)| (p.0, q.0)),
                        )
                    })
                };
                if let Some((bid, ask)) = bid_ask {
                    let mut md_mut = state.market_data.lock().await;
                    md_mut.update_bbo(symbol, bid, ask);
                    if let Some(bbo) = md_mut.last_bbo(symbol) {
                        if let Ok(payload) = codec::encode_cbor(&bbo) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "marketdata/bbo",
                            ));
                        }
                    }
                }
            }
            SubscriptionKind::Ticker { symbol: sym } if sym == symbol => {
                let md = state.market_data.lock().await;
                if let Some(ticker) = md.ticker(symbol) {
                    if let Ok(payload) = codec::encode_cbor(&ticker) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "marketdata/ticker",
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    frames.extend(crate::server::push_book_depth(state, symbol).await);
    frames
}

fn ack_subscribe(frame: Frame) -> Frame {
    Frame::new(FrameType::Response, frame.channel_id)
        .with_seq(frame.stream_seq)
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
}

fn stream_item(channel_id: u16, routing_key: &str, payload: Vec<u8>, path: &str) -> Frame {
    Frame::new(FrameType::StreamItem, channel_id)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(ExtensionTag::RoutingKey, routing_key))
        .with_extension(Extension::text(ExtensionTag::ChannelPath, path))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(payload)
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
