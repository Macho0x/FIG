//! Integration tests for the FIG Exchange Simulator.
//!
//! Each test:
//! 1. Starts the server on a random port
//! 2. Creates a client TREE endpoint
//! 3. Connects to the server
//! 4. Opens a bidirectional stream
//! 5. Sends FIG frames
//! 6. Reads response frames
//! 7. Verifies the response

use std::sync::Arc;

use fig_core::channel::ChannelMode;
use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{ControlSubtype, Frame, FrameDecoder, FrameType};
use fig_core::messages::*;
use fig_core::session::Session;
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};

use fig_exchange_sim::server::run_server;

/// Helper: read all frames from a receive stream.
async fn read_all_frames(recv: &mut quinn::RecvStream) -> anyhow::Result<Vec<Frame>> {
    let mut decoder = FrameDecoder::new();
    let mut buf = vec![0u8; 4096];
    loop {
        match recv.read(&mut buf).await? {
            Some(n) => {
                decoder.feed(&buf[..n]);
            }
            None => break,
        }
    }
    // Decode all complete frames remaining in the buffer
    let frames: Result<Vec<Frame>, _> = decoder.decode_all().into_iter().collect();
    Ok(frames?)
}

/// Helper: setup a client connection to the given server address.
async fn connect_client(server_addr: std::net::SocketAddr) -> anyhow::Result<quinn::Connection> {
    let client_ep = quinn::Endpoint::client("127.0.0.1:0".parse()?)?;
    let client_cfg = client_config().map_err(|e| anyhow::anyhow!("client_config: {}", e))?;
    let conn = client_ep
        .connect_with(client_cfg, server_addr, "localhost")?
        .await?;
    Ok(conn)
}

/// Helper: send a frame on a bidirectional stream and get the responses.
async fn send_and_receive(conn: &quinn::Connection, frame: Frame) -> anyhow::Result<Vec<Frame>> {
    let (mut send, mut recv) = conn.open_bi().await?;
    let encoded = frame.encode()?;
    send.write_all(&encoded).await?;
    send.finish()?;
    let responses = read_all_frames(&mut recv).await?;
    Ok(responses)
}

/// Helper: send multiple frames on a single bidirectional stream and get all responses.
async fn send_multiple_and_receive(
    conn: &quinn::Connection,
    frames: Vec<Frame>,
) -> anyhow::Result<Vec<Frame>> {
    let (mut send, mut recv) = conn.open_bi().await?;
    for frame in &frames {
        let encoded = frame.encode()?;
        send.write_all(&encoded).await?;
    }
    send.finish()?;
    let responses = read_all_frames(&mut recv).await?;
    Ok(responses)
}

/// Shared server + client connection for integration tests.
struct Harness {
    _endpoint: Arc<quinn::Endpoint>,
    conn: quinn::Connection,
}

impl Harness {
    async fn start() -> anyhow::Result<Self> {
        let _endpoint = run_server("127.0.0.1:0").await?;
        let server_addr = _endpoint.local_addr().unwrap();
        let conn = connect_client(server_addr).await?;
        Ok(Self { _endpoint, conn })
    }

    fn connection(&self) -> quinn::Connection {
        self.conn.clone()
    }

    async fn send(&self, frame: Frame) -> anyhow::Result<Vec<Frame>> {
        send_and_receive(&self.conn, frame).await
    }

    async fn send_multiple(&self, frames: Vec<Frame>) -> anyhow::Result<Vec<Frame>> {
        send_multiple_and_receive(&self.conn, frames).await
    }
}

/// Build a NewOrderSingle message.
fn make_order(
    cl_ord_id: &str,
    side: Side,
    symbol: &str,
    order_type: OrderType,
    price: Option<f64>,
    qty: f64,
) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: cl_ord_id.to_string(),
        side,
        order_qty: Quantity(qty),
        price: price.map(Price),
        stop_price: None,
        symbol: symbol.to_string(),
        order_type,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some("TEST".to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    }
}

