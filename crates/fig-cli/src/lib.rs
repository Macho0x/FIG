//! FIG CLI library — native FIG demo client (orders, streams, queries).

use std::net::SocketAddr;

use anyhow::{bail, Context, Result};
use fig_client::{dev_auth_token, FigSdkClient};
use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{ControlSubtype, Frame, FrameType};
use fig_core::messages::schema_id;
use fig_core::messages::*;
use fig_core::sbe;
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
    let client = FigSdkClient::new(conn);

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
        post_only: None,
        reduce_only: None,
    };
    let order_frames = client.post_order(ACCOUNT, &order, 1).await?;
    log_frames("order", &order_frames);
    assert_execution_report("order", &order_frames)?;

    info!("=== Demo 2: SUBSCRIBE candles/5m ===");
    let (_candle_state, candle_frames) = client.subscribe_candles("AAPL", "5m", 2).await?;
    log_frames("candles", &candle_frames);
    assert_subscribe_ack("candles", &candle_frames)?;

    info!("=== Demo 3: SUBSCRIBE balances ===");
    let (_bal_cache, bal_frames) = client.subscribe_balances(ACCOUNT, 3).await?;
    log_frames("balances", &bal_frames);
    assert_subscribe_ack("balances", &bal_frames)?;

    info!("=== Demo 4: GET candles/5m ===");
    let (_batch, candle_query) = client
        .request_candles(
            CandleBarRequest {
                symbol: "AAPL".to_string(),
                interval: "5m".to_string(),
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            4,
        )
        .await?;
    log_frames("candle_query", &candle_query);
    assert_query_response("candle_query", &candle_query)?;

    info!("=== Demo 5: private + ticker queries ===");
    let (_summary, account_frames) = client.request_account_summary(ACCOUNT, 5).await?;
    log_frames("account", &account_frames);
    assert_query_response("account", &account_frames)?;

    let (_ticker, ticker_frames) = client.request_ticker("AAPL", 6).await?;
    log_frames("ticker", &ticker_frames);
    assert_query_response("ticker", &ticker_frames)?;

    let (_funding, funding_frames) = client
        .request_funding_history(
            ACCOUNT,
            FundingHistoryRequest {
                account: ACCOUNT.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            7,
        )
        .await?;
    log_frames("funding", &funding_frames);
    assert_query_response("funding", &funding_frames)?;

    let (_ledger, ledger_frames) = client
        .request_ledger_history(
            ACCOUNT,
            LedgerHistoryRequest {
                account: ACCOUNT.to_string(),
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            8,
        )
        .await?;
    log_frames("ledger", &ledger_frames);
    assert_query_response("ledger", &ledger_frames)?;

    let caps = client.request_capabilities(12).await?;
    if caps.paths.is_empty() {
        bail!("capabilities: empty catalog");
    }
    info!("  Capabilities: {} paths", caps.paths.len());

    let instruments = client.request_instruments(13).await?;
    if instruments.instruments.is_empty() {
        bail!("instruments: empty catalog");
    }
    info!("  Instruments: {} entries", instruments.instruments.len());

    let fills = client
        .request_fills(
            ACCOUNT,
            FillHistoryRequest {
                account: ACCOUNT.to_string(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            14,
        )
        .await?;
    if fills.account != ACCOUNT {
        bail!("fills: expected account {ACCOUNT}");
    }
    info!(
        "  Fills: {} rows on accounts/{ACCOUNT}/fills",
        fills.fills.len()
    );

    info!("=== Demo 6: agg trades, mark price, margin ===");
    let (_agg, agg_frames) = client.subscribe_agg_trades("AAPL", 9).await?;
    log_frames("aggtrades", &agg_frames);
    assert_subscribe_ack("aggtrades", &agg_frames)?;

    let (_mark, mark_frames) = client.request_mark_price("AAPL", 10).await?;
    log_frames("mark", &mark_frames);
    assert_query_response("mark", &mark_frames)?;

    let (_margin, margin_frames) = client.subscribe_margin(ACCOUNT, 11).await?;
    log_frames("margin", &margin_frames);
    assert_subscribe_ack("margin", &margin_frames)?;

    info!("=== Demo 7: PING/PONG ===");
    let ping_frames = client.send_and_read(Frame::ping()).await?;
    log_frames("ping", &ping_frames);
    assert_pong("ping", &ping_frames)?;

    info!("=== Demo complete ===");
    Ok(())
}

/// SBE-encoded order entry demo (colo / market-maker path).
pub async fn run_sbe_order_demo(conn: &Connection) -> Result<()> {
    let client = FigSdkClient::new(conn);
    let order = NewOrderSingle {
        cl_ord_id: "CLI-SBE-1".to_string(),
        side: Side::Buy,
        order_qty: Quantity(10.0),
        price: Some(Price(50.0)),
        stop_price: None,
        symbol: "BTC".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some(ACCOUNT.to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: Some(true),
        reduce_only: None,
    };
    let payload = sbe::encode_new_order_single(&order);
    let frame = Frame::new(FrameType::Request, 20)
        .with_seq(1)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("trading/accounts/{ACCOUNT}/orders"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/fig+sbe",
        ))
        .with_extension(auth_ext(ACCOUNT))
        .with_payload(payload);
    let frames = client.send_and_read(frame).await?;
    log_frames("sbe_order", &frames);
    assert_success_frames("sbe_order", &frames)?;
    let has_report = frames.iter().any(|f| {
        if f.payload.is_empty() {
            return false;
        }
        if let Ok(report) = sbe::decode_execution_report(&f.payload) {
            return report.cl_ord_id == "CLI-SBE-1";
        }
        codec::decode_cbor::<ExecutionReport>(&f.payload)
            .map(|r| r.cl_ord_id == "CLI-SBE-1")
            .unwrap_or(false)
    });
    if !has_report {
        bail!("sbe_order: expected ExecutionReport for CLI-SBE-1");
    }
    Ok(())
}
