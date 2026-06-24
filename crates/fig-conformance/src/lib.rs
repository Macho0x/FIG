//! FIG conformance vector format and reference validation against `fig-core`.

pub mod vectors;

pub use vectors::{
    load_suite, write_suite, ChannelSpec, ConformanceSuite, ConformanceVector, ExtensionSpec,
    ExtensionValueSpec, FrameSpec,
};

use anyhow::{anyhow, Context, Result};
use fig_core::channel::ChannelManager;
use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    AggregateTrade, BalanceSnapshot, CandleBar, CandleBarBatch, CapabilitiesResponse, ExecType,
    ExecutionReport, InstrumentCatalogResponse, MarkPriceUpdate,
    MarketDataAction, MarketDataSnapshot, MarketDataUpdate, MiniTicker, NewOrderSingle,
    OpenOrdersSnapshot, OrdStatus, OrderBookDelta, OrderBookSnapshot, OrderHistoryBatch,
    OrderHistoryRequest, OrderListStatus, OrderListStatusStatus, OrderType, PositionUpdate, Price,
    Quantity, Side, SymbolTicker, TimeInForce,
};
use fig_core::sbe::{decode_new_order_single, encode_new_order_single};
use std::path::Path;

/// Load and validate every vector in a conformance file.
pub fn run_vectors_file(path: &Path) -> Result<()> {
    let suite = load_suite(path)?;
    for vector in &suite.vectors {
        run_vector(vector).with_context(|| format!("vector {}", vector.id))?;
    }
    Ok(())
}

/// Validate a single vector against the Rust reference implementation.
pub fn run_vector(vector: &ConformanceVector) -> Result<()> {
    match vector.category.as_str() {
        "frame" => run_frame_vector(vector),
        "cbor" => run_cbor_vector(vector),
        "sbe" => run_sbe_vector(vector),
        "channel" => run_channel_vector(vector),
        other => Err(anyhow!("unknown vector category: {other}")),
    }
}

fn run_frame_vector(vector: &ConformanceVector) -> Result<()> {
    let spec = vector
        .frame
        .as_ref()
        .ok_or_else(|| anyhow!("frame vector missing `frame` object"))?;
    let mut frame = Frame::new(parse_frame_type(&spec.frame_type)?, spec.channel_id)
        .with_seq(spec.stream_seq)
        .with_schema_id(spec.schema_id);
    for ext in &spec.extensions {
        frame = frame.with_extension(parse_extension(ext)?);
    }
    if let Some(payload_hex) = &spec.payload_hex {
        frame = frame.with_payload(hex::decode(payload_hex)?);
    }
    let encoded = frame.encode().map_err(|e| anyhow!("encode: {e}"))?;
    assert_hex(&vector.expected_hex, &encoded)?;
    Ok(())
}