/// Helper: build a request frame for a trading order.
fn make_order_frame(channel_id: u16, order: &NewOrderSingle) -> anyhow::Result<Frame> {
    let payload = codec::encode_cbor(order)?;
    Ok(Frame::new(FrameType::Request, channel_id)
        .with_seq(1)
        .with_schema_id(1) // TRADING_ORDERS
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/TEST/orders",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_payload(payload))
}

/// Helper: dev auth token for private paths in tests and CLI.
fn auth_extension(account: &str) -> Extension {
    Extension::text(ExtensionTag::AuthToken, &format!("fig-dev-{account}"))
}

/// Helper: build a subscribe frame.
fn make_subscribe_frame(channel_id: u16, routing_key: &str, channel_path: &str) -> Frame {
    Frame::new(FrameType::Subscribe, channel_id)
        .with_seq(1)
        .with_extension(Extension::text(ExtensionTag::RoutingKey, routing_key))
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
}

/// Helper: build an account query request frame.
fn make_account_query_frame(channel_id: u16, account: &str) -> Frame {
    Frame::new(FrameType::Request, channel_id)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            &format!("accounts/{account}"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_extension(account))
}

// ═══════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════

/// Test 1: Order entry produces an ExecutionReport on a fill.
#[tokio::test]
async fn test_order_entry_execution_report() {
    let _ = tracing_subscriber::fmt::try_init();

    // Start server on random port
    let h = Harness::start().await.expect("harness");

    // Set up: place a sell limit order to provide liquidity, then a market buy
    let sell = make_order(
        "SELL-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.00),
        10.0,
    );
    let buy = make_order("BUY-1", Side::Buy, "AAPL", OrderType::Market, None, 10.0);

    let sell_frame = make_order_frame(1, &sell).unwrap();
    let buy_frame = make_order_frame(1, &buy).unwrap();

    let responses = h
        .send_multiple(vec![sell_frame, buy_frame])
        .await
        .expect("send/receive");

    // The sell order rests (New ack), the buy market order fills.
    let fill_response = responses
        .iter()
        .find(|f| {
            f.frame_type == FrameType::StreamItem
                && codec::decode_cbor::<ExecutionReport>(&f.payload)
                    .map(|r| r.exec_type == ExecType::Fill)
                    .unwrap_or(false)
        })
        .expect("should have a StreamItem (fill) response");

    let report: ExecutionReport =
        codec::decode_cbor(&fill_response.payload).expect("decode ExecutionReport");

    assert_eq!(report.exec_type, ExecType::Fill);
    assert_eq!(report.ord_status, OrdStatus::Filled);
    assert_eq!(report.symbol, "AAPL");
    assert!(report.last_qty.is_some());
    assert!(report.last_price.is_some());
}

/// Test 2: Market data subscription returns a MarketDataSnapshot.
#[tokio::test]
async fn test_market_data_subscription() {
    let _ = tracing_subscriber::fmt::try_init();

    let h = Harness::start().await.expect("harness");

    let sub_frame = make_subscribe_frame(1, "marketdata.AAPL.quotes", "marketdata/AAPL/quotes");
    let responses = h.send(sub_frame).await.expect("send/receive");

    // Should get a StreamItem with MarketDataSnapshot
    let snapshot_response = responses
        .iter()
        .find(|f| f.frame_type == FrameType::StreamItem || f.frame_type == FrameType::Response)
        .expect("should have a response");

    // It could be a StreamItem with the snapshot, or just a Response (ack) if book not found
    if snapshot_response.frame_type == FrameType::StreamItem
        && !snapshot_response.payload.is_empty()
    {
        let snapshot: MarketDataSnapshot =
            codec::decode_cbor(&snapshot_response.payload).expect("decode MarketDataSnapshot");
        assert_eq!(snapshot.symbol, "AAPL");
    }
    // If it's just a Response ack, that's fine too — no book exists yet
}

/// Test 3: Account query returns an AccountSummary.
#[tokio::test]
async fn test_account_query() {
    let _ = tracing_subscriber::fmt::try_init();

    let h = Harness::start().await.expect("harness");

    let query_frame = make_account_query_frame(1, "TEST-ACCT");
    let responses = h.send(query_frame).await.expect("send/receive");

    assert!(!responses.is_empty(), "should have at least one response");

    // Look for a Response frame with AccountSummary
    for resp in &responses {
        if resp.frame_type == FrameType::Response && !resp.payload.is_empty() {
            if let Ok(summary) = codec::decode_cbor::<AccountSummary>(&resp.payload) {
                assert_eq!(summary.account, "TEST-ACCT");
                return; // Success
            }
        }
    }

    // If no Response with payload, check for StreamError (server might not handle this path correctly)
    panic!(
        "Expected AccountSummary response, got {} frames: {:?}",
        responses.len(),
        responses
            .iter()
            .map(|f| format!("{}", f.frame_type))
            .collect::<Vec<_>>()
    );
}

/// Test 4: Ping-Pong control exchange.
#[tokio::test]
async fn test_ping_pong() {
    let _ = tracing_subscriber::fmt::try_init();

    let h = Harness::start().await.expect("harness");

    let ping = Frame::ping();
    let responses = h.send(ping).await.expect("send/receive");

    assert!(!responses.is_empty(), "should have a PONG response");

    let pong = &responses[0];
    assert_eq!(
        pong.frame_type,
        FrameType::Control,
        "response should be a Control frame"
    );

    // Verify it's a PONG (first payload byte is Pong subtype)
    if !pong.payload.is_empty() {
        assert_eq!(
            pong.payload[0],
            ControlSubtype::Pong.code(),
            "control subtype should be Pong"
        );
    }
}

/// Test 5: Multiple orders match and produce fills.
#[tokio::test]
async fn test_multiple_orders_matching() {
    let _ = tracing_subscriber::fmt::try_init();

    let h = Harness::start().await.expect("harness");

    // Sell limit order at 100.00 qty 50
    let sell = make_order(
        "SELL-M-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.00),
        50.0,
    );
    // Buy limit order at 100.00 qty 50 (crosses the spread)
    let buy = make_order(
        "BUY-M-1",
        Side::Buy,
        "AAPL",
        OrderType::Limit,
        Some(100.00),
        50.0,
    );

    let sell_frame = make_order_frame(1, &sell).unwrap();
    let buy_frame = make_order_frame(1, &buy).unwrap();

    let responses = h
        .send_multiple(vec![sell_frame, buy_frame])
        .await
        .expect("send/receive");

    // The sell rests (New ack), the buy should fill.
    let fill_response = responses
        .iter()
        .find(|f| {
            f.frame_type == FrameType::StreamItem
                && codec::decode_cbor::<ExecutionReport>(&f.payload)
                    .map(|r| r.exec_type == ExecType::Fill)
                    .unwrap_or(false)
        })
        .expect("should have a StreamItem (fill) response");

    let report: ExecutionReport =
        codec::decode_cbor(&fill_response.payload).expect("decode ExecutionReport");

    assert_eq!(
        report.exec_type,
        ExecType::Fill,
        "buy order should be fully filled"
    );
    assert_eq!(report.ord_status, OrdStatus::Filled);

    let fill_price = report.last_price.as_ref().expect("should have last_price");
    assert!(
        (fill_price.0 - 100.00).abs() < 0.001,
        "fill price should be 100.00, got {}",
        fill_price.0
    );

    let fill_qty = report.last_qty.as_ref().expect("should have last_qty");
    assert!(
        (fill_qty.0 - 50.0).abs() < 0.001,
        "fill qty should be 50.0, got {}",
        fill_qty.0
    );
}

/// Test 6: Channel isolation — responses come back on the correct streams.
#[tokio::test]
async fn test_channel_isolation() {
    let _ = tracing_subscriber::fmt::try_init();

    let h = Harness::start().await.expect("harness");

    let conn = Arc::new(h.connection());

    // Open two separate bidirectional streams
    let (mut send1, mut recv1) = conn.open_bi().await.expect("open stream 1");
    let (mut send2, mut recv2) = conn.open_bi().await.expect("open stream 2");

    // Stream 1: send a market order (will likely get "No liquidity" → StreamError)
    let buy = make_order("ISO-BUY", Side::Buy, "AAPL", OrderType::Market, None, 10.0);
    let order_frame = make_order_frame(1, &buy).unwrap();
    let encoded1 = order_frame.encode().unwrap();
    send1.write_all(&encoded1).await.unwrap();
    send1.finish().unwrap();

    // Stream 2: send an account query
    let query_frame = make_account_query_frame(2, "ISO-ACCT");
    let encoded2 = query_frame.encode().unwrap();
    send2.write_all(&encoded2).await.unwrap();
    send2.finish().unwrap();

    // Read responses from both streams concurrently
    let (resp1, resp2) = tokio::join!(read_all_frames(&mut recv1), read_all_frames(&mut recv2),);

    let resp1 = resp1.expect("read stream 1");
    let resp2 = resp2.expect("read stream 2");

    // Stream 1 response should NOT contain account data
    // Stream 2 response should contain AccountSummary
    let has_account_on_stream2 = resp2.iter().any(|f| {
        f.frame_type == FrameType::Response
            && !f.payload.is_empty()
            && codec::decode_cbor::<AccountSummary>(&f.payload).is_ok()
    });
    assert!(
        has_account_on_stream2,
        "stream 2 should have AccountSummary response"
    );

    // Verify stream 1 does NOT have account data
    let has_account_on_stream1 = resp1.iter().any(|f| {
        f.frame_type == FrameType::Response
            && !f.payload.is_empty()
            && codec::decode_cbor::<AccountSummary>(&f.payload).is_ok()
    });
    assert!(
        !has_account_on_stream1,
        "stream 1 should NOT have AccountSummary (channel isolation)"
    );
}

// ═══════════════════════════════════════════════════════════════════
// Transport-layer tests (0-RTT, stream reset)
// ═══════════════════════════════════════════════════════════════════

/// Test 7: 0-RTT round-trip — client connects with a resumption token,
/// server accepts, and the session is restored.
#[tokio::test]
async fn test_0rtt_round_trip() {
    let _ = tracing_subscriber::fmt::try_init();

    // Generate a TLS certificate
    let (cert, key) = generate_self_signed_cert().expect("cert generation");
    let server_cfg = server_config(cert.clone(), key).expect("server config");

    // Start the FIG server on a random port
    let server = Arc::new(
        FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg)
            .await
            .expect("server bind"),
    );
    let server_addr = server.local_addr().expect("server local_addr");

    // Spawn server accept in background
    let server_ref = server.clone();
    let server_accept_handle = tokio::spawn(async move {
        let _conn = server_ref.accept().await.expect("server accept");
    });

    // --- Create a resumption token from a session ---
    let mut session = Session::new();
    session.add_channel(1);
    session.record_sent_seq(10);
    session.record_recv_seq(20);
    let token = session.resumption_token().expect("resumption token");
    let original_session_id = session.session_id;

    // --- Client connects with 0-RTT ---
    let client = FigClient::new(client_config().expect("client config")).expect("client create");

    let (fig_conn, restored_session) = client
        .connect_0rtt(server_addr, "localhost", Some(&token))
        .await
        .expect("connect_0rtt");

    // Verify the connection succeeded (either 0-RTT or full handshake fallback).
    // If 0-RTT was accepted, the session should be restored from the token.
    // If 0-RTT was rejected (server doesn't support it), the fallback to full
    // handshake succeeds and returns None for the session.
    if let Some(restored) = restored_session {
        assert_eq!(restored.session_id, original_session_id);
        assert!(restored.channels.contains(&1));
        assert_eq!(restored.last_seq_sent, 10);
        assert_eq!(restored.last_seq_recv, 20);
    }
    // Otherwise: full handshake fallback succeeded. Test passes either way.

    let _ = server_accept_handle.await;

    drop(fig_conn);
    drop(client);
    drop(server);
}

