//! Thin bridge from `messages` types to FSL-generated SBE wire codecs.

use crate::generated::sbe_wire::*;
use crate::messages::{
    AccountSummary, AggregateTradeBatch, AggregateTradeEvent, AggregateTradeRequest, AllMidsBatch,
    AllMidsRequest, BalanceSnapshot, BalanceUpdate, BestBidOffer, CancelReject,
    CancelReplaceRequest, CancelRequest, CandleBar, CandleBarBatch, CandleBarEvent,
    CandleBarRequest, CapabilitiesRequest, ExecutionReport, FillHistoryBatch, FillHistoryRequest,
    FundingHistoryBatch, FundingHistoryRequest, FundingPayment, InstrumentCatalogRequest,
    InstrumentCatalogResponse, LedgerHistoryBatch, LedgerHistoryRequest, LedgerUpdate,
    LiquidationTradeEvent, MarginSummary, MarginUpdate, MarkPriceRequest, MarkPriceUpdate,
    MarketDataIncrementalRefresh, MarketDataSnapshot, MassCancelReport, MassCancelRequest,
    MiniTicker, NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot, OrdStatus, OrderBookDelta,
    OrderBookRequest, OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    OrderType, PositionRequest, PositionSnapshot, PositionUpdate, PublicTradeBatch,
    PublicTradeEvent, SecurityIdSource, SecurityStatus, SecurityStatusRequest, Side, SymbolTicker,
    TickerRequest, TimeInForce, TradeHistoryRequest, TradingSessionStatus,
    TradingSessionStatusRequest, UserLiquidation,
};

/// Errors that can occur during SBE encoding or decoding.
#[derive(Debug, thiserror::Error)]
pub enum SbeError {
    #[error("buffer too short: need {needed} bytes, have {have}")]
    BufferTooShort { needed: usize, have: usize },

    #[error("invalid template id: {0}")]
    InvalidTemplateId(u16),

    #[error("invalid schema id: {0}")]
    InvalidSchemaId(u16),

    #[error("wire decode error: {0}")]
    Decode(String),

    #[error("invalid enum value for {field}: {value}")]
    InvalidEnumValue { field: &'static str, value: u8 },
}

fn decode_err(e: String) -> SbeError {
    SbeError::Decode(e)
}

fn unmap_side_u8(v: u8) -> Result<Side, SbeError> {
    Side::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "side",
        value: v,
    })
}

fn map_side(s: Side) -> NewOrderSingleSide {
    NewOrderSingleSide::from_value(s.to_value()).expect("side")
}

fn unmap_order_type_u8(v: u8) -> Result<OrderType, SbeError> {
    OrderType::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "order_type",
        value: v,
    })
}

fn map_order_type(t: OrderType) -> NewOrderSingleOrderType {
    NewOrderSingleOrderType::from_value(t.to_value()).expect("order_type")
}

fn unmap_tif_u8(v: u8) -> Result<TimeInForce, SbeError> {
    TimeInForce::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "time_in_force",
        value: v,
    })
}

fn map_tif(t: TimeInForce) -> NewOrderSingleTimeInForce {
    NewOrderSingleTimeInForce::from_value(t.to_value()).expect("time_in_force")
}

fn map_id_source(s: SecurityIdSource) -> NewOrderSingleIdSource {
    NewOrderSingleIdSource::from_value(s.to_value()).expect("id_source")
}

fn unmap_id_source_u8(v: u8) -> Result<SecurityIdSource, SbeError> {
    SecurityIdSource::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "id_source",
        value: v,
    })
}

fn map_exec_type(t: crate::messages::ExecType) -> ExecutionReportExecType {
    ExecutionReportExecType::from_value(t.to_value()).expect("exec_type")
}

fn unmap_exec_type_u8(v: u8) -> Result<crate::messages::ExecType, SbeError> {
    crate::messages::ExecType::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "exec_type",
        value: v,
    })
}

fn map_ord_status(s: OrdStatus) -> ExecutionReportOrdStatus {
    ExecutionReportOrdStatus::from_value(s.to_value()).expect("ord_status")
}

fn unmap_ord_status_u8(v: u8) -> Result<OrdStatus, SbeError> {
    OrdStatus::from_value(v).ok_or(SbeError::InvalidEnumValue {
        field: "ord_status",
        value: v,
    })
}

fn map_cancel_side(s: Side) -> CancelRequestSide {
    CancelRequestSide::from_value(s.to_value())
        .ok_or(SbeError::InvalidEnumValue {
            field: "side",
            value: s.to_value(),
        })
        .expect("cancel side")
}

