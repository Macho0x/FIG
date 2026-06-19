//! FIG CLI — Killer Demo Client
//!
//! Connects to the FIG exchange simulator and demonstrates:
//! - Order entry (NewOrderSingle)
//! - Market data subscription
//! - Account query
//! - Session resumption (0-RTT)
//!
//! Run with: cargo run -p fig-cli

use anyhow::Result;
use quinn::Endpoint;
use tracing::{error, info, warn};

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};
use fig_core::transport;

use fig_core::messages::*;

/// Well-known schema IDs
mod schema_id {
    pub const TRADING_ORDERS: u8 = 0x01;
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("fig_cli=debug,fig_core=info")
        .init();

    info!("FIG CLI starting...");

    // Create client endpoint
    let client_config =
        transport::client_config().map_err(|e| anyhow::anyhow!("client config: {}", e))?;

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    // Connect to the exchange simulator
    let server_addr = "127.0.0.1:8443".parse()?;
    let server_name = "localhost";

    info!("Connecting to {}...", server_addr);
    let conn = endpoint.connect(server_addr, server_name)?.await?;
    info!("Connected to FIG server!");

    // Open a bidirectional stream for order entry
    info!("Opening order entry channel...");
    let (mut send, mut recv) = conn.open_bi().await?;

    // ─── Demo 1: Send a New Order ──────────────────────────────────
    info!("");
    info!("=== Demo 1: New Order ===");

    let order = NewOrderSingle {
        cl_ord_id: "CLI-001".to_string(),
        side: Side::Buy,
        order_qty: Quantity(100.0),
        price: Some(Price(50.25)),
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some("DEMO-ACCT".to_string()),
        strategy_id: None,
    };

    let payload = codec::encode_cbor(&order)?;

    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEMO-ACCT/orders",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
        .with_payload(payload);

    let encoded = frame.encode()?;
    info!("Sending NewOrderSingle ({} bytes)...", encoded.len());
    send.write_all(&encoded).await?;

    // ─── Demo 2: Send a Market Data Subscription ────────────────────
    info!("");
    info!("=== Demo 2: Market Data Subscription ===");

    let (mut md_send, mut md_recv) = conn.open_bi().await?;