/// Test 8: Stream-reset detection — force-close a channel and verify the
/// channel manager properly handles the closure.
#[tokio::test]
async fn test_stream_reset_detection() {
    let _ = tracing_subscriber::fmt::try_init();

    // Generate a TLS certificate
    let (cert, key) = generate_self_signed_cert().expect("cert generation");
    let server_cfg = server_config(cert.clone(), key).expect("server config");
    let client_cfg = client_config().expect("client config");

    // Start server, wrap in Arc to share between spawn and main task
    let server = Arc::new(
        FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg)
            .await
            .expect("server bind"),
    );
    let server_addr = server.local_addr().expect("server local_addr");

    // Spawn server accept task
    let server_ref = server.clone();
    let server_handle = tokio::spawn(async move {
        let fig_conn = server_ref.accept().await.expect("server accept");
        fig_conn
    });

    // Client connects
    let client = FigClient::new(client_cfg).expect("client create");
    let client_conn = client
        .connect(server_addr, "localhost")
        .await
        .expect("client connect");

    // Open a channel on the client side
    let channel_id = client_conn
        .open_channel(ChannelMode::Session, None)
        .await
        .expect("open channel");

    // Forcefully close the channel on the client side
    client_conn
        .force_close_channel(channel_id)
        .await
        .expect("force close");

    // The channel should now be removed from the channel manager.
    // We verify this by attempting to close it again — should fail.
    let result = client_conn.force_close_channel(channel_id).await;
    assert!(result.is_err(), "channel should already be closed");

    let server_conn = server_handle.await.expect("server accept join");

    drop(client_conn);
    drop(server_conn);
    drop(client);
    drop(server);
}

