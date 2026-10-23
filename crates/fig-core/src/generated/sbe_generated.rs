// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' v1.0.0
// Standard order entry and execution messages

use crate::messages::{
    AccountSummary, AggregateTrade, AggregateTradeBatch, AggregateTradeEvent,
    AggregateTradeRequest, AllMidsBatch, AllMidsRequest, BalanceEntry, BalanceSnapshot,
    BalanceUpdate, BalanceUpdateReason, BatchAction, BatchActionKind, BatchReport,
    BatchReportOutcome, BatchRequest, BatchResult, BatchResultKind, BatchResultStatus,
    BestBidOffer, CancelReject, CancelRejectReason, CancelReplaceRequest, CancelRequest, CandleBar,
    CandleBarBatch, CandleBarEvent, CandleBarRequest, CandleInterval, CapabilitiesRequest,
    CapabilitiesResponse, CapabilityPath, CapabilityPathPattern, ClientOrderId, ExecType,
    ExecutionReport, FillHistoryBatch, FillHistoryRequest, FundingHistoryBatch,
    FundingHistoryRequest, FundingPayment, InstrumentCatalogRequest, InstrumentCatalogResponse,
    InstrumentMetadata, LedgerHistoryBatch, LedgerHistoryRequest, LedgerUpdate, LedgerUpdateKind,
    LiquidationTrade, LiquidationTradeEvent, MarginSummary, MarginUpdate, MarkPriceRequest,
    MarkPriceUpdate, MarketDataAction, MarketDataIncrementalRefresh, MarketDataSnapshot,
    MarketDataUpdate, MassCancelReport, MassCancelReportStatus, MassCancelRequest, MiniTicker,
    NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot, OrdStatus, OrderBookDelta,
    OrderBookRequest, OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    OrderListStatusStatus, OrderType, PageInfo, PositionEntry, PositionRequest, PositionSnapshot,
    PositionUpdate, Price, PriceLevel, PriceLimits, PriceLimitsRequest, PublicTrade,
    PublicTradeBatch, PublicTradeEvent, Quantity, SecurityIdSource, SecurityStatus,
    SecurityStatusHaltReason, SecurityStatusRequest, SecurityStatusStatus, Side, Symbol,
    SymbolTicker, TickerRequest, TimeInForce, TradeHistoryRequest, TradeTimestamp,
    TradingSessionStatus, TradingSessionStatusHaltReason, TradingSessionStatusRequest,
    TradingSessionStatusStatus, UserLiquidation,
};

// --- Bounds-checked SBE decode helpers ---
#[inline]
fn sbe_take<'a>(buf: &'a [u8], pos: &mut usize, n: usize) -> Result<&'a [u8], String> {
    let start = *pos;
    let end = start
        .checked_add(n)
        .ok_or_else(|| format!("sbe: offset overflow at {}", start))?;
    if end > buf.len() {
        return Err(format!(
            "sbe: buffer too short: need {} bytes at offset {}, have {}",
            n,
            start,
            buf.len()
        ));
    }
    *pos = end;
    Ok(&buf[start..end])
}

#[inline]
fn sbe_u8(buf: &[u8], pos: &mut usize) -> Result<u8, String> {
    Ok(sbe_take(buf, pos, 1)?[0])
}

#[inline]
fn sbe_u16(buf: &[u8], pos: &mut usize) -> Result<u16, String> {
    let b = sbe_take(buf, pos, 2)?;
    Ok(u16::from_be_bytes([b[0], b[1]]))
}

