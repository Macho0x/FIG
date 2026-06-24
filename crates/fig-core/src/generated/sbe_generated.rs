// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

use crate::messages::{
    AccountSummary, AggregateTrade, AggregateTradeBatch, AggregateTradeEvent,
    AggregateTradeRequest, AllMidsBatch, AllMidsRequest, BalanceEntry, BalanceSnapshot,
    BalanceUpdate, BalanceUpdateReason, BestBidOffer, CancelReject, CancelRejectReason,
    CancelReplaceRequest, CancelRequest, CandleBar, CandleBarBatch, CandleBarEvent,
    CandleBarRequest, CandleInterval, CapabilitiesRequest, CapabilitiesResponse, CapabilityPath,
    CapabilityPathPattern, ClientOrderId, ExecType, ExecutionReport, FillHistoryBatch,
    FillHistoryRequest, FundingHistoryBatch, FundingHistoryRequest, FundingPayment,
    InstrumentCatalogRequest, InstrumentCatalogResponse, InstrumentMetadata, LedgerHistoryBatch,
    LedgerHistoryRequest, LedgerUpdate, LedgerUpdateKind, LiquidationTrade, LiquidationTradeEvent,
    MarginSummary, MarginUpdate, MarkPriceRequest, MarkPriceUpdate, MarketDataAction,
    MarketDataIncrementalRefresh, MarketDataSnapshot, MarketDataUpdate, MiniTicker, NewOrderSingle,
    OpenOrdersRequest, OpenOrdersSnapshot, OrdStatus, OrderBookDelta, OrderBookRequest,
    OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    OrderListStatusStatus, OrderType, PageInfo, PositionEntry, PositionRequest, PositionSnapshot,
    PositionUpdate, Price, PriceLevel, PublicTrade, PublicTradeBatch, PublicTradeEvent, Quantity,
    SecurityIdSource, Side, Symbol, SymbolTicker, TickerRequest, TimeInForce, TradeHistoryRequest,
    TradeTimestamp, UserLiquidation,
};

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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let agg_trade_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let agg_trade_id_bytes = &buf[pos..pos + agg_trade_id_len];
        pos += agg_trade_id_len;
        let agg_trade_id = std::str::from_utf8(agg_trade_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let first_trade_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let first_trade_id_bytes = &buf[pos..pos + first_trade_id_len];
        pos += first_trade_id_len;
        let first_trade_id = std::str::from_utf8(first_trade_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let last_trade_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let last_trade_id_bytes = &buf[pos..pos + last_trade_id_len];
        pos += last_trade_id_len;
        let last_trade_id = std::str::from_utf8(last_trade_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok((
            AggregateTrade {
                symbol,
                agg_trade_id,
                price,
                qty,
                side,
                first_trade_id,
                last_trade_id,
                timestamp,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok((
            LiquidationTrade {
                symbol,
                side,
                price,
                qty,
                timestamp,
            },
            pos,
        ))
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

        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let order_count_raw =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let order_count = if buf[pos] == 1 {
            pos += 1;
            Some(order_count_raw)
        } else {
            pos += 1;
            None
        };

        Ok((
            PriceLevel {
                price,
                qty,
                order_count,
            },
            pos,
        ))
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

        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let action_raw = buf[pos];
        pos += 1;
        let action = MarketDataAction::from_value(action_raw)
            .ok_or_else(|| format!("invalid MarketDataAction value: {}", action_raw))?;
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);

        Ok((
            MarketDataUpdate {
                side,
                action,
                price,
                qty,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let interval_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let interval_bytes = &buf[pos..pos + interval_len];
        pos += interval_len;
        let interval = std::str::from_utf8(interval_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let open_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let open = Price(open_raw);
        let high_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let high = Price(high_raw);
        let low_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let low = Price(low_raw);
        let close_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let close = Price(close_raw);
        let volume_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let volume = Quantity(volume_raw);
        let bar_start_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bar_start = bar_start_raw;
        let bar_end_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bar_end = bar_end_raw;
        let is_final = buf[pos] != 0;
        pos += 1;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            CandleBar {
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
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let trade_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let trade_id_bytes = &buf[pos..pos + trade_id_len];
        pos += trade_id_len;
        let trade_id = std::str::from_utf8(trade_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok((
            PublicTrade {
                symbol,
                trade_id,
                price,
                qty,
                side,
                timestamp,
            },
            pos,
        ))
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

        let instrument_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let instrument_id_bytes = &buf[pos..pos + instrument_id_len];
        pos += instrument_id_len;
        let instrument_id = std::str::from_utf8(instrument_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let product_kind_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let product_kind_bytes = &buf[pos..pos + product_kind_len];
        pos += product_kind_len;
        let product_kind = std::str::from_utf8(product_kind_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let margin_asset = if buf[pos] == 1 {
            pos += 1;
            let margin_asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let margin_asset_bytes = &buf[pos..pos + margin_asset_len];
            pos += margin_asset_len;
            Some(
                std::str::from_utf8(margin_asset_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let display_name = if buf[pos] == 1 {
            pos += 1;
            let display_name_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let display_name_bytes = &buf[pos..pos + display_name_len];
            pos += display_name_len;
            Some(
                std::str::from_utf8(display_name_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let tick_size_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let tick_size = if buf[pos] == 1 {
            pos += 1;
            Some(tick_size_raw)
        } else {
            pos += 1;
            None
        };
        let lot_size_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let lot_size = if buf[pos] == 1 {
            pos += 1;
            Some(lot_size_raw)
        } else {
            pos += 1;
            None
        };

        Ok((
            InstrumentMetadata {
                instrument_id,
                symbol,
                product_kind,
                margin_asset,
                display_name,
                tick_size,
                lot_size,
            },
            pos,
        ))
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

        let asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let asset_bytes = &buf[pos..pos + asset_len];
        pos += asset_len;
        let asset = std::str::from_utf8(asset_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let total_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let total = total_raw;
        let available_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let available = available_raw;
        let hold_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let hold = hold_raw;

        Ok((
            BalanceEntry {
                asset,
                total,
                available,
                hold,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let entry_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let entry_price = Price(entry_price_raw);
        let unrealized_pnl_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let unrealized_pnl = unrealized_pnl_raw;

        Ok((
            PositionEntry {
                symbol,
                qty,
                entry_price,
                unrealized_pnl,
            },
            pos,
        ))
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

        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            PageInfo {
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let path_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let path_bytes = &buf[pos..pos + path_len];
        pos += path_len;
        let path = std::str::from_utf8(path_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let pattern_raw = buf[pos];
        pos += 1;
        let pattern = CapabilityPathPattern::from_value(pattern_raw)
            .ok_or_else(|| format!("invalid CapabilityPathPattern value: {}", pattern_raw))?;
        let auth_required = buf[pos] != 0;
        pos += 1;

        Ok((
            CapabilityPath {
                path,
                pattern,
                auth_required,
            },
            pos,
        ))
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(price_raw))
        } else {
            pos += 1;
            None
        };
        let stop_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let stop_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(stop_price_raw))
        } else {
            pos += 1;
            None
        };
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let order_type_raw = buf[pos];
        pos += 1;
        let order_type = OrderType::from_value(order_type_raw)
            .ok_or_else(|| format!("invalid OrderType value: {}", order_type_raw))?;
        let time_in_force_raw = buf[pos];
        pos += 1;
        let time_in_force = TimeInForce::from_value(time_in_force_raw)
            .ok_or_else(|| format!("invalid TimeInForce value: {}", time_in_force_raw))?;
        let expire_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let expire_time = if buf[pos] == 1 {
            pos += 1;
            Some(expire_time_raw)
        } else {
            pos += 1;
            None
        };
        let account = if buf[pos] == 1 {
            pos += 1;
            let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let account_bytes = &buf[pos..pos + account_len];
            pos += account_len;
            Some(
                std::str::from_utf8(account_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let strategy_id = if buf[pos] == 1 {
            pos += 1;
            let strategy_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let strategy_id_bytes = &buf[pos..pos + strategy_id_len];
            pos += strategy_id_len;
            Some(
                std::str::from_utf8(strategy_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let security_id = if buf[pos] == 1 {
            pos += 1;
            let security_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let security_id_bytes = &buf[pos..pos + security_id_len];
            pos += security_id_len;
            Some(
                std::str::from_utf8(security_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let id_source_raw = buf[pos];
        pos += 1;
        let id_source = if buf[pos] == 1 {
            pos += 1;
            Some(
                SecurityIdSource::from_value(id_source_raw)
                    .ok_or_else(|| format!("invalid SecurityIdSource value: {}", id_source_raw))?,
            )
        } else {
            pos += 1;
            None
        };
        let security_exchange = if buf[pos] == 1 {
            pos += 1;
            let security_exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let security_exchange_bytes = &buf[pos..pos + security_exchange_len];
            pos += security_exchange_len;
            Some(
                std::str::from_utf8(security_exchange_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let post_only_val = buf[pos];
        pos += 1;
        let post_only = if buf[pos] == 1 {
            pos += 1;
            Some(post_only_val != 0)
        } else {
            pos += 1;
            None
        };
        let reduce_only_val = buf[pos];
        pos += 1;
        let reduce_only = if buf[pos] == 1 {
            pos += 1;
            Some(reduce_only_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            NewOrderSingle {
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
            },
            pos,
        ))
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(order_qty_raw))
        } else {
            pos += 1;
            None
        };

        Ok((
            CancelRequest {
                cl_ord_id,
                orig_cl_ord_id,
                symbol,
                side,
                order_qty,
            },
            pos,
        ))
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(price_raw))
        } else {
            pos += 1;
            None
        };

        Ok((
            CancelReplaceRequest {
                cl_ord_id,
                orig_cl_ord_id,
                symbol,
                side,
                order_qty,
                price,
            },
            pos,
        ))
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let order_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let order_id_bytes = &buf[pos..pos + order_id_len];
        pos += order_id_len;
        let order_id = std::str::from_utf8(order_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exec_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exec_id_bytes = &buf[pos..pos + exec_id_len];
        pos += exec_id_len;
        let exec_id = std::str::from_utf8(exec_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exec_type_raw = buf[pos];
        pos += 1;
        let exec_type = ExecType::from_value(exec_type_raw)
            .ok_or_else(|| format!("invalid ExecType value: {}", exec_type_raw))?;
        let ord_status_raw = buf[pos];
        pos += 1;
        let ord_status = OrdStatus::from_value(ord_status_raw)
            .ok_or_else(|| format!("invalid OrdStatus value: {}", ord_status_raw))?;
        let side_raw = buf[pos];
        pos += 1;
        let side = Side::from_value(side_raw)
            .ok_or_else(|| format!("invalid Side value: {}", side_raw))?;
        let last_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(last_qty_raw))
        } else {
            pos += 1;
            None
        };
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(last_price_raw))
        } else {
            pos += 1;
            None
        };
        let leaves_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let leaves_qty = Quantity(leaves_qty_raw);
        let cum_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let cum_qty = Quantity(cum_qty_raw);
        let avg_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let avg_price = Price(avg_price_raw);
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let transact_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let transact_time = transact_time_raw;

        Ok((
            ExecutionReport {
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
            },
            pos,
        ))
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let reject_reason_raw = buf[pos];
        pos += 1;
        let reject_reason = CancelRejectReason::from_value(reject_reason_raw)
            .ok_or_else(|| format!("invalid CancelRejectReason value: {}", reject_reason_raw))?;
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((
            CancelReject {
                cl_ord_id,
                orig_cl_ord_id,
                reject_reason,
                symbol,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exchange_bytes = &buf[pos..pos + exchange_len];
        pos += exchange_len;
        let exchange = std::str::from_utf8(exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bids.push(item);
        }
        let asks_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            asks.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            MarketDataSnapshot {
                symbol,
                exchange,
                bids,
                asks,
                timestamp,
                sequence,
                is_snapshot,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let updates_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MarketDataUpdateDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            updates.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };

        Ok((
            MarketDataIncrementalRefresh {
                symbol,
                updates,
                timestamp,
                sequence,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let depth_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let depth = if buf[pos] == 1 {
            pos += 1;
            Some(depth_raw)
        } else {
            pos += 1;
            None
        };
        let at_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let at_time = if buf[pos] == 1 {
            pos += 1;
            Some(at_time_raw)
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderBookRequest {
                symbol,
                depth,
                at_time,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exchange_bytes = &buf[pos..pos + exchange_len];
        pos += exchange_len;
        let exchange = std::str::from_utf8(exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bids.push(item);
        }
        let asks_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            asks.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderBookSnapshot {
                symbol,
                exchange,
                bids,
                asks,
                timestamp,
                sequence,
                is_snapshot,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let updates_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MarketDataUpdateDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            updates.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderBookDelta {
                symbol,
                updates,
                timestamp,
                sequence,
            },
            pos,
        ))
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

        let (trade, trade_n) = AggregateTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok((AggregateTradeEvent { trade }, pos))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            AggregateTradeRequest {
                symbol,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let trades_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut trades = Vec::with_capacity(trades_count);
        for _ in 0..trades_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = AggregateTradeDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "AggregateTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            trades.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            AggregateTradeBatch {
                symbol,
                trades,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = Price(last_price_raw);
        let volume_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let volume = Quantity(volume_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            MiniTicker {
                symbol,
                last_price,
                volume,
                timestamp,
                is_snapshot,
            },
            pos,
        ))
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

        Ok((AllMidsRequest {}, pos))
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

        let tickers_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut tickers = Vec::with_capacity(tickers_count);
        for _ in 0..tickers_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MiniTickerBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MiniTicker length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            tickers.push(item);
        }

        Ok((AllMidsBatch { tickers }, pos))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let mark_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let mark_price = Price(mark_price_raw);
        let index_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let index_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(index_price_raw))
        } else {
            pos += 1;
            None
        };
        let funding_rate_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let funding_rate = if buf[pos] == 1 {
            pos += 1;
            Some(funding_rate_raw)
        } else {
            pos += 1;
            None
        };
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            MarkPriceUpdate {
                symbol,
                mark_price,
                index_price,
                funding_rate,
                timestamp,
                is_snapshot,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((MarkPriceRequest { symbol }, pos))
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

        let (trade, trade_n) = LiquidationTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok((LiquidationTradeEvent { trade }, pos))
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

        let (bar, bar_n) = CandleBarDecoder::decode(&buf[pos..])?;
        pos += bar_n;

        Ok((CandleBarEvent { bar }, pos))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let interval_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let interval_bytes = &buf[pos..pos + interval_len];
        pos += interval_len;
        let interval = std::str::from_utf8(interval_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            CandleBarRequest {
                symbol,
                interval,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let interval_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let interval_bytes = &buf[pos..pos + interval_len];
        pos += interval_len;
        let interval = std::str::from_utf8(interval_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bars_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bars = Vec::with_capacity(bars_count);
        for _ in 0..bars_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = CandleBarDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "CandleBar length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bars.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            CandleBarBatch {
                symbol,
                interval,
                bars,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let (trade, trade_n) = PublicTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok((PublicTradeEvent { trade }, pos))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            TradeHistoryRequest {
                symbol,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let trades_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut trades = Vec::with_capacity(trades_count);
        for _ in 0..trades_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PublicTradeDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PublicTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            trades.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            PublicTradeBatch {
                symbol,
                trades,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bid_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bid_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(bid_price_raw))
        } else {
            pos += 1;
            None
        };
        let bid_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bid_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(bid_qty_raw))
        } else {
            pos += 1;
            None
        };
        let ask_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let ask_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(ask_price_raw))
        } else {
            pos += 1;
            None
        };
        let ask_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let ask_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(ask_qty_raw))
        } else {
            pos += 1;
            None
        };
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            BestBidOffer {
                symbol,
                bid_price,
                bid_qty,
                ask_price,
                ask_qty,
                timestamp,
                is_snapshot,
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = Price(last_price_raw);
        let price_change_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price_change = price_change_raw;
        let price_change_pct_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price_change_pct = price_change_pct_raw;
        let volume_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let volume = Quantity(volume_raw);
        let high_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let high = Price(high_raw);
        let low_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let low = Price(low_raw);
        let open_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let open = Price(open_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            SymbolTicker {
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
            },
            pos,
        ))
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((TickerRequest { symbol }, pos))
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

        Ok((InstrumentCatalogRequest {}, pos))
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

        let instruments_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut instruments = Vec::with_capacity(instruments_count);
        for _ in 0..instruments_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = InstrumentMetadataDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "InstrumentMetadata length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            instruments.push(item);
        }

        Ok((InstrumentCatalogResponse { instruments }, pos))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balance_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let balance = balance_raw;
        let buying_power_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let buying_power = buying_power_raw;
        let currency_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let currency_bytes = &buf[pos..pos + currency_len];
        pos += currency_len;
        let currency = std::str::from_utf8(currency_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((
            AccountSummary {
                account,
                balance,
                buying_power,
                currency,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balance_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let balance = balance_raw;
        let buying_power_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let buying_power = buying_power_raw;
        let equity_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let equity = equity_raw;
        let margin_used_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let margin_used = margin_used_raw;
        let available_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let available = available_raw;
        let currency_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let currency_bytes = &buf[pos..pos + currency_len];
        pos += currency_len;
        let currency = std::str::from_utf8(currency_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((
            MarginSummary {
                account,
                balance,
                buying_power,
                equity,
                margin_used,
                available,
                currency,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balances_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut balances = Vec::with_capacity(balances_count);
        for _ in 0..balances_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = BalanceEntryDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "BalanceEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            balances.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            BalanceSnapshot {
                account,
                balances,
                is_snapshot,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let asset_bytes = &buf[pos..pos + asset_len];
        pos += asset_len;
        let asset = std::str::from_utf8(asset_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let delta_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let delta = delta_raw;
        let total_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let total = total_raw;
        let available_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let available = available_raw;
        let reason_raw = buf[pos];
        pos += 1;
        let reason = BalanceUpdateReason::from_value(reason_raw)
            .ok_or_else(|| format!("invalid BalanceUpdateReason value: {}", reason_raw))?;

        Ok((
            BalanceUpdate {
                account,
                asset,
                delta,
                total,
                available,
                reason,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let positions_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut positions = Vec::with_capacity(positions_count);
        for _ in 0..positions_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PositionEntryDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PositionEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            positions.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            PositionSnapshot {
                account,
                positions,
                is_snapshot,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let entry_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let entry_price = Price(entry_price_raw);
        let unrealized_pnl_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let unrealized_pnl = unrealized_pnl_raw;

        Ok((
            PositionUpdate {
                account,
                symbol,
                qty,
                entry_price,
                unrealized_pnl,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let (summary, summary_n) = MarginSummaryBodyDecoder::decode(&buf[pos..])?;
        pos += summary_n;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            MarginUpdate {
                account,
                summary,
                is_snapshot,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok((
            UserLiquidation {
                account,
                symbol,
                qty,
                price,
                timestamp,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let list_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let list_id_bytes = &buf[pos..pos + list_id_len];
        pos += list_id_len;
        let list_id = std::str::from_utf8(list_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let status_raw = buf[pos];
        pos += 1;
        let status = OrderListStatusStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid OrderListStatusStatus value: {}", status_raw))?;
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderListStatus {
                account,
                list_id,
                status,
                symbol,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            FillHistoryRequest {
                account,
                symbol,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let fills_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut fills = Vec::with_capacity(fills_count);
        for _ in 0..fills_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            fills.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            FillHistoryBatch {
                account,
                fills,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let amount_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let amount = amount_raw;
        let rate_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let rate = rate_raw;
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok((
            FundingPayment {
                account,
                symbol,
                amount,
                rate,
                timestamp,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            FundingHistoryRequest {
                account,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let payments_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut payments = Vec::with_capacity(payments_count);
        for _ in 0..payments_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = FundingPaymentBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "FundingPayment length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            payments.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            FundingHistoryBatch {
                account,
                payments,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let asset_bytes = &buf[pos..pos + asset_len];
        pos += asset_len;
        let asset = std::str::from_utf8(asset_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let delta_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let delta = delta_raw;
        let kind_raw = buf[pos];
        pos += 1;
        let kind = LedgerUpdateKind::from_value(kind_raw)
            .ok_or_else(|| format!("invalid LedgerUpdateKind value: {}", kind_raw))?;
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let reference_id = if buf[pos] == 1 {
            pos += 1;
            let reference_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let reference_id_bytes = &buf[pos..pos + reference_id_len];
            pos += reference_id_len;
            Some(
                std::str::from_utf8(reference_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            LedgerUpdate {
                account,
                asset,
                delta,
                kind,
                timestamp,
                reference_id,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            LedgerHistoryRequest {
                account,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let entries_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut entries = Vec::with_capacity(entries_count);
        for _ in 0..entries_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = LedgerUpdateBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "LedgerUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            entries.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            LedgerHistoryBatch {
                account,
                entries,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((OpenOrdersRequest { account, symbol }, pos))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orders_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut orders = Vec::with_capacity(orders_count);
        for _ in 0..orders_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            orders.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok((
            OpenOrdersSnapshot {
                account,
                orders,
                is_snapshot,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderHistoryRequest {
                account,
                symbol,
                start_time,
                end_time,
                limit,
                cursor,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orders_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut orders = Vec::with_capacity(orders_count);
        for _ in 0..orders_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            orders.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok((
            OrderHistoryBatch {
                account,
                orders,
                has_more,
                next_cursor,
            },
            pos,
        ))
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

        Ok((CapabilitiesRequest {}, pos))
    }
}

/// SBE body encoder for nested CapabilitiesResponse
pub struct CapabilitiesResponseBodyEncoder;

impl CapabilitiesResponseBodyEncoder {
    pub fn encode(value: &CapabilitiesResponse, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&(value.schema_ids.len() as u32).to_be_bytes());
        for item in &value.schema_ids {
            // TODO: encode list<u8>
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
            // TODO: encode list<Symbol>
        }
        buf.extend_from_slice(&(value.intervals.len() as u32).to_be_bytes());
        for item in &value.intervals {
            // TODO: encode list<CandleInterval>
        }
    }
}

/// SBE body decoder for nested CapabilitiesResponse
pub struct CapabilitiesResponseBodyDecoder;

impl CapabilitiesResponseBodyDecoder {
    pub fn decode(buf: &[u8]) -> Result<(CapabilitiesResponse, usize), String> {
        let mut pos: usize = 0;

        let schema_ids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut schema_ids = Vec::with_capacity(schema_ids_count);
        // TODO: decode list<u8>
        let paths_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut paths = Vec::with_capacity(paths_count);
        for _ in 0..paths_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = CapabilityPathDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "CapabilityPath length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            paths.push(item);
        }
        let symbols_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut symbols = Vec::with_capacity(symbols_count);
        // TODO: decode list<Symbol>
        let intervals_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut intervals = Vec::with_capacity(intervals_count);
        // TODO: decode list<CandleInterval>

        Ok((
            CapabilitiesResponse {
                schema_ids,
                paths,
                symbols,
                intervals,
            },
            pos,
        ))
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

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok((PositionRequest { account }, pos))
    }
}

/// SBE encoder for NewOrderSingle
pub struct NewOrderSingleEncoder;

impl NewOrderSingleEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        cl_ord_id: ClientOrderId,
        side: NewOrderSingleSide,
        order_qty: Quantity,
        price: Option<Price>,
        stop_price: Option<Price>,
        symbol: Symbol,
        order_type: NewOrderSingleOrderType,
        time_in_force: NewOrderSingleTimeInForce,
        expire_time: Option<TradeTimestamp>,
        account: Option<String>,
        strategy_id: Option<String>,
        security_id: Option<String>,
        id_source: Option<NewOrderSingleIdSource>,
        security_exchange: Option<String>,
        post_only: Option<bool>,
        reduce_only: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&1u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&38u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        buf.push(side as u8);
        buf.extend_from_slice(&order_qty.0.to_be_bytes());
        buf.extend_from_slice(&price.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &stop_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if stop_price.is_some() { 1 } else { 0 });
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(order_type as u8);
        buf.push(time_in_force as u8);
        buf.extend_from_slice(&expire_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(expire_time.is_some()));
        buf.push(if account.is_some() { 1 } else { 0 });
        if let Some(ref s) = account {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(if strategy_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = strategy_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(if security_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = security_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(id_source.map(|v| v as u8).unwrap_or(0));
        buf.push(u8::from(id_source.is_some()));
        buf.push(if security_exchange.is_some() { 1 } else { 0 });
        if let Some(ref s) = security_exchange {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.push(post_only.unwrap_or(false) as u8);
        buf.push(u8::from(post_only.is_some()));
        buf.push(reduce_only.unwrap_or(false) as u8);
        buf.push(u8::from(reduce_only.is_some()));

        buf
    }
}

/// SBE decoder for NewOrderSingle
#[derive(Debug, Clone, PartialEq)]
pub struct NewOrderSingleDecoder {
    pub cl_ord_id: ClientOrderId,
    pub side: NewOrderSingleSide,
    pub order_qty: Quantity,
    pub price: Option<Price>,
    pub stop_price: Option<Price>,
    pub symbol: Symbol,
    pub order_type: NewOrderSingleOrderType,
    pub time_in_force: NewOrderSingleTimeInForce,
    pub expire_time: Option<TradeTimestamp>,
    pub account: Option<String>,
    pub strategy_id: Option<String>,
    pub security_id: Option<String>,
    pub id_source: Option<NewOrderSingleIdSource>,
    pub security_exchange: Option<String>,
    pub post_only: Option<bool>,
    pub reduce_only: Option<bool>,
}

impl NewOrderSingleDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = NewOrderSingleSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(price_raw))
        } else {
            pos += 1;
            None
        };
        let stop_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let stop_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(stop_price_raw))
        } else {
            pos += 1;
            None
        };
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let order_type_raw = buf[pos];
        pos += 1;
        let order_type = NewOrderSingleOrderType::from_value(order_type_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleOrderType value: {}", order_type_raw))?;
        let time_in_force_raw = buf[pos];
        pos += 1;
        let time_in_force =
            NewOrderSingleTimeInForce::from_value(time_in_force_raw).ok_or_else(|| {
                format!(
                    "invalid NewOrderSingleTimeInForce value: {}",
                    time_in_force_raw
                )
            })?;
        let expire_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let expire_time = if buf[pos] == 1 {
            pos += 1;
            Some(expire_time_raw)
        } else {
            pos += 1;
            None
        };
        let account = if buf[pos] == 1 {
            pos += 1;
            let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let account_bytes = &buf[pos..pos + account_len];
            pos += account_len;
            Some(
                std::str::from_utf8(account_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let strategy_id = if buf[pos] == 1 {
            pos += 1;
            let strategy_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let strategy_id_bytes = &buf[pos..pos + strategy_id_len];
            pos += strategy_id_len;
            Some(
                std::str::from_utf8(strategy_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let security_id = if buf[pos] == 1 {
            pos += 1;
            let security_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let security_id_bytes = &buf[pos..pos + security_id_len];
            pos += security_id_len;
            Some(
                std::str::from_utf8(security_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let id_source_raw = buf[pos];
        pos += 1;
        let id_source = if buf[pos] == 1 {
            pos += 1;
            Some(
                NewOrderSingleIdSource::from_value(id_source_raw).ok_or_else(|| {
                    format!("invalid NewOrderSingleIdSource value: {}", id_source_raw)
                })?,
            )
        } else {
            pos += 1;
            None
        };
        let security_exchange = if buf[pos] == 1 {
            pos += 1;
            let security_exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let security_exchange_bytes = &buf[pos..pos + security_exchange_len];
            pos += security_exchange_len;
            Some(
                std::str::from_utf8(security_exchange_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let post_only_val = buf[pos];
        pos += 1;
        let post_only = if buf[pos] == 1 {
            pos += 1;
            Some(post_only_val != 0)
        } else {
            pos += 1;
            None
        };
        let reduce_only_val = buf[pos];
        pos += 1;
        let reduce_only = if buf[pos] == 1 {
            pos += 1;
            Some(reduce_only_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
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
        })
    }
}

/// SBE encoder for CancelRequest
pub struct CancelRequestEncoder;

impl CancelRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        cl_ord_id: ClientOrderId,
        orig_cl_ord_id: ClientOrderId,
        symbol: Symbol,
        side: CancelRequestSide,
        order_qty: Option<Quantity>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&2u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&9u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(side as u8);
        buf.extend_from_slice(&order_qty.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if order_qty.is_some() { 1 } else { 0 });

        buf
    }
}

/// SBE decoder for CancelRequest
#[derive(Debug, Clone, PartialEq)]
pub struct CancelRequestDecoder {
    pub cl_ord_id: ClientOrderId,
    pub orig_cl_ord_id: ClientOrderId,
    pub symbol: Symbol,
    pub side: CancelRequestSide,
    pub order_qty: Option<Quantity>,
}

impl CancelRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = CancelRequestSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid CancelRequestSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(order_qty_raw))
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty,
        })
    }
}

/// SBE encoder for CancelReplaceRequest
pub struct CancelReplaceRequestEncoder;

impl CancelReplaceRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        cl_ord_id: ClientOrderId,
        orig_cl_ord_id: ClientOrderId,
        symbol: Symbol,
        side: CancelReplaceRequestSide,
        order_qty: Quantity,
        price: Option<Price>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&3u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(side as u8);
        buf.extend_from_slice(&order_qty.0.to_be_bytes());
        buf.extend_from_slice(&price.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if price.is_some() { 1 } else { 0 });

        buf
    }
}

/// SBE decoder for CancelReplaceRequest
#[derive(Debug, Clone, PartialEq)]
pub struct CancelReplaceRequestDecoder {
    pub cl_ord_id: ClientOrderId,
    pub orig_cl_ord_id: ClientOrderId,
    pub symbol: Symbol,
    pub side: CancelReplaceRequestSide,
    pub order_qty: Quantity,
    pub price: Option<Price>,
}

impl CancelReplaceRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = CancelReplaceRequestSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid CancelReplaceRequestSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let order_qty = Quantity(order_qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(price_raw))
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty,
            price,
        })
    }
}

/// SBE encoder for ExecutionReport
pub struct ExecutionReportEncoder;

impl ExecutionReportEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        cl_ord_id: ClientOrderId,
        order_id: String,
        exec_id: String,
        exec_type: ExecutionReportExecType,
        ord_status: ExecutionReportOrdStatus,
        side: ExecutionReportSide,
        last_qty: Option<Quantity>,
        last_price: Option<Price>,
        leaves_qty: Quantity,
        cum_qty: Quantity,
        avg_price: Price,
        symbol: Symbol,
        transact_time: TradeTimestamp,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&4u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&51u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let order_id_bytes = order_id.as_bytes();
        buf.extend_from_slice(&(order_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(order_id_bytes);
        let exec_id_bytes = exec_id.as_bytes();
        buf.extend_from_slice(&(exec_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exec_id_bytes);
        buf.push(exec_type as u8);
        buf.push(ord_status as u8);
        buf.push(side as u8);
        buf.extend_from_slice(&last_qty.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if last_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(
            &last_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if last_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&leaves_qty.0.to_be_bytes());
        buf.extend_from_slice(&cum_qty.0.to_be_bytes());
        buf.extend_from_slice(&avg_price.0.to_be_bytes());
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&transact_time.to_be_bytes());

        buf
    }
}

/// SBE decoder for ExecutionReport
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionReportDecoder {
    pub cl_ord_id: ClientOrderId,
    pub order_id: String,
    pub exec_id: String,
    pub exec_type: ExecutionReportExecType,
    pub ord_status: ExecutionReportOrdStatus,
    pub side: ExecutionReportSide,
    pub last_qty: Option<Quantity>,
    pub last_price: Option<Price>,
    pub leaves_qty: Quantity,
    pub cum_qty: Quantity,
    pub avg_price: Price,
    pub symbol: Symbol,
    pub transact_time: TradeTimestamp,
}

impl ExecutionReportDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let order_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let order_id_bytes = &buf[pos..pos + order_id_len];
        pos += order_id_len;
        let order_id = std::str::from_utf8(order_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exec_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exec_id_bytes = &buf[pos..pos + exec_id_len];
        pos += exec_id_len;
        let exec_id = std::str::from_utf8(exec_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exec_type_raw = buf[pos];
        pos += 1;
        let exec_type = ExecutionReportExecType::from_value(exec_type_raw)
            .ok_or_else(|| format!("invalid ExecutionReportExecType value: {}", exec_type_raw))?;
        let ord_status_raw = buf[pos];
        pos += 1;
        let ord_status = ExecutionReportOrdStatus::from_value(ord_status_raw)
            .ok_or_else(|| format!("invalid ExecutionReportOrdStatus value: {}", ord_status_raw))?;
        let side_raw = buf[pos];
        pos += 1;
        let side = ExecutionReportSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid ExecutionReportSide value: {}", side_raw))?;
        let last_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(last_qty_raw))
        } else {
            pos += 1;
            None
        };
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(last_price_raw))
        } else {
            pos += 1;
            None
        };
        let leaves_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let leaves_qty = Quantity(leaves_qty_raw);
        let cum_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let cum_qty = Quantity(cum_qty_raw);
        let avg_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let avg_price = Price(avg_price_raw);
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let transact_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let transact_time = transact_time_raw;

        Ok(Self {
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
        })
    }
}

/// SBE encoder for CancelReject
pub struct CancelRejectEncoder;

impl CancelRejectEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        cl_ord_id: ClientOrderId,
        orig_cl_ord_id: ClientOrderId,
        reject_reason: CancelRejectRejectReason,
        symbol: Symbol,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&5u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        let orig_cl_ord_id_bytes = orig_cl_ord_id.as_bytes();
        buf.extend_from_slice(&(orig_cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(orig_cl_ord_id_bytes);
        buf.push(reject_reason as u8);
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);

        buf
    }
}

/// SBE decoder for CancelReject
#[derive(Debug, Clone, PartialEq)]
pub struct CancelRejectDecoder {
    pub cl_ord_id: ClientOrderId,
    pub orig_cl_ord_id: ClientOrderId,
    pub reject_reason: CancelRejectRejectReason,
    pub symbol: Symbol,
}

impl CancelRejectDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos + cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos + orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let reject_reason_raw = buf[pos];
        pos += 1;
        let reject_reason =
            CancelRejectRejectReason::from_value(reject_reason_raw).ok_or_else(|| {
                format!(
                    "invalid CancelRejectRejectReason value: {}",
                    reject_reason_raw
                )
            })?;
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self {
            cl_ord_id,
            orig_cl_ord_id,
            reject_reason,
            symbol,
        })
    }
}

/// SBE encoder for MarketDataSnapshot
pub struct MarketDataSnapshotEncoder;

impl MarketDataSnapshotEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        exchange: String,
        bids: Vec<PriceLevel>,
        asks: Vec<PriceLevel>,
        timestamp: TradeTimestamp,
        sequence: Option<u64>,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&6u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let exchange_bytes = exchange.as_bytes();
        buf.extend_from_slice(&(exchange_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exchange_bytes);
        buf.extend_from_slice(&(bids.len() as u32).to_be_bytes());
        for item in &bids {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(asks.len() as u32).to_be_bytes());
        for item in &asks {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.extend_from_slice(&sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(sequence.is_some()));
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for MarketDataSnapshot
#[derive(Debug, Clone, PartialEq)]
pub struct MarketDataSnapshotDecoder {
    pub symbol: Symbol,
    pub exchange: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: TradeTimestamp,
    pub sequence: Option<u64>,
    pub is_snapshot: Option<bool>,
}

impl MarketDataSnapshotDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exchange_bytes = &buf[pos..pos + exchange_len];
        pos += exchange_len;
        let exchange = std::str::from_utf8(exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bids.push(item);
        }
        let asks_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            asks.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
            sequence,
            is_snapshot,
        })
    }
}

/// SBE encoder for MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshEncoder;

impl MarketDataIncrementalRefreshEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        updates: Vec<MarketDataUpdate>,
        timestamp: TradeTimestamp,
        sequence: Option<u64>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&7u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(updates.len() as u32).to_be_bytes());
        for item in &updates {
            let mut item_buf = Vec::new();
            MarketDataUpdateEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.extend_from_slice(&sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(sequence.is_some()));

        buf
    }
}

/// SBE decoder for MarketDataIncrementalRefresh
#[derive(Debug, Clone, PartialEq)]
pub struct MarketDataIncrementalRefreshDecoder {
    pub symbol: Symbol,
    pub updates: Vec<MarketDataUpdate>,
    pub timestamp: TradeTimestamp,
    pub sequence: Option<u64>,
}

impl MarketDataIncrementalRefreshDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let updates_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MarketDataUpdateDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            updates.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            updates,
            timestamp,
            sequence,
        })
    }
}

/// SBE encoder for OrderBookRequest
pub struct OrderBookRequestEncoder;

impl OrderBookRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(symbol: Symbol, depth: Option<u32>, at_time: Option<TradeTimestamp>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&8u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&12u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&depth.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(depth.is_some()));
        buf.extend_from_slice(&at_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(at_time.is_some()));

        buf
    }
}

/// SBE decoder for OrderBookRequest
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBookRequestDecoder {
    pub symbol: Symbol,
    pub depth: Option<u32>,
    pub at_time: Option<TradeTimestamp>,
}

impl OrderBookRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let depth_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let depth = if buf[pos] == 1 {
            pos += 1;
            Some(depth_raw)
        } else {
            pos += 1;
            None
        };
        let at_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let at_time = if buf[pos] == 1 {
            pos += 1;
            Some(at_time_raw)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            depth,
            at_time,
        })
    }
}

/// SBE encoder for OrderBookSnapshot
pub struct OrderBookSnapshotEncoder;

impl OrderBookSnapshotEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        exchange: String,
        bids: Vec<PriceLevel>,
        asks: Vec<PriceLevel>,
        timestamp: TradeTimestamp,
        sequence: Option<u64>,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&9u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let exchange_bytes = exchange.as_bytes();
        buf.extend_from_slice(&(exchange_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exchange_bytes);
        buf.extend_from_slice(&(bids.len() as u32).to_be_bytes());
        for item in &bids {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(asks.len() as u32).to_be_bytes());
        for item in &asks {
            let mut item_buf = Vec::new();
            PriceLevelEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.extend_from_slice(&sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(sequence.is_some()));
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for OrderBookSnapshot
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBookSnapshotDecoder {
    pub symbol: Symbol,
    pub exchange: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub timestamp: TradeTimestamp,
    pub sequence: Option<u64>,
    pub is_snapshot: Option<bool>,
}

impl OrderBookSnapshotDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let exchange_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let exchange_bytes = &buf[pos..pos + exchange_len];
        pos += exchange_len;
        let exchange = std::str::from_utf8(exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bids.push(item);
        }
        let asks_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PriceLevelDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PriceLevel length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            asks.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
            sequence,
            is_snapshot,
        })
    }
}

/// SBE encoder for OrderBookDelta
pub struct OrderBookDeltaEncoder;

impl OrderBookDeltaEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        updates: Vec<MarketDataUpdate>,
        timestamp: TradeTimestamp,
        sequence: Option<u64>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&10u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(updates.len() as u32).to_be_bytes());
        for item in &updates {
            let mut item_buf = Vec::new();
            MarketDataUpdateEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.extend_from_slice(&sequence.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(sequence.is_some()));

        buf
    }
}

/// SBE decoder for OrderBookDelta
#[derive(Debug, Clone, PartialEq)]
pub struct OrderBookDeltaDecoder {
    pub symbol: Symbol,
    pub updates: Vec<MarketDataUpdate>,
    pub timestamp: TradeTimestamp,
    pub sequence: Option<u64>,
}

impl OrderBookDeltaDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let updates_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MarketDataUpdateDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MarketDataUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            updates.push(item);
        }
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let sequence_raw = u64::from_be_bytes([
            buf[pos],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let sequence = if buf[pos] == 1 {
            pos += 1;
            Some(sequence_raw)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            updates,
            timestamp,
            sequence,
        })
    }
}

/// SBE encoder for AggregateTradeEvent
pub struct AggregateTradeEventEncoder;

impl AggregateTradeEventEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(trade: AggregateTrade) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&11u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        AggregateTradeEncoder::encode(&trade, &mut buf);

        buf
    }
}

/// SBE decoder for AggregateTradeEvent
#[derive(Debug, Clone, PartialEq)]
pub struct AggregateTradeEventDecoder {
    pub trade: AggregateTrade,
}

impl AggregateTradeEventDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let (trade, trade_n) = AggregateTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok(Self { trade })
    }
}

/// SBE encoder for AggregateTradeRequest
pub struct AggregateTradeRequestEncoder;

impl AggregateTradeRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&12u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for AggregateTradeRequest
#[derive(Debug, Clone, PartialEq)]
pub struct AggregateTradeRequestDecoder {
    pub symbol: Symbol,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl AggregateTradeRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for AggregateTradeBatch
pub struct AggregateTradeBatchEncoder;

impl AggregateTradeBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        trades: Vec<AggregateTrade>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&13u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(trades.len() as u32).to_be_bytes());
        for item in &trades {
            let mut item_buf = Vec::new();
            AggregateTradeEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for AggregateTradeBatch
#[derive(Debug, Clone, PartialEq)]
pub struct AggregateTradeBatchDecoder {
    pub symbol: Symbol,
    pub trades: Vec<AggregateTrade>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl AggregateTradeBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let trades_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut trades = Vec::with_capacity(trades_count);
        for _ in 0..trades_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = AggregateTradeDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "AggregateTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            trades.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            trades,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for MiniTicker
pub struct MiniTickerEncoder;

impl MiniTickerEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        last_price: Price,
        volume: Quantity,
        timestamp: TradeTimestamp,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&14u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&25u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&last_price.0.to_be_bytes());
        buf.extend_from_slice(&volume.0.to_be_bytes());
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for MiniTicker
#[derive(Debug, Clone, PartialEq)]
pub struct MiniTickerDecoder {
    pub symbol: Symbol,
    pub last_price: Price,
    pub volume: Quantity,
    pub timestamp: TradeTimestamp,
    pub is_snapshot: Option<bool>,
}

impl MiniTickerDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = Price(last_price_raw);
        let volume_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let volume = Quantity(volume_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            last_price,
            volume,
            timestamp,
            is_snapshot,
        })
    }
}

/// SBE encoder for AllMidsRequest
pub struct AllMidsRequestEncoder;

impl AllMidsRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode() -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&15u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields

        buf
    }
}

/// SBE decoder for AllMidsRequest
#[derive(Debug, Clone, PartialEq)]
pub struct AllMidsRequestDecoder {}

impl AllMidsRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        Ok(Self {})
    }
}

/// SBE encoder for AllMidsBatch
pub struct AllMidsBatchEncoder;

impl AllMidsBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(tickers: Vec<MiniTicker>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&16u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        buf.extend_from_slice(&(tickers.len() as u32).to_be_bytes());
        for item in &tickers {
            let mut item_buf = Vec::new();
            MiniTickerBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }

        buf
    }
}

/// SBE decoder for AllMidsBatch
#[derive(Debug, Clone, PartialEq)]
pub struct AllMidsBatchDecoder {
    pub tickers: Vec<MiniTicker>,
}

impl AllMidsBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let tickers_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut tickers = Vec::with_capacity(tickers_count);
        for _ in 0..tickers_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = MiniTickerBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "MiniTicker length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            tickers.push(item);
        }

        Ok(Self { tickers })
    }
}

/// SBE encoder for MarkPriceUpdate
pub struct MarkPriceUpdateEncoder;

impl MarkPriceUpdateEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        mark_price: Price,
        index_price: Option<Price>,
        funding_rate: Option<f64>,
        timestamp: TradeTimestamp,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&17u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&33u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&mark_price.0.to_be_bytes());
        buf.extend_from_slice(
            &index_price
                .as_ref()
                .map(|v| v.0)
                .unwrap_or(0.0)
                .to_be_bytes(),
        );
        buf.push(if index_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&funding_rate.unwrap_or(0.0).to_be_bytes());
        buf.push(if funding_rate.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for MarkPriceUpdate
#[derive(Debug, Clone, PartialEq)]
pub struct MarkPriceUpdateDecoder {
    pub symbol: Symbol,
    pub mark_price: Price,
    pub index_price: Option<Price>,
    pub funding_rate: Option<f64>,
    pub timestamp: TradeTimestamp,
    pub is_snapshot: Option<bool>,
}

impl MarkPriceUpdateDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let mark_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let mark_price = Price(mark_price_raw);
        let index_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let index_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(index_price_raw))
        } else {
            pos += 1;
            None
        };
        let funding_rate_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let funding_rate = if buf[pos] == 1 {
            pos += 1;
            Some(funding_rate_raw)
        } else {
            pos += 1;
            None
        };
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            mark_price,
            index_price,
            funding_rate,
            timestamp,
            is_snapshot,
        })
    }
}

/// SBE encoder for MarkPriceRequest
pub struct MarkPriceRequestEncoder;

impl MarkPriceRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(symbol: Symbol) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&18u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);

        buf
    }
}

/// SBE decoder for MarkPriceRequest
#[derive(Debug, Clone, PartialEq)]
pub struct MarkPriceRequestDecoder {
    pub symbol: Symbol,
}

impl MarkPriceRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self { symbol })
    }
}

/// SBE encoder for LiquidationTradeEvent
pub struct LiquidationTradeEventEncoder;

impl LiquidationTradeEventEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(trade: LiquidationTrade) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&19u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        LiquidationTradeEncoder::encode(&trade, &mut buf);

        buf
    }
}

/// SBE decoder for LiquidationTradeEvent
#[derive(Debug, Clone, PartialEq)]
pub struct LiquidationTradeEventDecoder {
    pub trade: LiquidationTrade,
}

impl LiquidationTradeEventDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let (trade, trade_n) = LiquidationTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok(Self { trade })
    }
}

/// SBE encoder for CandleBarEvent
pub struct CandleBarEventEncoder;

impl CandleBarEventEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(bar: CandleBar) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&20u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        CandleBarEncoder::encode(&bar, &mut buf);

        buf
    }
}