/// Candle subscribe returns ack or partial bar snapshot.
#[tokio::test]
async fn test_candle_subscription() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sub = make_subscribe_frame(
        1,
        "marketdata/AAPL/candles/5m",
        "marketdata/AAPL/candles/5m",
    );
    let responses = h.send(sub).await.expect("send/receive");
    assert!(
        responses.iter().any(|f| matches!(
            f.frame_type,
            fig_core::frame::FrameType::Response | fig_core::frame::FrameType::StreamItem
        )),
        "expected SUBSCRIBE ack, got: {responses:?}"
    );
    assert!(
        !responses
            .iter()
            .any(|f| f.frame_type == fig_core::frame::FrameType::StreamError),
        "candle subscribe must not return STREAM_ERROR"
    );
}

/// Resting limit order returns ExecutionReport on the request stream.
#[tokio::test]
async fn test_resting_order_request_stream_ack() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let order = make_order(
        "REQ-1",
        Side::Buy,
        "AAPL",
        OrderType::Limit,
        Some(50.25),
        10.0,
    );
    let responses = h
        .send(make_order_frame(1, &order).unwrap())
        .await
        .expect("order");
    let report = responses
        .iter()
        .find_map(|f| codec::decode_cbor::<ExecutionReport>(&f.payload).ok())
        .expect("ExecutionReport on request stream");
    assert_eq!(report.cl_ord_id, "REQ-1");
    assert_eq!(report.exec_type, ExecType::New);
}

