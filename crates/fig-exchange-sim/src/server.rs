//! FIG Exchange Simulator — Server Library
//!
//! Provides the exchange server logic as a library, so integration
//! tests can start and control the server programmatically.

use std::sync::Arc;

use anyhow::Result;
use quinn::Endpoint;
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use uuid::Uuid;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{ControlSubtype, Frame, FrameDecoder, FrameType};
use fig_core::messages::*;
use fig_core::session::{FileSessionStore, MemorySessionStore, Session, SessionStore};
use fig_core::transport;

use crate::account_state::{AccountHub, AccountSubscription};
use crate::market_data::{MarketDataHub, StreamSubscription};
use crate::matching::MatchingEngine;

/// Well-known schema IDs for trading messages.
pub mod schema_id {
    pub const TRADING_ORDERS: u8 = 0x01;
}

/// Well-known channel paths.
pub mod paths {
    pub const ORDERS: &str = "trading/accounts/{account}/orders";
    pub const CANCEL: &str = "trading/accounts/{account}/orders/{order_id}/cancel";
    pub const REPLACE: &str = "trading/accounts/{account}/orders/{order_id}/replace";
    pub const EXECUTIONS: &str = "trading/accounts/{account}/executions";
    pub const MARKET_DATA: &str = "marketdata/{symbol}/quotes";
    pub const CANDLES: &str = "marketdata/{symbol}/candles/{interval}";
    pub const TRADES: &str = "marketdata/{symbol}/trades";
    pub const BBO: &str = "marketdata/{symbol}/bbo";
    pub const BALANCES: &str = "accounts/{account}/balances";
    pub const POSITIONS: &str = "accounts/{account}/positions";
    pub const MARGIN: &str = "accounts/{account}/margin";
    pub const FILLS: &str = "trading/accounts/{account}/fills";
    pub const ACCOUNT: &str = "accounts/{account}";
}

pub struct ExchangeState {
    pub engine: Mutex<MatchingEngine>,
    pub sessions: MemorySessionStore,
    pub file_sessions: FileSessionStore,
    pub subscriptions: Mutex<Vec<StreamSubscription>>,
    pub account_subscriptions: Mutex<Vec<AccountSubscription>>,
    pub market_data: Mutex<MarketDataHub>,
    pub accounts: Mutex<AccountHub>,
}

/// Start the FIG exchange server on the given address.
///
/// Spawns connection accept loop and connection handlers in the background.
/// Returns a handle to the endpoint so callers can discover the bound address
/// via `endpoint.local_addr()`.
pub async fn run_server(addr: &str) -> anyhow::Result<Arc<Endpoint>> {
    // Initialize tracing if not already done
    tracing::info!("FIG Exchange Simulator starting...");

    // Generate self-signed TLS certificate using fig-core transport
    let (cert, key) = transport::generate_self_signed_cert()
        .map_err(|e| anyhow::anyhow!("Failed to generate cert: {}", e))?;

    // Configure TREE server using fig-core transport
    let server_config = if std::env::var("FIG_MTLS").ok().as_deref() == Some("1") {
        transport::server_config_mtls(cert, key)
            .map_err(|e| anyhow::anyhow!("Failed to create mTLS server config: {}", e))?
    } else {
        transport::server_config(cert, key)
            .map_err(|e| anyhow::anyhow!("Failed to create server config: {}", e))?
    };

    // Bind to UDP socket
    let addr: std::net::SocketAddr = addr.parse()?;
    let endpoint = Arc::new(Endpoint::server(server_config, addr)?);
    info!("FIG server listening on {}", endpoint.local_addr()?);

    let state = Arc::new(ExchangeState {
        engine: Mutex::new(MatchingEngine::new()),
        sessions: MemorySessionStore::new(),
        file_sessions: FileSessionStore::new(std::env::temp_dir().join("fig-exchange-sessions")),
        subscriptions: Mutex::new(Vec::new()),
        account_subscriptions: Mutex::new(Vec::new()),
        market_data: Mutex::new(MarketDataHub::default()),
        accounts: Mutex::new(AccountHub::default()),
    });

    info!("Waiting for connections...");

    // Accept connections in the background
    let ep = endpoint.clone();
    tokio::spawn(async move {
        while let Some(incoming) = ep.accept().await {
            let conn = match incoming.await {
                Ok(c) => c,
                Err(e) => {
                    error!("Incoming connection error: {}", e);
                    continue;
                }
            };
            let remote = conn.remote_address();
            info!("Connection from {}", remote);

            let state = state.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(conn, state).await {
                    error!("Connection error: {}", e);
                }
            });
        }
    });

    Ok(endpoint)
}