/// SBE decoder for CandleBarEvent
#[derive(Debug, Clone, PartialEq)]
pub struct CandleBarEventDecoder {
    pub bar: CandleBar,
}

impl CandleBarEventDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let (bar, bar_n) = CandleBarDecoder::decode(&buf[pos..])?;
        pos += bar_n;

        Ok(Self { bar })
    }
}

/// SBE encoder for CandleBarRequest
pub struct CandleBarRequestEncoder;

impl CandleBarRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        interval: CandleInterval,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&21u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let interval_bytes = interval.as_bytes();
        buf.extend_from_slice(&(interval_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(interval_bytes);
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for CandleBarRequest
#[derive(Debug, Clone, PartialEq)]
pub struct CandleBarRequestDecoder {
    pub symbol: Symbol,
    pub interval: CandleInterval,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl CandleBarRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let interval_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let interval_bytes = &buf[pos..pos + interval_len];
        pos += interval_len;
        let interval = std::str::from_utf8(interval_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            interval,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for CandleBarBatch
pub struct CandleBarBatchEncoder;

impl CandleBarBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        interval: CandleInterval,
        bars: Vec<CandleBar>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&22u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let interval_bytes = interval.as_bytes();
        buf.extend_from_slice(&(interval_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(interval_bytes);
        buf.extend_from_slice(&(bars.len() as u32).to_be_bytes());
        for item in &bars {
            let mut item_buf = Vec::new();
            CandleBarEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for CandleBarBatch
#[derive(Debug, Clone, PartialEq)]
pub struct CandleBarBatchDecoder {
    pub symbol: Symbol,
    pub interval: CandleInterval,
    pub bars: Vec<CandleBar>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl CandleBarBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let interval_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let interval_bytes = &buf[pos..pos + interval_len];
        pos += interval_len;
        let interval = std::str::from_utf8(interval_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bars_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bars = Vec::with_capacity(bars_count);
        for _ in 0..bars_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = CandleBarDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "CandleBar length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            bars.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            interval,
            bars,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for PublicTradeEvent
pub struct PublicTradeEventEncoder;

impl PublicTradeEventEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(trade: PublicTrade) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&23u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        PublicTradeEncoder::encode(&trade, &mut buf);

        buf
    }
}

/// SBE decoder for PublicTradeEvent
#[derive(Debug, Clone, PartialEq)]
pub struct PublicTradeEventDecoder {
    pub trade: PublicTrade,
}

impl PublicTradeEventDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let (trade, trade_n) = PublicTradeDecoder::decode(&buf[pos..])?;
        pos += trade_n;

        Ok(Self { trade })
    }
}

/// SBE encoder for TradeHistoryRequest
pub struct TradeHistoryRequestEncoder;

impl TradeHistoryRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&24u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for TradeHistoryRequest
#[derive(Debug, Clone, PartialEq)]
pub struct TradeHistoryRequestDecoder {
    pub symbol: Symbol,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl TradeHistoryRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for PublicTradeBatch
pub struct PublicTradeBatchEncoder;

impl PublicTradeBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        trades: Vec<PublicTrade>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&25u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(trades.len() as u32).to_be_bytes());
        for item in &trades {
            let mut item_buf = Vec::new();
            PublicTradeEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for PublicTradeBatch
#[derive(Debug, Clone, PartialEq)]
pub struct PublicTradeBatchDecoder {
    pub symbol: Symbol,
    pub trades: Vec<PublicTrade>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl PublicTradeBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let trades_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut trades = Vec::with_capacity(trades_count);
        for _ in 0..trades_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PublicTradeDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PublicTrade length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            trades.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            trades,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for BestBidOffer
pub struct BestBidOfferEncoder;

impl BestBidOfferEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        bid_price: Option<Price>,
        bid_qty: Option<Quantity>,
        ask_price: Option<Price>,
        ask_qty: Option<Quantity>,
        timestamp: TradeTimestamp,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&26u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&41u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&bid_price.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if bid_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&bid_qty.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if bid_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&ask_price.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if ask_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&ask_qty.as_ref().map(|v| v.0).unwrap_or(0.0).to_be_bytes());
        buf.push(if ask_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for BestBidOffer
#[derive(Debug, Clone, PartialEq)]
pub struct BestBidOfferDecoder {
    pub symbol: Symbol,
    pub bid_price: Option<Price>,
    pub bid_qty: Option<Quantity>,
    pub ask_price: Option<Price>,
    pub ask_qty: Option<Quantity>,
    pub timestamp: TradeTimestamp,
    pub is_snapshot: Option<bool>,
}

impl BestBidOfferDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let bid_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bid_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(bid_price_raw))
        } else {
            pos += 1;
            None
        };
        let bid_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let bid_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(bid_qty_raw))
        } else {
            pos += 1;
            None
        };
        let ask_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let ask_price = if buf[pos] == 1 {
            pos += 1;
            Some(Price(ask_price_raw))
        } else {
            pos += 1;
            None
        };
        let ask_qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let ask_qty = if buf[pos] == 1 {
            pos += 1;
            Some(Quantity(ask_qty_raw))
        } else {
            pos += 1;
            None
        };
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            symbol,
            bid_price,
            bid_qty,
            ask_price,
            ask_qty,
            timestamp,
            is_snapshot,
        })
    }
}