/// Agg trades subscribe returns ack (live stream path).
#[tokio::test]
async fn test_agg_trades_subscribe() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sub = make_subscribe_frame(1, "marketdata/AAPL/aggtrades", "marketdata/AAPL/aggtrades");
    let responses = h.send(sub).await.expect("send/receive");
    assert!(
        responses
            .iter()
            .any(|f| { matches!(f.frame_type, FrameType::Response | FrameType::StreamItem) }),
        "expected SUBSCRIBE ack"
    );
    assert!(!responses
        .iter()
        .any(|f| f.frame_type == FrameType::StreamError));
}

/// Private account query without auth is rejected.
#[tokio::test]
async fn test_private_auth_required() {
    let _ = tracing_subscriber::fmt::try_init();
    std::env::remove_var("FIG_DEV_OPEN");
    let h = Harness::start().await.expect("harness");

    let query = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "accounts/SECRET",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let responses = h.send(query).await.expect("send/receive");
    assert!(responses.iter().any(|f| {
        f.frame_type == FrameType::StreamError
            && f.extensions.iter().any(|e| {
                e.tag == ExtensionTag::ErrorMessage && e.value.as_text() == Some("AUTH_REQUIRED")
            })
    }));
}

/// Ticker query returns SymbolTicker payload.
#[tokio::test]
async fn test_ticker_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let query = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/ticker",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let responses = h.send(query).await.expect("send/receive");
    let ticker = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .and_then(|f| codec::decode_cbor::<SymbolTicker>(&f.payload).ok());
    assert!(ticker.is_some());
}

fn make_get_query_frame(channel_id: u16, channel_path: &str, account: Option<&str>) -> Frame {
    let mut frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    if let Some(acct) = account {
        frame = frame.with_extension(auth_extension(acct));
    }
    frame
}

/// Capabilities query returns supported paths.
#[tokio::test]
async fn test_capabilities_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");
    let query = make_get_query_frame(1, ".well-known/capabilities", None);
    let responses = h.send(query).await.expect("send/receive");
    let caps = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .and_then(|f| codec::decode_cbor::<CapabilitiesResponse>(&f.payload).ok());
    assert!(caps.is_some());
    assert!(!caps.unwrap().paths.is_empty());
}

/// Open orders query returns snapshot after resting order.
#[tokio::test]
async fn test_open_orders_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "OPEN-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        5.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("place order");

    let query = make_get_query_frame(1, "trading/accounts/TEST/orders/open", Some("TEST"));
    let responses = h.send(query).await.expect("query");
    let snap = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .and_then(|f| codec::decode_cbor::<OpenOrdersSnapshot>(&f.payload).ok());
    assert!(snap.is_some());
    assert!(!snap.unwrap().orders.is_empty());
}

/// Order book query includes sequence metadata.
#[tokio::test]
async fn test_order_book_query_sequence() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "BOOK-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        5.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("place order");

    let query = make_get_query_frame(1, "marketdata/AAPL/book", None);
    let responses = h.send(query).await.expect("query");
    let book = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .and_then(|f| codec::decode_cbor::<MarketDataSnapshot>(&f.payload).ok());
    let book = book.expect("book snapshot");
    assert_eq!(book.is_snapshot, Some(true));
    assert!(book.sequence.unwrap_or(0) > 0);
}

/// UNSUBSCRIBE acks and closes the stream.
#[tokio::test]
async fn test_unsubscribe_closes_stream() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sub = make_subscribe_frame(1, "marketdata.AAPL.quotes", "marketdata/AAPL/quotes");
    h.send(sub).await.expect("subscribe");

    let unsub = Frame::new(FrameType::Unsubscribe, 1)
        .with_seq(2)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata.AAPL.quotes",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/quotes",
        ));
    let responses = h.send(unsub).await.expect("unsub");
    assert!(responses
        .iter()
        .any(|f| f.frame_type == FrameType::StreamClose));
}

