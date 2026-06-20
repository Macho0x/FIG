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
use crate::broker_session::{
    capabilities_response, parse_capabilities_path, parse_open_orders_path, parse_order_book_path,
    parse_order_history_path, parse_position_query_path, respond_cbor, stream_agg_trade_batch,
    stream_candle_batch, stream_fill_history, stream_funding_batch, stream_ledger_batch,
    stream_order_history, stream_public_trade_batch,
};
use crate::market_data::{
    parse_agg_trade_query_path, parse_all_mids_path, parse_candle_query_path,
    parse_mark_query_path, parse_md_subscription, parse_ticker_query_path, parse_trade_query_path,
    StreamSubscription, SubscriptionKind,
};
use crate::matching::Fill;
use crate::path_policy::validate_interaction;
use crate::server::{make_error_frame, schema_id, ExchangeState};

pub async fn handle_query_request(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    if let Some(code) = validate_interaction(&frame) {
        return vec![make_error_frame(frame.channel_id, frame.stream_seq, code)];
    }

    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);
    let method = extension_text(&frame, ExtensionTag::Method);
    let method = if method.is_empty() {
        "GET".to_string()
    } else {
        method
    };

    if let Some(account) = account_from_private_path(&channel_path) {
        if let Some(err) = authorize_private(&frame, &account) {
            return vec![err];
        }
    }

    if let Some(symbol) = parse_mark_query_path(&channel_path) {
        let md = state.market_data.lock().await;
        let mark = md.mark_price(&symbol);
        drop(md);
        return respond_cbor(frame, &mark, None);
    }

    if parse_all_mids_path(&channel_path) {
        let md = state.market_data.lock().await;
        let batch = md.all_mini_tickers();
        drop(md);
        return respond_cbor(frame, &batch, None);
    }

    if let Some(symbol) = parse_agg_trade_query_path(&channel_path) {
        let req = decode_or(
            &frame,
            AggregateTradeRequest {
                symbol: symbol.clone(),
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let md = state.market_data.lock().await;
        let batch = md.query_agg_trades(
            &req.symbol,
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        drop(md);
        return stream_agg_trade_batch(frame, &batch);
    }

    if let Some(symbol) = parse_ticker_query_path(&channel_path) {
        let md = state.market_data.lock().await;
        let ticker = md.ticker_snapshot(&symbol);
        drop(md);
        return respond_cbor(frame, &ticker, None);
    }

    if parse_capabilities_path(&channel_path) {
        return respond_cbor(frame, &capabilities_response(), None);
    }

    if let Some(symbol) = parse_order_book_path(&channel_path) {
        let req = decode_or(
            &frame,
            OrderBookRequest {
                symbol: symbol.clone(),
                depth: Some(20),
                at_time: None,
            },
        );
        let depth = req.depth.unwrap_or(20) as usize;
        if let Some(at_time) = req.at_time {
            let md = state.market_data.lock().await;
            if let Some(snap) = md.book_at_time(&req.symbol, at_time, depth) {
                drop(md);
                return respond_cbor(frame, &snap, None);
            }
            drop(md);
        }
        let engine = state.engine.lock().await;
        let snap = engine
            .order_book_snapshot_typed(&req.symbol, depth)
            .unwrap_or(OrderBookSnapshot {
                symbol: req.symbol.clone(),
                exchange: "SIM".to_string(),
                bids: vec![],
                asks: vec![],
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as i64,
                sequence: Some(0),
                is_snapshot: Some(true),
            });
        drop(engine);
        let mut md = state.market_data.lock().await;
        md.record_book_snapshot(snap.clone());
        drop(md);
        return respond_cbor(frame, &snap, None);
    }

    if let Some(account) = parse_open_orders_path(&channel_path) {
        let req = decode_or(
            &frame,
            OpenOrdersRequest {
                account: account.clone(),
                symbol: None,
            },
        );
        let engine = state.engine.lock().await;
        let snap = engine.open_orders(&req.account, req.symbol.as_deref());
        drop(engine);
        return respond_cbor(frame, &snap, None);
    }

    if let Some(account) = parse_order_history_path(&channel_path) {
        let req = decode_or(
            &frame,
            OrderHistoryRequest {
                account: account.clone(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let engine = state.engine.lock().await;
        let batch = engine.query_order_history(
            &req.account,
            req.symbol.as_deref(),
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        drop(engine);
        return stream_order_history(frame, &batch);
    }

    if let Some(account) = parse_position_query_path(&channel_path) {
        let mut accounts = state.accounts.lock().await;
        let snap = accounts.get_or_create(&account).position_snapshot(true);
        drop(accounts);
        return respond_cbor(frame, &snap, None);
    }

    if method != "GET" && !frame.payload.is_empty() {
        // POST with query payload is allowed
    }

    if let Some((symbol, interval)) = parse_candle_query_path(&channel_path) {
        let req = decode_or(
            &frame,
            CandleBarRequest {
                symbol: symbol.clone(),
                interval: interval.clone(),
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let md = state.market_data.lock().await;
        let batch = md.query_candles(
            &req.symbol,
            &req.interval,
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        drop(md);
        return stream_candle_batch(frame, &batch);
    }

    if let Some(symbol) = parse_trade_query_path(&channel_path) {
        let req = decode_or(
            &frame,
            TradeHistoryRequest {
                symbol: symbol.clone(),
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let md = state.market_data.lock().await;
        let batch = md.query_trades(
            &req.symbol,
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        drop(md);
        return stream_public_trade_batch(frame, &batch);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/fills") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame,
            FillHistoryRequest {
                account: account.to_string(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
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
            req.cursor.as_deref(),
        );
        return stream_fill_history(frame, &batch);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/funding") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame,
            FundingHistoryRequest {
                account: account.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let accounts = state.accounts.lock().await;
        let acct = accounts
            .get(&req.account)
            .cloned()
            .unwrap_or_else(|| crate::account_state::SimAccount::demo(&req.account));
        drop(accounts);
        let batch = acct.query_funding(
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        return stream_funding_batch(frame, &batch);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/ledger") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = decode_or(
            &frame,
            LedgerHistoryRequest {
                account: account.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(500),
                cursor: None,
            },
        );
        let accounts = state.accounts.lock().await;
        let acct = accounts
            .get(&req.account)
            .cloned()
            .unwrap_or_else(|| crate::account_state::SimAccount::demo(&req.account));
        drop(accounts);
        let batch = acct.query_ledger(
            req.start_time,
            req.end_time,
            req.limit,
            req.cursor.as_deref(),
        );
        return stream_ledger_batch(frame, &batch);
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

fn decode_or<T: serde::de::DeserializeOwned + Clone>(frame: &Frame, default: T) -> T {
    codec::decode_request_payload_or(frame, default)
}

fn ok_response<T: serde::Serialize>(frame: Frame, payload: &T) -> Vec<Frame> {
    respond_cbor(frame, payload, None)
}

pub async fn handle_market_subscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    if let Some(code) = validate_interaction(&frame) {
        return vec![make_error_frame(frame.channel_id, frame.stream_seq, code)];
    }

    let routing_key = extension_text(&frame, ExtensionTag::RoutingKey);
    let channel_path = extension_text(&frame, ExtensionTag::ChannelPath);

    let Some(kind) = parse_md_subscription(&routing_key, &channel_path) else {
        return vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "UNKNOWN_MD_SUBSCRIPTION",
        )];
    };

    state
        .subscriptions
        .lock()
        .await
        .retain(|existing| !(existing.channel_id == frame.channel_id && existing.kind == kind));
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
            let engine = state.engine.lock().await;
            let snap = engine
                .order_book_snapshot_typed(&symbol, 20)
                .unwrap_or(OrderBookSnapshot {
                    symbol: symbol.clone(),
                    exchange: "SIM".to_string(),
                    bids: vec![],
                    asks: vec![],
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as i64,
                    sequence: Some(0),
                    is_snapshot: Some(true),
                });
            drop(engine);
            if let Ok(payload) = codec::encode_cbor(&snap) {
                return vec![stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "marketdata/book",
                )];
            }
            vec![ack_subscribe(frame)]
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
            let batch = md.query_trades(&symbol, None, None, Some(50), None);
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
        SubscriptionKind::AggTrades { symbol } => {
            let md = state.market_data.lock().await;
            let batch = md.query_agg_trades(&symbol, None, None, Some(50), None);
            drop(md);
            let mut frames = Vec::new();
            for trade in batch.trades {
                if let Ok(payload) = codec::encode_cbor(&AggregateTradeEvent { trade }) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/aggtrades",
                    ));
                }
            }
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
        SubscriptionKind::MiniTicker { symbol } => {
            let md = state.market_data.lock().await;
            let mut frames = Vec::new();
            if let Some(sym) = symbol {
                let mut mini = md.mini_ticker(&sym);
                mini.is_snapshot = Some(true);
                if let Ok(payload) = codec::encode_cbor(&mini) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/miniticker",
                    ));
                }
            } else {
                for mut mini in md.all_mini_tickers().tickers {
                    mini.is_snapshot = Some(true);
                    if let Ok(payload) = codec::encode_cbor(&mini) {
                        frames.push(stream_item(
                            frame.channel_id,
                            &routing_key,
                            payload,
                            "marketdata/ticker/all",
                        ));
                    }
                }
            }
            drop(md);
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
        SubscriptionKind::MarkPrice { symbol } => {
            let md = state.market_data.lock().await;
            let mut mark = md.mark_price(&symbol);
            mark.is_snapshot = Some(true);
            drop(md);
            if let Ok(payload) = codec::encode_cbor(&mark) {
                return vec![stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "marketdata/mark",
                )];
            }
            vec![ack_subscribe(frame)]
        }
        SubscriptionKind::Liquidations => {
            let md = state.market_data.lock().await;
            let liqs = md.recent_liquidations(20);
            drop(md);
            let mut frames = Vec::new();
            for trade in liqs {
                if let Ok(payload) = codec::encode_cbor(&LiquidationTradeEvent { trade }) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "marketdata/liquidations",
                    ));
                }
            }
            if frames.is_empty() {
                frames.push(ack_subscribe(frame));
            }
            frames
        }
    }
}

pub async fn handle_account_subscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    if let Some(code) = validate_interaction(&frame) {
        return vec![make_error_frame(frame.channel_id, frame.stream_seq, code)];
    }

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

    state.account_subscriptions.lock().await.retain(|existing| {
        !(existing.channel_id == frame.channel_id
            && existing.account == account
            && existing.kind == kind)
    });
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
        AccountSubscriptionKind::Margin => {
            let update = acct.margin_update(true);
            if let Ok(payload) = codec::encode_cbor(&update) {
                frames.push(stream_item(
                    frame.channel_id,
                    &routing_key,
                    payload,
                    "accounts/margin",
                ));
            }
        }
        AccountSubscriptionKind::Liquidations => {
            for liq in &acct.liquidations {
                if let Ok(payload) = codec::encode_cbor(liq) {
                    frames.push(stream_item(
                        frame.channel_id,
                        &routing_key,
                        payload,
                        "accounts/liquidations",
                    ));
                }
            }
        }
        AccountSubscriptionKind::OrderLists => {}
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
    let mut public_liquidation: Option<LiquidationTrade> = None;
    {
        let mut md = state.market_data.lock().await;
        for fill in fills {
            if let Some(liq) = md.on_trade(symbol, fill.fill_price.0, fill.fill_qty.0, fill.side) {
                public_liquidation = Some(liq);
            }
        }
    }

    {
        let mut accounts = state.accounts.lock().await;
        let acct = accounts.get_or_create(account);
        let subs: Vec<AccountSubscription> = state.account_subscriptions.lock().await.clone();
        for report in reports {
            let (balance_update, position_update, user_liquidation) =
                acct.record_fill(report.clone());
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
                    AccountSubscriptionKind::Positions => {
                        if let Some(update) = &position_update {
                            if let Ok(payload) = codec::encode_cbor(update) {
                                frames.push(stream_item(
                                    sub.channel_id,
                                    &sub.routing_key,
                                    payload,
                                    "accounts/positions",
                                ));
                            }
                        }
                    }
                    AccountSubscriptionKind::Margin => {
                        let update = acct.margin_update(false);
                        if let Ok(payload) = codec::encode_cbor(&update) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "accounts/margin",
                            ));
                        }
                    }
                    AccountSubscriptionKind::Liquidations => {
                        if let Some(liq) = &user_liquidation {
                            if let Ok(payload) = codec::encode_cbor(liq) {
                                frames.push(stream_item(
                                    sub.channel_id,
                                    &sub.routing_key,
                                    payload,
                                    "accounts/liquidations",
                                ));
                            }
                        }
                    }
                    AccountSubscriptionKind::OrderLists => {
                        let status = order_list_status_from_report(account, report);
                        if let Ok(payload) = codec::encode_cbor(&status) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "trading/orderlists",
                            ));
                        }
                    }
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
            SubscriptionKind::Candles {
                symbol: sym,
                interval,
            } if sym == symbol => {
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
            SubscriptionKind::AggTrades { symbol: sym } if sym == symbol => {
                let md = state.market_data.lock().await;
                if let Some(trade) = md.last_agg_trade(symbol) {
                    if let Ok(payload) = codec::encode_cbor(&AggregateTradeEvent { trade }) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "marketdata/aggtrades",
                        ));
                    }
                }
            }
            SubscriptionKind::MiniTicker { symbol: sym_opt } => {
                let md = state.market_data.lock().await;
                if let Some(sym) = sym_opt {
                    if sym == symbol {
                        if let Ok(payload) = codec::encode_cbor(&md.mini_ticker(symbol)) {
                            frames.push(stream_item(
                                sub.channel_id,
                                &sub.routing_key,
                                payload,
                                "marketdata/ticker/all",
                            ));
                        }
                    }
                } else if let Ok(payload) = codec::encode_cbor(&md.mini_ticker(symbol)) {
                    frames.push(stream_item(
                        sub.channel_id,
                        &sub.routing_key,
                        payload,
                        "marketdata/ticker/all",
                    ));
                }
            }
            SubscriptionKind::MarkPrice { symbol: sym } if sym == symbol => {
                let md = state.market_data.lock().await;
                if let Ok(payload) = codec::encode_cbor(&md.mark_price(symbol)) {
                    frames.push(stream_item(
                        sub.channel_id,
                        &sub.routing_key,
                        payload,
                        "marketdata/mark",
                    ));
                }
            }
            SubscriptionKind::Liquidations => {
                if let Some(trade) = &public_liquidation {
                    if let Ok(payload) = codec::encode_cbor(&LiquidationTradeEvent {
                        trade: trade.clone(),
                    }) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "marketdata/liquidations",
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

/// Fan out execution reports to execution/order-list subscribers (no fill side-effects).
pub async fn post_execution_reports(
    state: &Arc<ExchangeState>,
    account: &str,
    reports: &[ExecutionReport],
) -> Vec<Frame> {
    let subs = state.account_subscriptions.lock().await.clone();
    let mut frames = Vec::new();
    for report in reports {
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
                AccountSubscriptionKind::OrderLists => {
                    let status = order_list_status_from_report(account, report);
                    if let Ok(payload) = codec::encode_cbor(&status) {
                        frames.push(stream_item(
                            sub.channel_id,
                            &sub.routing_key,
                            payload,
                            "trading/orderlists",
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    frames
}

fn order_list_status_from_report(account: &str, report: &ExecutionReport) -> OrderListStatus {
    let status = match report.exec_type {
        ExecType::New => OrderListStatusStatus::Executing,
        ExecType::Canceled => OrderListStatusStatus::AllDone,
        ExecType::Fill | ExecType::PartialFill if report.leaves_qty.0 <= f64::EPSILON => {
            OrderListStatusStatus::AllDone
        }
        _ => OrderListStatusStatus::Executing,
    };
    OrderListStatus {
        account: account.to_string(),
        list_id: report.cl_ord_id.clone(),
        status,
        symbol: Some(report.symbol.clone()),
    }
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