/// SBE encoder for SymbolTicker
pub struct SymbolTickerEncoder;

impl SymbolTickerEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        symbol: Symbol,
        last_price: Price,
        price_change: f64,
        price_change_pct: f64,
        volume: Quantity,
        high: Price,
        low: Price,
        open: Price,
        timestamp: TradeTimestamp,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&27u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&65u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&last_price.0.to_be_bytes());
        buf.extend_from_slice(&price_change.to_be_bytes());
        buf.extend_from_slice(&price_change_pct.to_be_bytes());
        buf.extend_from_slice(&volume.0.to_be_bytes());
        buf.extend_from_slice(&high.0.to_be_bytes());
        buf.extend_from_slice(&low.0.to_be_bytes());
        buf.extend_from_slice(&open.0.to_be_bytes());
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for SymbolTicker
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolTickerDecoder {
    pub symbol: Symbol,
    pub last_price: Price,
    pub price_change: f64,
    pub price_change_pct: f64,
    pub volume: Quantity,
    pub high: Price,
    pub low: Price,
    pub open: Price,
    pub timestamp: TradeTimestamp,
    pub is_snapshot: Option<bool>,
}

impl SymbolTickerDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let last_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let last_price = Price(last_price_raw);
        let price_change_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price_change = price_change_raw;
        let price_change_pct_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price_change_pct = price_change_pct_raw;
        let volume_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let volume = Quantity(volume_raw);
        let high_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let high = Price(high_raw);
        let low_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let low = Price(low_raw);
        let open_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let open = Price(open_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
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
        })
    }
}

