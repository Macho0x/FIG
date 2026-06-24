//! FIG CLI library — native FIG demo client (orders, streams, queries).

use std::net::SocketAddr;

use anyhow::{bail, Context, Result};
use fig_client::{dev_auth_token, FigSdkClient};
use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{ControlSubtype, Frame, FrameType};
use fig_core::messages::*;
use fig_core::transport;
use quinn::{Connection, Endpoint};
use tracing::info;

pub const ACCOUNT: &str = "DEMO-ACCT";
pub const DEFAULT_SERVER: &str = "127.0.0.1:8443";

pub fn auth_ext(account: &str) -> Extension {
    Extension::text(ExtensionTag::AuthToken, dev_auth_token(account))
}

pub async fn connect(server_addr: SocketAddr) -> Result<(Endpoint, Connection)> {
    let client_config = transport::client_config().map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);
    let conn = endpoint
        .connect(server_addr, "localhost")?
        .await
        .context("connect to FIG server")?;
    Ok((endpoint, conn))
}

pub async fn send_and_read(conn: &Connection, frame: Frame) -> Result<Vec<Frame>> {
    FigSdkClient::new(conn)
        .send_and_read(frame)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))
}

fn assert_success_frames(label: &str, frames: &[Frame]) -> Result<()> {
    if frames.is_empty() {
        bail!("{label}: expected at least one response frame");
    }
    if frames
        .iter()
        .any(|f| f.frame_type == FrameType::StreamError)
    {
        bail!("{label}: received STREAM_ERROR");
    }
    Ok(())
}

fn assert_execution_report(label: &str, frames: &[Frame]) -> Result<()> {
    assert_success_frames(label, frames)?;
    let has_report = frames.iter().any(|f| {
        codec::decode_cbor::<ExecutionReport>(&f.payload)
            .map(|r| r.cl_ord_id == "CLI-001")
            .unwrap_or(false)
    });
    if !has_report {
        bail!("{label}: expected ExecutionReport for CLI-001");
    }
    Ok(())
}

fn assert_subscribe_ack(label: &str, frames: &[Frame]) -> Result<()> {
    assert_success_frames(label, frames)?;
    let ok = frames
        .iter()
        .any(|f| matches!(f.frame_type, FrameType::Response | FrameType::StreamItem));
    if !ok {
        bail!("{label}: expected SUBSCRIBE ack (Response or StreamItem)");
    }
    Ok(())
}

fn assert_query_response(label: &str, frames: &[Frame]) -> Result<()> {
    assert_success_frames(label, frames)?;
    if !frames.iter().any(|f| f.frame_type == FrameType::Response) {
        bail!("{label}: expected RESPONSE frame");
    }
    Ok(())
}

fn assert_pong(label: &str, frames: &[Frame]) -> Result<()> {
    assert_success_frames(label, frames)?;
    if !frames.iter().any(|f| {
        f.frame_type == FrameType::Control && f.control_subtype() == Some(ControlSubtype::Pong)
    }) {
        bail!("{label}: expected CONTROL PONG");
    }
    Ok(())
}

pub fn log_frames(label: &str, frames: &[Frame]) {
    for frame in frames {
        info!("{label}: {}", frame);
        if frame.payload.is_empty() {
            continue;
        }
        if let Ok(report) = codec::decode_cbor::<ExecutionReport>(&frame.payload) {
            info!(
                "  ExecutionReport: {} {:?} {}",
                report.cl_ord_id, report.exec_type, report.symbol
            );
        } else if let Ok(snapshot) = codec::decode_cbor::<MarketDataSnapshot>(&frame.payload) {
            info!(
                "  MarketDataSnapshot: {} bids={} asks={}",
                snapshot.symbol,
                snapshot.bids.len(),
                snapshot.asks.len()
            );
        } else if let Ok(event) = codec::decode_cbor::<CandleBarEvent>(&frame.payload) {
            info!(
                "  CandleBar: {} {} close={}",
                event.bar.symbol, event.bar.interval, event.bar.close.0
            );
        } else if let Ok(ticker) = codec::decode_cbor::<SymbolTicker>(&frame.payload) {
            info!(
                "  SymbolTicker: {} last={}",
                ticker.symbol, ticker.last_price.0
            );
        } else if let Ok(summary) = codec::decode_cbor::<AccountSummary>(&frame.payload) {
            info!(
                "  AccountSummary: {} balance={}",
                summary.account, summary.balance
            );
        } else if let Ok(batch) = codec::decode_cbor::<CandleBarBatch>(&frame.payload) {
            info!(
                "  CandleBarBatch: {} bars={}",
                batch.symbol,
                batch.bars.len()
            );
        } else if let Ok(snap) = codec::decode_cbor::<BalanceSnapshot>(&frame.payload) {
            info!(
                "  BalanceSnapshot: {} entries={}",
                snap.account,
                snap.balances.len()
            );
        } else if let Ok(batch) = codec::decode_cbor::<FundingHistoryBatch>(&frame.payload) {
            info!(
                "  FundingHistoryBatch: {} payments={}",
                batch.account,
                batch.payments.len()
            );
        } else if let Ok(batch) = codec::decode_cbor::<LedgerHistoryBatch>(&frame.payload) {
            info!(
                "  LedgerHistoryBatch: {} entries={}",
                batch.account,
                batch.entries.len()
            );
        } else if let Ok(snap) = codec::decode_cbor::<OrderBookSnapshot>(&frame.payload) {
            info!(
                "  OrderBookSnapshot: {} bids={} asks={} seq={:?}",
                snap.symbol,
                snap.bids.len(),
                snap.asks.len(),
                snap.sequence
            );
        } else if let Ok(delta) = codec::decode_cbor::<OrderBookDelta>(&frame.payload) {
            info!(
                "  OrderBookDelta: {} updates={} seq={:?}",
                delta.symbol,
                delta.updates.len(),
                delta.sequence
            );
        } else if let Ok(event) = codec::decode_cbor::<AggregateTradeEvent>(&frame.payload) {
            info!(
                "  AggregateTrade: {} @ {}",
                event.trade.symbol, event.trade.price.0
            );
        } else if let Ok(mini) = codec::decode_cbor::<MiniTicker>(&frame.payload) {
            info!("  MiniTicker: {} last={}", mini.symbol, mini.last_price.0);
        } else if let Ok(mark) = codec::decode_cbor::<MarkPriceUpdate>(&frame.payload) {
            info!("  MarkPrice: {} mark={}", mark.symbol, mark.mark_price.0);
        } else if let Ok(update) = codec::decode_cbor::<MarginUpdate>(&frame.payload) {
            info!(
                "  MarginUpdate: {} equity={}",
                update.account, update.summary.equity
            );
        } else if let Ok(pos) = codec::decode_cbor::<PositionUpdate>(&frame.payload) {
            info!("  PositionUpdate: {} qty={}", pos.symbol, pos.qty.0);
        }
    }
}