pub async fn handle_connection(conn: quinn::Connection, state: Arc<ExchangeState>) -> Result<()> {
    // Create or restore a session for this connection.
    // TODO: extract resumption token from TREE transport parameters
    // when the transport layer supports token embedding.
    let session = Session::new();
    let session_id = session.session_id;
    info!("Session created: {}", session_id);

    // Persist the session to the file store.
    if let Err(e) = state.file_sessions.put(&session) {
        warn!("Failed to persist session {}: {}", session_id, e);
    }

    // Accept bidirectional streams
    loop {
        let (send, recv) = match conn.accept_bi().await {
            Ok(streams) => streams,
            Err(quinn::ConnectionError::ApplicationClosed(_)) => {
                info!("Connection closed by peer");
                break;
            }
            Err(e) => {
                warn!("Stream error: {}", e);
                break;
            }
        };

        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_stream(send, recv, state).await {
                error!("Stream error: {}", e);
            }
        });
    }

    // Persist the session before disconnecting.
    info!("Connection ended, persisting session {}", session_id);
    let mut final_session = session;
    final_session.touch();
    if let Err(e) = state.file_sessions.update(&final_session) {
        warn!("Failed to update session {}: {}", session_id, e);
    }

    Ok(())
}

pub async fn handle_stream(
    mut send: quinn::SendStream,
    mut recv: quinn::RecvStream,
    state: Arc<ExchangeState>,
) -> Result<()> {
    let mut decoder = FrameDecoder::new();

    // Read frames from the stream
    let mut buf = vec![0u8; 4096];
    loop {
        let n = match recv.read(&mut buf).await {
            Ok(Some(n)) => n,
            Ok(None) => {
                info!("Stream closed by peer");
                break;
            }
            Err(e) => {
                warn!("Read error: {}", e);
                break;
            }
        };

        decoder.feed(&buf[..n]);

        // Process all complete frames
        while let Some(result) = decoder.decode_next() {
            let frame = match result {
                Ok(f) => f,
                Err(e) => {
                    error!("Frame decode error: {}", e);
                    continue;
                }
            };

            info!("Received: {}", frame);

            let responses = handle_frame(frame, &state).await;
            for resp_frame in responses {
                let encoded = resp_frame.encode()?;
                send.write_all(&encoded).await?;
            }
        }
    }

    send.finish()?;
    Ok(())
}

pub async fn handle_frame(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    match frame.frame_type {
        FrameType::Control => handle_control(frame),
        FrameType::Request => handle_request(frame, state).await,
        FrameType::Subscribe => handle_subscribe(frame, state).await,
        _ => {
            warn!("Unhandled frame type: {:?}", frame.frame_type);
            vec![make_error_frame(
                frame.channel_id,
                frame.stream_seq,
                "UNSUPPORTED_FRAME_TYPE",
            )]
        }
    }
}

pub fn handle_control(frame: Frame) -> Vec<Frame> {
    if frame.payload.is_empty() {
        return vec![];
    }

    let subtype = frame.payload[0];
    match ControlSubtype::from_code(subtype) {
        Some(ControlSubtype::Ping) => {
            info!("PING received on channel {}", frame.channel_id);
            let pong = Frame::pong();
            vec![pong]
        }
        Some(ControlSubtype::Settings) => {
            info!("SETTINGS received");
            // Acknowledge with a SETTINGS response
            let settings = Frame::control(ControlSubtype::Settings).with_seq(0);
            vec![settings]
        }
        _ => {
            warn!("Unhandled control subtype: 0x{:02x}", subtype);
            vec![]
        }
    }
}