/// SBE encoder for TickerRequest
pub struct TickerRequestEncoder;

impl TickerRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(symbol: Symbol) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&28u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);

        buf
    }
}

/// SBE decoder for TickerRequest
#[derive(Debug, Clone, PartialEq)]
pub struct TickerRequestDecoder {
    pub symbol: Symbol,
}

impl TickerRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self { symbol })
    }
}

/// SBE encoder for InstrumentCatalogRequest
pub struct InstrumentCatalogRequestEncoder;

impl InstrumentCatalogRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode() -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&29u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields

        buf
    }
}

/// SBE decoder for InstrumentCatalogRequest
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentCatalogRequestDecoder {}

impl InstrumentCatalogRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        Ok(Self {})
    }
}

/// SBE encoder for InstrumentCatalogResponse
pub struct InstrumentCatalogResponseEncoder;

impl InstrumentCatalogResponseEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(instruments: Vec<InstrumentMetadata>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&30u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        buf.extend_from_slice(&(instruments.len() as u32).to_be_bytes());
        for item in &instruments {
            let mut item_buf = Vec::new();
            InstrumentMetadataEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }

        buf
    }
}

/// SBE decoder for InstrumentCatalogResponse
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentCatalogResponseDecoder {
    pub instruments: Vec<InstrumentMetadata>,
}

impl InstrumentCatalogResponseDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let instruments_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut instruments = Vec::with_capacity(instruments_count);
        for _ in 0..instruments_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = InstrumentMetadataDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "InstrumentMetadata length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            instruments.push(item);
        }

        Ok(Self { instruments })
    }
}

/// SBE encoder for AccountSummary
pub struct AccountSummaryEncoder;

impl AccountSummaryEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(account: String, balance: f64, buying_power: f64, currency: String) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&31u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&16u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&balance.to_be_bytes());
        buf.extend_from_slice(&buying_power.to_be_bytes());
        let currency_bytes = currency.as_bytes();
        buf.extend_from_slice(&(currency_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(currency_bytes);

        buf
    }
}

/// SBE decoder for AccountSummary
#[derive(Debug, Clone, PartialEq)]
pub struct AccountSummaryDecoder {
    pub account: String,
    pub balance: f64,
    pub buying_power: f64,
    pub currency: String,
}

impl AccountSummaryDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balance_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let balance = balance_raw;
        let buying_power_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let buying_power = buying_power_raw;
        let currency_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let currency_bytes = &buf[pos..pos + currency_len];
        pos += currency_len;
        let currency = std::str::from_utf8(currency_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self {
            account,
            balance,
            buying_power,
            currency,
        })
    }
}

