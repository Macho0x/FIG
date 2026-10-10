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
    AccountSummary, AggregateTrade, AggregateTradeBatch, AggregateTradeEvent,
    AggregateTradeRequest, AllMidsBatch, AllMidsRequest, BalanceSnapshot, BalanceUpdate,
    BestBidOffer, CancelReject, CancelReplaceRequest, CancelRequest, CandleBar, CandleBarBatch,
    CandleBarEvent, CandleBarRequest, CapabilitiesRequest, CapabilitiesResponse, ExecType,
    ExecutionReport, FillHistoryBatch, FillHistoryRequest, FundingHistoryBatch,
    FundingHistoryRequest, FundingPayment, InstrumentCatalogRequest, InstrumentCatalogResponse,
    InstrumentMetadata, LedgerHistoryBatch, LedgerHistoryRequest, LedgerUpdate, LiquidationTrade,
    LiquidationTradeEvent, MarginSummary, MarginUpdate, MarkPriceRequest, MarkPriceUpdate,
    MarketDataAction, MarketDataIncrementalRefresh, MarketDataSnapshot, MarketDataUpdate,
    MiniTicker, NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot, OrdStatus, OrderBookDelta,
    OrderBookRequest, OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    OrderListStatusStatus, OrderType, PositionEntry, PositionRequest, PositionSnapshot,
    PositionUpdate, Price, PublicTrade, PublicTradeBatch, PublicTradeEvent, Quantity, Side,
    SymbolTicker, TickerRequest, TimeInForce, TradeHistoryRequest, UserLiquidation,
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