pub async fn handle_request(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let channel_path = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");
    let method = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::Method)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");

    let is_query = method.eq_ignore_ascii_case("GET")
        || channel_path.contains("/candles/")
        || channel_path.contains("/ticker")
        || (channel_path.contains("/trades") && !channel_path.contains("/orders"))
        || channel_path.ends_with("/fills")
        || channel_path.ends_with("/funding")
        || channel_path.ends_with("/ledger")
        || channel_path.ends_with("/margin")
        || (channel_path.starts_with("accounts/")
            && !channel_path.contains("/orders")
            && channel_path.matches('/').count() == 1);

    if is_query {
        return crate::broker_api::handle_query_request(frame, state).await;
    }

    match frame.schema_id {
        schema_id::TRADING_ORDERS => handle_trading_request(frame, state).await,
        _ => handle_generic_request(frame, state).await,
    }
}

pub async fn handle_trading_request(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    // Find the channel path to determine the request type
    let channel_path = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string();

    let method = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::Method)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string();

    info!("Trading request: {} {}", method, channel_path);

    // Decode the payload as CBOR
    let result = if channel_path.contains("/orders")
        && !channel_path.contains("/cancel")
        && !channel_path.contains("/replace")
    {
        // New order
        match codec::decode_new_order_single_frame(&frame) {
            Ok(order) => {
                info!(
                    "NewOrderSingle: {} {:?} {} @ {:?}",
                    order.cl_ord_id,
                    order.side,
                    order.symbol,
                    order.price.as_ref()
                );

                let mut engine = state.engine.lock().await;
                let result = engine.process_new_order(&order);
                drop(engine);

                let account = order.account.as_deref().unwrap_or("DEMO-ACCT");

                // Build execution report(s)
                let mut responses = Vec::new();
                let mut reports = Vec::new();
                for fill in &result.fills {
                    let report = ExecutionReport {
                        cl_ord_id: order.cl_ord_id.clone(),
                        order_id: fill.fill_id.clone(),
                        exec_id: format!("EX-{}", Uuid::new_v4()),
                        exec_type: fill.exec_type,
                        ord_status: fill.ord_status,
                        side: fill.side,
                        last_qty: Some(fill.fill_qty.clone()),
                        last_price: Some(fill.fill_price.clone()),
                        leaves_qty: fill.leaves_qty.clone(),
                        cum_qty: fill.cum_qty.clone(),
                        avg_price: fill.avg_price.clone(),
                        symbol: fill.symbol.clone(),
                        transact_time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64,
                    };
                    reports.push(report.clone());

                    if let Ok(payload) = codec::encode_cbor(&report) {
                        responses.push(
                            Frame::new(FrameType::StreamItem, frame.channel_id)
                                .with_seq(frame.stream_seq)
                                .with_schema_id(schema_id::TRADING_ORDERS)
                                .with_extension(Extension::text(
                                    ExtensionTag::ChannelPath,
                                    paths::EXECUTIONS,
                                ))
                                .with_payload(payload),
                        );
                    }
                }

                if let Some(reject) = &result.reject_reason {
                    responses.push(make_error_frame(frame.channel_id, frame.stream_seq, reject));
                }

                if !result.fills.is_empty() {
                    responses.extend(
                        crate::broker_api::post_fill_updates(
                            state,
                            &order.symbol,
                            account,
                            &result.fills,
                            &reports,
                        )
                        .await,
                    );
                } else {
                    responses.extend(push_book_depth(state, &order.symbol).await);
                }

                responses
            }
            Err(e) => {
                error!("Failed to decode NewOrderSingle: {}", e);
                vec![make_error_frame(
                    frame.channel_id,
                    frame.stream_seq,
                    "DECODE_ERROR",
                )]
            }
        }
    } else if channel_path.contains("/cancel") {
        // Cancel request
        match codec::decode_cbor::<CancelRequest>(&frame.payload) {
            Ok(cancel) => {
                info!("CancelRequest: {}", cancel.orig_cl_ord_id);
                let mut engine = state.engine.lock().await;
                use crate::matching::CancelOutcome;
                let outcome = engine.process_cancel(&cancel);

                let response = match outcome {
                    CancelOutcome::Cancelled(_order) => {
                        let report = ExecutionReport {
                            cl_ord_id: cancel.cl_ord_id.clone(),
                            order_id: format!("OX-{}", cancel.orig_cl_ord_id),
                            exec_id: format!("EX-{}", Uuid::new_v4()),
                            exec_type: ExecType::Canceled,
                            ord_status: OrdStatus::Canceled,
                            side: cancel.side,
                            last_qty: None,
                            last_price: None,
                            leaves_qty: Quantity(0.0),
                            cum_qty: Quantity(0.0),
                            avg_price: Price(0.0),
                            symbol: cancel.symbol.clone(),
                            transact_time: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_nanos() as i64,
                        };
                        if let Ok(payload) = codec::encode_cbor(&report) {
                            Frame::new(FrameType::Response, frame.channel_id)
                                .with_seq(frame.stream_seq)
                                .with_schema_id(schema_id::TRADING_ORDERS)
                                .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
                                .with_payload(payload)
                        } else {
                            make_error_frame(frame.channel_id, frame.stream_seq, "ENCODE_ERROR")
                        }
                    }
                    CancelOutcome::Rejected(reason) => {
                        let reject = CancelReject {
                            cl_ord_id: cancel.cl_ord_id.clone(),
                            orig_cl_ord_id: cancel.orig_cl_ord_id.clone(),
                            reject_reason: reason,
                            symbol: cancel.symbol.clone(),
                        };
                        if let Ok(payload) = codec::encode_cbor(&reject) {
                            Frame::new(FrameType::Response, frame.channel_id)
                                .with_seq(frame.stream_seq)
                                .with_schema_id(schema_id::TRADING_ORDERS)
                                .with_extension(Extension::u16(ExtensionTag::StatusCode, 409))
                                .with_payload(payload)
                        } else {
                            make_error_frame(frame.channel_id, frame.stream_seq, "ENCODE_ERROR")
                        }
                    }
                };
                let mut responses = vec![response];
                responses.extend(push_book_depth(state, &cancel.symbol).await);
                responses
            }
            Err(e) => {
                error!("Failed to decode CancelRequest: {}", e);
                vec![make_error_frame(
                    frame.channel_id,
                    frame.stream_seq,
                    "DECODE_ERROR",
                )]
            }
        }
    } else if channel_path.contains("/replace") {
        match codec::decode_cbor::<CancelReplaceRequest>(&frame.payload) {
            Ok(replace) => {
                info!(
                    "CancelReplaceRequest: {} -> {}",
                    replace.orig_cl_ord_id, replace.cl_ord_id
                );
                let mut engine = state.engine.lock().await;
                let result = engine.process_replace(&replace);
                drop(engine);

                let mut responses = Vec::new();
                for fill in &result.fills {
                    let report = ExecutionReport {
                        cl_ord_id: replace.cl_ord_id.clone(),
                        order_id: fill.fill_id.clone(),
                        exec_id: format!("EX-{}", Uuid::new_v4()),
                        exec_type: fill.exec_type,
                        ord_status: fill.ord_status,
                        side: fill.side,
                        last_qty: Some(fill.fill_qty.clone()),
                        last_price: Some(fill.fill_price.clone()),
                        leaves_qty: fill.leaves_qty.clone(),
                        cum_qty: fill.cum_qty.clone(),
                        avg_price: fill.avg_price.clone(),
                        symbol: fill.symbol.clone(),
                        transact_time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as i64,
                    };
                    if let Ok(payload) = codec::encode_cbor(&report) {
                        responses.push(
                            Frame::new(FrameType::StreamItem, frame.channel_id)
                                .with_seq(frame.stream_seq)
                                .with_schema_id(schema_id::TRADING_ORDERS)
                                .with_extension(Extension::text(
                                    ExtensionTag::ChannelPath,
                                    paths::EXECUTIONS,
                                ))
                                .with_payload(payload),
                        );
                    }
                }
                if let Some(reject) = &result.reject_reason {
                    responses.push(make_error_frame(frame.channel_id, frame.stream_seq, reject));
                }
                responses.extend(push_book_depth(state, &replace.symbol).await);
                responses
            }
            Err(e) => {
                error!("Failed to decode CancelReplaceRequest: {}", e);
                vec![make_error_frame(
                    frame.channel_id,
                    frame.stream_seq,
                    "DECODE_ERROR",
                )]
            }
        }
    } else {
        vec![make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "UNKNOWN_PATH",
        )]
    };

    result
}