fn map_exec_side(s: Side) -> ExecutionReportSide {
    ExecutionReportSide::from_value(s.to_value()).expect("side")
}

/// Encode a `NewOrderSingle` into SBE binary format.
pub fn encode_new_order_single(order: &NewOrderSingle) -> Vec<u8> {
    NewOrderSingleEncoder::encode(
        order.cl_ord_id.clone(),
        map_side(order.side),
        order.order_qty.clone(),
        order.price.clone(),
        order.stop_price.clone(),
        order.symbol.clone(),
        map_order_type(order.order_type),
        map_tif(order.time_in_force),
        order.expire_time,
        order.account.clone(),
        order.strategy_id.clone(),
        order.security_id.clone(),
        order.id_source.map(map_id_source),
        order.security_exchange.clone(),
        order.post_only,
        order.reduce_only,
    )
}

/// Decode a `NewOrderSingle` from SBE binary format.
pub fn decode_new_order_single(buf: &[u8]) -> Result<NewOrderSingle, SbeError> {
    let d = NewOrderSingleDecoder::decode(buf).map_err(decode_err)?;
    Ok(NewOrderSingle {
        cl_ord_id: d.cl_ord_id,
        side: unmap_side_u8(d.side.to_value())?,
        order_qty: d.order_qty,
        price: d.price,
        stop_price: d.stop_price,
        symbol: d.symbol,
        order_type: unmap_order_type_u8(d.order_type.to_value())?,
        time_in_force: unmap_tif_u8(d.time_in_force.to_value())?,
        expire_time: d.expire_time,
        account: d.account,
        strategy_id: d.strategy_id,
        security_id: d.security_id,
        id_source: d
            .id_source
            .map(|s| unmap_id_source_u8(s.to_value()))
            .transpose()?,
        security_exchange: d.security_exchange,
        post_only: d.post_only,
        reduce_only: d.reduce_only,
    })
}

/// Encode an `ExecutionReport` into SBE binary format.
pub fn encode_execution_report(report: &ExecutionReport) -> Vec<u8> {
    ExecutionReportEncoder::encode(
        report.cl_ord_id.clone(),
        report.order_id.clone(),
        report.exec_id.clone(),
        map_exec_type(report.exec_type),
        map_ord_status(report.ord_status),
        map_exec_side(report.side),
        report.last_qty.clone(),
        report.last_price.clone(),
        report.leaves_qty.clone(),
        report.cum_qty.clone(),
        report.avg_price.clone(),
        report.symbol.clone(),
        report.transact_time,
    )
}

/// Decode an `ExecutionReport` from SBE binary format.
pub fn decode_execution_report(buf: &[u8]) -> Result<ExecutionReport, SbeError> {
    let d = ExecutionReportDecoder::decode(buf).map_err(decode_err)?;
    Ok(ExecutionReport {
        cl_ord_id: d.cl_ord_id,
        order_id: d.order_id,
        exec_id: d.exec_id,
        exec_type: unmap_exec_type_u8(d.exec_type.to_value())?,
        ord_status: unmap_ord_status_u8(d.ord_status.to_value())?,
        side: unmap_side_u8(d.side.to_value())?,
        last_qty: d.last_qty,
        last_price: d.last_price,
        leaves_qty: d.leaves_qty,
        cum_qty: d.cum_qty,
        avg_price: d.avg_price,
        symbol: d.symbol,
        transact_time: d.transact_time,
    })
}

/// Encode a `CancelRequest` into SBE binary format.
pub fn encode_cancel_request(cancel: &CancelRequest) -> Vec<u8> {
    CancelRequestEncoder::encode(
        cancel.cl_ord_id.clone(),
        cancel.orig_cl_ord_id.clone(),
        cancel.symbol.clone(),
        map_cancel_side(cancel.side),
        cancel.order_qty.clone(),
    )
}

/// Decode a `CancelRequest` from SBE binary format.
pub fn decode_cancel_request(buf: &[u8]) -> Result<CancelRequest, SbeError> {
    let d = CancelRequestDecoder::decode(buf).map_err(decode_err)?;
    Ok(CancelRequest {
        cl_ord_id: d.cl_ord_id,
        orig_cl_ord_id: d.orig_cl_ord_id,
        symbol: d.symbol,
        side: unmap_side_u8(d.side.to_value())?,
        order_qty: d.order_qty,
    })
}