pub fn sample_candle_bar() -> CandleBar {
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

pub fn sample_balance_snapshot() -> BalanceSnapshot {
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

pub fn sample_capabilities_response() -> CapabilitiesResponse {
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

pub fn sample_open_orders_snapshot() -> OpenOrdersSnapshot {
    OpenOrdersSnapshot {
        account: "DEMO".to_string(),
        orders: vec![],
        is_snapshot: Some(true),
    }
}

pub fn sample_market_data_snapshot() -> MarketDataSnapshot {
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

pub fn sample_order_history_request() -> OrderHistoryRequest {
    OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    }
}

pub fn sample_order_history_batch() -> OrderHistoryBatch {
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

pub fn sample_order_list_status() -> OrderListStatus {
    OrderListStatus {
        account: "DEMO".to_string(),
        list_id: "OL-1".to_string(),
        status: OrderListStatusStatus::Executing,
        symbol: Some("AAPL".to_string()),
    }
}

pub fn sample_order_book_snapshot() -> OrderBookSnapshot {
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

pub fn sample_order_book_delta() -> OrderBookDelta {
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

pub fn sample_position_update() -> PositionUpdate {
    PositionUpdate {
        account: "DEMO".to_string(),
        symbol: "AAPL".to_string(),
        qty: Quantity(10.0),
        entry_price: Price(100.0),
        unrealized_pnl: 0.0,
    }
}

pub fn sample_aggregate_trade() -> AggregateTrade {
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

pub fn sample_mini_ticker() -> MiniTicker {
    MiniTicker {
        symbol: "AAPL".to_string(),
        last_price: Price(100.0),
        volume: Quantity(50.0),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    }
}

pub fn sample_mark_price_update() -> MarkPriceUpdate {
    MarkPriceUpdate {
        symbol: "AAPL".to_string(),
        mark_price: Price(100.01),
        index_price: Some(Price(100.0)),
        funding_rate: Some(0.0001),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    }
}

pub fn sample_candle_bar_batch() -> CandleBarBatch {
    CandleBarBatch {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        bars: vec![sample_candle_bar()],
        has_more: false,
        next_cursor: None,
    }
}

pub fn sample_symbol_ticker() -> SymbolTicker {
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

pub fn sample_account_summary() -> AccountSummary {
    AccountSummary {
        account: "ACC-1".to_string(),
        balance: 6.0,
        buying_power: 3.0,
        currency: "CURRENCY-4".to_string(),
    }
}
pub fn sample_aggregate_trade_batch() -> AggregateTradeBatch {
    AggregateTradeBatch {
        symbol: "AAPL".to_string(),
        trades: vec![
            AggregateTrade {
                symbol: "MSFT".to_string(),
                agg_trade_id: "AGG-TRADE-ID-3".to_string(),
                price: Price(103.0),
                qty: Quantity(9.0),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                first_trade_id: "FIRST-TRADE-ID-7".to_string(),
                last_trade_id: "LAST-TRADE-ID-8".to_string(),
                timestamp: 1_700_000_000_000_000_000i64 + 8,
            },
            AggregateTrade {
                symbol: "AAPL".to_string(),
                agg_trade_id: "AGG-TRADE-ID-4".to_string(),
                price: Price(104.0),
                qty: Quantity(10.0),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                first_trade_id: "FIRST-TRADE-ID-8".to_string(),
                last_trade_id: "LAST-TRADE-ID-9".to_string(),
                timestamp: 1_700_000_000_000_000_000i64 + 9,
            },
        ],
        has_more: true,
        next_cursor: Some("NEXT-2".to_string()),
    }
}
pub fn sample_aggregate_trade_event() -> AggregateTradeEvent {
    AggregateTradeEvent {
        trade: AggregateTrade {
            symbol: "AAPL".to_string(),
            agg_trade_id: "AGG-TRADE-ID-2".to_string(),
            price: Price(102.0),
            qty: Quantity(8.0),
            side: fig_core::messages::Side::from_value(2).expect("variant"),
            first_trade_id: "FIRST-TRADE-ID-6".to_string(),
            last_trade_id: "LAST-TRADE-ID-7".to_string(),
            timestamp: 1_700_000_000_000_000_000i64 + 7,
        },
    }
}
pub fn sample_aggregate_trade_request() -> AggregateTradeRequest {
    AggregateTradeRequest {
        symbol: "AAPL".to_string(),
        start_time: Some(1_700_000_000_000_000_000i64 + 1),
        end_time: Some(1_700_000_000_000_000_000i64 + 2),
        limit: Some(4_u32),
        cursor: Some("CUR-1".to_string()),
    }
}
pub fn sample_all_mids_batch() -> AllMidsBatch {
    AllMidsBatch {
        tickers: vec![
            MiniTicker {
                symbol: "AAPL".to_string(),
                last_price: Price(101.0),
                volume: Quantity(7.0),
                timestamp: 1_700_000_000_000_000_000i64 + 3,
                is_snapshot: Some(true),
            },
            MiniTicker {
                symbol: "MSFT".to_string(),
                last_price: Price(102.0),
                volume: Quantity(8.0),
                timestamp: 1_700_000_000_000_000_000i64 + 4,
                is_snapshot: Some(true),
            },
        ],
    }
}
pub fn sample_all_mids_request() -> AllMidsRequest {
    AllMidsRequest {}
}
pub fn sample_balance_update() -> BalanceUpdate {
    BalanceUpdate {
        account: "ACC-1".to_string(),
        asset: "ASSET-2".to_string(),
        delta: 3.0,
        total: 4.0,
        available: 5.0,
        reason: fig_core::messages::BalanceUpdateReason::from_value(2).expect("variant"),
    }
}
pub fn sample_best_bid_offer() -> BestBidOffer {
    BestBidOffer {
        symbol: "AAPL".to_string(),
        bid_price: Some(Price(101.0)),
        bid_qty: Some(Quantity(7.0)),
        ask_price: Some(Price(103.0)),
        ask_qty: Some(Quantity(9.0)),
        timestamp: 1_700_000_000_000_000_000i64 + 5,
        is_snapshot: Some(true),
    }
}
pub fn sample_cancel_reject() -> CancelReject {
    CancelReject {
        cl_ord_id: "CL-ORD-ID-1".to_string(),
        orig_cl_ord_id: "ORIG-CL-ORD-ID-2".to_string(),
        reject_reason: fig_core::messages::CancelRejectReason::from_value(2).expect("variant"),
        symbol: "MSFT".to_string(),
    }
}
pub fn sample_cancel_replace_request() -> CancelReplaceRequest {
    CancelReplaceRequest {
        cl_ord_id: "CL-ORD-ID-1".to_string(),
        orig_cl_ord_id: "ORIG-CL-ORD-ID-2".to_string(),
        symbol: "AAPL".to_string(),
        side: fig_core::messages::Side::from_value(2).expect("variant"),
        order_qty: Quantity(9.0),
        price: Some(Price(105.0)),
    }
}
pub fn sample_cancel_request() -> CancelRequest {
    CancelRequest {
        cl_ord_id: "CL-ORD-ID-1".to_string(),
        orig_cl_ord_id: "ORIG-CL-ORD-ID-2".to_string(),
        symbol: "AAPL".to_string(),
        side: fig_core::messages::Side::from_value(2).expect("variant"),
        order_qty: Some(Quantity(9.0)),
    }
}
pub fn sample_candle_bar_event() -> CandleBarEvent {
    CandleBarEvent {
        bar: CandleBar {
            symbol: "AAPL".to_string(),
            interval: "1h".to_string(),
            open: Price(102.0),
            high: Price(103.0),
            low: Price(104.0),
            close: Price(105.0),
            volume: Quantity(11.0),
            bar_start: 1_700_000_000_000_000_000i64 + 7,
            bar_end: 1_700_000_000_000_000_000i64 + 8,
            is_final: true,
            is_snapshot: Some(true),
        },
    }
}
pub fn sample_candle_bar_request() -> CandleBarRequest {
    CandleBarRequest {
        symbol: "AAPL".to_string(),
        interval: "1h".to_string(),
        start_time: Some(1_700_000_000_000_000_000i64 + 2),
        end_time: Some(1_700_000_000_000_000_000i64 + 3),
        limit: Some(5_u32),
        cursor: Some("CUR-2".to_string()),
    }
}
pub fn sample_capabilities_request() -> CapabilitiesRequest {
    CapabilitiesRequest {}
}
pub fn sample_execution_report() -> ExecutionReport {
    ExecutionReport {
        cl_ord_id: "CL-ORD-ID-1".to_string(),
        order_id: "ORDER-ID-2".to_string(),
        exec_id: "EXEC-ID-3".to_string(),
        exec_type: fig_core::messages::ExecType::from_value(2).expect("variant"),
        ord_status: fig_core::messages::OrdStatus::from_value(2).expect("variant"),
        side: fig_core::messages::Side::from_value(2).expect("variant"),
        last_qty: Some(Quantity(11.0)),
        last_price: Some(Price(107.0)),
        leaves_qty: Quantity(13.0),
        cum_qty: Quantity(14.0),
        avg_price: Price(110.0),
        symbol: "MSFT".to_string(),
        transact_time: 1_700_000_000_000_000_000i64 + 12,
    }
}
pub fn sample_fill_history_batch() -> FillHistoryBatch {
    FillHistoryBatch {
        account: "ACC-1".to_string(),
        fills: vec![
            ExecutionReport {
                cl_ord_id: "CL-ORD-ID-2".to_string(),
                order_id: "ORDER-ID-3".to_string(),
                exec_id: "EXEC-ID-4".to_string(),
                exec_type: fig_core::messages::ExecType::from_value(2).expect("variant"),
                ord_status: fig_core::messages::OrdStatus::from_value(2).expect("variant"),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                last_qty: Some(Quantity(12.0)),
                last_price: Some(Price(108.0)),
                leaves_qty: Quantity(14.0),
                cum_qty: Quantity(15.0),
                avg_price: Price(111.0),
                symbol: "AAPL".to_string(),
                transact_time: 1_700_000_000_000_000_000i64 + 13,
            },
            ExecutionReport {
                cl_ord_id: "CL-ORD-ID-3".to_string(),
                order_id: "ORDER-ID-4".to_string(),
                exec_id: "EXEC-ID-5".to_string(),
                exec_type: fig_core::messages::ExecType::from_value(2).expect("variant"),
                ord_status: fig_core::messages::OrdStatus::from_value(2).expect("variant"),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                last_qty: Some(Quantity(13.0)),
                last_price: Some(Price(109.0)),
                leaves_qty: Quantity(15.0),
                cum_qty: Quantity(16.0),
                avg_price: Price(112.0),
                symbol: "MSFT".to_string(),
                transact_time: 1_700_000_000_000_000_000i64 + 14,
            },
        ],
        has_more: true,
        next_cursor: Some("NEXT-2".to_string()),
    }
}
pub fn sample_fill_history_request() -> FillHistoryRequest {
    FillHistoryRequest {
        account: "ACC-1".to_string(),
        symbol: Some("MSFT".to_string()),
        start_time: Some(1_700_000_000_000_000_000i64 + 2),
        end_time: Some(1_700_000_000_000_000_000i64 + 3),
        limit: Some(5_u32),
        cursor: Some("CUR-2".to_string()),
    }
}
pub fn sample_funding_history_batch() -> FundingHistoryBatch {
    FundingHistoryBatch {
        account: "ACC-1".to_string(),
        payments: vec![
            FundingPayment {
                account: "ACC-2".to_string(),
                symbol: Some("AAPL".to_string()),
                amount: 8.0,
                rate: 104.0,
                timestamp: 1_700_000_000_000_000_000i64 + 5,
            },
            FundingPayment {
                account: "ACC-1".to_string(),
                symbol: Some("MSFT".to_string()),
                amount: 9.0,
                rate: 105.0,
                timestamp: 1_700_000_000_000_000_000i64 + 6,
            },
        ],
        has_more: true,
        next_cursor: Some("NEXT-2".to_string()),
    }
}
pub fn sample_funding_history_request() -> FundingHistoryRequest {
    FundingHistoryRequest {
        account: "ACC-1".to_string(),
        start_time: Some(1_700_000_000_000_000_000i64 + 1),
        end_time: Some(1_700_000_000_000_000_000i64 + 2),
        limit: Some(4_u32),
        cursor: Some("CUR-1".to_string()),
    }
}
pub fn sample_funding_payment() -> FundingPayment {
    FundingPayment {
        account: "ACC-1".to_string(),
        symbol: Some("MSFT".to_string()),
        amount: 7.0,
        rate: 103.0,
        timestamp: 1_700_000_000_000_000_000i64 + 4,
    }
}
pub fn sample_instrument_catalog_request() -> InstrumentCatalogRequest {
    InstrumentCatalogRequest {}
}
pub fn sample_instrument_catalog_response() -> InstrumentCatalogResponse {
    InstrumentCatalogResponse {
        instruments: vec![
            InstrumentMetadata {
                instrument_id: "INSTRUMENT-ID-1".to_string(),
                symbol: "MSFT".to_string(),
                product_kind: "PRODUCT-KIND-3".to_string(),
                margin_asset: Some("MARGIN-ASSET-4".to_string()),
                display_name: Some("DISPLAY-NAME-5".to_string()),
                tick_size: Some(10.0),
                lot_size: Some(11.0),
            },
            InstrumentMetadata {
                instrument_id: "INSTRUMENT-ID-2".to_string(),
                symbol: "AAPL".to_string(),
                product_kind: "PRODUCT-KIND-4".to_string(),
                margin_asset: Some("MARGIN-ASSET-5".to_string()),
                display_name: Some("DISPLAY-NAME-6".to_string()),
                tick_size: Some(11.0),
                lot_size: Some(12.0),
            },
        ],
    }
}
pub fn sample_ledger_history_batch() -> LedgerHistoryBatch {
    LedgerHistoryBatch {
        account: "ACC-1".to_string(),
        entries: vec![
            LedgerUpdate {
                account: "ACC-2".to_string(),
                asset: "ASSET-3".to_string(),
                delta: 4.0,
                kind: fig_core::messages::LedgerUpdateKind::from_value(2).expect("variant"),
                timestamp: 1_700_000_000_000_000_000i64 + 5,
                reference_id: Some("REFERENCE-ID-7".to_string()),
            },
            LedgerUpdate {
                account: "ACC-1".to_string(),
                asset: "ASSET-4".to_string(),
                delta: 5.0,
                kind: fig_core::messages::LedgerUpdateKind::from_value(2).expect("variant"),
                timestamp: 1_700_000_000_000_000_000i64 + 6,
                reference_id: Some("REFERENCE-ID-8".to_string()),
            },
        ],
        has_more: true,
        next_cursor: Some("NEXT-2".to_string()),
    }
}
pub fn sample_ledger_history_request() -> LedgerHistoryRequest {
    LedgerHistoryRequest {
        account: "ACC-1".to_string(),
        start_time: Some(1_700_000_000_000_000_000i64 + 1),
        end_time: Some(1_700_000_000_000_000_000i64 + 2),
        limit: Some(4_u32),
        cursor: Some("CUR-1".to_string()),
    }
}
pub fn sample_ledger_update() -> LedgerUpdate {
    LedgerUpdate {
        account: "ACC-1".to_string(),
        asset: "ASSET-2".to_string(),
        delta: 3.0,
        kind: fig_core::messages::LedgerUpdateKind::from_value(2).expect("variant"),
        timestamp: 1_700_000_000_000_000_000i64 + 4,
        reference_id: Some("REFERENCE-ID-6".to_string()),
    }
}
pub fn sample_liquidation_trade_event() -> LiquidationTradeEvent {
    LiquidationTradeEvent {
        trade: LiquidationTrade {
            symbol: "AAPL".to_string(),
            side: fig_core::messages::Side::from_value(2).expect("variant"),
            price: Price(102.0),
            qty: Quantity(8.0),
            timestamp: 1_700_000_000_000_000_000i64 + 4,
        },
    }
}
pub fn sample_margin_summary() -> MarginSummary {
    MarginSummary {
        account: "ACC-1".to_string(),
        balance: 6.0,
        buying_power: 3.0,
        equity: 8.0,
        margin_used: 5.0,
        available: 6.0,
        currency: "CURRENCY-7".to_string(),
    }
}
pub fn sample_margin_update() -> MarginUpdate {
    MarginUpdate {
        account: "ACC-1".to_string(),
        summary: MarginSummary {
            account: "ACC-2".to_string(),
            balance: 7.0,
            buying_power: 4.0,
            equity: 9.0,
            margin_used: 6.0,
            available: 7.0,
            currency: "CURRENCY-8".to_string(),
        },
        is_snapshot: Some(true),
    }
}
pub fn sample_mark_price_request() -> MarkPriceRequest {
    MarkPriceRequest {
        symbol: "AAPL".to_string(),
    }
}
pub fn sample_market_data_incremental_refresh() -> MarketDataIncrementalRefresh {
    MarketDataIncrementalRefresh {
        symbol: "AAPL".to_string(),
        updates: vec![
            MarketDataUpdate {
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                action: fig_core::messages::MarketDataAction::from_value(2).expect("variant"),
                price: Price(103.0),
                qty: Quantity(9.0),
            },
            MarketDataUpdate {
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                action: fig_core::messages::MarketDataAction::from_value(2).expect("variant"),
                price: Price(104.0),
                qty: Quantity(10.0),
            },
        ],
        timestamp: 1_700_000_000_000_000_000i64 + 2,
        sequence: Some(10_u64),
    }
}
pub fn sample_open_orders_request() -> OpenOrdersRequest {
    OpenOrdersRequest {
        account: "ACC-1".to_string(),
        symbol: Some("MSFT".to_string()),
    }
}
pub fn sample_order_book_request() -> OrderBookRequest {
    OrderBookRequest {
        symbol: "AAPL".to_string(),
        depth: Some(2_u32),
        at_time: Some(1_700_000_000_000_000_000i64 + 2),
    }
}
pub fn sample_position_request() -> PositionRequest {
    PositionRequest {
        account: "ACC-1".to_string(),
    }
}
pub fn sample_position_snapshot() -> PositionSnapshot {
    PositionSnapshot {
        account: "ACC-1".to_string(),
        positions: vec![
            PositionEntry {
                symbol: "MSFT".to_string(),
                qty: Quantity(7.0),
                entry_price: Price(103.0),
                unrealized_pnl: 104.0,
            },
            PositionEntry {
                symbol: "AAPL".to_string(),
                qty: Quantity(8.0),
                entry_price: Price(104.0),
                unrealized_pnl: 105.0,
            },
        ],
        is_snapshot: Some(true),
    }
}
pub fn sample_public_trade_batch() -> PublicTradeBatch {
    PublicTradeBatch {
        symbol: "AAPL".to_string(),
        trades: vec![
            PublicTrade {
                symbol: "MSFT".to_string(),
                trade_id: "TRADE-ID-3".to_string(),
                price: Price(103.0),
                qty: Quantity(9.0),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                timestamp: 1_700_000_000_000_000_000i64 + 6,
            },
            PublicTrade {
                symbol: "AAPL".to_string(),
                trade_id: "TRADE-ID-4".to_string(),
                price: Price(104.0),
                qty: Quantity(10.0),
                side: fig_core::messages::Side::from_value(2).expect("variant"),
                timestamp: 1_700_000_000_000_000_000i64 + 7,
            },
        ],
        has_more: true,
        next_cursor: Some("NEXT-2".to_string()),
    }
}
pub fn sample_public_trade_event() -> PublicTradeEvent {
    PublicTradeEvent {
        trade: PublicTrade {
            symbol: "AAPL".to_string(),
            trade_id: "TRADE-ID-2".to_string(),
            price: Price(102.0),
            qty: Quantity(8.0),
            side: fig_core::messages::Side::from_value(2).expect("variant"),
            timestamp: 1_700_000_000_000_000_000i64 + 5,
        },
    }
}
pub fn sample_ticker_request() -> TickerRequest {
    TickerRequest {
        symbol: "AAPL".to_string(),
    }
}
pub fn sample_trade_history_request() -> TradeHistoryRequest {
    TradeHistoryRequest {
        symbol: "AAPL".to_string(),
        start_time: Some(1_700_000_000_000_000_000i64 + 1),
        end_time: Some(1_700_000_000_000_000_000i64 + 2),
        limit: Some(4_u32),
        cursor: Some("CUR-1".to_string()),
    }
}
pub fn sample_user_liquidation() -> UserLiquidation {
    UserLiquidation {
        account: "ACC-1".to_string(),
        symbol: "MSFT".to_string(),
        qty: Quantity(7.0),
        price: Price(103.0),
        timestamp: 1_700_000_000_000_000_000i64 + 4,
    }
}

fn run_sbe_vector(vector: &ConformanceVector) -> Result<()> {
    use fig_core::sbe::{
        encode_account_summary, encode_aggregate_trade_batch, encode_aggregate_trade_event,
        encode_aggregate_trade_request, encode_all_mids_batch, encode_all_mids_request,
        encode_balance_snapshot, encode_balance_update, encode_best_bid_offer,
        encode_cancel_reject, encode_cancel_replace_request, encode_cancel_request,
        encode_candle_bar, encode_candle_bar_batch, encode_candle_bar_event,
        encode_candle_bar_request, encode_capabilities_request, encode_capabilities_response,
        encode_execution_report, encode_fill_history_batch, encode_fill_history_request,
        encode_funding_history_batch, encode_funding_history_request, encode_funding_payment,
        encode_instrument_catalog_request, encode_instrument_catalog_response,
        encode_ledger_history_batch, encode_ledger_history_request, encode_ledger_update,
        encode_liquidation_trade_event, encode_margin_summary, encode_margin_update,
        encode_mark_price_request, encode_mark_price_update,
        encode_market_data_incremental_refresh, encode_market_data_snapshot, encode_mini_ticker,
        encode_open_orders_request, encode_open_orders_snapshot, encode_order_book_delta,
        encode_order_book_request, encode_order_book_snapshot, encode_order_history_batch,
        encode_order_history_request, encode_order_list_status, encode_position_request,
        encode_position_snapshot, encode_position_update, encode_public_trade_batch,
        encode_public_trade_event, encode_symbol_ticker, encode_ticker_request,
        encode_trade_history_request, encode_user_liquidation,
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
        "CapabilitiesResponse" => encode_capabilities_response(&sample_capabilities_response()),
        "MarkPriceUpdate" => encode_mark_price_update(&sample_mark_price_update()),
        "MarketDataSnapshot" => encode_market_data_snapshot(&sample_market_data_snapshot()),
        "MiniTicker" => encode_mini_ticker(&sample_mini_ticker()),
        "OpenOrdersSnapshot" => encode_open_orders_snapshot(&sample_open_orders_snapshot()),
        "OrderBookDelta" => encode_order_book_delta(&sample_order_book_delta()),
        "OrderHistoryBatch" => encode_order_history_batch(&sample_order_history_batch()),
        "OrderHistoryRequest" => encode_order_history_request(&sample_order_history_request()),
        "OrderListStatus" => encode_order_list_status(&sample_order_list_status()),
        "PositionUpdate" => encode_position_update(&sample_position_update()),
        "AccountSummary" => encode_account_summary(&sample_account_summary()),
        "AggregateTradeBatch" => encode_aggregate_trade_batch(&sample_aggregate_trade_batch()),
        "AggregateTradeEvent" => encode_aggregate_trade_event(&sample_aggregate_trade_event()),
        "AggregateTradeRequest" => {
            encode_aggregate_trade_request(&sample_aggregate_trade_request())
        }
        "AllMidsBatch" => encode_all_mids_batch(&sample_all_mids_batch()),
        "AllMidsRequest" => encode_all_mids_request(&sample_all_mids_request()),
        "BalanceUpdate" => encode_balance_update(&sample_balance_update()),
        "BestBidOffer" => encode_best_bid_offer(&sample_best_bid_offer()),
        "CancelReject" => encode_cancel_reject(&sample_cancel_reject()),
        "CancelReplaceRequest" => encode_cancel_replace_request(&sample_cancel_replace_request()),
        "CancelRequest" => encode_cancel_request(&sample_cancel_request()),
        "CandleBarEvent" => encode_candle_bar_event(&sample_candle_bar_event()),
        "CandleBarRequest" => encode_candle_bar_request(&sample_candle_bar_request()),
        "CapabilitiesRequest" => encode_capabilities_request(&sample_capabilities_request()),
        "ExecutionReport" => encode_execution_report(&sample_execution_report()),
        "FillHistoryBatch" => encode_fill_history_batch(&sample_fill_history_batch()),
        "FillHistoryRequest" => encode_fill_history_request(&sample_fill_history_request()),
        "FundingHistoryBatch" => encode_funding_history_batch(&sample_funding_history_batch()),
        "FundingHistoryRequest" => {
            encode_funding_history_request(&sample_funding_history_request())
        }
        "FundingPayment" => encode_funding_payment(&sample_funding_payment()),
        "InstrumentCatalogRequest" => {
            encode_instrument_catalog_request(&sample_instrument_catalog_request())
        }
        "InstrumentCatalogResponse" => {
            encode_instrument_catalog_response(&sample_instrument_catalog_response())
        }
        "LedgerHistoryBatch" => encode_ledger_history_batch(&sample_ledger_history_batch()),
        "LedgerHistoryRequest" => encode_ledger_history_request(&sample_ledger_history_request()),
        "LedgerUpdate" => encode_ledger_update(&sample_ledger_update()),
        "LiquidationTradeEvent" => {
            encode_liquidation_trade_event(&sample_liquidation_trade_event())
        }
        "MarginSummary" => encode_margin_summary(&sample_margin_summary()),
        "MarginUpdate" => encode_margin_update(&sample_margin_update()),
        "MarkPriceRequest" => encode_mark_price_request(&sample_mark_price_request()),
        "MarketDataIncrementalRefresh" => {
            encode_market_data_incremental_refresh(&sample_market_data_incremental_refresh())
        }
        "OpenOrdersRequest" => encode_open_orders_request(&sample_open_orders_request()),
        "OrderBookRequest" => encode_order_book_request(&sample_order_book_request()),
        "PositionRequest" => encode_position_request(&sample_position_request()),
        "PositionSnapshot" => encode_position_snapshot(&sample_position_snapshot()),
        "PublicTradeBatch" => encode_public_trade_batch(&sample_public_trade_batch()),
        "PublicTradeEvent" => encode_public_trade_event(&sample_public_trade_event()),
        "TickerRequest" => encode_ticker_request(&sample_ticker_request()),
        "TradeHistoryRequest" => encode_trade_history_request(&sample_trade_history_request()),
        "UserLiquidation" => encode_user_liquidation(&sample_user_liquidation()),
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
    if vector.message_type == "CapabilitiesResponse" {
        // Round-trips the list-of-scalar layout (list<uint8>, list<string>, list<enum>).
        let want = sample_capabilities_response();
        let decoded = fig_core::sbe::decode_capabilities_response(&encoded)
            .map_err(|e| anyhow!("sbe decode: {e}"))?;
        if decoded != want {
            return Err(anyhow!("sbe round-trip capabilities mismatch: {decoded:?}"));
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