pub async fn handle_generic_request(frame: Frame, _state: &Arc<ExchangeState>) -> Vec<Frame> {
    // Try to decode as CBOR and echo back
    let response = Frame::new(FrameType::Response, frame.channel_id)
        .with_seq(frame.stream_seq)
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(frame.payload.clone());
    vec![response]
}

pub async fn handle_subscribe(frame: Frame, state: &Arc<ExchangeState>) -> Vec<Frame> {
    let routing_key = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::RoutingKey)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string();
    let channel_path = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string();

    info!("Subscribe to: {} path={}", routing_key, channel_path);

    if channel_path.contains("/executions")
        || channel_path.contains("/balances")
        || channel_path.contains("/positions")
        || channel_path.ends_with("/funding")
        || channel_path.ends_with("/ledger")
    {
        return crate::broker_api::handle_account_subscribe(frame, state).await;
    }

    if routing_key.contains("marketdata")
        || routing_key.contains("quotes")
        || routing_key.contains("candles")
        || routing_key.contains("trades")
        || routing_key.contains("ticker")
        || channel_path.starts_with("marketdata/")
    {
        return crate::broker_api::handle_market_subscribe(frame, state).await;
    }

    vec![Frame::new(FrameType::Response, frame.channel_id)
        .with_seq(frame.stream_seq)
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))]
}