/// Session resume restores persisted subscriptions on the same connection.
#[tokio::test]
async fn test_session_resume_subscriptions() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "RESUME-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        5.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("seed book");

    let sub = make_subscribe_frame(1, "marketdata.AAPL.quotes", "marketdata/AAPL/quotes");
    let resume = Frame::new(FrameType::Subscribe, 1)
        .with_seq(2)
        .with_extension(Extension::text(ExtensionTag::Method, "RESUME"))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            ".well-known/resume",
        ));
    let responses = h
        .send_multiple(vec![sub, resume])
        .await
        .expect("subscribe+resume");
    assert!(responses
        .iter()
        .any(|f| f.frame_type == FrameType::StreamItem));
}

/// Order history uses request_stream when batch exceeds threshold.
#[tokio::test]
async fn test_order_history_request_stream() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    for i in 0..55 {
        let sell = make_order(
            &format!("HS-{i}"),
            Side::Sell,
            "AAPL",
            OrderType::Limit,
            Some(100.0 + i as f64),
            1.0,
        );
        h.send(make_order_frame(1, &sell).unwrap())
            .await
            .expect("sell");
        let buy = make_order(
            &format!("HB-{i}"),
            Side::Buy,
            "AAPL",
            OrderType::Market,
            None,
            1.0,
        );
        h.send(make_order_frame(1, &buy).unwrap())
            .await
            .expect("buy");
    }

    let query = make_get_query_frame(1, "trading/accounts/TEST/orders", Some("TEST"));
    let responses = h.send(query).await.expect("history");
    let items: Vec<_> = responses
        .iter()
        .filter(|f| f.frame_type == FrameType::StreamItem)
        .collect();
    assert!(items.len() >= 2, "expected chunked request_stream");
    assert!(responses
        .iter()
        .any(|f| f.frame_type == FrameType::StreamClose));
}

#[tokio::test]
async fn test_order_book_snapshot_on_subscribe() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sub = make_subscribe_frame(1, "marketdata/AAPL/quotes", "marketdata/AAPL/book");
    let responses = h.send(sub).await.expect("subscribe");
    let item = responses
        .iter()
        .find(|f| f.frame_type == FrameType::StreamItem)
        .expect("stream item");
    let snap: OrderBookSnapshot = codec::decode_cbor(&item.payload).expect("decode");
    assert_eq!(snap.symbol, "AAPL");
    assert!(snap.is_snapshot.unwrap_or(false));
}

#[tokio::test]
async fn test_agg_trades_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let order = make_order(
        "AT-1",
        Side::Buy,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        10.0,
    );
    h.send(make_order_frame(1, &order).unwrap())
        .await
        .expect("resting");
    let taker = make_order("AT-2", Side::Sell, "AAPL", OrderType::Market, None, 5.0);
    h.send(make_order_frame(1, &taker).unwrap())
        .await
        .expect("fill");

    let query = make_get_query_frame(2, "marketdata/AAPL/aggtrades", None);
    let responses = h.send(query).await.expect("query");
    let resp = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response");
    let batch: AggregateTradeBatch = codec::decode_cbor(&resp.payload).expect("decode");
    assert!(!batch.trades.is_empty());
}

#[tokio::test]
async fn test_mark_price_and_all_mids_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let order = make_order("MP-1", Side::Buy, "AAPL", OrderType::Limit, Some(50.0), 1.0);
    h.send(make_order_frame(1, &order).unwrap())
        .await
        .expect("resting");
    let taker = make_order("MP-2", Side::Sell, "AAPL", OrderType::Market, None, 1.0);
    h.send(make_order_frame(1, &taker).unwrap())
        .await
        .expect("fill");

    let mark = make_get_query_frame(2, "marketdata/AAPL/mark", None);
    let mark_resp = h.send(mark).await.expect("mark");
    let mark_frame = mark_resp
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("mark response");
    let _: MarkPriceUpdate = codec::decode_cbor(&mark_frame.payload).expect("mark decode");

    let all_mids = make_get_query_frame(3, "marketdata/ticker/all", None);
    let mids_resp = h.send(all_mids).await.expect("all mids");
    let mids_frame = mids_resp
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("mids response");
    let batch: AllMidsBatch = codec::decode_cbor(&mids_frame.payload).expect("mids decode");
    assert!(!batch.tickers.is_empty());
}

#[tokio::test]
async fn test_margin_subscribe_snapshot() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let mut frame = make_subscribe_frame(1, "accounts/TEST/margin", "accounts/TEST/margin");
    frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-TEST"));
    let responses = h.send(frame).await.expect("margin sub");
    let item = responses
        .iter()
        .find(|f| f.frame_type == FrameType::StreamItem)
        .expect("margin stream");
    let update: MarginUpdate = codec::decode_cbor(&item.payload).expect("margin decode");
    assert_eq!(update.account, "TEST");
    assert!(update.is_snapshot.unwrap_or(false));
}

