//! FIG CLI — native FIG demo client (orders, streams, queries).

use anyhow::Result;
use quinn::Endpoint;
use tokio::io::AsyncWriteExt;
use tracing::info;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};
use fig_core::messages::*;
use fig_core::transport;

const ACCOUNT: &str = "DEMO-ACCT";

fn auth_ext(account: &str) -> Extension {
    Extension::text(ExtensionTag::AuthToken, &format!("fig-dev-{account}"))
}

async fn send_and_read(conn: &quinn::Connection, frame: Frame) -> Result<Vec<Frame>> {
    let (mut send, mut recv) = conn.open_bi().await?;
    send.write_all(&frame.encode()?).await?;
    send.finish()?;
    let mut decoder = FrameDecoder::new();
    let mut buf = vec![0u8; 8192];
    let mut frames = Vec::new();
    while let Some(n) = recv.read(&mut buf).await? {
        decoder.feed(&buf[..n]);
        while let Some(r) = decoder.decode_next() {
            frames.push(r?);
        }
    }
    Ok(frames)
}

fn log_frames(label: &str, frames: &[Frame]) {
    for frame in frames {
        info!("{label}: {}", frame);
        if frame.payload.is_empty() {
            continue;
        }
        if let Ok(report) = codec::decode_cbor::<ExecutionReport>(&frame.payload) {
            info!("  ExecutionReport: {} {:?} {}", report.cl_ord_id, report.exec_type, report.symbol);
        } else if let Ok(snapshot) = codec::decode_cbor::<MarketDataSnapshot>(&frame.payload) {
            info!("  MarketDataSnapshot: {} bids={} asks={}", snapshot.symbol, snapshot.bids.len(), snapshot.asks.len());
        } else if let Ok(event) = codec::decode_cbor::<CandleBarEvent>(&frame.payload) {
            info!("  CandleBar: {} {} close={}", event.bar.symbol, event.bar.interval, event.bar.close.0);
        } else if let Ok(ticker) = codec::decode_cbor::<SymbolTicker>(&frame.payload) {
            info!("  SymbolTicker: {} last={}", ticker.symbol, ticker.last_price.0);
        } else if let Ok(summary) = codec::decode_cbor::<AccountSummary>(&frame.payload) {
            info!("  AccountSummary: {} balance={}", summary.account, summary.balance);
        } else if let Ok(batch) = codec::decode_cbor::<CandleBarBatch>(&frame.payload) {
            info!("  CandleBarBatch: {} bars={}", batch.symbol, batch.bars.len());
        } else if let Ok(snap) = codec::decode_cbor::<BalanceSnapshot>(&frame.payload) {
            info!("  BalanceSnapshot: {} entries={}", snap.account, snap.balances.len());
        } else if let Ok(batch) = codec::decode_cbor::<FundingHistoryBatch>(&frame.payload) {
            info!("  FundingHistoryBatch: {} payments={}", batch.account, batch.payments.len());
        } else if let Ok(batch) = codec::decode_cbor::<LedgerHistoryBatch>(&frame.payload) {
            info!("  LedgerHistoryBatch: {} entries={}", batch.account, batch.entries.len());
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fig_cli=info,fig_core=warn")
        .init();

    let client_config = transport::client_config().map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let server_addr = std::env::var("FIG_SERVER")
        .unwrap_or_else(|_| "127.0.0.1:8443".to_string())
        .parse()?;
    info!("Connecting to {server_addr}...");
    let conn = endpoint.connect(server_addr, "localhost")?.await?;
    info!("Connected.");

    // Demo 1: order entry
    info!("=== Demo 1: NewOrderSingle ===");
    let order = NewOrderSingle {
        cl_ord_id: "CLI-001".to_string(),
        side: Side::Buy,
        order_qty: Quantity(100.0),
        price: Some(Price(50.25)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some(ACCOUNT.to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    };
    let order_frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("trading/accounts/{ACCOUNT}/orders"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(auth_ext(ACCOUNT))
        .with_payload(codec::encode_cbor(&order)?);
    log_frames("order", &send_and_read(&conn, order_frame).await?);

    // Demo 2: candle subscribe
    info!("=== Demo 2: SUBSCRIBE candles/5m ===");
    let candle_sub = Frame::new(FrameType::Subscribe, 2)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata/AAPL/candles/5m",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/candles/5m",
        ));
    log_frames("candles", &send_and_read(&conn, candle_sub).await?);

    // Demo 3: balance subscribe (private)
    info!("=== Demo 3: SUBSCRIBE balances ===");
    let bal_sub = Frame::new(FrameType::Subscribe, 3)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            format!("accounts/{ACCOUNT}/balances"),
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/balances"),
        ))
        .with_extension(auth_ext(ACCOUNT));
    log_frames("balances", &send_and_read(&conn, bal_sub).await?);

    // Demo 4: historical candle query
    info!("=== Demo 4: GET candles/5m ===");
    let candle_get = Frame::new(FrameType::Request, 4)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/candles/5m",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_payload(codec::encode_cbor(&CandleBarRequest {
            symbol: "AAPL".to_string(),
            interval: "5m".to_string(),
            start_time: None,
            end_time: None,
            limit: Some(10),
        })?);
    log_frames("candle_query", &send_and_read(&conn, candle_get).await?);

    // Demo 5: account + ticker + funding/ledger queries
    info!("=== Demo 5: private + ticker queries ===");
    let account_get = Frame::new(FrameType::Request, 5)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    log_frames("account", &send_and_read(&conn, account_get).await?);

    let ticker_get = Frame::new(FrameType::Request, 6)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/ticker",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    log_frames("ticker", &send_and_read(&conn, ticker_get).await?);

    let funding_get = Frame::new(FrameType::Request, 7)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/funding"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    log_frames("funding", &send_and_read(&conn, funding_get).await?);

    let ledger_get = Frame::new(FrameType::Request, 8)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/ledger"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    log_frames("ledger", &send_and_read(&conn, ledger_get).await?);

    // Demo 6: PING
    info!("=== Demo 6: PING/PONG ===");
    log_frames("ping", &send_and_read(&conn, Frame::ping()).await?);

    info!("=== Demo complete ===");
    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;
    Ok(())
}