/// Build incremental market data STREAM_ITEM frames for all subscribers of a symbol.
pub async fn build_market_data_push(state: &Arc<ExchangeState>, symbol: &str) -> Vec<Frame> {
    use crate::market_data::SubscriptionKind;

    let subs: Vec<StreamSubscription> = state
        .subscriptions
        .lock()
        .await
        .iter()
        .filter(|s| match &s.kind {
            SubscriptionKind::Quotes { symbol: sym } => sym == symbol,
            _ => false,
        })
        .cloned()
        .collect();

    if subs.is_empty() {
        return Vec::new();
    }

    let engine = state.engine.lock().await;
    let Some(book) = engine.get_book(&symbol.to_string()) else {
        return Vec::new();
    };

    let snapshot = MarketDataSnapshot {
        symbol: symbol.to_string(),
        exchange: "SIM".to_string(),
        bids: book.bid_depth(5),
        asks: book.ask_depth(5),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64,
    };

    drop(engine);

    let Ok(payload) = codec::encode_cbor(&snapshot) else {
        return Vec::new();
    };

    subs.into_iter()
        .map(|sub| {
            Frame::new(FrameType::StreamItem, sub.channel_id)
                .with_schema_id(schema_id::TRADING_ORDERS)
                .with_extension(Extension::text(ExtensionTag::RoutingKey, &sub.routing_key))
                .with_extension(Extension::text(
                    ExtensionTag::ContentType,
                    "application/cbor",
                ))
                .with_payload(payload.clone())
        })
        .collect()
}

/// Push order book depth snapshots to all subscribers when the book changes.
pub async fn push_book_depth(state: &Arc<ExchangeState>, symbol: &str) -> Vec<Frame> {
    build_market_data_push(state, symbol).await
}

pub fn make_error_frame(channel_id: u16, stream_seq: u32, message: &str) -> Frame {
    Frame::new(FrameType::StreamError, channel_id)
        .with_seq(stream_seq)
        .with_extension(Extension::text(ExtensionTag::ErrorCode, "SERVER_ERROR"))
        .with_extension(Extension::text(ExtensionTag::ErrorMessage, message))
}