/// Encode a `CandleBar` as a `CandleBarEvent` SBE message (template 20).
pub fn encode_candle_bar(bar: &CandleBar) -> Vec<u8> {
    CandleBarEventEncoder::encode(bar.clone())
}

/// Encode a `CandleBarBatch` SBE message (template 22).
pub fn encode_candle_bar_batch(batch: &CandleBarBatch) -> Vec<u8> {
    CandleBarBatchEncoder::encode(
        batch.symbol.clone(),
        batch.interval.clone(),
        batch.bars.clone(),
        batch.has_more,
        batch.next_cursor.clone(),
    )
}

/// Encode a `SymbolTicker` SBE message (template 27).
pub fn encode_symbol_ticker(ticker: &SymbolTicker) -> Vec<u8> {
    SymbolTickerEncoder::encode(
        ticker.symbol.clone(),
        ticker.last_price.clone(),
        ticker.price_change,
        ticker.price_change_pct,
        ticker.volume.clone(),
        ticker.high.clone(),
        ticker.low.clone(),
        ticker.open.clone(),
        ticker.timestamp,
        ticker.is_snapshot,
    )
}

/// Encode an `OrderBookSnapshot` SBE message (template 9).
pub fn encode_order_book_snapshot(snap: &OrderBookSnapshot) -> Vec<u8> {
    OrderBookSnapshotEncoder::encode(
        snap.symbol.clone(),
        snap.exchange.clone(),
        snap.bids.clone(),
        snap.asks.clone(),
        snap.timestamp,
        snap.sequence,
        snap.is_snapshot,
    )
}

/// Decode a `CapabilitiesResponse` SBE message.
pub fn decode_capabilities_response(
    buf: &[u8],
) -> Result<crate::messages::CapabilitiesResponse, SbeError> {
    let d = CapabilitiesResponseDecoder::decode(buf).map_err(decode_err)?;
    Ok(crate::messages::CapabilitiesResponse {
        schema_ids: d.schema_ids,
        paths: d
            .paths
            .into_iter()
            .map(|p| crate::messages::CapabilityPath {
                path: p.path,
                pattern: p.pattern,
                auth_required: p.auth_required,
            })
            .collect(),
        symbols: d.symbols,
        intervals: d.intervals,
    })
}

/// Encode a `MarkPriceUpdate` SBE message.
pub fn encode_mark_price_update(m: &MarkPriceUpdate) -> Vec<u8> {
    MarkPriceUpdateEncoder::encode(
        m.symbol.clone(),
        m.mark_price.clone(),
        m.index_price.clone(),
        m.funding_rate,
        m.timestamp,
        m.is_snapshot,
    )
}

/// Encode a `MarketDataSnapshot` SBE message.
pub fn encode_market_data_snapshot(m: &MarketDataSnapshot) -> Vec<u8> {
    MarketDataSnapshotEncoder::encode(
        m.symbol.clone(),
        m.exchange.clone(),
        m.bids.clone(),
        m.asks.clone(),
        m.timestamp,
        m.sequence,
        m.is_snapshot,
    )
}

/// Encode a `MiniTicker` SBE message.
pub fn encode_mini_ticker(m: &MiniTicker) -> Vec<u8> {
    MiniTickerEncoder::encode(
        m.symbol.clone(),
        m.last_price.clone(),
        m.volume.clone(),
        m.timestamp,
        m.is_snapshot,
    )
}

/// Encode a `OpenOrdersSnapshot` SBE message.
pub fn encode_open_orders_snapshot(m: &OpenOrdersSnapshot) -> Vec<u8> {
    OpenOrdersSnapshotEncoder::encode(m.account.clone(), m.orders.clone(), m.is_snapshot)
}

/// Encode a `OrderBookDelta` SBE message.
pub fn encode_order_book_delta(m: &OrderBookDelta) -> Vec<u8> {
    OrderBookDeltaEncoder::encode(m.symbol.clone(), m.updates.clone(), m.timestamp, m.sequence)
}