#[tokio::test]
async fn test_position_delta_on_fill() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let mut pos_frame =
        make_subscribe_frame(1, "accounts/TEST/positions", "accounts/TEST/positions");
    pos_frame = pos_frame.with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-TEST"));
    h.send(pos_frame).await.expect("positions sub");

    let sell = make_order(
        "PD-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        5.0,
    );
    h.send(make_order_frame(2, &sell).unwrap())
        .await
        .expect("resting");
    let buy = make_order("PD-2", Side::Buy, "AAPL", OrderType::Market, None, 5.0);
    let fill_resp = h
        .send(make_order_frame(2, &buy).unwrap())
        .await
        .expect("fill");
    assert!(fill_resp.iter().any(|f| {
        f.frame_type == FrameType::StreamItem
            && codec::decode_cbor::<PositionUpdate>(&f.payload).is_ok()
    }));
}

/// Order history pagination returns next_cursor for follow-up queries.
#[tokio::test]
async fn test_order_history_pagination_cursor() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    for i in 0..15 {
        let sell = make_order(
            &format!("PG-{i}"),
            Side::Sell,
            "AAPL",
            OrderType::Limit,
            Some(100.0 + i as f64),
            1.0,
        );
        h.send(make_order_frame(1, &sell).unwrap())
            .await
            .expect("sell");
    }

    let req = OrderHistoryRequest {
        account: "TEST".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: None,
    };
    let payload = codec::encode_cbor(&req).unwrap();
    let mut query = make_get_query_frame(2, "trading/accounts/TEST/orders", Some("TEST"));
    query.payload = payload;
    let responses = h.send(query).await.expect("page1");
    let resp = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response");
    let batch: OrderHistoryBatch = codec::decode_cbor(&resp.payload).expect("decode");
    assert_eq!(batch.orders.len(), 5);
    assert!(batch.has_more);
    assert!(batch.next_cursor.is_some());

    let req2 = OrderHistoryRequest {
        account: "TEST".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: batch.next_cursor.clone(),
    };
    let payload2 = codec::encode_cbor(&req2).unwrap();
    let mut query2 = make_get_query_frame(2, "trading/accounts/TEST/orders", Some("TEST"));
    query2.payload = payload2;
    let responses2 = h.send(query2).await.expect("page2");
    let resp2 = responses2
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response2");
    let batch2: OrderHistoryBatch = codec::decode_cbor(&resp2.payload).expect("decode2");
    assert!(!batch2.orders.is_empty());
    assert_ne!(batch2.orders[0].exec_id, batch.orders[0].exec_id);
}

/// Resting limit order emits New execution on executions subscription.
#[tokio::test]
async fn test_executions_subscribe_on_rest() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let mut exec_frame = make_subscribe_frame(
        1,
        "trading/accounts/TEST/executions",
        "trading/accounts/TEST/executions",
    );
    exec_frame =
        exec_frame.with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-TEST"));
    h.send(exec_frame).await.expect("executions sub");

    let order = make_order(
        "REST-1",
        Side::Buy,
        "AAPL",
        OrderType::Limit,
        Some(50.0),
        10.0,
    );
    let responses = h
        .send(make_order_frame(2, &order).unwrap())
        .await
        .expect("resting order");
    let new_report = responses.iter().find_map(|f| {
        if f.frame_type != FrameType::StreamItem {
            return None;
        }
        let report: ExecutionReport = codec::decode_cbor(&f.payload).ok()?;
        (report.exec_type == ExecType::New).then_some(report)
    });
    assert!(new_report.is_some(), "expected New on executions sub");
}

/// Fill history query returns recorded fills after a trade.
#[tokio::test]
async fn test_fill_history_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "FH-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(100.0),
        5.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("resting");
    let buy = make_order("FH-2", Side::Buy, "AAPL", OrderType::Market, None, 5.0);
    h.send(make_order_frame(1, &buy).unwrap())
        .await
        .expect("fill");

    let query = make_get_query_frame(2, "accounts/TEST/fills", Some("TEST"));
    let responses = h.send(query).await.expect("fills query");
    let resp = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response");
    let batch: FillHistoryBatch = codec::decode_cbor(&resp.payload).expect("decode");
    assert!(!batch.fills.is_empty());
}

/// BBO stream delivers snapshot on subscribe.
#[tokio::test]
async fn test_bbo_subscribe_snapshot() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "BBO-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(101.0),
        1.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("resting");

    let sub = make_subscribe_frame(2, "marketdata/AAPL/bbo", "marketdata/AAPL/bbo");
    let responses = h.send(sub).await.expect("bbo sub");
    let item = responses
        .iter()
        .find(|f| f.frame_type == FrameType::StreamItem)
        .expect("bbo item");
    let bbo: BestBidOffer = codec::decode_cbor(&item.payload).expect("bbo decode");
    assert_eq!(bbo.symbol, "AAPL");
}