#[inline]
fn sbe_u32(buf: &[u8], pos: &mut usize) -> Result<u32, String> {
    let b = sbe_take(buf, pos, 4)?;
    Ok(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

#[inline]
fn sbe_u64(buf: &[u8], pos: &mut usize) -> Result<u64, String> {
    let b = sbe_take(buf, pos, 8)?;
    Ok(u64::from_be_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

#[inline]
fn sbe_i64(buf: &[u8], pos: &mut usize) -> Result<i64, String> {
    let b = sbe_take(buf, pos, 8)?;
    Ok(i64::from_be_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

#[inline]
fn sbe_f64(buf: &[u8], pos: &mut usize) -> Result<f64, String> {
    let b = sbe_take(buf, pos, 8)?;
    Ok(f64::from_be_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

#[inline]
fn sbe_str(buf: &[u8], pos: &mut usize) -> Result<String, String> {
    let len = sbe_u16(buf, pos)? as usize;
    let bytes = sbe_take(buf, pos, len)?;
    match std::str::from_utf8(bytes) {
        Ok(s) => {
            let mut out = String::new();
            out.reserve_exact(s.len());
            out.push_str(s);
            Ok(out)
        }
        Err(e) => Err(format!("invalid UTF-8: {}", e)),
    }
}

/// Clamp a wire-supplied element count to the bytes actually remaining so
/// that a hostile count cannot force an unbounded allocation.
#[inline]
fn sbe_capacity(count: usize, remaining: usize) -> usize {
    count.min(remaining)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewOrderSingleSide {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
}

impl NewOrderSingleSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(NewOrderSingleSide::Buy),
            2 => Some(NewOrderSingleSide::Sell),
            3 => Some(NewOrderSingleSide::SellShort),
            4 => Some(NewOrderSingleSide::SellShortExempt),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewOrderSingleOrderType {
    Market = 1,
    Limit = 2,
    Stop = 3,
    StopLimit = 4,
    MarketOnClose = 5,
    LimitOnClose = 6,
    Pegged = 7,
}

impl NewOrderSingleOrderType {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(NewOrderSingleOrderType::Market),
            2 => Some(NewOrderSingleOrderType::Limit),
            3 => Some(NewOrderSingleOrderType::Stop),
            4 => Some(NewOrderSingleOrderType::StopLimit),
            5 => Some(NewOrderSingleOrderType::MarketOnClose),
            6 => Some(NewOrderSingleOrderType::LimitOnClose),
            7 => Some(NewOrderSingleOrderType::Pegged),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewOrderSingleTimeInForce {
    Day = 1,
    Gtc = 2,
    Ioc = 3,
    Fok = 4,
    Gtd = 5,
}

impl NewOrderSingleTimeInForce {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(NewOrderSingleTimeInForce::Day),
            2 => Some(NewOrderSingleTimeInForce::Gtc),
            3 => Some(NewOrderSingleTimeInForce::Ioc),
            4 => Some(NewOrderSingleTimeInForce::Fok),
            5 => Some(NewOrderSingleTimeInForce::Gtd),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewOrderSingleIdSource {
    Cusip = 1,
    Sedol = 2,
    Isin = 3,
    Ric = 4,
    ExchangeSymbol = 5,
}

impl NewOrderSingleIdSource {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(NewOrderSingleIdSource::Cusip),
            2 => Some(NewOrderSingleIdSource::Sedol),
            3 => Some(NewOrderSingleIdSource::Isin),
            4 => Some(NewOrderSingleIdSource::Ric),
            5 => Some(NewOrderSingleIdSource::ExchangeSymbol),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CancelRequestSide {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
}

impl CancelRequestSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(CancelRequestSide::Buy),
            2 => Some(CancelRequestSide::Sell),
            3 => Some(CancelRequestSide::SellShort),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CancelReplaceRequestSide {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
}

impl CancelReplaceRequestSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(CancelReplaceRequestSide::Buy),
            2 => Some(CancelReplaceRequestSide::Sell),
            3 => Some(CancelReplaceRequestSide::SellShort),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionReportExecType {
    New = 1,
    PartialFill = 2,
    Fill = 3,
    DoneForDay = 4,
    Canceled = 5,
    Replaced = 6,
    PendingCancel = 7,
    Stopped = 8,
    Rejected = 9,
    Suspended = 10,
    PendingNew = 11,
    Expired = 12,
}

impl ExecutionReportExecType {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(ExecutionReportExecType::New),
            2 => Some(ExecutionReportExecType::PartialFill),
            3 => Some(ExecutionReportExecType::Fill),
            4 => Some(ExecutionReportExecType::DoneForDay),
            5 => Some(ExecutionReportExecType::Canceled),
            6 => Some(ExecutionReportExecType::Replaced),
            7 => Some(ExecutionReportExecType::PendingCancel),
            8 => Some(ExecutionReportExecType::Stopped),
            9 => Some(ExecutionReportExecType::Rejected),
            10 => Some(ExecutionReportExecType::Suspended),
            11 => Some(ExecutionReportExecType::PendingNew),
            12 => Some(ExecutionReportExecType::Expired),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionReportOrdStatus {
    New = 1,
    PartiallyFilled = 2,
    Filled = 3,
    DoneForDay = 4,
    Canceled = 5,
    PendingCancel = 6,
    Stopped = 7,
    Rejected = 8,
    Suspended = 9,
    PendingNew = 10,
    Expired = 11,
    Replaced = 12,
}

impl ExecutionReportOrdStatus {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(ExecutionReportOrdStatus::New),
            2 => Some(ExecutionReportOrdStatus::PartiallyFilled),
            3 => Some(ExecutionReportOrdStatus::Filled),
            4 => Some(ExecutionReportOrdStatus::DoneForDay),
            5 => Some(ExecutionReportOrdStatus::Canceled),
            6 => Some(ExecutionReportOrdStatus::PendingCancel),
            7 => Some(ExecutionReportOrdStatus::Stopped),
            8 => Some(ExecutionReportOrdStatus::Rejected),
            9 => Some(ExecutionReportOrdStatus::Suspended),
            10 => Some(ExecutionReportOrdStatus::PendingNew),
            11 => Some(ExecutionReportOrdStatus::Expired),
            12 => Some(ExecutionReportOrdStatus::Replaced),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecutionReportSide {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
}

impl ExecutionReportSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(ExecutionReportSide::Buy),
            2 => Some(ExecutionReportSide::Sell),
            3 => Some(ExecutionReportSide::SellShort),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CancelRejectRejectReason {
    OrderNotFound = 1,
    AlreadyCanceled = 2,
    AlreadyFilled = 3,
    TooLateToCancel = 4,
}

impl CancelRejectRejectReason {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(CancelRejectRejectReason::OrderNotFound),
            2 => Some(CancelRejectRejectReason::AlreadyCanceled),
            3 => Some(CancelRejectRejectReason::AlreadyFilled),
            4 => Some(CancelRejectRejectReason::TooLateToCancel),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MassCancelRequestSide {
    Buy = 1,
    Sell = 2,
    SellShort = 3,
    SellShortExempt = 4,
}

impl MassCancelRequestSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(MassCancelRequestSide::Buy),
            2 => Some(MassCancelRequestSide::Sell),
            3 => Some(MassCancelRequestSide::SellShort),
            4 => Some(MassCancelRequestSide::SellShortExempt),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

/// SBE body encoder for AggregateTrade
pub struct AggregateTradeEncoder;

impl AggregateTradeEncoder {
    pub fn encode(value: &AggregateTrade, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let agg_trade_id_bytes = value.agg_trade_id.as_bytes();
        buf.extend_from_slice(&(agg_trade_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(agg_trade_id_bytes);
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.push(value.side.to_value());
        let first_trade_id_bytes = value.first_trade_id.as_bytes();
        buf.extend_from_slice(&(first_trade_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(first_trade_id_bytes);
        let last_trade_id_bytes = value.last_trade_id.as_bytes();
        buf.extend_from_slice(&(last_trade_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(last_trade_id_bytes);
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for AggregateTrade
pub struct AggregateTradeDecoder;

impl AggregateTradeDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AggregateTrade, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let agg_trade_id = sbe_str(buf, &mut pos)?;
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let first_trade_id = sbe_str(buf, &mut pos)?;
        let last_trade_id = sbe_str(buf, &mut pos)?;
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = AggregateTrade {
            symbol,
            agg_trade_id,
            price,
            qty,
            side,
            first_trade_id,
            last_trade_id,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for LiquidationTrade
pub struct LiquidationTradeEncoder;

impl LiquidationTradeEncoder {
    pub fn encode(value: &LiquidationTrade, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(value.side.to_value());
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for LiquidationTrade
pub struct LiquidationTradeDecoder;

impl LiquidationTradeDecoder {
    pub fn decode(buf: &[u8]) -> Result<(LiquidationTrade, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = LiquidationTrade {
            symbol,
            side,
            price,
            qty,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for PriceLevel
pub struct PriceLevelEncoder;

impl PriceLevelEncoder {
    pub fn encode(value: &PriceLevel, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.extend_from_slice(&value.order_count.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.order_count.is_some()));
    }
}

/// SBE body decoder for PriceLevel
pub struct PriceLevelDecoder;

impl PriceLevelDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PriceLevel, usize), String> {
        let mut pos: usize = 0;

        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let order_count_raw = sbe_u32(buf, &mut pos)?;
        let order_count = if sbe_u8(buf, &mut pos)? == 1 {
            Some(order_count_raw)
        } else {
            None
        };

        let decoded = PriceLevel {
            price,
            qty,
            order_count,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for MarketDataUpdate
pub struct MarketDataUpdateEncoder;

impl MarketDataUpdateEncoder {
    pub fn encode(value: &MarketDataUpdate, buf: &mut Vec<u8>) {
        buf.push(value.side.to_value());
        buf.push(value.action.to_value());
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
    }
}

/// SBE body decoder for MarketDataUpdate
pub struct MarketDataUpdateDecoder;

impl MarketDataUpdateDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarketDataUpdate, usize), String> {
        let mut pos: usize = 0;

        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let action_raw = sbe_u8(buf, &mut pos)?;
        let action = MarketDataAction::from_value(action_raw)
            .ok_or_else(|| format!("invalid MarketDataAction value: {}", action_raw))?;
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);

        let decoded = MarketDataUpdate {
            side,
            action,
            price,
            qty,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for CandleBar
pub struct CandleBarEncoder;

impl CandleBarEncoder {
    pub fn encode(value: &CandleBar, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let interval_bytes = value.interval.as_bytes();
        buf.extend_from_slice(&(interval_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(interval_bytes);
        buf.extend_from_slice(&value.open.0.to_be_bytes());
        buf.extend_from_slice(&value.high.0.to_be_bytes());
        buf.extend_from_slice(&value.low.0.to_be_bytes());
        buf.extend_from_slice(&value.close.0.to_be_bytes());
        buf.extend_from_slice(&value.volume.0.to_be_bytes());
        buf.extend_from_slice(&value.bar_start.to_be_bytes());
        buf.extend_from_slice(&value.bar_end.to_be_bytes());
        buf.push(value.is_final as u8);
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for CandleBar
pub struct CandleBarDecoder;

impl CandleBarDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CandleBar, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let interval = sbe_str(buf, &mut pos)?;
        let open_raw = sbe_f64(buf, &mut pos)?;
        let open = Price(open_raw);
        let high_raw = sbe_f64(buf, &mut pos)?;
        let high = Price(high_raw);
        let low_raw = sbe_f64(buf, &mut pos)?;
        let low = Price(low_raw);
        let close_raw = sbe_f64(buf, &mut pos)?;
        let close = Price(close_raw);
        let volume_raw = sbe_f64(buf, &mut pos)?;
        let volume = Quantity(volume_raw);
        let bar_start_raw = sbe_i64(buf, &mut pos)?;
        let bar_start = bar_start_raw;
        let bar_end_raw = sbe_i64(buf, &mut pos)?;
        let bar_end = bar_end_raw;
        let is_final = sbe_u8(buf, &mut pos)? != 0;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = CandleBar {
            symbol,
            interval,
            open,
            high,
            low,
            close,
            volume,
            bar_start,
            bar_end,
            is_final,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for PublicTrade
pub struct PublicTradeEncoder;

impl PublicTradeEncoder {
    pub fn encode(value: &PublicTrade, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let trade_id_bytes = value.trade_id.as_bytes();
        buf.extend_from_slice(&(trade_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(trade_id_bytes);
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.push(value.side.to_value());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for PublicTrade
pub struct PublicTradeDecoder;

impl PublicTradeDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PublicTrade, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let trade_id = sbe_str(buf, &mut pos)?;
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = PublicTrade {
            symbol,
            trade_id,
            price,
            qty,
            side,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for InstrumentMetadata
pub struct InstrumentMetadataEncoder;

impl InstrumentMetadataEncoder {
    pub fn encode(value: &InstrumentMetadata, buf: &mut Vec<u8>) {
        let instrument_id_bytes = value.instrument_id.as_bytes();
        buf.extend_from_slice(&(instrument_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(instrument_id_bytes);
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let product_kind_bytes = value.product_kind.as_bytes();
        buf.extend_from_slice(&(product_kind_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(product_kind_bytes);
        buf.push(if value.margin_asset.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.margin_asset {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(if value.display_name.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.display_name {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&value.tick_size.unwrap_or(0.0).to_be_bytes());
        buf.push(if value.tick_size.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&value.lot_size.unwrap_or(0.0).to_be_bytes());
        buf.push(if value.lot_size.is_some() { 1 } else { 0 });
    }
}

/// SBE body decoder for InstrumentMetadata
pub struct InstrumentMetadataDecoder;

impl InstrumentMetadataDecoder {
    pub fn decode(buf: &[u8]) -> Result<(InstrumentMetadata, usize), String> {
        let mut pos: usize = 0;

        let instrument_id = sbe_str(buf, &mut pos)?;
        let symbol = sbe_str(buf, &mut pos)?;
        let product_kind = sbe_str(buf, &mut pos)?;
        let margin_asset = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let display_name = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let tick_size_raw = sbe_f64(buf, &mut pos)?;
        let tick_size = if sbe_u8(buf, &mut pos)? == 1 {
            Some(tick_size_raw)
        } else {
            None
        };
        let lot_size_raw = sbe_f64(buf, &mut pos)?;
        let lot_size = if sbe_u8(buf, &mut pos)? == 1 {
            Some(lot_size_raw)
        } else {
            None
        };

        let decoded = InstrumentMetadata {
            instrument_id,
            symbol,
            product_kind,
            margin_asset,
            display_name,
            tick_size,
            lot_size,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for BalanceEntry
pub struct BalanceEntryEncoder;

impl BalanceEntryEncoder {
    pub fn encode(value: &BalanceEntry, buf: &mut Vec<u8>) {
        let asset_bytes = value.asset.as_bytes();
        buf.extend_from_slice(&(asset_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(asset_bytes);
        buf.extend_from_slice(&value.total.to_be_bytes());
        buf.extend_from_slice(&value.available.to_be_bytes());
        buf.extend_from_slice(&value.hold.to_be_bytes());
    }
}

/// SBE body decoder for BalanceEntry
pub struct BalanceEntryDecoder;

impl BalanceEntryDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BalanceEntry, usize), String> {
        let mut pos: usize = 0;

        let asset = sbe_str(buf, &mut pos)?;
        let total_raw = sbe_f64(buf, &mut pos)?;
        let total = total_raw;
        let available_raw = sbe_f64(buf, &mut pos)?;
        let available = available_raw;
        let hold_raw = sbe_f64(buf, &mut pos)?;
        let hold = hold_raw;

        let decoded = BalanceEntry {
            asset,
            total,
            available,
            hold,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for PositionEntry
pub struct PositionEntryEncoder;

impl PositionEntryEncoder {
    pub fn encode(value: &PositionEntry, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.extend_from_slice(&value.entry_price.0.to_be_bytes());
        buf.extend_from_slice(&value.unrealized_pnl.to_be_bytes());
    }
}

/// SBE body decoder for PositionEntry
pub struct PositionEntryDecoder;

impl PositionEntryDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PositionEntry, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let entry_price_raw = sbe_f64(buf, &mut pos)?;
        let entry_price = Price(entry_price_raw);
        let unrealized_pnl_raw = sbe_f64(buf, &mut pos)?;
        let unrealized_pnl = unrealized_pnl_raw;

        let decoded = PositionEntry {
            symbol,
            qty,
            entry_price,
            unrealized_pnl,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for PageInfo
pub struct PageInfoEncoder;

impl PageInfoEncoder {
    pub fn encode(value: &PageInfo, buf: &mut Vec<u8>) {
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for PageInfo
pub struct PageInfoDecoder;

impl PageInfoDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PageInfo, usize), String> {
        let mut pos: usize = 0;

        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = PageInfo {
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for CapabilityPath
pub struct CapabilityPathEncoder;

impl CapabilityPathEncoder {
    pub fn encode(value: &CapabilityPath, buf: &mut Vec<u8>) {
        let path_bytes = value.path.as_bytes();
        buf.extend_from_slice(&(path_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(path_bytes);
        buf.push(value.pattern.to_value());
        buf.push(value.auth_required as u8);
    }
}

/// SBE body decoder for CapabilityPath
pub struct CapabilityPathDecoder;

impl CapabilityPathDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CapabilityPath, usize), String> {
        let mut pos: usize = 0;

        let path = sbe_str(buf, &mut pos)?;
        let pattern_raw = sbe_u8(buf, &mut pos)?;
        let pattern = CapabilityPathPattern::from_value(pattern_raw)
            .ok_or_else(|| format!("invalid CapabilityPathPattern value: {}", pattern_raw))?;
        let auth_required = sbe_u8(buf, &mut pos)? != 0;

        let decoded = CapabilityPath {
            path,
            pattern,
            auth_required,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for BatchAction
pub struct BatchActionEncoder;

impl BatchActionEncoder {
    pub fn encode(value: &BatchAction, buf: &mut Vec<u8>) {
        buf.push(value.kind.to_value());
        buf.push(u8::from(value.new_order.is_some()));
        if let Some(ref v) = value.new_order {
            NewOrderSingleBodyEncoder::encode(v, buf);
        }
        buf.push(u8::from(value.cancel.is_some()));
        if let Some(ref v) = value.cancel {
            CancelRequestBodyEncoder::encode(v, buf);
        }
        buf.push(u8::from(value.replace.is_some()));
        if let Some(ref v) = value.replace {
            CancelReplaceRequestBodyEncoder::encode(v, buf);
        }
    }
}

/// SBE body decoder for BatchAction
pub struct BatchActionDecoder;

impl BatchActionDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BatchAction, usize), String> {
        let mut pos: usize = 0;

        let kind_raw = sbe_u8(buf, &mut pos)?;
        let kind = BatchActionKind::from_value(kind_raw)
            .ok_or_else(|| format!("invalid BatchActionKind value: {}", kind_raw))?;
        let new_order = if sbe_u8(buf, &mut pos)? == 1 {
            let rest = buf
                .get(pos..)
                .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
            let (item, n) = NewOrderSingleBodyDecoder::decode(rest)?;
            pos += n;
            Some(item)
        } else {
            None
        };
        let cancel = if sbe_u8(buf, &mut pos)? == 1 {
            let rest = buf
                .get(pos..)
                .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
            let (item, n) = CancelRequestBodyDecoder::decode(rest)?;
            pos += n;
            Some(item)
        } else {
            None
        };
        let replace = if sbe_u8(buf, &mut pos)? == 1 {
            let rest = buf
                .get(pos..)
                .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
            let (item, n) = CancelReplaceRequestBodyDecoder::decode(rest)?;
            pos += n;
            Some(item)
        } else {
            None
        };

        let decoded = BatchAction {
            kind,
            new_order,
            cancel,
            replace,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for BatchResult
pub struct BatchResultEncoder;

impl BatchResultEncoder {
    pub fn encode(value: &BatchResult, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&value.index.to_be_bytes());
        buf.push(value.kind.to_value());
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        buf.push(value.status.to_value());
        buf.push(if value.reject_reason.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.reject_reason {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for BatchResult
pub struct BatchResultDecoder;

impl BatchResultDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BatchResult, usize), String> {
        let mut pos: usize = 0;

        let index_raw = sbe_u32(buf, &mut pos)?;
        let index = index_raw;
        let kind_raw = sbe_u8(buf, &mut pos)?;
        let kind = BatchResultKind::from_value(kind_raw)
            .ok_or_else(|| format!("invalid BatchResultKind value: {}", kind_raw))?;
        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let status_raw = sbe_u8(buf, &mut pos)?;
        let status = BatchResultStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid BatchResultStatus value: {}", status_raw))?;
        let reject_reason = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = BatchResult {
            index,
            kind,
            cl_ord_id,
            status,
            reject_reason,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested NewOrderSingle
pub struct NewOrderSingleBodyEncoder;

impl NewOrderSingleBodyEncoder {
    pub fn encode(value: &NewOrderSingle, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        buf.push(value.side.to_value());
        buf.extend_from_slice(&value.order_qty.0.to_be_bytes());
        buf.extend_from_slice(
            &value
                .price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .stop_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.stop_price.is_some() { 1 } else { 0 });
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(value.order_type.to_value());
        buf.push(value.time_in_force.to_value());
        buf.extend_from_slice(&value.expire_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.expire_time.is_some()));
        buf.push(if value.account.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.account {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(if value.strategy_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.strategy_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(if value.security_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.security_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(value.id_source.map(|v| v.to_value()).unwrap_or(0));
        buf.push(u8::from(value.id_source.is_some()));
        buf.push(if value.security_exchange.is_some() {
            1
        } else {
            0
        });
        if let Some(ref s) = value.security_exchange {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(value.post_only.unwrap_or(false) as u8);
        buf.push(u8::from(value.post_only.is_some()));
        buf.push(value.reduce_only.unwrap_or(false) as u8);
        buf.push(u8::from(value.reduce_only.is_some()));
    }
}

/// SBE body decoder for nested NewOrderSingle
pub struct NewOrderSingleBodyDecoder;

impl NewOrderSingleBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(NewOrderSingle, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = sbe_f64(buf, &mut pos)?;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(price_raw))
        } else {
            None
        };
        let stop_price_raw = sbe_f64(buf, &mut pos)?;
        let stop_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(stop_price_raw))
        } else {
            None
        };
        let symbol = sbe_str(buf, &mut pos)?;
        let order_type_raw = sbe_u8(buf, &mut pos)?;
        let order_type = OrderType::from_value(order_type_raw)
            .ok_or_else(|| format!("invalid OrderType value: {}", order_type_raw))?;
        let time_in_force_raw = sbe_u8(buf, &mut pos)?;
        let time_in_force = TimeInForce::from_value(time_in_force_raw)
            .ok_or_else(|| format!("invalid TimeInForce value: {}", time_in_force_raw))?;
        let expire_time_raw = sbe_i64(buf, &mut pos)?;
        let expire_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(expire_time_raw)
        } else {
            None
        };
        let account = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let strategy_id = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let security_id = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let id_source_raw = sbe_u8(buf, &mut pos)?;
        let id_source = if sbe_u8(buf, &mut pos)? == 1 {
            Some(
                SecurityIdSource::from_value(id_source_raw)
                    .ok_or_else(|| format!("invalid SecurityIdSource value: {}", id_source_raw))?,
            )
        } else {
            None
        };
        let security_exchange = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let post_only_val = sbe_u8(buf, &mut pos)?;
        let post_only = if sbe_u8(buf, &mut pos)? == 1 {
            Some(post_only_val != 0)
        } else {
            None
        };
        let reduce_only_val = sbe_u8(buf, &mut pos)?;
        let reduce_only = if sbe_u8(buf, &mut pos)? == 1 {
            Some(reduce_only_val != 0)
        } else {
            None
        };

        let decoded = NewOrderSingle {
            cl_ord_id,
            side,
            order_qty,
            price,
            stop_price,
            symbol,
            order_type,
            time_in_force,
            expire_time,
            account,
            strategy_id,
            security_id,
            id_source,
            security_exchange,
            post_only,
            reduce_only,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CancelRequest
pub struct CancelRequestBodyEncoder;

impl CancelRequestBodyEncoder {
    pub fn encode(value: &CancelRequest, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = value.orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(value.side.to_value());
        buf.extend_from_slice(
            &value
                .order_qty
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.order_qty.is_some() { 1 } else { 0 });
    }
}

/// SBE body decoder for nested CancelRequest
pub struct CancelRequestBodyDecoder;

impl CancelRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CancelRequest, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let orig_cl_ord_id = sbe_str(buf, &mut pos)?;
        let symbol = sbe_str(buf, &mut pos)?;
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = sbe_f64(buf, &mut pos)?;
        let order_qty = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Quantity(order_qty_raw))
        } else {
            None
        };

        let decoded = CancelRequest {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CancelReplaceRequest
pub struct CancelReplaceRequestBodyEncoder;

impl CancelReplaceRequestBodyEncoder {
    pub fn encode(value: &CancelReplaceRequest, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = value.orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(value.side.to_value());
        buf.extend_from_slice(&value.order_qty.0.to_be_bytes());
        buf.extend_from_slice(
            &value
                .price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.price.is_some() { 1 } else { 0 });
    }
}

/// SBE body decoder for nested CancelReplaceRequest
pub struct CancelReplaceRequestBodyDecoder;

impl CancelReplaceRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CancelReplaceRequest, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let orig_cl_ord_id = sbe_str(buf, &mut pos)?;
        let symbol = sbe_str(buf, &mut pos)?;
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = sbe_f64(buf, &mut pos)?;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(price_raw))
        } else {
            None
        };

        let decoded = CancelReplaceRequest {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty,
            price,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested ExecutionReport
pub struct ExecutionReportBodyEncoder;

impl ExecutionReportBodyEncoder {
    pub fn encode(value: &ExecutionReport, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let order_id_bytes = value.order_id.as_bytes();
        buf.extend_from_slice(&(order_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(order_id_bytes);
        let exec_id_bytes = value.exec_id.as_bytes();
        buf.extend_from_slice(&(exec_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exec_id_bytes);
        buf.push(value.exec_type.to_value());
        buf.push(value.ord_status.to_value());
        buf.push(value.side.to_value());
        buf.extend_from_slice(
            &value
                .last_qty
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.last_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .last_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.last_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&value.leaves_qty.0.to_be_bytes());
        buf.extend_from_slice(&value.cum_qty.0.to_be_bytes());
        buf.extend_from_slice(&value.avg_price.0.to_be_bytes());
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.transact_time.to_be_bytes());
    }
}

/// SBE body decoder for nested ExecutionReport
pub struct ExecutionReportBodyDecoder;

impl ExecutionReportBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(ExecutionReport, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let order_id = sbe_str(buf, &mut pos)?;
        let exec_id = sbe_str(buf, &mut pos)?;
        let exec_type_raw = sbe_u8(buf, &mut pos)?;
        let exec_type = ExecType::from_value(exec_type_raw)
            .ok_or_else(|| format!("invalid ExecType value: {}", exec_type_raw))?;
        let ord_status_raw = sbe_u8(buf, &mut pos)?;
        let ord_status = OrdStatus::from_value(ord_status_raw)
            .ok_or_else(|| format!("invalid OrdStatus value: {}", ord_status_raw))?;
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let last_qty_raw = sbe_f64(buf, &mut pos)?;
        let last_qty = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Quantity(last_qty_raw))
        } else {
            None
        };
        let last_price_raw = sbe_f64(buf, &mut pos)?;
        let last_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(last_price_raw))
        } else {
            None
        };
        let leaves_qty_raw = sbe_f64(buf, &mut pos)?;
        let leaves_qty = Quantity(leaves_qty_raw);
        let cum_qty_raw = sbe_f64(buf, &mut pos)?;
        let cum_qty = Quantity(cum_qty_raw);
        let avg_price_raw = sbe_f64(buf, &mut pos)?;
        let avg_price = Price(avg_price_raw);
        let symbol = sbe_str(buf, &mut pos)?;
        let transact_time_raw = sbe_i64(buf, &mut pos)?;
        let transact_time = transact_time_raw;

        let decoded = ExecutionReport {
            cl_ord_id,
            order_id,
            exec_id,
            exec_type,
            ord_status,
            side,
            last_qty,
            last_price,
            leaves_qty,
            cum_qty,
            avg_price,
            symbol,
            transact_time,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CancelReject
pub struct CancelRejectBodyEncoder;

impl CancelRejectBodyEncoder {
    pub fn encode(value: &CancelReject, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = value.orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        buf.push(value.reject_reason.to_value());
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
    }
}

/// SBE body decoder for nested CancelReject
pub struct CancelRejectBodyDecoder;

impl CancelRejectBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CancelReject, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let orig_cl_ord_id = sbe_str(buf, &mut pos)?;
        let reject_reason_raw = sbe_u8(buf, &mut pos)?;
        let reject_reason = CancelRejectReason::from_value(reject_reason_raw)
            .ok_or_else(|| format!("invalid CancelRejectReason value: {}", reject_reason_raw))?;
        let symbol = sbe_str(buf, &mut pos)?;

        let decoded = CancelReject {
            cl_ord_id,
            orig_cl_ord_id,
            reject_reason,
            symbol,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarketDataSnapshot
pub struct MarketDataSnapshotBodyEncoder;

impl MarketDataSnapshotBodyEncoder {
    pub fn encode(value: &MarketDataSnapshot, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let exchange_bytes = value.exchange.as_bytes();
        buf.extend_from_slice(&(exchange_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exchange_bytes);
        buf.extend_from_slice(&(value.bids.len() as u32).to_be_bytes());
        for item in &value.bids {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(value.asks.len() as u32).to_be_bytes());
        for item in &value.asks {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.extend_from_slice(&value.sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.sequence.is_some()));
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested MarketDataSnapshot
pub struct MarketDataSnapshotBodyDecoder;

impl MarketDataSnapshotBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarketDataSnapshot, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let exchange = sbe_str(buf, &mut pos)?;
        let bids_count = sbe_u32(buf, &mut pos)? as usize;
        let mut bids = Vec::with_capacity(sbe_capacity(bids_count, buf.len().saturating_sub(pos)));
        for _ in 0..bids_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PriceLevelDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            bids.push(item);
        }
        let asks_count = sbe_u32(buf, &mut pos)? as usize;
        let mut asks = Vec::with_capacity(sbe_capacity(asks_count, buf.len().saturating_sub(pos)));
        for _ in 0..asks_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PriceLevelDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            asks.push(item);
        }
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let sequence_raw = sbe_u64(buf, &mut pos)?;
        let sequence = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sequence_raw)
        } else {
            None
        };
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = MarketDataSnapshot {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
            sequence,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshBodyEncoder;

impl MarketDataIncrementalRefreshBodyEncoder {
    pub fn encode(value: &MarketDataIncrementalRefresh, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(value.updates.len() as u32).to_be_bytes());
        for item in &value.updates {
            let mut item_buf = Vec::new();
            MarketDataUpdateEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.extend_from_slice(&value.sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.sequence.is_some()));
    }
}

/// SBE body decoder for nested MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshBodyDecoder;

impl MarketDataIncrementalRefreshBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarketDataIncrementalRefresh, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let updates_count = sbe_u32(buf, &mut pos)? as usize;
        let mut updates =
            Vec::with_capacity(sbe_capacity(updates_count, buf.len().saturating_sub(pos)));
        for _ in 0..updates_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = MarketDataUpdateDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            updates.push(item);
        }
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let sequence_raw = sbe_u64(buf, &mut pos)?;
        let sequence = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sequence_raw)
        } else {
            None
        };

        let decoded = MarketDataIncrementalRefresh {
            symbol,
            updates,
            timestamp,
            sequence,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderBookRequest
pub struct OrderBookRequestBodyEncoder;

impl OrderBookRequestBodyEncoder {
    pub fn encode(value: &OrderBookRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.depth.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.depth.is_some()));
        buf.extend_from_slice(&value.at_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.at_time.is_some()));
    }
}

/// SBE body decoder for nested OrderBookRequest
pub struct OrderBookRequestBodyDecoder;

impl OrderBookRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderBookRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let depth_raw = sbe_u32(buf, &mut pos)?;
        let depth = if sbe_u8(buf, &mut pos)? == 1 {
            Some(depth_raw)
        } else {
            None
        };
        let at_time_raw = sbe_i64(buf, &mut pos)?;
        let at_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(at_time_raw)
        } else {
            None
        };

        let decoded = OrderBookRequest {
            symbol,
            depth,
            at_time,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderBookSnapshot
pub struct OrderBookSnapshotBodyEncoder;

impl OrderBookSnapshotBodyEncoder {
    pub fn encode(value: &OrderBookSnapshot, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let exchange_bytes = value.exchange.as_bytes();
        buf.extend_from_slice(&(exchange_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exchange_bytes);
        buf.extend_from_slice(&(value.bids.len() as u32).to_be_bytes());
        for item in &value.bids {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(value.asks.len() as u32).to_be_bytes());
        for item in &value.asks {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.extend_from_slice(&value.sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.sequence.is_some()));
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested OrderBookSnapshot
pub struct OrderBookSnapshotBodyDecoder;

impl OrderBookSnapshotBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderBookSnapshot, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let exchange = sbe_str(buf, &mut pos)?;
        let bids_count = sbe_u32(buf, &mut pos)? as usize;
        let mut bids = Vec::with_capacity(sbe_capacity(bids_count, buf.len().saturating_sub(pos)));
        for _ in 0..bids_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PriceLevelDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            bids.push(item);
        }
        let asks_count = sbe_u32(buf, &mut pos)? as usize;
        let mut asks = Vec::with_capacity(sbe_capacity(asks_count, buf.len().saturating_sub(pos)));
        for _ in 0..asks_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PriceLevelDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            asks.push(item);
        }
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let sequence_raw = sbe_u64(buf, &mut pos)?;
        let sequence = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sequence_raw)
        } else {
            None
        };
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = OrderBookSnapshot {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
            sequence,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderBookDelta
pub struct OrderBookDeltaBodyEncoder;

impl OrderBookDeltaBodyEncoder {
    pub fn encode(value: &OrderBookDelta, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(value.updates.len() as u32).to_be_bytes());
        for item in &value.updates {
            let mut item_buf = Vec::new();
            MarketDataUpdateEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.extend_from_slice(&value.sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.sequence.is_some()));
    }
}

/// SBE body decoder for nested OrderBookDelta
pub struct OrderBookDeltaBodyDecoder;

impl OrderBookDeltaBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderBookDelta, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let updates_count = sbe_u32(buf, &mut pos)? as usize;
        let mut updates =
            Vec::with_capacity(sbe_capacity(updates_count, buf.len().saturating_sub(pos)));
        for _ in 0..updates_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = MarketDataUpdateDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            updates.push(item);
        }
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let sequence_raw = sbe_u64(buf, &mut pos)?;
        let sequence = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sequence_raw)
        } else {
            None
        };

        let decoded = OrderBookDelta {
            symbol,
            updates,
            timestamp,
            sequence,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AggregateTradeEvent
pub struct AggregateTradeEventBodyEncoder;

impl AggregateTradeEventBodyEncoder {
    pub fn encode(value: &AggregateTradeEvent, buf: &mut Vec<u8>) {
        AggregateTradeEncoder::encode(&value.trade, buf);
    }
}

/// SBE body decoder for nested AggregateTradeEvent
pub struct AggregateTradeEventBodyDecoder;

impl AggregateTradeEventBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AggregateTradeEvent, usize), String> {
        let mut pos: usize = 0;

        let rest = buf
            .get(pos..)
            .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
        let (trade, trade_n) = AggregateTradeDecoder::decode(rest)?;
        pos += trade_n;

        let decoded = AggregateTradeEvent { trade };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AggregateTradeRequest
pub struct AggregateTradeRequestBodyEncoder;

impl AggregateTradeRequestBodyEncoder {
    pub fn encode(value: &AggregateTradeRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested AggregateTradeRequest
pub struct AggregateTradeRequestBodyDecoder;

impl AggregateTradeRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AggregateTradeRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = AggregateTradeRequest {
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AggregateTradeBatch
pub struct AggregateTradeBatchBodyEncoder;

impl AggregateTradeBatchBodyEncoder {
    pub fn encode(value: &AggregateTradeBatch, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(value.trades.len() as u32).to_be_bytes());
        for item in &value.trades {
            let mut item_buf = Vec::new();
            AggregateTradeEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested AggregateTradeBatch
pub struct AggregateTradeBatchBodyDecoder;

impl AggregateTradeBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AggregateTradeBatch, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let trades_count = sbe_u32(buf, &mut pos)? as usize;
        let mut trades =
            Vec::with_capacity(sbe_capacity(trades_count, buf.len().saturating_sub(pos)));
        for _ in 0..trades_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = AggregateTradeDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "AggregateTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            trades.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = AggregateTradeBatch {
            symbol,
            trades,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MiniTicker
pub struct MiniTickerBodyEncoder;

impl MiniTickerBodyEncoder {
    pub fn encode(value: &MiniTicker, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.last_price.0.to_be_bytes());
        buf.extend_from_slice(&value.volume.0.to_be_bytes());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested MiniTicker
pub struct MiniTickerBodyDecoder;

impl MiniTickerBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MiniTicker, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let last_price_raw = sbe_f64(buf, &mut pos)?;
        let last_price = Price(last_price_raw);
        let volume_raw = sbe_f64(buf, &mut pos)?;
        let volume = Quantity(volume_raw);
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = MiniTicker {
            symbol,
            last_price,
            volume,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AllMidsRequest
pub struct AllMidsRequestBodyEncoder;

impl AllMidsRequestBodyEncoder {
    pub fn encode(value: &AllMidsRequest, buf: &mut Vec<u8>) {}
}

/// SBE body decoder for nested AllMidsRequest
pub struct AllMidsRequestBodyDecoder;

impl AllMidsRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AllMidsRequest, usize), String> {
        let mut pos: usize = 0;

        let decoded = AllMidsRequest {};
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AllMidsBatch
pub struct AllMidsBatchBodyEncoder;

impl AllMidsBatchBodyEncoder {
    pub fn encode(value: &AllMidsBatch, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(value.tickers.len() as u32).to_be_bytes());
        for item in &value.tickers {
            let mut item_buf = Vec::new();
            MiniTickerBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
    }
}

/// SBE body decoder for nested AllMidsBatch
pub struct AllMidsBatchBodyDecoder;

impl AllMidsBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AllMidsBatch, usize), String> {
        let mut pos: usize = 0;

        let tickers_count = sbe_u32(buf, &mut pos)? as usize;
        let mut tickers =
            Vec::with_capacity(sbe_capacity(tickers_count, buf.len().saturating_sub(pos)));
        for _ in 0..tickers_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = MiniTickerBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "MiniTicker length mismatch: expected {item_len}, got {n}"
                ));
            }
            tickers.push(item);
        }

        let decoded = AllMidsBatch { tickers };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarkPriceUpdate
pub struct MarkPriceUpdateBodyEncoder;

impl MarkPriceUpdateBodyEncoder {
    pub fn encode(value: &MarkPriceUpdate, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.mark_price.0.to_be_bytes());
        buf.extend_from_slice(
            &value
                .index_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.index_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&value.funding_rate.unwrap_or(0.0).to_be_bytes());
        buf.push(if value.funding_rate.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested MarkPriceUpdate
pub struct MarkPriceUpdateBodyDecoder;

impl MarkPriceUpdateBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarkPriceUpdate, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let mark_price_raw = sbe_f64(buf, &mut pos)?;
        let mark_price = Price(mark_price_raw);
        let index_price_raw = sbe_f64(buf, &mut pos)?;
        let index_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(index_price_raw))
        } else {
            None
        };
        let funding_rate_raw = sbe_f64(buf, &mut pos)?;
        let funding_rate = if sbe_u8(buf, &mut pos)? == 1 {
            Some(funding_rate_raw)
        } else {
            None
        };
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = MarkPriceUpdate {
            symbol,
            mark_price,
            index_price,
            funding_rate,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarkPriceRequest
pub struct MarkPriceRequestBodyEncoder;

impl MarkPriceRequestBodyEncoder {
    pub fn encode(value: &MarkPriceRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
    }
}

/// SBE body decoder for nested MarkPriceRequest
pub struct MarkPriceRequestBodyDecoder;

impl MarkPriceRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarkPriceRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;

        let decoded = MarkPriceRequest { symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested LiquidationTradeEvent
pub struct LiquidationTradeEventBodyEncoder;

impl LiquidationTradeEventBodyEncoder {
    pub fn encode(value: &LiquidationTradeEvent, buf: &mut Vec<u8>) {
        LiquidationTradeEncoder::encode(&value.trade, buf);
    }
}

/// SBE body decoder for nested LiquidationTradeEvent
pub struct LiquidationTradeEventBodyDecoder;

impl LiquidationTradeEventBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(LiquidationTradeEvent, usize), String> {
        let mut pos: usize = 0;

        let rest = buf
            .get(pos..)
            .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
        let (trade, trade_n) = LiquidationTradeDecoder::decode(rest)?;
        pos += trade_n;

        let decoded = LiquidationTradeEvent { trade };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CandleBarEvent
pub struct CandleBarEventBodyEncoder;

impl CandleBarEventBodyEncoder {
    pub fn encode(value: &CandleBarEvent, buf: &mut Vec<u8>) {
        CandleBarEncoder::encode(&value.bar, buf);
    }
}

/// SBE body decoder for nested CandleBarEvent
pub struct CandleBarEventBodyDecoder;

impl CandleBarEventBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CandleBarEvent, usize), String> {
        let mut pos: usize = 0;

        let rest = buf
            .get(pos..)
            .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
        let (bar, bar_n) = CandleBarDecoder::decode(rest)?;
        pos += bar_n;

        let decoded = CandleBarEvent { bar };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CandleBarRequest
pub struct CandleBarRequestBodyEncoder;

impl CandleBarRequestBodyEncoder {
    pub fn encode(value: &CandleBarRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let interval_bytes = value.interval.as_bytes();
        buf.extend_from_slice(&(interval_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(interval_bytes);
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested CandleBarRequest
pub struct CandleBarRequestBodyDecoder;

impl CandleBarRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CandleBarRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let interval = sbe_str(buf, &mut pos)?;
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = CandleBarRequest {
            symbol,
            interval,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CandleBarBatch
pub struct CandleBarBatchBodyEncoder;

impl CandleBarBatchBodyEncoder {
    pub fn encode(value: &CandleBarBatch, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let interval_bytes = value.interval.as_bytes();
        buf.extend_from_slice(&(interval_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(interval_bytes);
        buf.extend_from_slice(&(value.bars.len() as u32).to_be_bytes());
        for item in &value.bars {
            let mut item_buf = Vec::new();
            CandleBarEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested CandleBarBatch
pub struct CandleBarBatchBodyDecoder;

impl CandleBarBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CandleBarBatch, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let interval = sbe_str(buf, &mut pos)?;
        let bars_count = sbe_u32(buf, &mut pos)? as usize;
        let mut bars = Vec::with_capacity(sbe_capacity(bars_count, buf.len().saturating_sub(pos)));
        for _ in 0..bars_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = CandleBarDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "CandleBar length mismatch: expected {item_len}, got {n}"
                ));
            }
            bars.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = CandleBarBatch {
            symbol,
            interval,
            bars,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PublicTradeEvent
pub struct PublicTradeEventBodyEncoder;

impl PublicTradeEventBodyEncoder {
    pub fn encode(value: &PublicTradeEvent, buf: &mut Vec<u8>) {
        PublicTradeEncoder::encode(&value.trade, buf);
    }
}

/// SBE body decoder for nested PublicTradeEvent
pub struct PublicTradeEventBodyDecoder;

impl PublicTradeEventBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PublicTradeEvent, usize), String> {
        let mut pos: usize = 0;

        let rest = buf
            .get(pos..)
            .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
        let (trade, trade_n) = PublicTradeDecoder::decode(rest)?;
        pos += trade_n;

        let decoded = PublicTradeEvent { trade };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested TradeHistoryRequest
pub struct TradeHistoryRequestBodyEncoder;

impl TradeHistoryRequestBodyEncoder {
    pub fn encode(value: &TradeHistoryRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested TradeHistoryRequest
pub struct TradeHistoryRequestBodyDecoder;

impl TradeHistoryRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(TradeHistoryRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = TradeHistoryRequest {
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PublicTradeBatch
pub struct PublicTradeBatchBodyEncoder;

impl PublicTradeBatchBodyEncoder {
    pub fn encode(value: &PublicTradeBatch, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(value.trades.len() as u32).to_be_bytes());
        for item in &value.trades {
            let mut item_buf = Vec::new();
            PublicTradeEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested PublicTradeBatch
pub struct PublicTradeBatchBodyDecoder;

impl PublicTradeBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PublicTradeBatch, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let trades_count = sbe_u32(buf, &mut pos)? as usize;
        let mut trades =
            Vec::with_capacity(sbe_capacity(trades_count, buf.len().saturating_sub(pos)));
        for _ in 0..trades_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PublicTradeDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PublicTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            trades.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = PublicTradeBatch {
            symbol,
            trades,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested BestBidOffer
pub struct BestBidOfferBodyEncoder;

impl BestBidOfferBodyEncoder {
    pub fn encode(value: &BestBidOffer, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(
            &value
                .bid_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.bid_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .bid_qty
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.bid_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .ask_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.ask_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .ask_qty
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.ask_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested BestBidOffer
pub struct BestBidOfferBodyDecoder;

impl BestBidOfferBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BestBidOffer, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let bid_price_raw = sbe_f64(buf, &mut pos)?;
        let bid_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(bid_price_raw))
        } else {
            None
        };
        let bid_qty_raw = sbe_f64(buf, &mut pos)?;
        let bid_qty = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Quantity(bid_qty_raw))
        } else {
            None
        };
        let ask_price_raw = sbe_f64(buf, &mut pos)?;
        let ask_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(ask_price_raw))
        } else {
            None
        };
        let ask_qty_raw = sbe_f64(buf, &mut pos)?;
        let ask_qty = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Quantity(ask_qty_raw))
        } else {
            None
        };
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = BestBidOffer {
            symbol,
            bid_price,
            bid_qty,
            ask_price,
            ask_qty,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested SymbolTicker
pub struct SymbolTickerBodyEncoder;

impl SymbolTickerBodyEncoder {
    pub fn encode(value: &SymbolTicker, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.last_price.0.to_be_bytes());
        buf.extend_from_slice(&value.price_change.to_be_bytes());
        buf.extend_from_slice(&value.price_change_pct.to_be_bytes());
        buf.extend_from_slice(&value.volume.0.to_be_bytes());
        buf.extend_from_slice(&value.high.0.to_be_bytes());
        buf.extend_from_slice(&value.low.0.to_be_bytes());
        buf.extend_from_slice(&value.open.0.to_be_bytes());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested SymbolTicker
pub struct SymbolTickerBodyDecoder;

impl SymbolTickerBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(SymbolTicker, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let last_price_raw = sbe_f64(buf, &mut pos)?;
        let last_price = Price(last_price_raw);
        let price_change_raw = sbe_f64(buf, &mut pos)?;
        let price_change = price_change_raw;
        let price_change_pct_raw = sbe_f64(buf, &mut pos)?;
        let price_change_pct = price_change_pct_raw;
        let volume_raw = sbe_f64(buf, &mut pos)?;
        let volume = Quantity(volume_raw);
        let high_raw = sbe_f64(buf, &mut pos)?;
        let high = Price(high_raw);
        let low_raw = sbe_f64(buf, &mut pos)?;
        let low = Price(low_raw);
        let open_raw = sbe_f64(buf, &mut pos)?;
        let open = Price(open_raw);
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = SymbolTicker {
            symbol,
            last_price,
            price_change,
            price_change_pct,
            volume,
            high,
            low,
            open,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested TickerRequest
pub struct TickerRequestBodyEncoder;

impl TickerRequestBodyEncoder {
    pub fn encode(value: &TickerRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
    }
}

/// SBE body decoder for nested TickerRequest
pub struct TickerRequestBodyDecoder;

impl TickerRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(TickerRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;

        let decoded = TickerRequest { symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested InstrumentCatalogRequest
pub struct InstrumentCatalogRequestBodyEncoder;

impl InstrumentCatalogRequestBodyEncoder {
    pub fn encode(value: &InstrumentCatalogRequest, buf: &mut Vec<u8>) {}
}

/// SBE body decoder for nested InstrumentCatalogRequest
pub struct InstrumentCatalogRequestBodyDecoder;

impl InstrumentCatalogRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(InstrumentCatalogRequest, usize), String> {
        let mut pos: usize = 0;

        let decoded = InstrumentCatalogRequest {};
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested InstrumentCatalogResponse
pub struct InstrumentCatalogResponseBodyEncoder;

impl InstrumentCatalogResponseBodyEncoder {
    pub fn encode(value: &InstrumentCatalogResponse, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(value.instruments.len() as u32).to_be_bytes());
        for item in &value.instruments {
            let mut item_buf = Vec::new();
            InstrumentMetadataEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
    }
}

/// SBE body decoder for nested InstrumentCatalogResponse
pub struct InstrumentCatalogResponseBodyDecoder;

impl InstrumentCatalogResponseBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(InstrumentCatalogResponse, usize), String> {
        let mut pos: usize = 0;

        let instruments_count = sbe_u32(buf, &mut pos)? as usize;
        let mut instruments = Vec::with_capacity(sbe_capacity(
            instruments_count,
            buf.len().saturating_sub(pos),
        ));
        for _ in 0..instruments_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = InstrumentMetadataDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "InstrumentMetadata length mismatch: expected {item_len}, got {n}"
                ));
            }
            instruments.push(item);
        }

        let decoded = InstrumentCatalogResponse { instruments };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested AccountSummary
pub struct AccountSummaryBodyEncoder;

impl AccountSummaryBodyEncoder {
    pub fn encode(value: &AccountSummary, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&value.balance.to_be_bytes());
        buf.extend_from_slice(&value.buying_power.to_be_bytes());
        let currency_bytes = value.currency.as_bytes();
        buf.extend_from_slice(&(currency_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(currency_bytes);
    }
}

/// SBE body decoder for nested AccountSummary
pub struct AccountSummaryBodyDecoder;

impl AccountSummaryBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(AccountSummary, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let balance_raw = sbe_f64(buf, &mut pos)?;
        let balance = balance_raw;
        let buying_power_raw = sbe_f64(buf, &mut pos)?;
        let buying_power = buying_power_raw;
        let currency = sbe_str(buf, &mut pos)?;

        let decoded = AccountSummary {
            account,
            balance,
            buying_power,
            currency,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarginSummary
pub struct MarginSummaryBodyEncoder;

impl MarginSummaryBodyEncoder {
    pub fn encode(value: &MarginSummary, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&value.balance.to_be_bytes());
        buf.extend_from_slice(&value.buying_power.to_be_bytes());
        buf.extend_from_slice(&value.equity.to_be_bytes());
        buf.extend_from_slice(&value.margin_used.to_be_bytes());
        buf.extend_from_slice(&value.available.to_be_bytes());
        let currency_bytes = value.currency.as_bytes();
        buf.extend_from_slice(&(currency_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(currency_bytes);
    }
}

/// SBE body decoder for nested MarginSummary
pub struct MarginSummaryBodyDecoder;

impl MarginSummaryBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarginSummary, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let balance_raw = sbe_f64(buf, &mut pos)?;
        let balance = balance_raw;
        let buying_power_raw = sbe_f64(buf, &mut pos)?;
        let buying_power = buying_power_raw;
        let equity_raw = sbe_f64(buf, &mut pos)?;
        let equity = equity_raw;
        let margin_used_raw = sbe_f64(buf, &mut pos)?;
        let margin_used = margin_used_raw;
        let available_raw = sbe_f64(buf, &mut pos)?;
        let available = available_raw;
        let currency = sbe_str(buf, &mut pos)?;

        let decoded = MarginSummary {
            account,
            balance,
            buying_power,
            equity,
            margin_used,
            available,
            currency,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested BalanceSnapshot
pub struct BalanceSnapshotBodyEncoder;

impl BalanceSnapshotBodyEncoder {
    pub fn encode(value: &BalanceSnapshot, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.balances.len() as u32).to_be_bytes());
        for item in &value.balances {
            let mut item_buf = Vec::new();
            BalanceEntryEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested BalanceSnapshot
pub struct BalanceSnapshotBodyDecoder;

impl BalanceSnapshotBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BalanceSnapshot, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let balances_count = sbe_u32(buf, &mut pos)? as usize;
        let mut balances =
            Vec::with_capacity(sbe_capacity(balances_count, buf.len().saturating_sub(pos)));
        for _ in 0..balances_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = BalanceEntryDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "BalanceEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            balances.push(item);
        }
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = BalanceSnapshot {
            account,
            balances,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested BalanceUpdate
pub struct BalanceUpdateBodyEncoder;

impl BalanceUpdateBodyEncoder {
    pub fn encode(value: &BalanceUpdate, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let asset_bytes = value.asset.as_bytes();
        buf.extend_from_slice(&(asset_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(asset_bytes);
        buf.extend_from_slice(&value.delta.to_be_bytes());
        buf.extend_from_slice(&value.total.to_be_bytes());
        buf.extend_from_slice(&value.available.to_be_bytes());
        buf.push(value.reason.to_value());
    }
}

/// SBE body decoder for nested BalanceUpdate
pub struct BalanceUpdateBodyDecoder;

impl BalanceUpdateBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BalanceUpdate, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let asset = sbe_str(buf, &mut pos)?;
        let delta_raw = sbe_f64(buf, &mut pos)?;
        let delta = delta_raw;
        let total_raw = sbe_f64(buf, &mut pos)?;
        let total = total_raw;
        let available_raw = sbe_f64(buf, &mut pos)?;
        let available = available_raw;
        let reason_raw = sbe_u8(buf, &mut pos)?;
        let reason = BalanceUpdateReason::from_value(reason_raw)
            .ok_or_else(|| format!("invalid BalanceUpdateReason value: {}", reason_raw))?;

        let decoded = BalanceUpdate {
            account,
            asset,
            delta,
            total,
            available,
            reason,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PositionSnapshot
pub struct PositionSnapshotBodyEncoder;

impl PositionSnapshotBodyEncoder {
    pub fn encode(value: &PositionSnapshot, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.positions.len() as u32).to_be_bytes());
        for item in &value.positions {
            let mut item_buf = Vec::new();
            PositionEntryEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested PositionSnapshot
pub struct PositionSnapshotBodyDecoder;

impl PositionSnapshotBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PositionSnapshot, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let positions_count = sbe_u32(buf, &mut pos)? as usize;
        let mut positions =
            Vec::with_capacity(sbe_capacity(positions_count, buf.len().saturating_sub(pos)));
        for _ in 0..positions_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = PositionEntryDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "PositionEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            positions.push(item);
        }
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = PositionSnapshot {
            account,
            positions,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PositionUpdate
pub struct PositionUpdateBodyEncoder;

impl PositionUpdateBodyEncoder {
    pub fn encode(value: &PositionUpdate, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.extend_from_slice(&value.entry_price.0.to_be_bytes());
        buf.extend_from_slice(&value.unrealized_pnl.to_be_bytes());
    }
}

/// SBE body decoder for nested PositionUpdate
pub struct PositionUpdateBodyDecoder;

impl PositionUpdateBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PositionUpdate, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = sbe_str(buf, &mut pos)?;
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let entry_price_raw = sbe_f64(buf, &mut pos)?;
        let entry_price = Price(entry_price_raw);
        let unrealized_pnl_raw = sbe_f64(buf, &mut pos)?;
        let unrealized_pnl = unrealized_pnl_raw;

        let decoded = PositionUpdate {
            account,
            symbol,
            qty,
            entry_price,
            unrealized_pnl,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MarginUpdate
pub struct MarginUpdateBodyEncoder;

impl MarginUpdateBodyEncoder {
    pub fn encode(value: &MarginUpdate, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        MarginSummaryBodyEncoder::encode(&value.summary, buf);
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested MarginUpdate
pub struct MarginUpdateBodyDecoder;

impl MarginUpdateBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MarginUpdate, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let rest = buf
            .get(pos..)
            .ok_or_else(|| format!("sbe: offset {} out of range", pos))?;
        let (summary, summary_n) = MarginSummaryBodyDecoder::decode(rest)?;
        pos += summary_n;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = MarginUpdate {
            account,
            summary,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested UserLiquidation
pub struct UserLiquidationBodyEncoder;

impl UserLiquidationBodyEncoder {
    pub fn encode(value: &UserLiquidation, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&value.qty.0.to_be_bytes());
        buf.extend_from_slice(&value.price.0.to_be_bytes());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for nested UserLiquidation
pub struct UserLiquidationBodyDecoder;

impl UserLiquidationBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(UserLiquidation, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = sbe_str(buf, &mut pos)?;
        let qty_raw = sbe_f64(buf, &mut pos)?;
        let qty = Quantity(qty_raw);
        let price_raw = sbe_f64(buf, &mut pos)?;
        let price = Price(price_raw);
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = UserLiquidation {
            account,
            symbol,
            qty,
            price,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderListStatus
pub struct OrderListStatusBodyEncoder;

impl OrderListStatusBodyEncoder {
    pub fn encode(value: &OrderListStatus, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let list_id_bytes = value.list_id.as_bytes();
        buf.extend_from_slice(&(list_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(list_id_bytes);
        buf.push(value.status.to_value());
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested OrderListStatus
pub struct OrderListStatusBodyDecoder;

impl OrderListStatusBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderListStatus, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let list_id = sbe_str(buf, &mut pos)?;
        let status_raw = sbe_u8(buf, &mut pos)?;
        let status = OrderListStatusStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid OrderListStatusStatus value: {}", status_raw))?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = OrderListStatus {
            account,
            list_id,
            status,
            symbol,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested FillHistoryRequest
pub struct FillHistoryRequestBodyEncoder;

impl FillHistoryRequestBodyEncoder {
    pub fn encode(value: &FillHistoryRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested FillHistoryRequest
pub struct FillHistoryRequestBodyDecoder;

impl FillHistoryRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(FillHistoryRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = FillHistoryRequest {
            account,
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested FillHistoryBatch
pub struct FillHistoryBatchBodyEncoder;

impl FillHistoryBatchBodyEncoder {
    pub fn encode(value: &FillHistoryBatch, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.fills.len() as u32).to_be_bytes());
        for item in &value.fills {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested FillHistoryBatch
pub struct FillHistoryBatchBodyDecoder;

impl FillHistoryBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(FillHistoryBatch, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let fills_count = sbe_u32(buf, &mut pos)? as usize;
        let mut fills =
            Vec::with_capacity(sbe_capacity(fills_count, buf.len().saturating_sub(pos)));
        for _ in 0..fills_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = ExecutionReportBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            fills.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = FillHistoryBatch {
            account,
            fills,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested FundingPayment
pub struct FundingPaymentBodyEncoder;

impl FundingPaymentBodyEncoder {
    pub fn encode(value: &FundingPayment, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&value.amount.to_be_bytes());
        buf.extend_from_slice(&value.rate.to_be_bytes());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for nested FundingPayment
pub struct FundingPaymentBodyDecoder;

impl FundingPaymentBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(FundingPayment, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let amount_raw = sbe_f64(buf, &mut pos)?;
        let amount = amount_raw;
        let rate_raw = sbe_f64(buf, &mut pos)?;
        let rate = rate_raw;
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = FundingPayment {
            account,
            symbol,
            amount,
            rate,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested FundingHistoryRequest
pub struct FundingHistoryRequestBodyEncoder;

impl FundingHistoryRequestBodyEncoder {
    pub fn encode(value: &FundingHistoryRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested FundingHistoryRequest
pub struct FundingHistoryRequestBodyDecoder;

impl FundingHistoryRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(FundingHistoryRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = FundingHistoryRequest {
            account,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested FundingHistoryBatch
pub struct FundingHistoryBatchBodyEncoder;

impl FundingHistoryBatchBodyEncoder {
    pub fn encode(value: &FundingHistoryBatch, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.payments.len() as u32).to_be_bytes());
        for item in &value.payments {
            let mut item_buf = Vec::new();
            FundingPaymentBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested FundingHistoryBatch
pub struct FundingHistoryBatchBodyDecoder;

impl FundingHistoryBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(FundingHistoryBatch, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let payments_count = sbe_u32(buf, &mut pos)? as usize;
        let mut payments =
            Vec::with_capacity(sbe_capacity(payments_count, buf.len().saturating_sub(pos)));
        for _ in 0..payments_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = FundingPaymentBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "FundingPayment length mismatch: expected {item_len}, got {n}"
                ));
            }
            payments.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = FundingHistoryBatch {
            account,
            payments,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested LedgerUpdate
pub struct LedgerUpdateBodyEncoder;

impl LedgerUpdateBodyEncoder {
    pub fn encode(value: &LedgerUpdate, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let asset_bytes = value.asset.as_bytes();
        buf.extend_from_slice(&(asset_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(asset_bytes);
        buf.extend_from_slice(&value.delta.to_be_bytes());
        buf.push(value.kind.to_value());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(if value.reference_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.reference_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested LedgerUpdate
pub struct LedgerUpdateBodyDecoder;

impl LedgerUpdateBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(LedgerUpdate, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let asset = sbe_str(buf, &mut pos)?;
        let delta_raw = sbe_f64(buf, &mut pos)?;
        let delta = delta_raw;
        let kind_raw = sbe_u8(buf, &mut pos)?;
        let kind = LedgerUpdateKind::from_value(kind_raw)
            .ok_or_else(|| format!("invalid LedgerUpdateKind value: {}", kind_raw))?;
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let reference_id = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = LedgerUpdate {
            account,
            asset,
            delta,
            kind,
            timestamp,
            reference_id,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested LedgerHistoryRequest
pub struct LedgerHistoryRequestBodyEncoder;

impl LedgerHistoryRequestBodyEncoder {
    pub fn encode(value: &LedgerHistoryRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested LedgerHistoryRequest
pub struct LedgerHistoryRequestBodyDecoder;

impl LedgerHistoryRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(LedgerHistoryRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = LedgerHistoryRequest {
            account,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested LedgerHistoryBatch
pub struct LedgerHistoryBatchBodyEncoder;

impl LedgerHistoryBatchBodyEncoder {
    pub fn encode(value: &LedgerHistoryBatch, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.entries.len() as u32).to_be_bytes());
        for item in &value.entries {
            let mut item_buf = Vec::new();
            LedgerUpdateBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested LedgerHistoryBatch
pub struct LedgerHistoryBatchBodyDecoder;

impl LedgerHistoryBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(LedgerHistoryBatch, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let entries_count = sbe_u32(buf, &mut pos)? as usize;
        let mut entries =
            Vec::with_capacity(sbe_capacity(entries_count, buf.len().saturating_sub(pos)));
        for _ in 0..entries_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = LedgerUpdateBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "LedgerUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            entries.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = LedgerHistoryBatch {
            account,
            entries,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OpenOrdersRequest
pub struct OpenOrdersRequestBodyEncoder;

impl OpenOrdersRequestBodyEncoder {
    pub fn encode(value: &OpenOrdersRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested OpenOrdersRequest
pub struct OpenOrdersRequestBodyDecoder;

impl OpenOrdersRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OpenOrdersRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = OpenOrdersRequest { account, symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OpenOrdersSnapshot
pub struct OpenOrdersSnapshotBodyEncoder;

impl OpenOrdersSnapshotBodyEncoder {
    pub fn encode(value: &OpenOrdersSnapshot, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.orders.len() as u32).to_be_bytes());
        for item in &value.orders {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested OpenOrdersSnapshot
pub struct OpenOrdersSnapshotBodyDecoder;

impl OpenOrdersSnapshotBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OpenOrdersSnapshot, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let orders_count = sbe_u32(buf, &mut pos)? as usize;
        let mut orders =
            Vec::with_capacity(sbe_capacity(orders_count, buf.len().saturating_sub(pos)));
        for _ in 0..orders_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = ExecutionReportBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            orders.push(item);
        }
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = OpenOrdersSnapshot {
            account,
            orders,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderHistoryRequest
pub struct OrderHistoryRequestBodyEncoder;

impl OrderHistoryRequestBodyEncoder {
    pub fn encode(value: &OrderHistoryRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&value.start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.start_time.is_some()));
        buf.extend_from_slice(&value.end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.end_time.is_some()));
        buf.extend_from_slice(&value.limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(value.limit.is_some()));
        buf.push(if value.cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested OrderHistoryRequest
pub struct OrderHistoryRequestBodyDecoder;

impl OrderHistoryRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderHistoryRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let start_time_raw = sbe_i64(buf, &mut pos)?;
        let start_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(start_time_raw)
        } else {
            None
        };
        let end_time_raw = sbe_i64(buf, &mut pos)?;
        let end_time = if sbe_u8(buf, &mut pos)? == 1 {
            Some(end_time_raw)
        } else {
            None
        };
        let limit_raw = sbe_u32(buf, &mut pos)?;
        let limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(limit_raw)
        } else {
            None
        };
        let cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = OrderHistoryRequest {
            account,
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested OrderHistoryBatch
pub struct OrderHistoryBatchBodyEncoder;

impl OrderHistoryBatchBodyEncoder {
    pub fn encode(value: &OrderHistoryBatch, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.orders.len() as u32).to_be_bytes());
        for item in &value.orders {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(value.has_more as u8);
        buf.push(if value.next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested OrderHistoryBatch
pub struct OrderHistoryBatchBodyDecoder;

impl OrderHistoryBatchBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(OrderHistoryBatch, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;
        let orders_count = sbe_u32(buf, &mut pos)? as usize;
        let mut orders =
            Vec::with_capacity(sbe_capacity(orders_count, buf.len().saturating_sub(pos)));
        for _ in 0..orders_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = ExecutionReportBodyDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            orders.push(item);
        }
        let has_more = sbe_u8(buf, &mut pos)? != 0;
        let next_cursor = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = OrderHistoryBatch {
            account,
            orders,
            has_more,
            next_cursor,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CapabilitiesRequest
pub struct CapabilitiesRequestBodyEncoder;

impl CapabilitiesRequestBodyEncoder {
    pub fn encode(value: &CapabilitiesRequest, buf: &mut Vec<u8>) {}
}

/// SBE body decoder for nested CapabilitiesRequest
pub struct CapabilitiesRequestBodyDecoder;

impl CapabilitiesRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CapabilitiesRequest, usize), String> {
        let mut pos: usize = 0;

        let decoded = CapabilitiesRequest {};
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested CapabilitiesResponse
pub struct CapabilitiesResponseBodyEncoder;

impl CapabilitiesResponseBodyEncoder {
    pub fn encode(value: &CapabilitiesResponse, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(value.schema_ids.len() as u32).to_be_bytes());
        for item in &value.schema_ids {
            buf.push(*item);
        }
        buf.extend_from_slice(&(value.paths.len() as u32).to_be_bytes());
        for item in &value.paths {
            let mut item_buf = Vec::new();
            CapabilityPathEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(value.symbols.len() as u32).to_be_bytes());
        for item in &value.symbols {
            let b = item.as_bytes();
            buf.extend_from_slice(&(b.len() as u32).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&(value.intervals.len() as u32).to_be_bytes());
        for item in &value.intervals {
            let b = item.as_bytes();
            buf.extend_from_slice(&(b.len() as u32).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested CapabilitiesResponse
pub struct CapabilitiesResponseBodyDecoder;

impl CapabilitiesResponseBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CapabilitiesResponse, usize), String> {
        let mut pos: usize = 0;

        let schema_ids_count = sbe_u32(buf, &mut pos)? as usize;
        let mut schema_ids = Vec::with_capacity(sbe_capacity(
            schema_ids_count,
            buf.len().saturating_sub(pos),
        ));
        for _ in 0..schema_ids_count {
            schema_ids.push(sbe_u8(buf, &mut pos)?);
        }
        let paths_count = sbe_u32(buf, &mut pos)? as usize;
        let mut paths =
            Vec::with_capacity(sbe_capacity(paths_count, buf.len().saturating_sub(pos)));
        for _ in 0..paths_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = CapabilityPathDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "CapabilityPath length mismatch: expected {item_len}, got {n}"
                ));
            }
            paths.push(item);
        }
        let symbols_count = sbe_u32(buf, &mut pos)? as usize;
        let mut symbols =
            Vec::with_capacity(sbe_capacity(symbols_count, buf.len().saturating_sub(pos)));
        for _ in 0..symbols_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            symbols.push(String::from_utf8_lossy(item_bytes).into_owned());
        }
        let intervals_count = sbe_u32(buf, &mut pos)? as usize;
        let mut intervals =
            Vec::with_capacity(sbe_capacity(intervals_count, buf.len().saturating_sub(pos)));
        for _ in 0..intervals_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            intervals.push(String::from_utf8_lossy(item_bytes).into_owned());
        }

        let decoded = CapabilitiesResponse {
            schema_ids,
            paths,
            symbols,
            intervals,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PositionRequest
pub struct PositionRequestBodyEncoder;

impl PositionRequestBodyEncoder {
    pub fn encode(value: &PositionRequest, buf: &mut Vec<u8>) {
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
    }
}

/// SBE body decoder for nested PositionRequest
pub struct PositionRequestBodyDecoder;

impl PositionRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PositionRequest, usize), String> {
        let mut pos: usize = 0;

        let account = sbe_str(buf, &mut pos)?;

        let decoded = PositionRequest { account };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested TradingSessionStatusRequest
pub struct TradingSessionStatusRequestBodyEncoder;

impl TradingSessionStatusRequestBodyEncoder {
    pub fn encode(value: &TradingSessionStatusRequest, buf: &mut Vec<u8>) {
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
    }
}

/// SBE body decoder for nested TradingSessionStatusRequest
pub struct TradingSessionStatusRequestBodyDecoder;

impl TradingSessionStatusRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(TradingSessionStatusRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };

        let decoded = TradingSessionStatusRequest { symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested TradingSessionStatus
pub struct TradingSessionStatusBodyEncoder;

impl TradingSessionStatusBodyEncoder {
    pub fn encode(value: &TradingSessionStatus, buf: &mut Vec<u8>) {
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        let trading_session_id_bytes = value.trading_session_id.as_bytes();
        buf.extend_from_slice(&(trading_session_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(trading_session_id_bytes);
        buf.push(value.status.to_value());
        buf.push(value.halt_reason.map(|v| v.to_value()).unwrap_or(0));
        buf.push(u8::from(value.halt_reason.is_some()));
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested TradingSessionStatus
pub struct TradingSessionStatusBodyDecoder;

impl TradingSessionStatusBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(TradingSessionStatus, usize), String> {
        let mut pos: usize = 0;

        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let trading_session_id = sbe_str(buf, &mut pos)?;
        let status_raw = sbe_u8(buf, &mut pos)?;
        let status = TradingSessionStatusStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid TradingSessionStatusStatus value: {}", status_raw))?;
        let halt_reason_raw = sbe_u8(buf, &mut pos)?;
        let halt_reason = if sbe_u8(buf, &mut pos)? == 1 {
            Some(
                TradingSessionStatusHaltReason::from_value(halt_reason_raw).ok_or_else(|| {
                    format!(
                        "invalid TradingSessionStatusHaltReason value: {}",
                        halt_reason_raw
                    )
                })?,
            )
        } else {
            None
        };
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = TradingSessionStatus {
            symbol,
            trading_session_id,
            status,
            halt_reason,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested SecurityStatusRequest
pub struct SecurityStatusRequestBodyEncoder;

impl SecurityStatusRequestBodyEncoder {
    pub fn encode(value: &SecurityStatusRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
    }
}

/// SBE body decoder for nested SecurityStatusRequest
pub struct SecurityStatusRequestBodyDecoder;

impl SecurityStatusRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(SecurityStatusRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;

        let decoded = SecurityStatusRequest { symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested SecurityStatus
pub struct SecurityStatusBodyEncoder;

impl SecurityStatusBodyEncoder {
    pub fn encode(value: &SecurityStatus, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(value.status.to_value());
        buf.push(value.halt_reason.map(|v| v.to_value()).unwrap_or(0));
        buf.push(u8::from(value.halt_reason.is_some()));
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested SecurityStatus
pub struct SecurityStatusBodyDecoder;

impl SecurityStatusBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(SecurityStatus, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let status_raw = sbe_u8(buf, &mut pos)?;
        let status = SecurityStatusStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid SecurityStatusStatus value: {}", status_raw))?;
        let halt_reason_raw = sbe_u8(buf, &mut pos)?;
        let halt_reason = if sbe_u8(buf, &mut pos)? == 1 {
            Some(
                SecurityStatusHaltReason::from_value(halt_reason_raw).ok_or_else(|| {
                    format!(
                        "invalid SecurityStatusHaltReason value: {}",
                        halt_reason_raw
                    )
                })?,
            )
        } else {
            None
        };
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = SecurityStatus {
            symbol,
            status,
            halt_reason,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MassCancelRequest
pub struct MassCancelRequestBodyEncoder;

impl MassCancelRequestBodyEncoder {
    pub fn encode(value: &MassCancelRequest, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(value.side.map(|v| v.to_value()).unwrap_or(0));
        buf.push(u8::from(value.side.is_some()));
    }
}

/// SBE body decoder for nested MassCancelRequest
pub struct MassCancelRequestBodyDecoder;

impl MassCancelRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MassCancelRequest, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let side_raw = sbe_u8(buf, &mut pos)?;
        let side = if sbe_u8(buf, &mut pos)? == 1 {
            Some(
                Side::from_value(side_raw)
                    .ok_or_else(|| format!("invalid Side value: {}", side_raw))?,
            )
        } else {
            None
        };

        let decoded = MassCancelRequest {
            cl_ord_id,
            account,
            symbol,
            side,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested MassCancelReport
pub struct MassCancelReportBodyEncoder;

impl MassCancelReportBodyEncoder {
    pub fn encode(value: &MassCancelReport, buf: &mut Vec<u8>) {
        let cl_ord_id_bytes = value.cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if value.symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = value.symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&value.total_affected.to_be_bytes());
        buf.push(value.status.to_value());
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for nested MassCancelReport
pub struct MassCancelReportBodyDecoder;

impl MassCancelReportBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(MassCancelReport, usize), String> {
        let mut pos: usize = 0;

        let cl_ord_id = sbe_str(buf, &mut pos)?;
        let account = sbe_str(buf, &mut pos)?;
        let symbol = if sbe_u8(buf, &mut pos)? == 1 {
            Some(sbe_str(buf, &mut pos)?)
        } else {
            None
        };
        let total_affected_raw = sbe_u32(buf, &mut pos)?;
        let total_affected = total_affected_raw;
        let status_raw = sbe_u8(buf, &mut pos)?;
        let status = MassCancelReportStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid MassCancelReportStatus value: {}", status_raw))?;
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = MassCancelReport {
            cl_ord_id,
            account,
            symbol,
            total_affected,
            status,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PriceLimitsRequest
pub struct PriceLimitsRequestBodyEncoder;

impl PriceLimitsRequestBodyEncoder {
    pub fn encode(value: &PriceLimitsRequest, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
    }
}

/// SBE body decoder for nested PriceLimitsRequest
pub struct PriceLimitsRequestBodyDecoder;

impl PriceLimitsRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PriceLimitsRequest, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;

        let decoded = PriceLimitsRequest { symbol };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested PriceLimits
pub struct PriceLimitsBodyEncoder;

impl PriceLimitsBodyEncoder {
    pub fn encode(value: &PriceLimits, buf: &mut Vec<u8>) {
        let symbol_bytes = value.symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(
            &value
                .lower_limit
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.lower_limit.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .upper_limit
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.upper_limit.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &value
                .reference_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if value.reference_price.is_some() {
            1
        } else {
            0
        });
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
        buf.push(value.is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(value.is_snapshot.is_some()));
    }
}

/// SBE body decoder for nested PriceLimits
pub struct PriceLimitsBodyDecoder;

impl PriceLimitsBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(PriceLimits, usize), String> {
        let mut pos: usize = 0;

        let symbol = sbe_str(buf, &mut pos)?;
        let lower_limit_raw = sbe_f64(buf, &mut pos)?;
        let lower_limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(lower_limit_raw))
        } else {
            None
        };
        let upper_limit_raw = sbe_f64(buf, &mut pos)?;
        let upper_limit = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(upper_limit_raw))
        } else {
            None
        };
        let reference_price_raw = sbe_f64(buf, &mut pos)?;
        let reference_price = if sbe_u8(buf, &mut pos)? == 1 {
            Some(Price(reference_price_raw))
        } else {
            None
        };
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;
        let is_snapshot_val = sbe_u8(buf, &mut pos)?;
        let is_snapshot = if sbe_u8(buf, &mut pos)? == 1 {
            Some(is_snapshot_val != 0)
        } else {
            None
        };

        let decoded = PriceLimits {
            symbol,
            lower_limit,
            upper_limit,
            reference_price,
            timestamp,
            is_snapshot,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested BatchRequest
pub struct BatchRequestBodyEncoder;

impl BatchRequestBodyEncoder {
    pub fn encode(value: &BatchRequest, buf: &mut Vec<u8>) {
        let batch_id_bytes = value.batch_id.as_bytes();
        buf.extend_from_slice(&(batch_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(batch_id_bytes);
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(value.actions.len() as u32).to_be_bytes());
        for item in &value.actions {
            let mut item_buf = Vec::new();
            BatchActionEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
    }
}

/// SBE body decoder for nested BatchRequest
pub struct BatchRequestBodyDecoder;

impl BatchRequestBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BatchRequest, usize), String> {
        let mut pos: usize = 0;

        let batch_id = sbe_str(buf, &mut pos)?;
        let account = sbe_str(buf, &mut pos)?;
        let actions_count = sbe_u32(buf, &mut pos)? as usize;
        let mut actions =
            Vec::with_capacity(sbe_capacity(actions_count, buf.len().saturating_sub(pos)));
        for _ in 0..actions_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = BatchActionDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "BatchAction length mismatch: expected {item_len}, got {n}"
                ));
            }
            actions.push(item);
        }

        let decoded = BatchRequest {
            batch_id,
            account,
            actions,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE body encoder for nested BatchReport
pub struct BatchReportBodyEncoder;

impl BatchReportBodyEncoder {
    pub fn encode(value: &BatchReport, buf: &mut Vec<u8>) {
        let batch_id_bytes = value.batch_id.as_bytes();
        buf.extend_from_slice(&(batch_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(batch_id_bytes);
        let account_bytes = value.account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(value.outcome.to_value());
        buf.extend_from_slice(&(value.results.len() as u32).to_be_bytes());
        for item in &value.results {
            let mut item_buf = Vec::new();
            BatchResultEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&value.timestamp.to_be_bytes());
    }
}

/// SBE body decoder for nested BatchReport
pub struct BatchReportBodyDecoder;

impl BatchReportBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(BatchReport, usize), String> {
        let mut pos: usize = 0;

        let batch_id = sbe_str(buf, &mut pos)?;
        let account = sbe_str(buf, &mut pos)?;
        let outcome_raw = sbe_u8(buf, &mut pos)?;
        let outcome = BatchReportOutcome::from_value(outcome_raw)
            .ok_or_else(|| format!("invalid BatchReportOutcome value: {}", outcome_raw))?;
        let results_count = sbe_u32(buf, &mut pos)? as usize;
        let mut results =
            Vec::with_capacity(sbe_capacity(results_count, buf.len().saturating_sub(pos)));
        for _ in 0..results_count {
            let item_len = sbe_u32(buf, &mut pos)? as usize;
            let item_bytes = sbe_take(buf, &mut pos, item_len)?;
            let (item, n) = BatchResultDecoder::decode(item_bytes)?;
            if n != item_len {
                return Err(format!(
                    "BatchResult length mismatch: expected {item_len}, got {n}"
                ));
            }
            results.push(item);
        }
        let timestamp_raw = sbe_i64(buf, &mut pos)?;
        let timestamp = timestamp_raw;

        let decoded = BatchReport {
            batch_id,
            account,
            outcome,
            results,
            timestamp,
        };
        decoded.validate()?;
        Ok((decoded, pos))
    }
}

/// SBE encoder for NewOrderSingle
pub struct NewOrderSingleEncoder;

impl NewOrderSingleEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &NewOrderSingle, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&1u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&38u16.to_be_bytes()); // block_length
        NewOrderSingleBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &NewOrderSingle) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 38);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for NewOrderSingle
pub struct NewOrderSingleDecoder;

impl NewOrderSingleDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<NewOrderSingle, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 1 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = NewOrderSingleBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CancelRequest
pub struct CancelRequestEncoder;

impl CancelRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CancelRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&2u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&9u16.to_be_bytes()); // block_length
        CancelRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CancelRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 9);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CancelRequest
pub struct CancelRequestDecoder;

impl CancelRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CancelRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 2 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CancelRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CancelReplaceRequest
pub struct CancelReplaceRequestEncoder;

impl CancelReplaceRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CancelReplaceRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&3u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length
        CancelReplaceRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CancelReplaceRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 17);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CancelReplaceRequest
pub struct CancelReplaceRequestDecoder;

impl CancelReplaceRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CancelReplaceRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 3 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CancelReplaceRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for ExecutionReport
pub struct ExecutionReportEncoder;

impl ExecutionReportEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &ExecutionReport, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&4u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&51u16.to_be_bytes()); // block_length
        ExecutionReportBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &ExecutionReport) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 51);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for ExecutionReport
pub struct ExecutionReportDecoder;

impl ExecutionReportDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<ExecutionReport, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 4 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = ExecutionReportBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CancelReject
pub struct CancelRejectEncoder;

impl CancelRejectEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CancelReject, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&5u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        CancelRejectBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CancelReject) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CancelReject
pub struct CancelRejectDecoder;

impl CancelRejectDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CancelReject, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 5 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CancelRejectBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarketDataSnapshot
pub struct MarketDataSnapshotEncoder;

impl MarketDataSnapshotEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarketDataSnapshot, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&6u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length
        MarketDataSnapshotBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarketDataSnapshot) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 17);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarketDataSnapshot
pub struct MarketDataSnapshotDecoder;

impl MarketDataSnapshotDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarketDataSnapshot, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 6 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarketDataSnapshotBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshEncoder;

impl MarketDataIncrementalRefreshEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarketDataIncrementalRefresh, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&7u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length
        MarketDataIncrementalRefreshBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarketDataIncrementalRefresh) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 16);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshDecoder;

impl MarketDataIncrementalRefreshDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarketDataIncrementalRefresh, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 7 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarketDataIncrementalRefreshBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderBookRequest
pub struct OrderBookRequestEncoder;

impl OrderBookRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderBookRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&8u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&12u16.to_be_bytes()); // block_length
        OrderBookRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderBookRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 12);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderBookRequest
pub struct OrderBookRequestDecoder;

impl OrderBookRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderBookRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 8 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderBookRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderBookSnapshot
pub struct OrderBookSnapshotEncoder;

impl OrderBookSnapshotEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderBookSnapshot, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&9u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length
        OrderBookSnapshotBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderBookSnapshot) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 17);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderBookSnapshot
pub struct OrderBookSnapshotDecoder;

impl OrderBookSnapshotDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderBookSnapshot, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 9 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderBookSnapshotBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderBookDelta
pub struct OrderBookDeltaEncoder;

impl OrderBookDeltaEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderBookDelta, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&10u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length
        OrderBookDeltaBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderBookDelta) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 16);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderBookDelta
pub struct OrderBookDeltaDecoder;

impl OrderBookDeltaDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderBookDelta, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 10 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderBookDeltaBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AggregateTradeEvent
pub struct AggregateTradeEventEncoder;

impl AggregateTradeEventEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AggregateTradeEvent, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&11u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        AggregateTradeEventBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AggregateTradeEvent) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AggregateTradeEvent
pub struct AggregateTradeEventDecoder;

impl AggregateTradeEventDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AggregateTradeEvent, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 11 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AggregateTradeEventBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AggregateTradeRequest
pub struct AggregateTradeRequestEncoder;

impl AggregateTradeRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AggregateTradeRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&12u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        AggregateTradeRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AggregateTradeRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AggregateTradeRequest
pub struct AggregateTradeRequestDecoder;

impl AggregateTradeRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AggregateTradeRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 12 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AggregateTradeRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AggregateTradeBatch
pub struct AggregateTradeBatchEncoder;

impl AggregateTradeBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AggregateTradeBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&13u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        AggregateTradeBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AggregateTradeBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AggregateTradeBatch
pub struct AggregateTradeBatchDecoder;

impl AggregateTradeBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AggregateTradeBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 13 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AggregateTradeBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MiniTicker
pub struct MiniTickerEncoder;

impl MiniTickerEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MiniTicker, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&14u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&25u16.to_be_bytes()); // block_length
        MiniTickerBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MiniTicker) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 25);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MiniTicker
pub struct MiniTickerDecoder;

impl MiniTickerDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MiniTicker, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 14 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MiniTickerBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AllMidsRequest
pub struct AllMidsRequestEncoder;

impl AllMidsRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AllMidsRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&15u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        AllMidsRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AllMidsRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AllMidsRequest
pub struct AllMidsRequestDecoder;

impl AllMidsRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AllMidsRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 15 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AllMidsRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AllMidsBatch
pub struct AllMidsBatchEncoder;

impl AllMidsBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AllMidsBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&16u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        AllMidsBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AllMidsBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AllMidsBatch
pub struct AllMidsBatchDecoder;

impl AllMidsBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AllMidsBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 16 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AllMidsBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarkPriceUpdate
pub struct MarkPriceUpdateEncoder;

impl MarkPriceUpdateEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarkPriceUpdate, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&17u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&33u16.to_be_bytes()); // block_length
        MarkPriceUpdateBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarkPriceUpdate) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 33);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarkPriceUpdate
pub struct MarkPriceUpdateDecoder;

impl MarkPriceUpdateDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarkPriceUpdate, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 17 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarkPriceUpdateBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarkPriceRequest
pub struct MarkPriceRequestEncoder;

impl MarkPriceRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarkPriceRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&18u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        MarkPriceRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarkPriceRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarkPriceRequest
pub struct MarkPriceRequestDecoder;

impl MarkPriceRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarkPriceRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 18 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarkPriceRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for LiquidationTradeEvent
pub struct LiquidationTradeEventEncoder;

impl LiquidationTradeEventEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &LiquidationTradeEvent, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&19u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        LiquidationTradeEventBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &LiquidationTradeEvent) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for LiquidationTradeEvent
pub struct LiquidationTradeEventDecoder;

impl LiquidationTradeEventDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<LiquidationTradeEvent, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 19 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = LiquidationTradeEventBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CandleBarEvent
pub struct CandleBarEventEncoder;

impl CandleBarEventEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CandleBarEvent, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&20u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        CandleBarEventBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CandleBarEvent) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CandleBarEvent
pub struct CandleBarEventDecoder;

impl CandleBarEventDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CandleBarEvent, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 20 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CandleBarEventBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CandleBarRequest
pub struct CandleBarRequestEncoder;

impl CandleBarRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CandleBarRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&21u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        CandleBarRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CandleBarRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CandleBarRequest
pub struct CandleBarRequestDecoder;

impl CandleBarRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CandleBarRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 21 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CandleBarRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CandleBarBatch
pub struct CandleBarBatchEncoder;

impl CandleBarBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CandleBarBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&22u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        CandleBarBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CandleBarBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CandleBarBatch
pub struct CandleBarBatchDecoder;

impl CandleBarBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CandleBarBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 22 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CandleBarBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PublicTradeEvent
pub struct PublicTradeEventEncoder;

impl PublicTradeEventEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PublicTradeEvent, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&23u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        PublicTradeEventBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PublicTradeEvent) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PublicTradeEvent
pub struct PublicTradeEventDecoder;

impl PublicTradeEventDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PublicTradeEvent, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 23 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PublicTradeEventBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for TradeHistoryRequest
pub struct TradeHistoryRequestEncoder;

impl TradeHistoryRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &TradeHistoryRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&24u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        TradeHistoryRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &TradeHistoryRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for TradeHistoryRequest
pub struct TradeHistoryRequestDecoder;

impl TradeHistoryRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<TradeHistoryRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 24 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = TradeHistoryRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PublicTradeBatch
pub struct PublicTradeBatchEncoder;

impl PublicTradeBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PublicTradeBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&25u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        PublicTradeBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PublicTradeBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PublicTradeBatch
pub struct PublicTradeBatchDecoder;

impl PublicTradeBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PublicTradeBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 25 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PublicTradeBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for BestBidOffer
pub struct BestBidOfferEncoder;

impl BestBidOfferEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &BestBidOffer, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&26u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&41u16.to_be_bytes()); // block_length
        BestBidOfferBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &BestBidOffer) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 41);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for BestBidOffer
pub struct BestBidOfferDecoder;

impl BestBidOfferDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<BestBidOffer, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 26 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = BestBidOfferBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for SymbolTicker
pub struct SymbolTickerEncoder;

impl SymbolTickerEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &SymbolTicker, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&27u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&65u16.to_be_bytes()); // block_length
        SymbolTickerBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &SymbolTicker) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 65);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for SymbolTicker
pub struct SymbolTickerDecoder;

impl SymbolTickerDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<SymbolTicker, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 27 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = SymbolTickerBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for TickerRequest
pub struct TickerRequestEncoder;

impl TickerRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &TickerRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&28u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        TickerRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &TickerRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for TickerRequest
pub struct TickerRequestDecoder;

impl TickerRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<TickerRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 28 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = TickerRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for InstrumentCatalogRequest
pub struct InstrumentCatalogRequestEncoder;

impl InstrumentCatalogRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &InstrumentCatalogRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&29u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        InstrumentCatalogRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &InstrumentCatalogRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for InstrumentCatalogRequest
pub struct InstrumentCatalogRequestDecoder;

impl InstrumentCatalogRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<InstrumentCatalogRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 29 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = InstrumentCatalogRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for InstrumentCatalogResponse
pub struct InstrumentCatalogResponseEncoder;

impl InstrumentCatalogResponseEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &InstrumentCatalogResponse, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&30u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        InstrumentCatalogResponseBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &InstrumentCatalogResponse) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for InstrumentCatalogResponse
pub struct InstrumentCatalogResponseDecoder;

impl InstrumentCatalogResponseDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<InstrumentCatalogResponse, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 30 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = InstrumentCatalogResponseBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for AccountSummary
pub struct AccountSummaryEncoder;

impl AccountSummaryEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &AccountSummary, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&31u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length
        AccountSummaryBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &AccountSummary) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 16);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for AccountSummary
pub struct AccountSummaryDecoder;

impl AccountSummaryDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<AccountSummary, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 31 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = AccountSummaryBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarginSummary
pub struct MarginSummaryEncoder;

impl MarginSummaryEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarginSummary, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&32u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&40u16.to_be_bytes()); // block_length
        MarginSummaryBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarginSummary) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 40);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarginSummary
pub struct MarginSummaryDecoder;

impl MarginSummaryDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarginSummary, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 32 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarginSummaryBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for BalanceSnapshot
pub struct BalanceSnapshotEncoder;

impl BalanceSnapshotEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &BalanceSnapshot, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&33u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        BalanceSnapshotBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &BalanceSnapshot) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for BalanceSnapshot
pub struct BalanceSnapshotDecoder;

impl BalanceSnapshotDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<BalanceSnapshot, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 33 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = BalanceSnapshotBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for BalanceUpdate
pub struct BalanceUpdateEncoder;

impl BalanceUpdateEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &BalanceUpdate, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&34u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&25u16.to_be_bytes()); // block_length
        BalanceUpdateBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &BalanceUpdate) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 25);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for BalanceUpdate
pub struct BalanceUpdateDecoder;

impl BalanceUpdateDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<BalanceUpdate, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 34 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = BalanceUpdateBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PositionSnapshot
pub struct PositionSnapshotEncoder;

impl PositionSnapshotEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PositionSnapshot, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&35u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        PositionSnapshotBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PositionSnapshot) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PositionSnapshot
pub struct PositionSnapshotDecoder;

impl PositionSnapshotDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PositionSnapshot, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 35 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PositionSnapshotBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PositionUpdate
pub struct PositionUpdateEncoder;

impl PositionUpdateEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PositionUpdate, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&36u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length
        PositionUpdateBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PositionUpdate) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 24);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PositionUpdate
pub struct PositionUpdateDecoder;

impl PositionUpdateDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PositionUpdate, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 36 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PositionUpdateBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MarginUpdate
pub struct MarginUpdateEncoder;

impl MarginUpdateEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MarginUpdate, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&37u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        MarginUpdateBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MarginUpdate) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MarginUpdate
pub struct MarginUpdateDecoder;

impl MarginUpdateDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MarginUpdate, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 37 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MarginUpdateBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for UserLiquidation
pub struct UserLiquidationEncoder;

impl UserLiquidationEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &UserLiquidation, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&38u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length
        UserLiquidationBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &UserLiquidation) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 24);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for UserLiquidation
pub struct UserLiquidationDecoder;

impl UserLiquidationDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<UserLiquidation, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 38 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = UserLiquidationBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderListStatus
pub struct OrderListStatusEncoder;

impl OrderListStatusEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderListStatus, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&39u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        OrderListStatusBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderListStatus) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderListStatus
pub struct OrderListStatusDecoder;

impl OrderListStatusDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderListStatus, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 39 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderListStatusBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for FillHistoryRequest
pub struct FillHistoryRequestEncoder;

impl FillHistoryRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &FillHistoryRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&40u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        FillHistoryRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &FillHistoryRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for FillHistoryRequest
pub struct FillHistoryRequestDecoder;

impl FillHistoryRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<FillHistoryRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 40 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = FillHistoryRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for FillHistoryBatch
pub struct FillHistoryBatchEncoder;

impl FillHistoryBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &FillHistoryBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&41u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        FillHistoryBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &FillHistoryBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for FillHistoryBatch
pub struct FillHistoryBatchDecoder;

impl FillHistoryBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<FillHistoryBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 41 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = FillHistoryBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for FundingPayment
pub struct FundingPaymentEncoder;

impl FundingPaymentEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &FundingPayment, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&42u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length
        FundingPaymentBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &FundingPayment) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 24);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for FundingPayment
pub struct FundingPaymentDecoder;

impl FundingPaymentDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<FundingPayment, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 42 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = FundingPaymentBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for FundingHistoryRequest
pub struct FundingHistoryRequestEncoder;

impl FundingHistoryRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &FundingHistoryRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&43u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        FundingHistoryRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &FundingHistoryRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for FundingHistoryRequest
pub struct FundingHistoryRequestDecoder;

impl FundingHistoryRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<FundingHistoryRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 43 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = FundingHistoryRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for FundingHistoryBatch
pub struct FundingHistoryBatchEncoder;

impl FundingHistoryBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &FundingHistoryBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&44u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        FundingHistoryBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &FundingHistoryBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for FundingHistoryBatch
pub struct FundingHistoryBatchDecoder;

impl FundingHistoryBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<FundingHistoryBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 44 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = FundingHistoryBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for LedgerUpdate
pub struct LedgerUpdateEncoder;

impl LedgerUpdateEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &LedgerUpdate, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&45u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length
        LedgerUpdateBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &LedgerUpdate) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 17);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for LedgerUpdate
pub struct LedgerUpdateDecoder;

impl LedgerUpdateDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<LedgerUpdate, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 45 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = LedgerUpdateBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for LedgerHistoryRequest
pub struct LedgerHistoryRequestEncoder;

impl LedgerHistoryRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &LedgerHistoryRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&46u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        LedgerHistoryRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &LedgerHistoryRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for LedgerHistoryRequest
pub struct LedgerHistoryRequestDecoder;

impl LedgerHistoryRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<LedgerHistoryRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 46 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = LedgerHistoryRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for LedgerHistoryBatch
pub struct LedgerHistoryBatchEncoder;

impl LedgerHistoryBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &LedgerHistoryBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&47u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        LedgerHistoryBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &LedgerHistoryBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for LedgerHistoryBatch
pub struct LedgerHistoryBatchDecoder;

impl LedgerHistoryBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<LedgerHistoryBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 47 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = LedgerHistoryBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OpenOrdersRequest
pub struct OpenOrdersRequestEncoder;

impl OpenOrdersRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OpenOrdersRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&48u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        OpenOrdersRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OpenOrdersRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OpenOrdersRequest
pub struct OpenOrdersRequestDecoder;

impl OpenOrdersRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OpenOrdersRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 48 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OpenOrdersRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OpenOrdersSnapshot
pub struct OpenOrdersSnapshotEncoder;

impl OpenOrdersSnapshotEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OpenOrdersSnapshot, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&49u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        OpenOrdersSnapshotBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OpenOrdersSnapshot) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OpenOrdersSnapshot
pub struct OpenOrdersSnapshotDecoder;

impl OpenOrdersSnapshotDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OpenOrdersSnapshot, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 49 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OpenOrdersSnapshotBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderHistoryRequest
pub struct OrderHistoryRequestEncoder;

impl OrderHistoryRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderHistoryRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&50u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length
        OrderHistoryRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderHistoryRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 20);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderHistoryRequest
pub struct OrderHistoryRequestDecoder;

impl OrderHistoryRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderHistoryRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 50 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderHistoryRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for OrderHistoryBatch
pub struct OrderHistoryBatchEncoder;

impl OrderHistoryBatchEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &OrderHistoryBatch, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&51u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        OrderHistoryBatchBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &OrderHistoryBatch) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for OrderHistoryBatch
pub struct OrderHistoryBatchDecoder;

impl OrderHistoryBatchDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<OrderHistoryBatch, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 51 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = OrderHistoryBatchBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CapabilitiesRequest
pub struct CapabilitiesRequestEncoder;

impl CapabilitiesRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CapabilitiesRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&52u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        CapabilitiesRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CapabilitiesRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CapabilitiesRequest
pub struct CapabilitiesRequestDecoder;

impl CapabilitiesRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CapabilitiesRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 52 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CapabilitiesRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for CapabilitiesResponse
pub struct CapabilitiesResponseEncoder;

impl CapabilitiesResponseEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &CapabilitiesResponse, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&53u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        CapabilitiesResponseBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &CapabilitiesResponse) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for CapabilitiesResponse
pub struct CapabilitiesResponseDecoder;

impl CapabilitiesResponseDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<CapabilitiesResponse, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 53 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = CapabilitiesResponseBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PositionRequest
pub struct PositionRequestEncoder;

impl PositionRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PositionRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&54u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        PositionRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PositionRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PositionRequest
pub struct PositionRequestDecoder;

impl PositionRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PositionRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 54 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PositionRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for TradingSessionStatusRequest
pub struct TradingSessionStatusRequestEncoder;

impl TradingSessionStatusRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &TradingSessionStatusRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&55u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        TradingSessionStatusRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &TradingSessionStatusRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for TradingSessionStatusRequest
pub struct TradingSessionStatusRequestDecoder;

impl TradingSessionStatusRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<TradingSessionStatusRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 55 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = TradingSessionStatusRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for TradingSessionStatus
pub struct TradingSessionStatusEncoder;

impl TradingSessionStatusEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &TradingSessionStatus, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&56u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&11u16.to_be_bytes()); // block_length
        TradingSessionStatusBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &TradingSessionStatus) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 11);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for TradingSessionStatus
pub struct TradingSessionStatusDecoder;

impl TradingSessionStatusDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<TradingSessionStatus, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 56 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = TradingSessionStatusBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for SecurityStatusRequest
pub struct SecurityStatusRequestEncoder;

impl SecurityStatusRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &SecurityStatusRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&57u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        SecurityStatusRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &SecurityStatusRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for SecurityStatusRequest
pub struct SecurityStatusRequestDecoder;

impl SecurityStatusRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<SecurityStatusRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 57 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = SecurityStatusRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for SecurityStatus
pub struct SecurityStatusEncoder;

impl SecurityStatusEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &SecurityStatus, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&58u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&11u16.to_be_bytes()); // block_length
        SecurityStatusBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &SecurityStatus) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 11);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for SecurityStatus
pub struct SecurityStatusDecoder;

impl SecurityStatusDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<SecurityStatus, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 58 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = SecurityStatusBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MassCancelRequest
pub struct MassCancelRequestEncoder;

impl MassCancelRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MassCancelRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&59u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length
        MassCancelRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MassCancelRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 1);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MassCancelRequest
pub struct MassCancelRequestDecoder;

impl MassCancelRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MassCancelRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 59 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MassCancelRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for MassCancelReport
pub struct MassCancelReportEncoder;

impl MassCancelReportEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &MassCancelReport, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&60u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&13u16.to_be_bytes()); // block_length
        MassCancelReportBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &MassCancelReport) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 13);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for MassCancelReport
pub struct MassCancelReportDecoder;

impl MassCancelReportDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<MassCancelReport, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 60 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = MassCancelReportBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PriceLimitsRequest
pub struct PriceLimitsRequestEncoder;

impl PriceLimitsRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PriceLimitsRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&61u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        PriceLimitsRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PriceLimitsRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PriceLimitsRequest
pub struct PriceLimitsRequestDecoder;

impl PriceLimitsRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PriceLimitsRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 61 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PriceLimitsRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for PriceLimits
pub struct PriceLimitsEncoder;

impl PriceLimitsEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &PriceLimits, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&62u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&33u16.to_be_bytes()); // block_length
        PriceLimitsBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &PriceLimits) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 33);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for PriceLimits
pub struct PriceLimitsDecoder;

impl PriceLimitsDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<PriceLimits, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 62 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = PriceLimitsBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for BatchRequest
pub struct BatchRequestEncoder;

impl BatchRequestEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &BatchRequest, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&63u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        BatchRequestBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &BatchRequest) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 0);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for BatchRequest
pub struct BatchRequestDecoder;

impl BatchRequestDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<BatchRequest, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 63 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = BatchRequestBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}

/// SBE encoder for BatchReport
pub struct BatchReportEncoder;

impl BatchReportEncoder {
    /// Append this message (header + body) onto `buf`.
    /// The caller should `buf.clear()` first when reusing the allocation.
    pub fn encode_into(value: &BatchReport, buf: &mut Vec<u8>) {
        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&64u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&9u16.to_be_bytes()); // block_length
        BatchReportBodyEncoder::encode(value, buf);
    }

    /// Encode this message into a new buffer.
    pub fn encode(value: &BatchReport) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + 9);
        Self::encode_into(value, &mut buf);
        buf
    }
}

/// SBE decoder for BatchReport
pub struct BatchReportDecoder;

impl BatchReportDecoder {
    /// Decode from a byte buffer into the FSL message type.
    pub fn decode(buf: &[u8]) -> Result<BatchReport, String> {
        if buf.len() < 8 {
            return Err("buffer too short for SBE header".to_string());
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let tmpl_id = u16::from_be_bytes([buf[2], buf[3]]);
        // version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length = u16::from_be_bytes([buf[6], buf[7]]);

        if schema_id != 1 {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if tmpl_id != 64 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let rest = buf
            .get(8..)
            .ok_or_else(|| "buffer too short for SBE header".to_string())?;
        let (msg, _) = BatchReportBodyDecoder::decode(rest)?;
        Ok(msg)
    }
}