/// Encode a `OrderHistoryBatch` SBE message.
pub fn encode_order_history_batch(m: &OrderHistoryBatch) -> Vec<u8> {
    OrderHistoryBatchEncoder::encode(
        m.account.clone(),
        m.orders.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}

/// Encode a `OrderHistoryRequest` SBE message.
pub fn encode_order_history_request(m: &OrderHistoryRequest) -> Vec<u8> {
    OrderHistoryRequestEncoder::encode(
        m.account.clone(),
        m.symbol.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}

/// Encode a `OrderListStatus` SBE message.
pub fn encode_order_list_status(m: &OrderListStatus) -> Vec<u8> {
    OrderListStatusEncoder::encode(
        m.account.clone(),
        m.list_id.clone(),
        m.status,
        m.symbol.clone(),
    )
}

/// Encode a `PositionUpdate` SBE message.
pub fn encode_position_update(m: &PositionUpdate) -> Vec<u8> {
    PositionUpdateEncoder::encode(
        m.account.clone(),
        m.symbol.clone(),
        m.qty.clone(),
        m.entry_price.clone(),
        m.unrealized_pnl,
    )
}

/// Encode a `AccountSummary` SBE message.
pub fn encode_account_summary(m: &AccountSummary) -> Vec<u8> {
    AccountSummaryEncoder::encode(
        m.account.clone(),
        m.balance,
        m.buying_power,
        m.currency.clone(),
    )
}
/// Encode a `AggregateTradeBatch` SBE message.
pub fn encode_aggregate_trade_batch(m: &AggregateTradeBatch) -> Vec<u8> {
    AggregateTradeBatchEncoder::encode(
        m.symbol.clone(),
        m.trades.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}
/// Encode a `AggregateTradeEvent` SBE message.
pub fn encode_aggregate_trade_event(m: &AggregateTradeEvent) -> Vec<u8> {
    AggregateTradeEventEncoder::encode(m.trade.clone())
}
/// Encode a `AggregateTradeRequest` SBE message.
pub fn encode_aggregate_trade_request(m: &AggregateTradeRequest) -> Vec<u8> {
    AggregateTradeRequestEncoder::encode(
        m.symbol.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `AllMidsBatch` SBE message.
pub fn encode_all_mids_batch(m: &AllMidsBatch) -> Vec<u8> {
    AllMidsBatchEncoder::encode(m.tickers.clone())
}
/// Encode a `AllMidsRequest` SBE message.
pub fn encode_all_mids_request(m: &AllMidsRequest) -> Vec<u8> {
    let _ = m;
    AllMidsRequestEncoder::encode()
}
/// Encode a `BalanceUpdate` SBE message.
pub fn encode_balance_update(m: &BalanceUpdate) -> Vec<u8> {
    BalanceUpdateEncoder::encode(
        m.account.clone(),
        m.asset.clone(),
        m.delta,
        m.total,
        m.available,
        m.reason,
    )
}
/// Encode a `BestBidOffer` SBE message.
pub fn encode_best_bid_offer(m: &BestBidOffer) -> Vec<u8> {
    BestBidOfferEncoder::encode(
        m.symbol.clone(),
        m.bid_price.clone(),
        m.bid_qty.clone(),
        m.ask_price.clone(),
        m.ask_qty.clone(),
        m.timestamp,
        m.is_snapshot,
    )
}
/// Encode a `CancelReject` SBE message.
pub fn encode_cancel_reject(m: &CancelReject) -> Vec<u8> {
    CancelRejectEncoder::encode(
        m.cl_ord_id.clone(),
        m.orig_cl_ord_id.clone(),
        CancelRejectRejectReason::from_value(m.reject_reason.to_value()).expect("reject_reason"),
        m.symbol.clone(),
    )
}
/// Encode a `CancelReplaceRequest` SBE message.
pub fn encode_cancel_replace_request(m: &CancelReplaceRequest) -> Vec<u8> {
    CancelReplaceRequestEncoder::encode(
        m.cl_ord_id.clone(),
        m.orig_cl_ord_id.clone(),
        m.symbol.clone(),
        CancelReplaceRequestSide::from_value(m.side.to_value()).expect("side"),
        m.order_qty.clone(),
        m.price.clone(),
    )
}
/// Encode a `CandleBarEvent` SBE message.
pub fn encode_candle_bar_event(m: &CandleBarEvent) -> Vec<u8> {
    CandleBarEventEncoder::encode(m.bar.clone())
}
/// Encode a `CandleBarRequest` SBE message.
pub fn encode_candle_bar_request(m: &CandleBarRequest) -> Vec<u8> {
    CandleBarRequestEncoder::encode(
        m.symbol.clone(),
        m.interval.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `CapabilitiesRequest` SBE message.
pub fn encode_capabilities_request(m: &CapabilitiesRequest) -> Vec<u8> {
    let _ = m;
    CapabilitiesRequestEncoder::encode()
}
/// Encode a `FillHistoryBatch` SBE message.
pub fn encode_fill_history_batch(m: &FillHistoryBatch) -> Vec<u8> {
    FillHistoryBatchEncoder::encode(
        m.account.clone(),
        m.fills.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}
/// Encode a `FillHistoryRequest` SBE message.
pub fn encode_fill_history_request(m: &FillHistoryRequest) -> Vec<u8> {
    FillHistoryRequestEncoder::encode(
        m.account.clone(),
        m.symbol.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `FundingHistoryBatch` SBE message.
pub fn encode_funding_history_batch(m: &FundingHistoryBatch) -> Vec<u8> {
    FundingHistoryBatchEncoder::encode(
        m.account.clone(),
        m.payments.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}
/// Encode a `FundingHistoryRequest` SBE message.
pub fn encode_funding_history_request(m: &FundingHistoryRequest) -> Vec<u8> {
    FundingHistoryRequestEncoder::encode(
        m.account.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `FundingPayment` SBE message.
pub fn encode_funding_payment(m: &FundingPayment) -> Vec<u8> {
    FundingPaymentEncoder::encode(
        m.account.clone(),
        m.symbol.clone(),
        m.amount,
        m.rate,
        m.timestamp,
    )
}
/// Encode a `InstrumentCatalogRequest` SBE message.
pub fn encode_instrument_catalog_request(m: &InstrumentCatalogRequest) -> Vec<u8> {
    let _ = m;
    InstrumentCatalogRequestEncoder::encode()
}
/// Encode a `InstrumentCatalogResponse` SBE message.
pub fn encode_instrument_catalog_response(m: &InstrumentCatalogResponse) -> Vec<u8> {
    InstrumentCatalogResponseEncoder::encode(m.instruments.clone())
}
/// Encode a `LedgerHistoryBatch` SBE message.
pub fn encode_ledger_history_batch(m: &LedgerHistoryBatch) -> Vec<u8> {
    LedgerHistoryBatchEncoder::encode(
        m.account.clone(),
        m.entries.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}
/// Encode a `LedgerHistoryRequest` SBE message.
pub fn encode_ledger_history_request(m: &LedgerHistoryRequest) -> Vec<u8> {
    LedgerHistoryRequestEncoder::encode(
        m.account.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `LedgerUpdate` SBE message.
pub fn encode_ledger_update(m: &LedgerUpdate) -> Vec<u8> {
    LedgerUpdateEncoder::encode(
        m.account.clone(),
        m.asset.clone(),
        m.delta,
        m.kind,
        m.timestamp,
        m.reference_id.clone(),
    )
}
/// Encode a `LiquidationTradeEvent` SBE message.
pub fn encode_liquidation_trade_event(m: &LiquidationTradeEvent) -> Vec<u8> {
    LiquidationTradeEventEncoder::encode(m.trade.clone())
}
/// Encode a `MarginSummary` SBE message.
pub fn encode_margin_summary(m: &MarginSummary) -> Vec<u8> {
    MarginSummaryEncoder::encode(
        m.account.clone(),
        m.balance,
        m.buying_power,
        m.equity,
        m.margin_used,
        m.available,
        m.currency.clone(),
    )
}
/// Encode a `MarginUpdate` SBE message.
pub fn encode_margin_update(m: &MarginUpdate) -> Vec<u8> {
    MarginUpdateEncoder::encode(m.account.clone(), m.summary.clone(), m.is_snapshot)
}
/// Encode a `MarkPriceRequest` SBE message.
pub fn encode_mark_price_request(m: &MarkPriceRequest) -> Vec<u8> {
    MarkPriceRequestEncoder::encode(m.symbol.clone())
}
/// Encode a `MarketDataIncrementalRefresh` SBE message.
pub fn encode_market_data_incremental_refresh(m: &MarketDataIncrementalRefresh) -> Vec<u8> {
    MarketDataIncrementalRefreshEncoder::encode(
        m.symbol.clone(),
        m.updates.clone(),
        m.timestamp,
        m.sequence,
    )
}
/// Encode a `OpenOrdersRequest` SBE message.
pub fn encode_open_orders_request(m: &OpenOrdersRequest) -> Vec<u8> {
    OpenOrdersRequestEncoder::encode(m.account.clone(), m.symbol.clone())
}
/// Encode a `OrderBookRequest` SBE message.
pub fn encode_order_book_request(m: &OrderBookRequest) -> Vec<u8> {
    OrderBookRequestEncoder::encode(m.symbol.clone(), m.depth, m.at_time)
}
/// Encode a `PositionRequest` SBE message.
pub fn encode_position_request(m: &PositionRequest) -> Vec<u8> {
    PositionRequestEncoder::encode(m.account.clone())
}
/// Encode a `PositionSnapshot` SBE message.
pub fn encode_position_snapshot(m: &PositionSnapshot) -> Vec<u8> {
    PositionSnapshotEncoder::encode(m.account.clone(), m.positions.clone(), m.is_snapshot)
}
/// Encode a `PublicTradeBatch` SBE message.
pub fn encode_public_trade_batch(m: &PublicTradeBatch) -> Vec<u8> {
    PublicTradeBatchEncoder::encode(
        m.symbol.clone(),
        m.trades.clone(),
        m.has_more,
        m.next_cursor.clone(),
    )
}
/// Encode a `PublicTradeEvent` SBE message.
pub fn encode_public_trade_event(m: &PublicTradeEvent) -> Vec<u8> {
    PublicTradeEventEncoder::encode(m.trade.clone())
}
/// Encode a `TickerRequest` SBE message.
pub fn encode_ticker_request(m: &TickerRequest) -> Vec<u8> {
    TickerRequestEncoder::encode(m.symbol.clone())
}
/// Encode a `TradeHistoryRequest` SBE message.
pub fn encode_trade_history_request(m: &TradeHistoryRequest) -> Vec<u8> {
    TradeHistoryRequestEncoder::encode(
        m.symbol.clone(),
        m.start_time,
        m.end_time,
        m.limit,
        m.cursor.clone(),
    )
}
/// Encode a `UserLiquidation` SBE message.
pub fn encode_user_liquidation(m: &UserLiquidation) -> Vec<u8> {
    UserLiquidationEncoder::encode(
        m.account.clone(),
        m.symbol.clone(),
        m.qty.clone(),
        m.price.clone(),
        m.timestamp,
    )
}
/// Encode a `CapabilitiesResponse` SBE message — exercises list-of-scalar fields
/// (`list<uint8>`, `list<Symbol>`, `list<CandleInterval>`).
pub fn encode_capabilities_response(resp: &crate::messages::CapabilitiesResponse) -> Vec<u8> {
    CapabilitiesResponseEncoder::encode(
        resp.schema_ids.clone(),
        resp.paths.clone(),
        resp.symbols.clone(),
        resp.intervals.clone(),
    )
}

/// Encode a `BalanceSnapshot` SBE message (template 33).
pub fn encode_balance_snapshot(snap: &BalanceSnapshot) -> Vec<u8> {
    BalanceSnapshotEncoder::encode(
        snap.account.clone(),
        snap.balances.clone(),
        snap.is_snapshot,
    )
}

/// Encode a `TradingSessionStatusRequest` SBE message.
pub fn encode_trading_session_status_request(m: &TradingSessionStatusRequest) -> Vec<u8> {
    TradingSessionStatusRequestEncoder::encode(m.symbol.clone())
}

/// Encode a `TradingSessionStatus` SBE message.
pub fn encode_trading_session_status(m: &TradingSessionStatus) -> Vec<u8> {
    TradingSessionStatusEncoder::encode(
        m.symbol.clone(),
        m.trading_session_id.clone(),
        m.status,
        m.halt_reason,
        m.timestamp,
        m.is_snapshot,
    )
}

/// Encode a `SecurityStatusRequest` SBE message.
pub fn encode_security_status_request(m: &SecurityStatusRequest) -> Vec<u8> {
    SecurityStatusRequestEncoder::encode(m.symbol.clone())
}

/// Encode a `SecurityStatus` SBE message.
pub fn encode_security_status(m: &SecurityStatus) -> Vec<u8> {
    SecurityStatusEncoder::encode(
        m.symbol.clone(),
        m.status,
        m.halt_reason,
        m.timestamp,
        m.is_snapshot,
    )
}

/// Encode a `MassCancelRequest` SBE message.
pub fn encode_mass_cancel_request(m: &MassCancelRequest) -> Vec<u8> {
    MassCancelRequestEncoder::encode(
        m.cl_ord_id.clone(),
        m.account.clone(),
        m.symbol.clone(),
        m.side
            .and_then(|s| MassCancelRequestSide::from_value(s.to_value())),
    )
}

/// Encode a `MassCancelReport` SBE message.
pub fn encode_mass_cancel_report(m: &MassCancelReport) -> Vec<u8> {
    MassCancelReportEncoder::encode(
        m.cl_ord_id.clone(),
        m.account.clone(),
        m.symbol.clone(),
        m.total_affected,
        m.status,
        m.timestamp,
    )
}
