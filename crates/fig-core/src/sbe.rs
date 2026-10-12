//! Thin bridge from `messages` types to FSL-generated SBE wire codecs.

use crate::generated::sbe_wire::*;
use crate::messages::{
    AccountSummary, AggregateTradeBatch, AggregateTradeEvent, AggregateTradeRequest, AllMidsBatch,
    AllMidsRequest, BalanceSnapshot, BalanceUpdate, BatchReport, BatchRequest, BestBidOffer,
    CancelReject, CancelReplaceRequest, CancelRequest, CandleBar, CandleBarBatch, CandleBarEvent,
    CandleBarRequest, CapabilitiesRequest, ExecutionReport, FillHistoryBatch, FillHistoryRequest,
    FundingHistoryBatch, FundingHistoryRequest, FundingPayment, InstrumentCatalogRequest,
    InstrumentCatalogResponse, LedgerHistoryBatch, LedgerHistoryRequest, LedgerUpdate,
    LiquidationTradeEvent, MarginSummary, MarginUpdate, MarkPriceRequest, MarkPriceUpdate,
    MarketDataIncrementalRefresh, MarketDataSnapshot, MassCancelReport, MassCancelRequest,
    MiniTicker, NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot, OrderBookDelta,
    OrderBookRequest, OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    PositionRequest, PositionSnapshot, PositionUpdate, PriceLimits, PriceLimitsRequest,
    PublicTradeBatch, PublicTradeEvent, SecurityStatus, SecurityStatusRequest, SymbolTicker,
    TickerRequest, TradeHistoryRequest, TradingSessionStatus, TradingSessionStatusRequest,
    UserLiquidation,
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

fn encode_into_cleared(buf: &mut Vec<u8>, write: impl FnOnce(&mut Vec<u8>)) {
    buf.clear();
    write(buf);
}

/// Encode a `NewOrderSingle` into SBE binary format.
pub fn encode_new_order_single(order: &NewOrderSingle) -> Vec<u8> {
    NewOrderSingleEncoder::encode(order)
}

/// Encode a `NewOrderSingle` into `buf`, replacing previous contents.
pub fn encode_new_order_single_into(order: &NewOrderSingle, buf: &mut Vec<u8>) {
    encode_into_cleared(buf, |buf| NewOrderSingleEncoder::encode_into(order, buf));
}

/// Decode a `NewOrderSingle` from SBE binary format.
pub fn decode_new_order_single(buf: &[u8]) -> Result<NewOrderSingle, SbeError> {
    NewOrderSingleDecoder::decode(buf).map_err(decode_err)
}

/// Encode an `ExecutionReport` into SBE binary format.
pub fn encode_execution_report(report: &ExecutionReport) -> Vec<u8> {
    ExecutionReportEncoder::encode(report)
}

/// Encode an `ExecutionReport` into `buf`, replacing previous contents.
pub fn encode_execution_report_into(report: &ExecutionReport, buf: &mut Vec<u8>) {
    encode_into_cleared(buf, |buf| ExecutionReportEncoder::encode_into(report, buf));
}

/// Decode an `ExecutionReport` from SBE binary format.
pub fn decode_execution_report(buf: &[u8]) -> Result<ExecutionReport, SbeError> {
    ExecutionReportDecoder::decode(buf).map_err(decode_err)
}

/// Encode a `CancelRequest` into SBE binary format.
pub fn encode_cancel_request(cancel: &CancelRequest) -> Vec<u8> {
    CancelRequestEncoder::encode(cancel)
}

/// Encode a `CancelRequest` into `buf`, replacing previous contents.
pub fn encode_cancel_request_into(cancel: &CancelRequest, buf: &mut Vec<u8>) {
    encode_into_cleared(buf, |buf| CancelRequestEncoder::encode_into(cancel, buf));
}

/// Decode a `CancelRequest` from SBE binary format.
pub fn decode_cancel_request(buf: &[u8]) -> Result<CancelRequest, SbeError> {
    CancelRequestDecoder::decode(buf).map_err(decode_err)
}

/// Encode a `CandleBar` as a `CandleBarEvent` SBE message (template 20).
pub fn encode_candle_bar(bar: &CandleBar) -> Vec<u8> {
    encode_candle_bar_event(&CandleBarEvent { bar: bar.clone() })
}

/// Encode a `CandleBarBatch` SBE message (template 22).
pub fn encode_candle_bar_batch(batch: &CandleBarBatch) -> Vec<u8> {
    CandleBarBatchEncoder::encode(batch)
}

/// Encode a `SymbolTicker` SBE message (template 27).
pub fn encode_symbol_ticker(ticker: &SymbolTicker) -> Vec<u8> {
    SymbolTickerEncoder::encode(ticker)
}

/// Encode an `OrderBookSnapshot` SBE message (template 9).
pub fn encode_order_book_snapshot(snap: &OrderBookSnapshot) -> Vec<u8> {
    OrderBookSnapshotEncoder::encode(snap)
}

/// Decode a `CapabilitiesResponse` SBE message.
pub fn decode_capabilities_response(
    buf: &[u8],
) -> Result<crate::messages::CapabilitiesResponse, SbeError> {
    CapabilitiesResponseDecoder::decode(buf).map_err(decode_err)
}

/// Encode a `MarkPriceUpdate` SBE message.
pub fn encode_mark_price_update(m: &MarkPriceUpdate) -> Vec<u8> {
    MarkPriceUpdateEncoder::encode(m)
}

/// Encode a `MarketDataSnapshot` SBE message.
pub fn encode_market_data_snapshot(m: &MarketDataSnapshot) -> Vec<u8> {
    MarketDataSnapshotEncoder::encode(m)
}

/// Encode a `MiniTicker` SBE message.
pub fn encode_mini_ticker(m: &MiniTicker) -> Vec<u8> {
    MiniTickerEncoder::encode(m)
}

/// Encode a `OpenOrdersSnapshot` SBE message.
pub fn encode_open_orders_snapshot(m: &OpenOrdersSnapshot) -> Vec<u8> {
    OpenOrdersSnapshotEncoder::encode(m)
}

/// Encode a `OrderBookDelta` SBE message.
pub fn encode_order_book_delta(m: &OrderBookDelta) -> Vec<u8> {
    OrderBookDeltaEncoder::encode(m)
}

/// Encode a `OrderHistoryBatch` SBE message.
pub fn encode_order_history_batch(m: &OrderHistoryBatch) -> Vec<u8> {
    OrderHistoryBatchEncoder::encode(m)
}

/// Encode a `OrderHistoryRequest` SBE message.
pub fn encode_order_history_request(m: &OrderHistoryRequest) -> Vec<u8> {
    OrderHistoryRequestEncoder::encode(m)
}

/// Encode a `OrderListStatus` SBE message.
pub fn encode_order_list_status(m: &OrderListStatus) -> Vec<u8> {
    OrderListStatusEncoder::encode(m)
}

/// Encode a `PositionUpdate` SBE message.
pub fn encode_position_update(m: &PositionUpdate) -> Vec<u8> {
    PositionUpdateEncoder::encode(m)
}

/// Encode a `AccountSummary` SBE message.
pub fn encode_account_summary(m: &AccountSummary) -> Vec<u8> {
    AccountSummaryEncoder::encode(m)
}
/// Encode a `AggregateTradeBatch` SBE message.
pub fn encode_aggregate_trade_batch(m: &AggregateTradeBatch) -> Vec<u8> {
    AggregateTradeBatchEncoder::encode(m)
}
/// Encode a `AggregateTradeEvent` SBE message.
pub fn encode_aggregate_trade_event(m: &AggregateTradeEvent) -> Vec<u8> {
    AggregateTradeEventEncoder::encode(m)
}
/// Encode a `AggregateTradeRequest` SBE message.
pub fn encode_aggregate_trade_request(m: &AggregateTradeRequest) -> Vec<u8> {
    AggregateTradeRequestEncoder::encode(m)
}
/// Encode a `AllMidsBatch` SBE message.
pub fn encode_all_mids_batch(m: &AllMidsBatch) -> Vec<u8> {
    AllMidsBatchEncoder::encode(m)
}
/// Encode a `AllMidsRequest` SBE message.
pub fn encode_all_mids_request(m: &AllMidsRequest) -> Vec<u8> {
    AllMidsRequestEncoder::encode(m)
}
/// Encode a `BalanceUpdate` SBE message.
pub fn encode_balance_update(m: &BalanceUpdate) -> Vec<u8> {
    BalanceUpdateEncoder::encode(m)
}
/// Encode a `BestBidOffer` SBE message.
pub fn encode_best_bid_offer(m: &BestBidOffer) -> Vec<u8> {
    BestBidOfferEncoder::encode(m)
}
/// Encode a `CancelReject` SBE message.
pub fn encode_cancel_reject(m: &CancelReject) -> Vec<u8> {
    CancelRejectEncoder::encode(m)
}
/// Encode a `CancelReplaceRequest` SBE message.
pub fn encode_cancel_replace_request(m: &CancelReplaceRequest) -> Vec<u8> {
    CancelReplaceRequestEncoder::encode(m)
}
/// Encode a `CandleBarEvent` SBE message.
pub fn encode_candle_bar_event(m: &CandleBarEvent) -> Vec<u8> {
    CandleBarEventEncoder::encode(m)
}
/// Encode a `CandleBarRequest` SBE message.
pub fn encode_candle_bar_request(m: &CandleBarRequest) -> Vec<u8> {
    CandleBarRequestEncoder::encode(m)
}
/// Encode a `CapabilitiesRequest` SBE message.
pub fn encode_capabilities_request(m: &CapabilitiesRequest) -> Vec<u8> {
    CapabilitiesRequestEncoder::encode(m)
}
/// Encode a `FillHistoryBatch` SBE message.
pub fn encode_fill_history_batch(m: &FillHistoryBatch) -> Vec<u8> {
    FillHistoryBatchEncoder::encode(m)
}
/// Encode a `FillHistoryRequest` SBE message.
pub fn encode_fill_history_request(m: &FillHistoryRequest) -> Vec<u8> {
    FillHistoryRequestEncoder::encode(m)
}
/// Encode a `FundingHistoryBatch` SBE message.
pub fn encode_funding_history_batch(m: &FundingHistoryBatch) -> Vec<u8> {
    FundingHistoryBatchEncoder::encode(m)
}
/// Encode a `FundingHistoryRequest` SBE message.
pub fn encode_funding_history_request(m: &FundingHistoryRequest) -> Vec<u8> {
    FundingHistoryRequestEncoder::encode(m)
}
/// Encode a `FundingPayment` SBE message.
pub fn encode_funding_payment(m: &FundingPayment) -> Vec<u8> {
    FundingPaymentEncoder::encode(m)
}
/// Encode a `InstrumentCatalogRequest` SBE message.
pub fn encode_instrument_catalog_request(m: &InstrumentCatalogRequest) -> Vec<u8> {
    InstrumentCatalogRequestEncoder::encode(m)
}
/// Encode a `InstrumentCatalogResponse` SBE message.
pub fn encode_instrument_catalog_response(m: &InstrumentCatalogResponse) -> Vec<u8> {
    InstrumentCatalogResponseEncoder::encode(m)
}
/// Encode a `LedgerHistoryBatch` SBE message.
pub fn encode_ledger_history_batch(m: &LedgerHistoryBatch) -> Vec<u8> {
    LedgerHistoryBatchEncoder::encode(m)
}
/// Encode a `LedgerHistoryRequest` SBE message.
pub fn encode_ledger_history_request(m: &LedgerHistoryRequest) -> Vec<u8> {
    LedgerHistoryRequestEncoder::encode(m)
}
/// Encode a `LedgerUpdate` SBE message.
pub fn encode_ledger_update(m: &LedgerUpdate) -> Vec<u8> {
    LedgerUpdateEncoder::encode(m)
}
/// Encode a `LiquidationTradeEvent` SBE message.
pub fn encode_liquidation_trade_event(m: &LiquidationTradeEvent) -> Vec<u8> {
    LiquidationTradeEventEncoder::encode(m)
}
/// Encode a `MarginSummary` SBE message.
pub fn encode_margin_summary(m: &MarginSummary) -> Vec<u8> {
    MarginSummaryEncoder::encode(m)
}
/// Encode a `MarginUpdate` SBE message.
pub fn encode_margin_update(m: &MarginUpdate) -> Vec<u8> {
    MarginUpdateEncoder::encode(m)
}
/// Encode a `MarkPriceRequest` SBE message.
pub fn encode_mark_price_request(m: &MarkPriceRequest) -> Vec<u8> {
    MarkPriceRequestEncoder::encode(m)
}
/// Encode a `MarketDataIncrementalRefresh` SBE message.
pub fn encode_market_data_incremental_refresh(m: &MarketDataIncrementalRefresh) -> Vec<u8> {
    MarketDataIncrementalRefreshEncoder::encode(m)
}
/// Encode a `OpenOrdersRequest` SBE message.
pub fn encode_open_orders_request(m: &OpenOrdersRequest) -> Vec<u8> {
    OpenOrdersRequestEncoder::encode(m)
}
/// Encode a `OrderBookRequest` SBE message.
pub fn encode_order_book_request(m: &OrderBookRequest) -> Vec<u8> {
    OrderBookRequestEncoder::encode(m)
}
/// Encode a `PositionRequest` SBE message.
pub fn encode_position_request(m: &PositionRequest) -> Vec<u8> {
    PositionRequestEncoder::encode(m)
}
/// Encode a `PositionSnapshot` SBE message.
pub fn encode_position_snapshot(m: &PositionSnapshot) -> Vec<u8> {
    PositionSnapshotEncoder::encode(m)
}
/// Encode a `PublicTradeBatch` SBE message.
pub fn encode_public_trade_batch(m: &PublicTradeBatch) -> Vec<u8> {
    PublicTradeBatchEncoder::encode(m)
}
/// Encode a `PublicTradeEvent` SBE message.
pub fn encode_public_trade_event(m: &PublicTradeEvent) -> Vec<u8> {
    PublicTradeEventEncoder::encode(m)
}
/// Encode a `TickerRequest` SBE message.
pub fn encode_ticker_request(m: &TickerRequest) -> Vec<u8> {
    TickerRequestEncoder::encode(m)
}
/// Encode a `TradeHistoryRequest` SBE message.
pub fn encode_trade_history_request(m: &TradeHistoryRequest) -> Vec<u8> {
    TradeHistoryRequestEncoder::encode(m)
}
/// Encode a `UserLiquidation` SBE message.
pub fn encode_user_liquidation(m: &UserLiquidation) -> Vec<u8> {
    UserLiquidationEncoder::encode(m)
}
/// Encode a `CapabilitiesResponse` SBE message — exercises list-of-scalar fields
/// (`list<uint8>`, `list<Symbol>`, `list<CandleInterval>`).
pub fn encode_capabilities_response(resp: &crate::messages::CapabilitiesResponse) -> Vec<u8> {
    CapabilitiesResponseEncoder::encode(resp)
}

/// Encode a `BalanceSnapshot` SBE message (template 33).
pub fn encode_balance_snapshot(snap: &BalanceSnapshot) -> Vec<u8> {
    BalanceSnapshotEncoder::encode(snap)
}

/// Encode a `TradingSessionStatusRequest` SBE message.
pub fn encode_trading_session_status_request(m: &TradingSessionStatusRequest) -> Vec<u8> {
    TradingSessionStatusRequestEncoder::encode(m)
}

/// Encode a `TradingSessionStatus` SBE message.
pub fn encode_trading_session_status(m: &TradingSessionStatus) -> Vec<u8> {
    TradingSessionStatusEncoder::encode(m)
}

/// Encode a `SecurityStatusRequest` SBE message.
pub fn encode_security_status_request(m: &SecurityStatusRequest) -> Vec<u8> {
    SecurityStatusRequestEncoder::encode(m)
}

/// Encode a `SecurityStatus` SBE message.
pub fn encode_security_status(m: &SecurityStatus) -> Vec<u8> {
    SecurityStatusEncoder::encode(m)
}

/// Encode a `MassCancelRequest` SBE message.
pub fn encode_mass_cancel_request(m: &MassCancelRequest) -> Vec<u8> {
    MassCancelRequestEncoder::encode(m)
}

/// Encode a `MassCancelReport` SBE message.
pub fn encode_mass_cancel_report(m: &MassCancelReport) -> Vec<u8> {
    MassCancelReportEncoder::encode(m)
}

/// Encode a `PriceLimitsRequest` SBE message (template 61).
pub fn encode_price_limits_request(m: &PriceLimitsRequest) -> Vec<u8> {
    PriceLimitsRequestEncoder::encode(m)
}

/// Encode a `PriceLimits` SBE message (template 62).
pub fn encode_price_limits(m: &PriceLimits) -> Vec<u8> {
    PriceLimitsEncoder::encode(m)
}

/// Encode a `BatchRequest` SBE message (template 63).
pub fn encode_batch_request(m: &BatchRequest) -> Vec<u8> {
    BatchRequestEncoder::encode(m)
}

/// Decode a `BatchRequest` SBE message (template 63).
pub fn decode_batch_request(buf: &[u8]) -> Result<BatchRequest, SbeError> {
    BatchRequestDecoder::decode(buf).map_err(decode_err)
}

/// Encode a `BatchReport` SBE message (template 64).
pub fn encode_batch_report(m: &BatchReport) -> Vec<u8> {
    BatchReportEncoder::encode(m)
}

/// Decode a `BatchReport` SBE message (template 64).
pub fn decode_batch_report(buf: &[u8]) -> Result<BatchReport, SbeError> {
    BatchReportDecoder::decode(buf).map_err(decode_err)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{OrderType, Price, Quantity, Side, TimeInForce};

    fn sample_order() -> NewOrderSingle {
        NewOrderSingle {
            cl_ord_id: "ORD-001".to_string(),
            side: Side::Buy,
            order_qty: Quantity(100.0),
            price: Some(Price(150.25)),
            stop_price: None,
            symbol: "AAPL".to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: Some("ACCT-123".to_string()),
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        }
    }

    #[test]
    fn new_order_round_trip() {
        let order = sample_order();
        let encoded = encode_new_order_single(&order);
        let decoded = decode_new_order_single(&encoded).expect("decode");
        assert_eq!(decoded, order);
    }

    #[test]
    fn new_order_encode_into_matches_encode() {
        let order = sample_order();
        let owned = encode_new_order_single(&order);
        let mut buf = Vec::new();
        encode_new_order_single_into(&order, &mut buf);
        assert_eq!(buf, owned);
        encode_new_order_single_into(&order, &mut buf);
        assert_eq!(buf, owned);
    }
}