/// Run all seven demo flows against a connected FIG server.
pub async fn run_demos(conn: &Connection) -> Result<()> {
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
    let order_frames = send_and_read(conn, order_frame).await?;
    log_frames("order", &order_frames);
    assert_execution_report("order", &order_frames)?;

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
    let candle_frames = send_and_read(conn, candle_sub).await?;
    log_frames("candles", &candle_frames);
    assert_subscribe_ack("candles", &candle_frames)?;

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
    let bal_frames = send_and_read(conn, bal_sub).await?;
    log_frames("balances", &bal_frames);
    assert_subscribe_ack("balances", &bal_frames)?;

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
            cursor: None,
        })?);
    let candle_query = send_and_read(conn, candle_get).await?;
    log_frames("candle_query", &candle_query);
    assert_query_response("candle_query", &candle_query)?;

    info!("=== Demo 5: private + ticker queries ===");
    let account_get = Frame::new(FrameType::Request, 5)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    let account_frames = send_and_read(conn, account_get).await?;
    log_frames("account", &account_frames);
    assert_query_response("account", &account_frames)?;

    let ticker_get = Frame::new(FrameType::Request, 6)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/ticker",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let ticker_frames = send_and_read(conn, ticker_get).await?;
    log_frames("ticker", &ticker_frames);
    assert_query_response("ticker", &ticker_frames)?;

    let funding_get = Frame::new(FrameType::Request, 7)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/funding"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    let funding_frames = send_and_read(conn, funding_get).await?;
    log_frames("funding", &funding_frames);
    assert_query_response("funding", &funding_frames)?;

    let ledger_get = Frame::new(FrameType::Request, 8)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/ledger"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(auth_ext(ACCOUNT));
    let ledger_frames = send_and_read(conn, ledger_get).await?;
    log_frames("ledger", &ledger_frames);
    assert_query_response("ledger", &ledger_frames)?;

    info!("=== Demo 6: agg trades, mark price, margin ===");
    let agg_sub = Frame::new(FrameType::Subscribe, 9)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata/AAPL/aggtrades",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/aggtrades",
        ));
    let agg_frames = send_and_read(conn, agg_sub).await?;
    log_frames("aggtrades", &agg_frames);
    assert_subscribe_ack("aggtrades", &agg_frames)?;

    let mark_get = Frame::new(FrameType::Request, 10)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/mark",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let mark_frames = send_and_read(conn, mark_get).await?;
    log_frames("mark", &mark_frames);
    assert_query_response("mark", &mark_frames)?;

    let margin_sub = Frame::new(FrameType::Subscribe, 11)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            format!("accounts/{ACCOUNT}/margin"),
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("accounts/{ACCOUNT}/margin"),
        ))
        .with_extension(auth_ext(ACCOUNT));
    let margin_frames = send_and_read(conn, margin_sub).await?;
    log_frames("margin", &margin_frames);
    assert_subscribe_ack("margin", &margin_frames)?;

    info!("=== Demo 7: PING/PONG ===");
    let ping_frames = send_and_read(conn, Frame::ping()).await?;
    log_frames("ping", &ping_frames);
    assert_pong("ping", &ping_frames)?;

    info!("=== Demo complete ===");
    Ok(())
}