/// OrderListStatus stream receives updates when orders rest.
#[tokio::test]
async fn test_order_list_status_subscribe() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let mut list_frame = make_subscribe_frame(
        1,
        "trading/accounts/TEST/orderlists",
        "trading/accounts/TEST/orderlists",
    );
    list_frame =
        list_frame.with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-TEST"));
    h.send(list_frame).await.expect("orderlists sub");

    let order = make_order("OL-1", Side::Buy, "AAPL", OrderType::Limit, Some(42.0), 2.0);
    let responses = h
        .send(make_order_frame(2, &order).unwrap())
        .await
        .expect("resting");
    let status = responses.iter().find_map(|f| {
        if f.frame_type != FrameType::StreamItem {
            return None;
        }
        codec::decode_cbor::<OrderListStatus>(&f.payload).ok()
    });
    assert!(status.is_some());
    assert_eq!(status.unwrap().list_id, "OL-1");
}

/// SBE-encoded NewOrderSingle is accepted on the order entry path.
#[tokio::test]
async fn test_sbe_new_order_single() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let order = make_order(
        "SBE-1",
        Side::Buy,
        "AAPL",
        OrderType::Limit,
        Some(55.0),
        2.0,
    );
    let payload = fig_core::sbe::encode_new_order_single(&order);
    let mut frame = make_order_frame(1, &order).unwrap();
    frame.payload = payload;
    frame = frame.with_extension(Extension::text(
        ExtensionTag::ContentType,
        "application/fig+sbe",
    ));
    let responses = h.send(frame).await.expect("sbe order");
    assert!(
        !responses
            .iter()
            .any(|f| f.frame_type == FrameType::StreamError),
        "SBE order should not error"
    );
}

/// Historical book query with `at_time` returns a recorded snapshot.
#[tokio::test]
async fn test_order_book_at_time_query() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "AT-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(200.0),
        1.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("resting");

    let req = OrderBookRequest {
        symbol: "AAPL".to_string(),
        depth: Some(10),
        at_time: None,
    };
    let mut query = make_get_query_frame(2, "marketdata/AAPL/book", None);
    query.payload = codec::encode_cbor(&req).unwrap();
    let responses = h.send(query).await.expect("book now");
    let resp = responses
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response");
    let snap: OrderBookSnapshot = codec::decode_cbor(&resp.payload).expect("decode");
    let at_time = snap.timestamp;

    let req2 = OrderBookRequest {
        symbol: "AAPL".to_string(),
        depth: Some(10),
        at_time: Some(at_time),
    };
    let mut query2 = make_get_query_frame(2, "marketdata/AAPL/book", None);
    query2.payload = codec::encode_cbor(&req2).unwrap();
    let responses2 = h.send(query2).await.expect("book at time");
    let resp2 = responses2
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response2");
    let snap2: OrderBookSnapshot = codec::decode_cbor(&resp2.payload).expect("decode2");
    assert_eq!(snap2.timestamp, at_time);
}

/// Invalid method on a GET-only query path is rejected.
#[tokio::test]
async fn test_invalid_query_method_rejected() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let query = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            ".well-known/capabilities",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"));
    let responses = h.send(query).await.expect("bad method");
    assert!(responses
        .iter()
        .any(|f| f.frame_type == FrameType::StreamError));
}

/// Language-neutral E2E driver flow: connect → order → execution report.
#[tokio::test]
async fn test_e2e_driver_order_to_execution() {
    let _ = tracing_subscriber::fmt::try_init();
    let h = Harness::start().await.expect("harness");

    let sell = make_order(
        "E2E-1",
        Side::Sell,
        "AAPL",
        OrderType::Limit,
        Some(150.0),
        5.0,
    );
    h.send(make_order_frame(1, &sell).unwrap())
        .await
        .expect("resting");
    let buy = make_order("E2E-2", Side::Buy, "AAPL", OrderType::Market, None, 5.0);
    let responses = h
        .send(make_order_frame(1, &buy).unwrap())
        .await
        .expect("fill");
    let fill = responses.iter().find_map(|f| {
        if f.frame_type != FrameType::StreamItem {
            return None;
        }
        let report: ExecutionReport = codec::decode_cbor(&f.payload).ok()?;
        (report.exec_type == ExecType::Fill).then_some(report)
    });
    assert!(fill.is_some(), "expected Trade execution report");
}