/// SBE encoder for MarginSummary
pub struct MarginSummaryEncoder;

impl MarginSummaryEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        balance: f64,
        buying_power: f64,
        equity: f64,
        margin_used: f64,
        available: f64,
        currency: String,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&32u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&40u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&balance.to_be_bytes());
        buf.extend_from_slice(&buying_power.to_be_bytes());
        buf.extend_from_slice(&equity.to_be_bytes());
        buf.extend_from_slice(&margin_used.to_be_bytes());
        buf.extend_from_slice(&available.to_be_bytes());
        let currency_bytes = currency.as_bytes();
        buf.extend_from_slice(&(currency_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(currency_bytes);

        buf
    }
}

/// SBE decoder for MarginSummary
#[derive(Debug, Clone, PartialEq)]
pub struct MarginSummaryDecoder {
    pub account: String,
    pub balance: f64,
    pub buying_power: f64,
    pub equity: f64,
    pub margin_used: f64,
    pub available: f64,
    pub currency: String,
}

impl MarginSummaryDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balance_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let balance = balance_raw;
        let buying_power_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let buying_power = buying_power_raw;
        let equity_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let equity = equity_raw;
        let margin_used_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let margin_used = margin_used_raw;
        let available_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let available = available_raw;
        let currency_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let currency_bytes = &buf[pos..pos + currency_len];
        pos += currency_len;
        let currency = std::str::from_utf8(currency_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self {
            account,
            balance,
            buying_power,
            equity,
            margin_used,
            available,
            currency,
        })
    }
}

/// SBE encoder for BalanceSnapshot
pub struct BalanceSnapshotEncoder;

impl BalanceSnapshotEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        balances: Vec<BalanceEntry>,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&33u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(balances.len() as u32).to_be_bytes());
        for item in &balances {
            let mut item_buf = Vec::new();
            BalanceEntryEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for BalanceSnapshot
#[derive(Debug, Clone, PartialEq)]
pub struct BalanceSnapshotDecoder {
    pub account: String,
    pub balances: Vec<BalanceEntry>,
    pub is_snapshot: Option<bool>,
}

impl BalanceSnapshotDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let balances_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut balances = Vec::with_capacity(balances_count);
        for _ in 0..balances_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = BalanceEntryDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "BalanceEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            balances.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            balances,
            is_snapshot,
        })
    }
}

/// SBE encoder for BalanceUpdate
pub struct BalanceUpdateEncoder;

impl BalanceUpdateEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        asset: String,
        delta: f64,
        total: f64,
        available: f64,
        reason: BalanceUpdateReason,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&34u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&25u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let asset_bytes = asset.as_bytes();
        buf.extend_from_slice(&(asset_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(asset_bytes);
        buf.extend_from_slice(&delta.to_be_bytes());
        buf.extend_from_slice(&total.to_be_bytes());
        buf.extend_from_slice(&available.to_be_bytes());
        buf.push(reason.to_value());

        buf
    }
}

/// SBE decoder for BalanceUpdate
#[derive(Debug, Clone, PartialEq)]
pub struct BalanceUpdateDecoder {
    pub account: String,
    pub asset: String,
    pub delta: f64,
    pub total: f64,
    pub available: f64,
    pub reason: BalanceUpdateReason,
}