fn run_cbor_vector(vector: &ConformanceVector) -> Result<()> {
    let payload = vector.payload.as_ref();
    let encoded =
        match vector.message_type.as_str() {
            "NewOrderSingle" => {
                let payload = payload.ok_or_else(|| anyhow!("cbor vector missing `payload`"))?;
                let order = json_to_new_order_single(payload)?;
                encode_cbor(&order).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "CandleBar" => {
                encode_cbor(&sample_candle_bar()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "BalanceSnapshot" => {
                encode_cbor(&sample_balance_snapshot()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "CapabilitiesResponse" => encode_cbor(&sample_capabilities_response())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "OpenOrdersSnapshot" => encode_cbor(&sample_open_orders_snapshot())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "MarketDataSnapshot" => encode_cbor(&sample_market_data_snapshot())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "OrderHistoryRequest" => encode_cbor(&sample_order_history_request())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "OrderHistoryBatch" => encode_cbor(&sample_order_history_batch())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "OrderListStatus" => {
                encode_cbor(&sample_order_list_status()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "OrderBookSnapshot" => encode_cbor(&sample_order_book_snapshot())
                .map_err(|e| anyhow!("encode_cbor: {e}"))?,
            "OrderBookDelta" => {
                encode_cbor(&sample_order_book_delta()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "PositionUpdate" => {
                encode_cbor(&sample_position_update()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "AggregateTrade" => {
                encode_cbor(&sample_aggregate_trade()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "MiniTicker" => {
                encode_cbor(&sample_mini_ticker()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "MarkPriceUpdate" => {
                encode_cbor(&sample_mark_price_update()).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            "InstrumentCatalogResponse" => {
                let payload = payload.ok_or_else(|| anyhow!("cbor vector missing `payload`"))?;
                let catalog: InstrumentCatalogResponse = serde_json::from_value(payload.clone())?;
                encode_cbor(&catalog).map_err(|e| anyhow!("encode_cbor: {e}"))?
            }
            other => return Err(anyhow!("unsupported cbor message_type: {other}")),
        };
    assert_hex(&vector.expected_hex, &encoded)?;
    Ok(())
}

fn sample_candle_bar() -> CandleBar {
    CandleBar {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        open: Price(150.0),
        high: Price(151.0),
        low: Price(149.5),
        close: Price(150.5),
        volume: Quantity(1000.0),
        bar_start: 1_700_000_000_000_000_000,
        bar_end: 1_700_000_300_000_000_000,
        is_final: true,
        is_snapshot: Some(false),
    }
}

fn sample_balance_snapshot() -> BalanceSnapshot {
    BalanceSnapshot {
        account: "DEMO".to_string(),
        balances: vec![fig_core::messages::BalanceEntry {
            asset: "USD".to_string(),
            total: 1_000_000.0,
            available: 2_000_000.0,
            hold: 0.0,
        }],
        is_snapshot: Some(true),
    }
}

fn sample_capabilities_response() -> CapabilitiesResponse {
    CapabilitiesResponse {
        schema_ids: vec![1, 2, 3],
        paths: vec![fig_core::messages::CapabilityPath {
            path: "marketdata/AAPL/candles/5m".to_string(),
            pattern: fig_core::messages::CapabilityPathPattern::PubSub,
            auth_required: false,
        }],
        symbols: vec!["AAPL".to_string()],
        intervals: vec!["5m".to_string()],
    }
}

fn sample_open_orders_snapshot() -> OpenOrdersSnapshot {
    OpenOrdersSnapshot {
        account: "DEMO".to_string(),
        orders: vec![],
        is_snapshot: Some(true),
    }
}

fn sample_market_data_snapshot() -> MarketDataSnapshot {
    use fig_core::messages::PriceLevel;
    MarketDataSnapshot {
        symbol: "AAPL".to_string(),
        exchange: "SIM".to_string(),
        bids: vec![PriceLevel {
            price: Price(100.0),
            qty: Quantity(5.0),
            order_count: Some(1),
        }],
        asks: vec![],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(42),
        is_snapshot: Some(true),
    }
}

fn sample_order_history_request() -> OrderHistoryRequest {
    OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    }
}

fn sample_order_history_batch() -> OrderHistoryBatch {
    OrderHistoryBatch {
        account: "DEMO".to_string(),
        orders: vec![ExecutionReport {
            cl_ord_id: "HIST-1".to_string(),
            order_id: "1".to_string(),
            exec_id: "exec-1".to_string(),
            exec_type: ExecType::New,
            ord_status: OrdStatus::New,
            side: Side::Buy,
            last_qty: None,
            last_price: None,
            leaves_qty: Quantity(10.0),
            cum_qty: Quantity(0.0),
            avg_price: Price(0.0),
            symbol: "AAPL".to_string(),
            transact_time: 1_700_000_000_000_000_000,
        }],
        has_more: true,
        next_cursor: Some("exec-1".to_string()),
    }
}

fn sample_order_list_status() -> OrderListStatus {
    OrderListStatus {
        account: "DEMO".to_string(),
        list_id: "OL-1".to_string(),
        status: OrderListStatusStatus::Executing,
        symbol: Some("AAPL".to_string()),
    }
}

fn sample_order_book_snapshot() -> OrderBookSnapshot {
    use fig_core::messages::PriceLevel;
    OrderBookSnapshot {
        symbol: "AAPL".to_string(),
        exchange: "SIM".to_string(),
        bids: vec![PriceLevel {
            price: Price(100.0),
            qty: Quantity(5.0),
            order_count: Some(1),
        }],
        asks: vec![],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(42),
        is_snapshot: Some(true),
    }
}

fn sample_order_book_delta() -> OrderBookDelta {
    OrderBookDelta {
        symbol: "AAPL".to_string(),
        updates: vec![MarketDataUpdate {
            action: MarketDataAction::Change,
            side: Side::Buy,
            price: Price(100.0),
            qty: Quantity(5.0),
        }],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(43),
    }
}

fn sample_position_update() -> PositionUpdate {
    PositionUpdate {
        account: "DEMO".to_string(),
        symbol: "AAPL".to_string(),
        qty: Quantity(10.0),
        entry_price: Price(100.0),
        unrealized_pnl: 0.0,
    }
}

fn sample_aggregate_trade() -> AggregateTrade {
    AggregateTrade {
        symbol: "AAPL".to_string(),
        agg_trade_id: "A-1".to_string(),
        price: Price(100.0),
        qty: Quantity(5.0),
        side: Side::Buy,
        first_trade_id: "T-1".to_string(),
        last_trade_id: "T-1".to_string(),
        timestamp: 1_700_000_000_000_000_000,
    }
}

fn sample_mini_ticker() -> MiniTicker {
    MiniTicker {
        symbol: "AAPL".to_string(),
        last_price: Price(100.0),
        volume: Quantity(50.0),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    }
}

fn sample_mark_price_update() -> MarkPriceUpdate {
    MarkPriceUpdate {
        symbol: "AAPL".to_string(),
        mark_price: Price(100.01),
        index_price: Some(Price(100.0)),
        funding_rate: Some(0.0001),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    }
}

fn sample_candle_bar_batch() -> CandleBarBatch {
    CandleBarBatch {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        bars: vec![sample_candle_bar()],
        has_more: false,
        next_cursor: None,
    }
}

fn sample_symbol_ticker() -> SymbolTicker {
    SymbolTicker {
        symbol: "AAPL".to_string(),
        last_price: Price(150.5),
        price_change: 0.5,
        price_change_pct: 0.33,
        volume: Quantity(1000.0),
        high: Price(151.0),
        low: Price(149.5),
        open: Price(150.0),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    }
}

fn run_sbe_vector(vector: &ConformanceVector) -> Result<()> {
    use fig_core::sbe::{
        encode_balance_snapshot, encode_candle_bar, encode_candle_bar_batch,
        encode_order_book_snapshot, encode_symbol_ticker,
    };

    let encoded = match vector.message_type.as_str() {
        "NewOrderSingle" => {
            let payload = vector
                .payload
                .as_ref()
                .ok_or_else(|| anyhow!("sbe vector missing `payload`"))?;
            let order = json_to_new_order_single(payload)?;
            encode_new_order_single(&order)
        }
        "CandleBar" => encode_candle_bar(&sample_candle_bar()),
        "CandleBarBatch" => encode_candle_bar_batch(&sample_candle_bar_batch()),
        "SymbolTicker" => encode_symbol_ticker(&sample_symbol_ticker()),
        "OrderBookSnapshot" => encode_order_book_snapshot(&sample_order_book_snapshot()),
        "BalanceSnapshot" => encode_balance_snapshot(&sample_balance_snapshot()),
        other => return Err(anyhow!("unsupported sbe message_type: {other}")),
    };
    assert_hex(&vector.expected_hex, &encoded)?;
    if vector.message_type == "NewOrderSingle" {
        let payload = vector.payload.as_ref().unwrap();
        let order = json_to_new_order_single(payload)?;
        let decoded = decode_new_order_single(&encoded).map_err(|e| anyhow!("sbe decode: {e}"))?;
        if decoded.cl_ord_id != order.cl_ord_id {
            return Err(anyhow!("sbe round-trip cl_ord_id mismatch"));
        }
    }
    Ok(())
}

fn run_channel_vector(vector: &ConformanceVector) -> Result<()> {
    let spec = vector
        .channel
        .as_ref()
        .ok_or_else(|| anyhow!("channel vector missing `channel` object"))?;
    let mut client = ChannelManager::new(false);
    let server = ChannelManager::new(true);
    let ch = client
        .open_channel(fig_core::channel::ChannelMode::Stateless, None)
        .map_err(|e| anyhow!("open channel: {e}"))?;
    let client_stream = client.tree_stream_id(ch);
    let server_stream = server.tree_stream_id(ch);
    if client_stream != spec.client_stream_id {
        return Err(anyhow!(
            "client stream id: expected {}, got {}",
            spec.client_stream_id,
            client_stream
        ));
    }
    if server_stream != spec.server_stream_id {
        return Err(anyhow!(
            "server stream id: expected {}, got {}",
            spec.server_stream_id,
            server_stream
        ));
    }
    Ok(())
}

fn assert_hex(expected_hex: &str, actual: &[u8]) -> Result<()> {
    let expected = hex::decode(expected_hex.trim())?;
    if expected != actual {
        return Err(anyhow!(
            "hex mismatch\nexpected: {}\nactual:   {}",
            expected_hex,
            hex::encode(actual)
        ));
    }
    Ok(())
}

fn parse_frame_type(name: &str) -> Result<FrameType> {
    match name {
        "Request" => Ok(FrameType::Request),
        "Response" => Ok(FrameType::Response),
        "StreamOpen" => Ok(FrameType::StreamOpen),
        "StreamClose" => Ok(FrameType::StreamClose),
        "StreamItem" => Ok(FrameType::StreamItem),
        "Subscribe" => Ok(FrameType::Subscribe),
        "Control" => Ok(FrameType::Control),
        _ => Err(anyhow!("unsupported frame type for conformance: {name}")),
    }
}

fn parse_extension(spec: &ExtensionSpec) -> Result<Extension> {
    let tag = match spec.tag.as_str() {
        "AuthToken" => ExtensionTag::AuthToken,
        "ChannelPath" => ExtensionTag::ChannelPath,
        "ContentType" => ExtensionTag::ContentType,
        "CorrelationId" => ExtensionTag::CorrelationId,
        "Method" => ExtensionTag::Method,
        "RequestUri" => ExtensionTag::RequestUri,
        "RoutingKey" => ExtensionTag::RoutingKey,
        other => return Err(anyhow!("unsupported extension tag: {other}")),
    };
    Ok(match &spec.value {
        ExtensionValueSpec::Text(s) => Extension::text(tag, s),
        ExtensionValueSpec::U16(n) => Extension::u16(tag, *n),
    })
}

fn json_to_new_order_single(v: &serde_json::Value) -> Result<NewOrderSingle> {
    let side = parse_side(v.get("side").and_then(|s| s.as_str()).unwrap_or("Buy"))?;
    let order_type = parse_order_type(
        v.get("order_type")
            .and_then(|s| s.as_str())
            .unwrap_or("Limit"),
    )?;
    let tif = parse_time_in_force(
        v.get("time_in_force")
            .and_then(|s| s.as_str())
            .unwrap_or("Day"),
    )?;
    Ok(NewOrderSingle {
        cl_ord_id: v
            .get("cl_ord_id")
            .and_then(|s| s.as_str())
            .unwrap_or("CONF-001")
            .to_string(),
        side,
        order_qty: Quantity(v.get("order_qty").and_then(|n| n.as_f64()).unwrap_or(100.0)),
        price: v.get("price").and_then(|n| n.as_f64()).map(Price),
        stop_price: None,
        symbol: v
            .get("symbol")
            .and_then(|s| s.as_str())
            .unwrap_or("AAPL")
            .to_string(),
        order_type,
        time_in_force: tif,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: v.get("post_only").and_then(|b| b.as_bool()),
        reduce_only: v.get("reduce_only").and_then(|b| b.as_bool()),
    })
}

fn parse_side(s: &str) -> Result<Side> {
    match s {
        "Buy" => Ok(Side::Buy),
        "Sell" => Ok(Side::Sell),
        "SellShort" => Ok(Side::SellShort),
        "SellShortExempt" => Ok(Side::SellShortExempt),
        _ => Err(anyhow!("unknown side: {s}")),
    }
}

fn parse_order_type(s: &str) -> Result<OrderType> {
    match s {
        "Market" => Ok(OrderType::Market),
        "Limit" => Ok(OrderType::Limit),
        "Stop" => Ok(OrderType::Stop),
        "StopLimit" => Ok(OrderType::StopLimit),
        "MarketOnClose" => Ok(OrderType::MarketOnClose),
        "LimitOnClose" => Ok(OrderType::LimitOnClose),
        "Pegged" => Ok(OrderType::Pegged),
        _ => Err(anyhow!("unknown order_type: {s}")),
    }
}

fn parse_time_in_force(s: &str) -> Result<TimeInForce> {
    match s {
        "Day" => Ok(TimeInForce::Day),
        "Gtc" => Ok(TimeInForce::Gtc),
        "Ioc" => Ok(TimeInForce::Ioc),
        "Fok" => Ok(TimeInForce::Fok),
        "Gtd" => Ok(TimeInForce::Gtd),
        _ => Err(anyhow!("unknown time_in_force: {s}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn vectors_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance/vectors/v1.json")
    }

    #[test]
    fn test_committed_vectors_pass() {
        run_vectors_file(&vectors_path()).expect("conformance vectors");
    }
}
