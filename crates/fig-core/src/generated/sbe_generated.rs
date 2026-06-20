// Auto-generated SBE encode/decode by fig-fsl from schema 'trading.orders' vv1.0.0
// Standard order entry and execution messages

use std::convert::TryFrom;

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
pub enum MarketDataUpdateSide {
    Buy = 1,
    Sell = 2,
}

impl MarketDataUpdateSide {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(MarketDataUpdateSide::Buy),
            2 => Some(MarketDataUpdateSide::Sell),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarketDataUpdateAction {
    New = 1,
    Change = 2,
    Delete = 3,
}

impl MarketDataUpdateAction {
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(MarketDataUpdateAction::New),
            2 => Some(MarketDataUpdateAction::Change),
            3 => Some(MarketDataUpdateAction::Delete),
            _ => None,
        }
    }

    pub fn to_value(self) -> u8 {
        self as u8
    }
}

/// SBE encoder for NewOrderSingle
pub struct NewOrderSingleEncoder;

impl NewOrderSingleEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(cl_ord_id: ClientOrderId, side: NewOrderSingleSide, order_qty: Quantity, price: Option<Price>, stop_price: Option<Price>, symbol: Symbol, order_type: NewOrderSingleOrderType, time_in_force: NewOrderSingleTimeInForce, expire_time: Option<TradeTimestamp>, account: Option<String>, strategy_id: Option<String>, security_id: Option<String>, id_source: Option<NewOrderSingleIdSource>, security_exchange: Option<String>) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&1u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&36u16.to_be_bytes()); // block_length

        // Fixed fields
        let cl_ord_id_bytes = cl_ord_id.as_bytes();
        buf.extend_from_slice(&(cl_ord_id_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(cl_ord_id_bytes);
        buf.push(side as u8);
        buf.extend_from_slice(&order_qty.to_be_bytes());
        buf.extend_from_slice(&price.unwrap_or(0.0).to_be_bytes());
        buf.push(if price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&stop_price.unwrap_or(0.0).to_be_bytes());
        buf.push(if stop_price.is_some() { 1 } else { 0 });
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.push(order_type as u8);
        buf.push(time_in_force as u8);
        buf.push(if expire_time.is_some() { 1 } else { 0 });
        if let Some(v) = expire_time {
            buf.extend_from_slice(&v.to_be_bytes());
        }
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
        buf.push(if security_exchange.is_some() { 1 } else { 0 });
        if let Some(ref s) = security_exchange {
            let b = s.as_bytes();
            buf.extend_from_slice(&(b.len() as u16).to_be_bytes());
            buf.extend_from_slice(b);
        }

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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos+cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = NewOrderSingleSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let order_qty = order_qty_raw;
        let price_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 { Some(price_raw) } else { None };
        pos += 1;
        let stop_price_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let stop_price = if buf[pos] == 1 { Some(stop_price_raw) } else { None };
        pos += 1;
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let order_type_raw = buf[pos];
        pos += 1;
        let order_type = NewOrderSingleOrderType::from_value(order_type_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleOrderType value: {}", order_type_raw))?;
        let time_in_force_raw = buf[pos];
        pos += 1;
        let time_in_force = NewOrderSingleTimeInForce::from_value(time_in_force_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleTimeInForce value: {}", time_in_force_raw))?;
        let expire_time = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let account_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos+account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let account = if buf[pos] == 1 {
            pos += 1;
            let account_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
            pos += 2;
            let account_bytes = &buf[pos..pos+account_len];
            pos += account_len;
            Some(std::str::from_utf8(account_bytes)
                .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string())
        } else {
            pos += 1;
            None
        };
        let strategy_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let strategy_id_bytes = &buf[pos..pos+strategy_id_len];
        pos += strategy_id_len;
        let strategy_id = std::str::from_utf8(strategy_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let strategy_id = if buf[pos] == 1 {
            pos += 1;
            let strategy_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
            pos += 2;
            let strategy_id_bytes = &buf[pos..pos+strategy_id_len];
            pos += strategy_id_len;
            Some(std::str::from_utf8(strategy_id_bytes)
                .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string())
        } else {
            pos += 1;
            None
        };
        let security_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let security_id_bytes = &buf[pos..pos+security_id_len];
        pos += security_id_len;
        let security_id = std::str::from_utf8(security_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let security_id = if buf[pos] == 1 {
            pos += 1;
            let security_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
            pos += 2;
            let security_id_bytes = &buf[pos..pos+security_id_len];
            pos += security_id_len;
            Some(std::str::from_utf8(security_id_bytes)
                .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string())
        } else {
            pos += 1;
            None
        };
        let id_source_raw = buf[pos];
        pos += 1;
        let id_source = NewOrderSingleIdSource::from_value(id_source_raw)
            .ok_or_else(|| format!("invalid NewOrderSingleIdSource value: {}", id_source_raw))?;
        let security_exchange_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let security_exchange_bytes = &buf[pos..pos+security_exchange_len];
        pos += security_exchange_len;
        let security_exchange = std::str::from_utf8(security_exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let security_exchange = if buf[pos] == 1 {
            pos += 1;
            let security_exchange_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
            pos += 2;
            let security_exchange_bytes = &buf[pos..pos+security_exchange_len];
            pos += security_exchange_len;
            Some(std::str::from_utf8(security_exchange_bytes)
                .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string())
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
        })
    }
}

/// SBE encoder for CancelRequest
pub struct CancelRequestEncoder;

impl CancelRequestEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(cl_ord_id: ClientOrderId, orig_cl_ord_id: ClientOrderId, symbol: Symbol, side: CancelRequestSide, order_qty: Option<Quantity>) -> Vec<u8> {
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
        buf.extend_from_slice(&order_qty.unwrap_or(0.0).to_be_bytes());
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos+cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos+orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = CancelRequestSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid CancelRequestSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let order_qty = if buf[pos] == 1 { Some(order_qty_raw) } else { None };
        pos += 1;

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
    pub fn encode(cl_ord_id: ClientOrderId, orig_cl_ord_id: ClientOrderId, symbol: Symbol, side: CancelReplaceRequestSide, order_qty: Quantity, price: Option<Price>) -> Vec<u8> {
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
        buf.extend_from_slice(&order_qty.to_be_bytes());
        buf.extend_from_slice(&price.unwrap_or(0.0).to_be_bytes());
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos+cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos+orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let side_raw = buf[pos];
        pos += 1;
        let side = CancelReplaceRequestSide::from_value(side_raw)
            .ok_or_else(|| format!("invalid CancelReplaceRequestSide value: {}", side_raw))?;
        let order_qty_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let order_qty = order_qty_raw;
        let price_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let price = if buf[pos] == 1 { Some(price_raw) } else { None };
        pos += 1;

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
    pub fn encode(cl_ord_id: ClientOrderId, order_id: String, exec_id: String, exec_type: ExecutionReportExecType, ord_status: ExecutionReportOrdStatus, side: ExecutionReportSide, last_qty: Option<Quantity>, last_price: Option<Price>, leaves_qty: Quantity, cum_qty: Quantity, avg_price: Price, symbol: Symbol, transact_time: TradeTimestamp) -> Vec<u8> {
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
        buf.extend_from_slice(&last_qty.unwrap_or(0.0).to_be_bytes());
        buf.push(if last_qty.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&last_price.unwrap_or(0.0).to_be_bytes());
        buf.push(if last_price.is_some() { 1 } else { 0 });
        buf.extend_from_slice(&leaves_qty.to_be_bytes());
        buf.extend_from_slice(&cum_qty.to_be_bytes());
        buf.extend_from_slice(&avg_price.to_be_bytes());
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos+cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let order_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let order_id_bytes = &buf[pos..pos+order_id_len];
        pos += order_id_len;
        let order_id = std::str::from_utf8(order_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let exec_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let exec_id_bytes = &buf[pos..pos+exec_id_len];
        pos += exec_id_len;
        let exec_id = std::str::from_utf8(exec_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
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
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let last_qty = if buf[pos] == 1 { Some(last_qty_raw) } else { None };
        pos += 1;
        let last_price_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let last_price = if buf[pos] == 1 { Some(last_price_raw) } else { None };
        pos += 1;
        let leaves_qty_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let leaves_qty = leaves_qty_raw;
        let cum_qty_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let cum_qty = cum_qty_raw;
        let avg_price_raw = f64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let avg_price = avg_price_raw;
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let transact_time = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;

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
    pub fn encode(cl_ord_id: ClientOrderId, orig_cl_ord_id: ClientOrderId, reject_reason: CancelRejectRejectReason, symbol: Symbol) -> Vec<u8> {
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

        let cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let cl_ord_id_bytes = &buf[pos..pos+cl_ord_id_len];
        pos += cl_ord_id_len;
        let cl_ord_id = std::str::from_utf8(cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let orig_cl_ord_id_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let orig_cl_ord_id_bytes = &buf[pos..pos+orig_cl_ord_id_len];
        pos += orig_cl_ord_id_len;
        let orig_cl_ord_id = std::str::from_utf8(orig_cl_ord_id_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let reject_reason_raw = buf[pos];
        pos += 1;
        let reject_reason = CancelRejectRejectReason::from_value(reject_reason_raw)
            .ok_or_else(|| format!("invalid CancelRejectRejectReason value: {}", reject_reason_raw))?;
        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();

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
    pub fn encode(symbol: Symbol, exchange: String, bids: Vec<PriceLevel>, asks: Vec<PriceLevel>, timestamp: TradeTimestamp) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&6u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&8u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        let exchange_bytes = exchange.as_bytes();
        buf.extend_from_slice(&(exchange_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(exchange_bytes);
        buf.extend_from_slice(&(bids.len() as u32).to_be_bytes());
        for item in &bids {
            PriceLevelEncoder::encode(item, buf);
        }
        buf.extend_from_slice(&(asks.len() as u32).to_be_bytes());
        for item in &asks {
            PriceLevelEncoder::encode(item, buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());

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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let exchange_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let exchange_bytes = &buf[pos..pos+exchange_len];
        pos += exchange_len;
        let exchange = std::str::from_utf8(exchange_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let bids_count = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            let item = PriceLevelDecoder::decode(&buf[pos..])?;
            let item_len = item.encoded_len();
            pos += item_len;
            bids.push(item);
        }
        let asks_count = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            let item = PriceLevelDecoder::decode(&buf[pos..])?;
            let item_len = item.encoded_len();
            pos += item_len;
            asks.push(item);
        }
        let timestamp = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;

        Ok(Self {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
        })
    }
}

/// SBE encoder for MarketDataIncrementalRefresh
pub struct MarketDataIncrementalRefreshEncoder;

impl MarketDataIncrementalRefreshEncoder {
    /// Encode this message into a byte buffer.
    /// Returns the filled buffer.
    pub fn encode(symbol: Symbol, updates: Vec<MarketDataUpdate>, timestamp: TradeTimestamp) -> Vec<u8> {
        let mut buf = Vec::new();

        // SBE Message Header (8 bytes)
        buf.extend_from_slice(&1u16.to_be_bytes()); // schema_id
        buf.extend_from_slice(&7u16.to_be_bytes()); // template_id
        buf.extend_from_slice(&0u16.to_be_bytes()); // version
        buf.extend_from_slice(&8u16.to_be_bytes()); // block_length

        // Fixed fields
        let symbol_bytes = symbol.as_bytes();
        buf.extend_from_slice(&(symbol_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(symbol_bytes);
        buf.extend_from_slice(&(updates.len() as u32).to_be_bytes());
        for item in &updates {
            MarketDataUpdateEncoder::encode(item, buf);
        }
        buf.extend_from_slice(&timestamp.to_be_bytes());

        buf
    }
}

/// SBE decoder for MarketDataIncrementalRefresh
#[derive(Debug, Clone, PartialEq)]
pub struct MarketDataIncrementalRefreshDecoder {
    pub symbol: Symbol,
    pub updates: Vec<MarketDataUpdate>,
    pub timestamp: TradeTimestamp,
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

        let symbol_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let symbol_bytes = &buf[pos..pos+symbol_len];
        pos += symbol_len;
        let symbol = std::str::from_utf8(symbol_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let updates_count = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            let item = MarketDataUpdateDecoder::decode(&buf[pos..])?;
            let item_len = item.encoded_len();
            pos += item_len;
            updates.push(item);
        }
        let timestamp = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;

        Ok(Self {
            symbol,
            updates,
            timestamp,
        })
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
        buf.extend_from_slice(&8u16.to_be_bytes()); // template_id
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
        if tmpl_id != 8 {
            return Err(format!("invalid template_id: {}", tmpl_id));
        }

        let mut pos: usize = 8;

        let account_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let account_bytes = &buf[pos..pos+account_len];
        pos += account_len;
        let account = std::str::from_utf8(account_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();
        let balance = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let buying_power = i64::from_be_bytes([
            buf[pos+0],
            buf[pos+1],
            buf[pos+2],
            buf[pos+3],
            buf[pos+4],
            buf[pos+5],
            buf[pos+6],
            buf[pos+7],
        ]);
        pos += 8;
        let currency_len = u16::from_be_bytes([buf[pos], buf[pos+1]]) as usize;
        pos += 2;
        let currency_bytes = &buf[pos..pos+currency_len];
        pos += currency_len;
        let currency = std::str::from_utf8(currency_bytes)
            .map_err(|e| format!("invalid UTF-8: {}", e))?.to_string();

        Ok(Self {
            account,
            balance,
            buying_power,
            currency,
        })
    }
}