impl BalanceUpdateDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let asset_bytes = &buf[pos..pos + asset_len];
        pos += asset_len;
        let asset = std::str::from_utf8(asset_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let delta_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let delta = delta_raw;
        let total_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let total = total_raw;
        let available_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let available = available_raw;
        let reason_raw = buf[pos];
        pos += 1;
        let reason = BalanceUpdateReason::from_value(reason_raw)
            .ok_or_else(|| format!("invalid BalanceUpdateReason value: {}", reason_raw))?;

        Ok(Self {
            account,
            asset,
            delta,
            total,
            available,
            reason,
        })
    }
}

/// SBE encoder for PositionSnapshot
pub struct PositionSnapshotEncoder;

impl PositionSnapshotEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        positions: Vec<PositionEntry>,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&35u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(positions.len() as u32).to_be_bytes());
        for item in &positions {
            let mut item_buf = Vec::new();
            PositionEntryEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for PositionSnapshot
#[derive(Debug, Clone, PartialEq)]
pub struct PositionSnapshotDecoder {
    pub account: String,
    pub positions: Vec<PositionEntry>,
    pub is_snapshot: Option<bool>,
}

impl PositionSnapshotDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let positions_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut positions = Vec::with_capacity(positions_count);
        for _ in 0..positions_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = PositionEntryDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "PositionEntry length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            positions.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            positions,
            is_snapshot,
        })
    }
}

/// SBE encoder for PositionUpdate
pub struct PositionUpdateEncoder;

impl PositionUpdateEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        symbol: Symbol,
        qty: Quantity,
        entry_price: Price,
        unrealized_pnl: f64,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&36u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&qty.0.to_be_bytes());
        buf.extend_from_slice(&entry_price.0.to_be_bytes());
        buf.extend_from_slice(&unrealized_pnl.to_be_bytes());

        buf
    }
}

/// SBE decoder for PositionUpdate
#[derive(Debug, Clone, PartialEq)]
pub struct PositionUpdateDecoder {
    pub account: String,
    pub symbol: Symbol,
    pub qty: Quantity,
    pub entry_price: Price,
    pub unrealized_pnl: f64,
}

impl PositionUpdateDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let entry_price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let entry_price = Price(entry_price_raw);
        let unrealized_pnl_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let unrealized_pnl = unrealized_pnl_raw;

        Ok(Self {
            account,
            symbol,
            qty,
            entry_price,
            unrealized_pnl,
        })
    }
}

/// SBE encoder for MarginUpdate
pub struct MarginUpdateEncoder;

impl MarginUpdateEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(account: String, summary: MarginSummary, is_snapshot: Option<bool>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&37u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        MarginSummaryBodyEncoder::encode(&summary, &mut buf);
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for MarginUpdate
#[derive(Debug, Clone, PartialEq)]
pub struct MarginUpdateDecoder {
    pub account: String,
    pub summary: MarginSummary,
    pub is_snapshot: Option<bool>,
}

impl MarginUpdateDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let (summary, summary_n) = MarginSummaryBodyDecoder::decode(&buf[pos..])?;
        pos += summary_n;
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            summary,
            is_snapshot,
        })
    }
}

/// SBE encoder for UserLiquidation
pub struct UserLiquidationEncoder;

impl UserLiquidationEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        symbol: Symbol,
        qty: Quantity,
        price: Price,
        timestamp: TradeTimestamp,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&38u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&qty.0.to_be_bytes());
        buf.extend_from_slice(&price.0.to_be_bytes());
        buf.extend_from_slice(&timestamp.to_be_bytes());

        buf
    }
}

/// SBE decoder for UserLiquidation
#[derive(Debug, Clone, PartialEq)]
pub struct UserLiquidationDecoder {
    pub account: String,
    pub symbol: Symbol,
    pub qty: Quantity,
    pub price: Price,
    pub timestamp: TradeTimestamp,
}

impl UserLiquidationDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos + symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let qty_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let qty = Quantity(qty_raw);
        let price_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let price = Price(price_raw);
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok(Self {
            account,
            symbol,
            qty,
            price,
            timestamp,
        })
    }
}

/// SBE encoder for OrderListStatus
pub struct OrderListStatusEncoder;

impl OrderListStatusEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        list_id: String,
        status: OrderListStatusStatus,
        symbol: Option<Symbol>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&39u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let list_id_bytes = list_id.as_bytes();
        buf.extend_from_slice(&(list_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(list_id_bytes);
        buf.push(status.to_value());
        buf.push(if symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for OrderListStatus
#[derive(Debug, Clone, PartialEq)]
pub struct OrderListStatusDecoder {
    pub account: String,
    pub list_id: String,
    pub status: OrderListStatusStatus,
    pub symbol: Option<Symbol>,
}

impl OrderListStatusDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let list_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let list_id_bytes = &buf[pos..pos + list_id_len];
        pos += list_id_len;
        let list_id = std::str::from_utf8(list_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let status_raw = buf[pos];
        pos += 1;
        let status = OrderListStatusStatus::from_value(status_raw)
            .ok_or_else(|| format!("invalid OrderListStatusStatus value: {}", status_raw))?;
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            list_id,
            status,
            symbol,
        })
    }
}

/// SBE encoder for FillHistoryRequest
pub struct FillHistoryRequestEncoder;

impl FillHistoryRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        symbol: Option<Symbol>,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&40u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for FillHistoryRequest
#[derive(Debug, Clone, PartialEq)]
pub struct FillHistoryRequestDecoder {
    pub account: String,
    pub symbol: Option<Symbol>,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl FillHistoryRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for FillHistoryBatch
pub struct FillHistoryBatchEncoder;

impl FillHistoryBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        fills: Vec<ExecutionReport>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&41u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(fills.len() as u32).to_be_bytes());
        for item in &fills {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for FillHistoryBatch
#[derive(Debug, Clone, PartialEq)]
pub struct FillHistoryBatchDecoder {
    pub account: String,
    pub fills: Vec<ExecutionReport>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl FillHistoryBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let fills_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut fills = Vec::with_capacity(fills_count);
        for _ in 0..fills_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            fills.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            fills,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for FundingPayment
pub struct FundingPaymentEncoder;

impl FundingPaymentEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        symbol: Option<Symbol>,
        amount: f64,
        rate: f64,
        timestamp: TradeTimestamp,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&42u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&24u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&amount.to_be_bytes());
        buf.extend_from_slice(&rate.to_be_bytes());
        buf.extend_from_slice(&timestamp.to_be_bytes());

        buf
    }
}

/// SBE decoder for FundingPayment
#[derive(Debug, Clone, PartialEq)]
pub struct FundingPaymentDecoder {
    pub account: String,
    pub symbol: Option<Symbol>,
    pub amount: f64,
    pub rate: f64,
    pub timestamp: TradeTimestamp,
}

impl FundingPaymentDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let amount_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let amount = amount_raw;
        let rate_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let rate = rate_raw;
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;

        Ok(Self {
            account,
            symbol,
            amount,
            rate,
            timestamp,
        })
    }
}

/// SBE encoder for FundingHistoryRequest
pub struct FundingHistoryRequestEncoder;

impl FundingHistoryRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&43u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for FundingHistoryRequest
#[derive(Debug, Clone, PartialEq)]
pub struct FundingHistoryRequestDecoder {
    pub account: String,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl FundingHistoryRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for FundingHistoryBatch
pub struct FundingHistoryBatchEncoder;

impl FundingHistoryBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        payments: Vec<FundingPayment>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&44u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(payments.len() as u32).to_be_bytes());
        for item in &payments {
            let mut item_buf = Vec::new();
            FundingPaymentBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for FundingHistoryBatch
#[derive(Debug, Clone, PartialEq)]
pub struct FundingHistoryBatchDecoder {
    pub account: String,
    pub payments: Vec<FundingPayment>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl FundingHistoryBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let payments_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut payments = Vec::with_capacity(payments_count);
        for _ in 0..payments_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = FundingPaymentBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "FundingPayment length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            payments.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            payments,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for LedgerUpdate
pub struct LedgerUpdateEncoder;

impl LedgerUpdateEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        asset: String,
        delta: f64,
        kind: LedgerUpdateKind,
        timestamp: TradeTimestamp,
        reference_id: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&45u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&17u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        let asset_bytes = asset.as_bytes();
        buf.extend_from_slice(&(asset_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(asset_bytes);
        buf.extend_from_slice(&delta.to_be_bytes());
        buf.push(kind.to_value());
        buf.extend_from_slice(&timestamp.to_be_bytes());
        buf.push(if reference_id.is_some() { 1 } else { 0 });
        if let Some(ref s) = reference_id {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for LedgerUpdate
#[derive(Debug, Clone, PartialEq)]
pub struct LedgerUpdateDecoder {
    pub account: String,
    pub asset: String,
    pub delta: f64,
    pub kind: LedgerUpdateKind,
    pub timestamp: TradeTimestamp,
    pub reference_id: Option<String>,
}

impl LedgerUpdateDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let asset_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let asset_bytes = &buf[pos..pos + asset_len];
        pos += asset_len;
        let asset = std::str::from_utf8(asset_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let delta_raw = f64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let delta = delta_raw;
        let kind_raw = buf[pos];
        pos += 1;
        let kind = LedgerUpdateKind::from_value(kind_raw)
            .ok_or_else(|| format!("invalid LedgerUpdateKind value: {}", kind_raw))?;
        let timestamp_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let timestamp = timestamp_raw;
        let reference_id = if buf[pos] == 1 {
            pos += 1;
            let reference_id_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let reference_id_bytes = &buf[pos..pos + reference_id_len];
            pos += reference_id_len;
            Some(
                std::str::from_utf8(reference_id_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            asset,
            delta,
            kind,
            timestamp,
            reference_id,
        })
    }
}

/// SBE encoder for LedgerHistoryRequest
pub struct LedgerHistoryRequestEncoder;

impl LedgerHistoryRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&46u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for LedgerHistoryRequest
#[derive(Debug, Clone, PartialEq)]
pub struct LedgerHistoryRequestDecoder {
    pub account: String,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl LedgerHistoryRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for LedgerHistoryBatch
pub struct LedgerHistoryBatchEncoder;

impl LedgerHistoryBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        entries: Vec<LedgerUpdate>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&47u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(entries.len() as u32).to_be_bytes());
        for item in &entries {
            let mut item_buf = Vec::new();
            LedgerUpdateBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for LedgerHistoryBatch
#[derive(Debug, Clone, PartialEq)]
pub struct LedgerHistoryBatchDecoder {
    pub account: String,
    pub entries: Vec<LedgerUpdate>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl LedgerHistoryBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let entries_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut entries = Vec::with_capacity(entries_count);
        for _ in 0..entries_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = LedgerUpdateBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "LedgerUpdate length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            entries.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            entries,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for OpenOrdersRequest
pub struct OpenOrdersRequestEncoder;

impl OpenOrdersRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(account: String, symbol: Option<Symbol>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&48u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for OpenOrdersRequest
#[derive(Debug, Clone, PartialEq)]
pub struct OpenOrdersRequestDecoder {
    pub account: String,
    pub symbol: Option<Symbol>,
}

impl OpenOrdersRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self { account, symbol })
    }
}

/// SBE encoder for OpenOrdersSnapshot
pub struct OpenOrdersSnapshotEncoder;

impl OpenOrdersSnapshotEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        orders: Vec<ExecutionReport>,
        is_snapshot: Option<bool>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&49u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(orders.len() as u32).to_be_bytes());
        for item in &orders {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(is_snapshot.unwrap_or(false) as u8);
        buf.push(u8::from(is_snapshot.is_some()));

        buf
    }
}

/// SBE decoder for OpenOrdersSnapshot
#[derive(Debug, Clone, PartialEq)]
pub struct OpenOrdersSnapshotDecoder {
    pub account: String,
    pub orders: Vec<ExecutionReport>,
    pub is_snapshot: Option<bool>,
}

impl OpenOrdersSnapshotDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orders_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut orders = Vec::with_capacity(orders_count);
        for _ in 0..orders_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            orders.push(item);
        }
        let is_snapshot_val = buf[pos];
        pos += 1;
        let is_snapshot = if buf[pos] == 1 {
            pos += 1;
            Some(is_snapshot_val != 0)
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            orders,
            is_snapshot,
        })
    }
}

