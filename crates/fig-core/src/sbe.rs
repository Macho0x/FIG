//! Thin bridge from `messages` types to FSL-generated SBE wire codecs.

use crate::generated::sbe_wire::*;
use crate::messages::{
    BalanceSnapshot, CancelRequest, CandleBar, CandleBarBatch, ExecutionReport, NewOrderSingle,
    OrdStatus, OrderBookSnapshot, OrderType, SecurityIdSource, Side, SymbolTicker, TimeInForce,
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

/// Encode a `BalanceSnapshot` SBE message (template 31).
pub fn encode_balance_snapshot(snap: &BalanceSnapshot) -> Vec<u8> {
    BalanceSnapshotEncoder::encode(
        snap.account.clone(),
        snap.balances.clone(),
        snap.is_snapshot,
    )
}