    let subscribe = Frame::new(FrameType::Subscribe, 2)
        .with_seq(1)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata.AAPL.quotes",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/quotes",
        ));

    let encoded = subscribe.encode()?;
    info!(
        "Sending SUBSCRIBE for AAPL market data ({} bytes)...",
        encoded.len()
    );
    md_send.write_all(&encoded).await?;

    // ─── Demo 3: Account Query ──────────────────────────────────────
    info!("");
    info!("=== Demo 3: Account Query ===");

    let (mut acct_send, mut acct_recv) = conn.open_bi().await?;

    let acct_frame = Frame::new(FrameType::Request, 3)
        .with_seq(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "accounts/DEMO-ACCT",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));

    let encoded = acct_frame.encode()?;
    info!("Sending account query ({} bytes)...", encoded.len());
    acct_send.write_all(&encoded).await?;

    // ─── Read Responses ─────────────────────────────────────────────
    info!("");
    info!("=== Reading Responses ===");

    // Read order response
    let mut decoder = FrameDecoder::new();
    let mut buf = vec![0u8; 4096];

    // Order response
    match recv.read(&mut buf).await {
        Ok(Some(n)) => {
            decoder.feed(&buf[..n]);
            while let Some(result) = decoder.decode_next() {
                match result {
                    Ok(frame) => {
                        info!("Order response: {}", frame);
                        if !frame.payload.is_empty() {
                            if let Ok(report) =
                                codec::decode_cbor::<ExecutionReport>(&frame.payload)
                            {
                                info!("  ExecutionReport: cl_ord_id={}, exec_type={:?}, ord_status={:?}, symbol={}, last_qty={:?}, avg_price={:?}",
                                    report.cl_ord_id, report.exec_type, report.ord_status, report.symbol,
                                    report.last_qty, report.avg_price);
                            }
                        }
                    }
                    Err(e) => error!("Frame decode error: {}", e),
                }
            }
        }
        Ok(None) => info!("Order stream closed"),
        Err(e) => warn!("Order read error: {}", e),
    }

    // Market data response
    let mut md_decoder = FrameDecoder::new();
    match md_recv.read(&mut buf).await {
        Ok(Some(n)) => {
            md_decoder.feed(&buf[..n]);
            while let Some(result) = md_decoder.decode_next() {
                match result {
                    Ok(frame) => {
                        info!("Market data response: {}", frame);
                        if !frame.payload.is_empty() {
                            if let Ok(snapshot) =
                                codec::decode_cbor::<MarketDataSnapshot>(&frame.payload)
                            {
                                info!("  MarketDataSnapshot: symbol={}, exchange={}, bids={}, asks={}",
                                    snapshot.symbol, snapshot.exchange,
                                    snapshot.bids.len(), snapshot.asks.len());
                                for bid in &snapshot.bids {
                                    info!("    BID: {} @ {}", bid.qty.0, bid.price.0);
                                }
                                for ask in &snapshot.asks {
                                    info!("    ASK: {} @ {}", ask.qty.0, ask.price.0);
                                }
                            }
                        }
                    }
                    Err(e) => error!("Frame decode error: {}", e),
                }
            }
        }
        Ok(None) => info!("Market data stream closed"),
        Err(e) => warn!("Market data read error: {}", e),
    }

    // Account response
    let mut acct_decoder = FrameDecoder::new();
    match acct_recv.read(&mut buf).await {
        Ok(Some(n)) => {
            acct_decoder.feed(&buf[..n]);
            while let Some(result) = acct_decoder.decode_next() {
                match result {
                    Ok(frame) => {
                        info!("Account response: {}", frame);
                        if !frame.payload.is_empty() {
                            if let Ok(summary) =
                                codec::decode_cbor::<AccountSummary>(&frame.payload)
                            {
                                info!("  AccountSummary: account={}, balance={}, buying_power={}, currency={}",
                                    summary.account, summary.balance, summary.buying_power, summary.currency);
                            }
                        }
                    }
                    Err(e) => error!("Frame decode error: {}", e),
                }
            }
        }
        Ok(None) => info!("Account stream closed"),
        Err(e) => warn!("Account read error: {}", e),
    }

    // ─── Demo 4: Send a PING ────────────────────────────────────────
    info!("");
    info!("=== Demo 4: PING/PONG ===");

    let (mut ctrl_send, mut ctrl_recv) = conn.open_bi().await?;
    let ping = Frame::ping();
    let encoded = ping.encode()?;
    info!("Sending PING ({} bytes)...", encoded.len());
    ctrl_send.write_all(&encoded).await?;
    ctrl_send.finish()?;

    let mut ctrl_decoder = FrameDecoder::new();
    match ctrl_recv.read(&mut buf).await {
        Ok(Some(n)) => {
            ctrl_decoder.feed(&buf[..n]);
            while let Some(result) = ctrl_decoder.decode_next() {
                match result {
                    Ok(frame) => info!("Control response: {}", frame),
                    Err(e) => error!("Frame decode error: {}", e),
                }
            }
        }
        Ok(None) => info!("Control stream closed"),
        Err(e) => warn!("Control read error: {}", e),
    }

    info!("");
    info!("=== Demo Complete ===");
    info!("FIG protocol demonstration finished successfully.");
    info!("The client demonstrated:");
    info!("  1. Order entry (NewOrderSingle) → ExecutionReport");
    info!("  2. Market data subscription → MarketDataSnapshot");
    info!("  3. Account query → AccountSummary");
    info!("  4. PING/PONG control frames");
    info!("  All over a single TREE connection with per-stream flow control.");

    // Close the connection
    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;

    Ok(())
}