/// SBE encoder for OrderHistoryRequest
pub struct OrderHistoryRequestEncoder;

impl OrderHistoryRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        symbol: Option<Symbol>,
        start_time: Option<TradeTimestamp>,
        end_time: Option<TradeTimestamp>,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&50u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&20u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.push(if symbol.is_some() { 1 } else { 0 });
        if let Some(ref s) = symbol {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }
        buf.extend_from_slice(&start_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(start_time.is_some()));
        buf.extend_from_slice(&end_time.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(end_time.is_some()));
        buf.extend_from_slice(&limit.unwrap_or(0).to_be_bytes());
        buf.push(u8::from(limit.is_some()));
        buf.push(if cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for OrderHistoryRequest
#[derive(Debug, Clone, PartialEq)]
pub struct OrderHistoryRequestDecoder {
    pub account: String,
    pub symbol: Option<Symbol>,
    pub start_time: Option<TradeTimestamp>,
    pub end_time: Option<TradeTimestamp>,
    pub limit: Option<u32>,
    pub cursor: Option<String>,
}

impl OrderHistoryRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let symbol = if buf[pos] == 1 {
            pos += 1;
            let symbol_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let symbol_bytes = &buf[pos..pos + symbol_len];
            pos += symbol_len;
            Some(
                std::str::from_utf8(symbol_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };
        let start_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let start_time = if buf[pos] == 1 {
            pos += 1;
            Some(start_time_raw)
        } else {
            pos += 1;
            None
        };
        let end_time_raw = i64::from_be_bytes([
            buf[pos + 0],
            buf[pos + 1],
            buf[pos + 2],
            buf[pos + 3],
            buf[pos + 4],
            buf[pos + 5],
            buf[pos + 6],
            buf[pos + 7],
        ]);
        pos += 8;
        let end_time = if buf[pos] == 1 {
            pos += 1;
            Some(end_time_raw)
        } else {
            pos += 1;
            None
        };
        let limit_raw = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;
        let limit = if buf[pos] == 1 {
            pos += 1;
            Some(limit_raw)
        } else {
            pos += 1;
            None
        };
        let cursor = if buf[pos] == 1 {
            pos += 1;
            let cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let cursor_bytes = &buf[pos..pos + cursor_len];
            pos += cursor_len;
            Some(
                std::str::from_utf8(cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            symbol,
            start_time,
            end_time,
            limit,
            cursor,
        })
    }
}

/// SBE encoder for OrderHistoryBatch
pub struct OrderHistoryBatchEncoder;

impl OrderHistoryBatchEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        account: String,
        orders: Vec<ExecutionReport>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&51u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&1u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);
        buf.extend_from_slice(&(orders.len() as u32).to_be_bytes());
        for item in &orders {
            let mut item_buf = Vec::new();
            ExecutionReportBodyEncoder::encode(item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.push(has_more as u8);
        buf.push(if next_cursor.is_some() { 1 } else { 0 });
        if let Some(ref s) = next_cursor {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

        buf
    }
}

/// SBE decoder for OrderHistoryBatch
#[derive(Debug, Clone, PartialEq)]
pub struct OrderHistoryBatchDecoder {
    pub account: String,
    pub orders: Vec<ExecutionReport>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

impl OrderHistoryBatchDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();
        let orders_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut orders = Vec::with_capacity(orders_count);
        for _ in 0..orders_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = ExecutionReportBodyDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "ExecutionReport length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            orders.push(item);
        }
        let has_more = buf[pos] != 0;
        pos += 1;
        let next_cursor = if buf[pos] == 1 {
            pos += 1;
            let next_cursor_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
            pos += 2;
            let next_cursor_bytes = &buf[pos..pos + next_cursor_len];
            pos += next_cursor_len;
            Some(
                std::str::from_utf8(next_cursor_bytes)
                    .map_err(|e| format!("invalid UTF-8: {}", e))?
                    .to_string(),
            )
        } else {
            pos += 1;
            None
        };

        Ok(Self {
            account,
            orders,
            has_more,
            next_cursor,
        })
    }
}

/// SBE encoder for CapabilitiesRequest
pub struct CapabilitiesRequestEncoder;

impl CapabilitiesRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode() -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&52u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields

        buf
    }
}

/// SBE decoder for CapabilitiesRequest
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilitiesRequestDecoder {}

impl CapabilitiesRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        Ok(Self {})
    }
}

/// SBE encoder for CapabilitiesResponse
pub struct CapabilitiesResponseEncoder;

impl CapabilitiesResponseEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(
        schema_ids: Vec<u8>,
        paths: Vec<CapabilityPath>,
        symbols: Vec<Symbol>,
        intervals: Vec<CandleInterval>,
    ) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&53u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        buf.extend_from_slice(&(schema_ids.len() as u32).to_be_bytes());
        for item in &schema_ids {
            // TODO: encode list<u8>
        }
        buf.extend_from_slice(&(paths.len() as u32).to_be_bytes());
        for item in &paths {
            let mut item_buf = Vec::new();
            CapabilityPathEncoder::encode(&item, &mut item_buf);
            buf.extend_from_slice(&(item_buf.len() as u32).to_be_bytes());
            buf.extend_from_slice(&item_buf);
        }
        buf.extend_from_slice(&(symbols.len() as u32).to_be_bytes());
        for item in &symbols {
            // TODO: encode list<Symbol>
        }
        buf.extend_from_slice(&(intervals.len() as u32).to_be_bytes());
        for item in &intervals {
            // TODO: encode list<CandleInterval>
        }

        buf
    }
}

/// SBE decoder for CapabilitiesResponse
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilitiesResponseDecoder {
    pub schema_ids: Vec<u8>,
    pub paths: Vec<CapabilityPath>,
    pub symbols: Vec<Symbol>,
    pub intervals: Vec<CandleInterval>,
}

impl CapabilitiesResponseDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let schema_ids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut schema_ids = Vec::with_capacity(schema_ids_count);
        // TODO: decode list<u8>
        let paths_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut paths = Vec::with_capacity(paths_count);
        for _ in 0..paths_count {
            let item_len =
                u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
            pos += 4;
            let (item, n) = CapabilityPathDecoder::decode(&buf[pos..pos + item_len])?;
            if n != item_len {
                return Err(format!(
                    "CapabilityPath length mismatch: expected {item_len}, got {n}"
                ));
            }
            pos += item_len;
            paths.push(item);
        }
        let symbols_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut symbols = Vec::with_capacity(symbols_count);
        // TODO: decode list<Symbol>
        let intervals_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut intervals = Vec::with_capacity(intervals_count);
        // TODO: decode list<CandleInterval>

        Ok(Self {
            schema_ids,
            paths,
            symbols,
            intervals,
        })
    }
}

/// SBE encoder for PositionRequest
pub struct PositionRequestEncoder;

impl PositionRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(account: String) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&54u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length

        // Fixed fields
        let account_bytes = account.as_bytes();
        buf.extend_from_slice(&(account_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(account_bytes);

        buf
    }
}

/// SBE decoder for PositionRequest
#[derive(Debug, Clone, PartialEq)]
pub struct PositionRequestDecoder {
    pub account: String,
}

impl PositionRequestDecoder {
    /// Decode from a byte buffer.
    pub fn decode(buf: &[u8]) -> Result<Self, String> {
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

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos + account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?
            .to_string();

        Ok(Self { account })
    }
}
