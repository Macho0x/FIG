//! Auto-generated SBE encode/decode for FIG trading messages.
//!
//! Generated from schemas/orders.fsl by fig-fsl SBE codegen.
//! Uses the message types from crate::messages.
//!
//! # Format
//!
//! Each SBE message consists of:
//! - **Message Header** (8 bytes): schema_id (u16), template_id (u16), version (u16), block_length (u16)
//! - **Fields**: encoded sequentially at offsets after the header
//! - **Strings**: length-prefixed (u16 length + UTF-8 bytes)
//! - **Optional fields**: presence flag byte (0 or 1), followed by value if present

use crate::messages::*;

// ─── Schema Constants ────────────────────────────────────────────

pub const SCHEMA_ID: u16 = 0x0001;
pub const VERSION: u16 = 0;

// ─── Template IDs ────────────────────────────────────────────────

pub mod template_id {
    pub const NEW_ORDER_SINGLE: u16 = 1;
    pub const EXECUTION_REPORT: u16 = 2;
    pub const CANCEL_REQUEST: u16 = 3;
    pub const CANCEL_REPLACE_REQUEST: u16 = 4;
    pub const CANCEL_REJECT: u16 = 5;
    pub const MARKET_DATA_SNAPSHOT: u16 = 6;
    pub const MARKET_DATA_INCREMENTAL_REFRESH: u16 = 7;
}

// ─── Enum Value Helpers ──────────────────────────────────────────

mod side_values {
    pub const BUY: u8 = 1;
    pub const SELL: u8 = 2;
    pub const SELL_SHORT: u8 = 3;
    pub const SELL_SHORT_EXEMPT: u8 = 4;
}

mod order_type_values {
    pub const MARKET: u8 = 1;
    pub const LIMIT: u8 = 2;
    pub const STOP: u8 = 3;
    pub const STOP_LIMIT: u8 = 4;
}

mod time_in_force_values {
    pub const DAY: u8 = 0;
    pub const GTC: u8 = 1;
    pub const GTD: u8 = 2;
    pub const IOC: u8 = 3;
    pub const FOK: u8 = 4;
}

mod exec_type_values {
    pub const NEW: u8 = 0;
    pub const PARTIAL_FILL: u8 = 1;
    pub const FILL: u8 = 2;
    pub const DONE_FOR_DAY: u8 = 3;
    pub const CANCELED: u8 = 4;
    pub const REPLACED: u8 = 5;
    pub const PENDING_CANCEL: u8 = 6;
    pub const STOPPED: u8 = 7;
    pub const REJECTED: u8 = 8;
    pub const SUSPENDED: u8 = 9;
    pub const PENDING_NEW: u8 = 10;
    pub const EXPIRED: u8 = 11;
}

mod ord_status_values {
    pub const NEW: u8 = 0;
    pub const PARTIALLY_FILLED: u8 = 1;
    pub const FILLED: u8 = 2;
    pub const DONE_FOR_DAY: u8 = 3;
    pub const CANCELED: u8 = 4;
    pub const PENDING_CANCEL: u8 = 5;
    pub const STOPPED: u8 = 6;
    pub const REJECTED: u8 = 7;
    pub const SUSPENDED: u8 = 8;
    pub const PENDING_NEW: u8 = 9;
    pub const EXPIRED: u8 = 10;
    pub const REPLACED: u8 = 11;
}

mod cancel_reject_reason_values {
    pub const ORDER_NOT_FOUND: u8 = 1;
    pub const ALREADY_CANCELED: u8 = 2;
    pub const ALREADY_FILLED: u8 = 3;
    pub const TOO_LATE_TO_CANCEL: u8 = 4;
}

mod market_data_action_values {
    pub const NEW: u8 = 1;
    pub const CHANGE: u8 = 2;
    pub const DELETE: u8 = 3;
}

// ─── Enum Mapping Functions ──────────────────────────────────────

fn side_to_u8(side: &Side) -> u8 {
    match side {
        Side::Buy => side_values::BUY,
        Side::Sell => side_values::SELL,
        Side::SellShort => side_values::SELL_SHORT,
        Side::SellShortExempt => side_values::SELL_SHORT_EXEMPT,
    }
}

fn side_from_u8(v: u8) -> Option<Side> {
    match v {
        v if v == side_values::BUY => Some(Side::Buy),
        v if v == side_values::SELL => Some(Side::Sell),
        v if v == side_values::SELL_SHORT => Some(Side::SellShort),
        v if v == side_values::SELL_SHORT_EXEMPT => Some(Side::SellShortExempt),
        _ => None,
    }
}

fn order_type_to_u8(ot: &OrderType) -> u8 {
    match ot {
        OrderType::Market => order_type_values::MARKET,
        OrderType::Limit => order_type_values::LIMIT,
        OrderType::Stop => order_type_values::STOP,
        OrderType::StopLimit => order_type_values::STOP_LIMIT,
    }
}

fn order_type_from_u8(v: u8) -> Option<OrderType> {
    match v {
        v if v == order_type_values::MARKET => Some(OrderType::Market),
        v if v == order_type_values::LIMIT => Some(OrderType::Limit),
        v if v == order_type_values::STOP => Some(OrderType::Stop),
        v if v == order_type_values::STOP_LIMIT => Some(OrderType::StopLimit),
        _ => None,
    }
}

fn time_in_force_to_u8(tif: &TimeInForce) -> u8 {
    match tif {
        TimeInForce::Day => time_in_force_values::DAY,
        TimeInForce::Gtc => time_in_force_values::GTC,
        TimeInForce::Gtd => time_in_force_values::GTD,
        TimeInForce::Ioc => time_in_force_values::IOC,
        TimeInForce::Fok => time_in_force_values::FOK,
    }
}

fn time_in_force_from_u8(v: u8) -> Option<TimeInForce> {
    match v {
        v if v == time_in_force_values::DAY => Some(TimeInForce::Day),
        v if v == time_in_force_values::GTC => Some(TimeInForce::Gtc),
        v if v == time_in_force_values::GTD => Some(TimeInForce::Gtd),
        v if v == time_in_force_values::IOC => Some(TimeInForce::Ioc),
        v if v == time_in_force_values::FOK => Some(TimeInForce::Fok),
        _ => None,
    }
}

fn exec_type_to_u8(et: &ExecType) -> u8 {
    match et {
        ExecType::New => exec_type_values::NEW,
        ExecType::PartialFill => exec_type_values::PARTIAL_FILL,
        ExecType::Fill => exec_type_values::FILL,
        ExecType::DoneForDay => exec_type_values::DONE_FOR_DAY,
        ExecType::Canceled => exec_type_values::CANCELED,
        ExecType::Replaced => exec_type_values::REPLACED,
        ExecType::PendingCancel => exec_type_values::PENDING_CANCEL,
        ExecType::Stopped => exec_type_values::STOPPED,
        ExecType::Rejected => exec_type_values::REJECTED,
        ExecType::Suspended => exec_type_values::SUSPENDED,
        ExecType::PendingNew => exec_type_values::PENDING_NEW,
        ExecType::Expired => exec_type_values::EXPIRED,
    }
}

fn exec_type_from_u8(v: u8) -> Option<ExecType> {
    match v {
        v if v == exec_type_values::NEW => Some(ExecType::New),
        v if v == exec_type_values::PARTIAL_FILL => Some(ExecType::PartialFill),
        v if v == exec_type_values::FILL => Some(ExecType::Fill),
        v if v == exec_type_values::DONE_FOR_DAY => Some(ExecType::DoneForDay),
        v if v == exec_type_values::CANCELED => Some(ExecType::Canceled),
        v if v == exec_type_values::REPLACED => Some(ExecType::Replaced),
        v if v == exec_type_values::PENDING_CANCEL => Some(ExecType::PendingCancel),
        v if v == exec_type_values::STOPPED => Some(ExecType::Stopped),
        v if v == exec_type_values::REJECTED => Some(ExecType::Rejected),
        v if v == exec_type_values::SUSPENDED => Some(ExecType::Suspended),
        v if v == exec_type_values::PENDING_NEW => Some(ExecType::PendingNew),
        v if v == exec_type_values::EXPIRED => Some(ExecType::Expired),
        _ => None,
    }
}

fn ord_status_to_u8(os: &OrdStatus) -> u8 {
    match os {
        OrdStatus::New => ord_status_values::NEW,
        OrdStatus::PartiallyFilled => ord_status_values::PARTIALLY_FILLED,
        OrdStatus::Filled => ord_status_values::FILLED,
        OrdStatus::DoneForDay => ord_status_values::DONE_FOR_DAY,
        OrdStatus::Canceled => ord_status_values::CANCELED,
        OrdStatus::PendingCancel => ord_status_values::PENDING_CANCEL,
        OrdStatus::Stopped => ord_status_values::STOPPED,
        OrdStatus::Rejected => ord_status_values::REJECTED,
        OrdStatus::Suspended => ord_status_values::SUSPENDED,
        OrdStatus::PendingNew => ord_status_values::PENDING_NEW,
        OrdStatus::Expired => ord_status_values::EXPIRED,
        OrdStatus::Replaced => ord_status_values::REPLACED,
    }
}

fn ord_status_from_u8(v: u8) -> Option<OrdStatus> {
    match v {
        v if v == ord_status_values::NEW => Some(OrdStatus::New),
        v if v == ord_status_values::PARTIALLY_FILLED => Some(OrdStatus::PartiallyFilled),
        v if v == ord_status_values::FILLED => Some(OrdStatus::Filled),
        v if v == ord_status_values::DONE_FOR_DAY => Some(OrdStatus::DoneForDay),
        v if v == ord_status_values::CANCELED => Some(OrdStatus::Canceled),
        v if v == ord_status_values::PENDING_CANCEL => Some(OrdStatus::PendingCancel),
        v if v == ord_status_values::STOPPED => Some(OrdStatus::Stopped),
        v if v == ord_status_values::REJECTED => Some(OrdStatus::Rejected),
        v if v == ord_status_values::SUSPENDED => Some(OrdStatus::Suspended),
        v if v == ord_status_values::PENDING_NEW => Some(OrdStatus::PendingNew),
        v if v == ord_status_values::EXPIRED => Some(OrdStatus::Expired),
        v if v == ord_status_values::REPLACED => Some(OrdStatus::Replaced),
        _ => None,
    }
}

fn cancel_reject_reason_to_u8(r: &CancelRejectReason) -> u8 {
    match r {
        CancelRejectReason::OrderNotFound => cancel_reject_reason_values::ORDER_NOT_FOUND,
        CancelRejectReason::AlreadyCanceled => cancel_reject_reason_values::ALREADY_CANCELED,
        CancelRejectReason::AlreadyFilled => cancel_reject_reason_values::ALREADY_FILLED,
        CancelRejectReason::TooLateToCancel => cancel_reject_reason_values::TOO_LATE_TO_CANCEL,
    }
}

fn cancel_reject_reason_from_u8(v: u8) -> Option<CancelRejectReason> {
    match v {
        v if v == cancel_reject_reason_values::ORDER_NOT_FOUND => {
            Some(CancelRejectReason::OrderNotFound)
        }
        v if v == cancel_reject_reason_values::ALREADY_CANCELED => {
            Some(CancelRejectReason::AlreadyCanceled)
        }
        v if v == cancel_reject_reason_values::ALREADY_FILLED => {
            Some(CancelRejectReason::AlreadyFilled)
        }
        v if v == cancel_reject_reason_values::TOO_LATE_TO_CANCEL => {
            Some(CancelRejectReason::TooLateToCancel)
        }
        _ => None,
    }
}

fn market_data_action_to_u8(a: &MarketDataAction) -> u8 {
    match a {
        MarketDataAction::New => market_data_action_values::NEW,
        MarketDataAction::Change => market_data_action_values::CHANGE,
        MarketDataAction::Delete => market_data_action_values::DELETE,
    }
}

fn market_data_action_from_u8(v: u8) -> Option<MarketDataAction> {
    match v {
        v if v == market_data_action_values::NEW => Some(MarketDataAction::New),
        v if v == market_data_action_values::CHANGE => Some(MarketDataAction::Change),
        v if v == market_data_action_values::DELETE => Some(MarketDataAction::Delete),
        _ => None,
    }
}

// ─── Header helpers ──────────────────────────────────────────────

fn write_header(buf: &mut Vec<u8>, template_id: u16, block_length: u16) {
    buf.extend_from_slice(&SCHEMA_ID.to_be_bytes());
    buf.extend_from_slice(&template_id.to_be_bytes());
    buf.extend_from_slice(&VERSION.to_be_bytes());
    buf.extend_from_slice(&block_length.to_be_bytes());
}

fn read_header(buf: &[u8]) -> Result<(u16, u16, u16, u16), String> {
    if buf.len() < 8 {
        return Err("buffer too short for SBE header".to_string());
    }
    let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
    let template_id = u16::from_be_bytes([buf[2], buf[3]]);
    let version = u16::from_be_bytes([buf[4], buf[5]]);
    let block_length = u16::from_be_bytes([buf[6], buf[7]]);
    Ok((schema_id, template_id, version, block_length))
}

fn write_str(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    buf.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
    buf.extend_from_slice(bytes);
}

fn read_str(buf: &[u8], pos: &mut usize) -> String {
    let len = u16::from_be_bytes([buf[*pos], buf[*pos + 1]]) as usize;
    *pos += 2;
    let bytes = &buf[*pos..*pos + len];
    *pos += len;
    std::str::from_utf8(bytes)
        .expect("invalid UTF-8 in SBE string")
        .to_string()
}

fn write_f64(buf: &mut Vec<u8>, v: f64) {
    buf.extend_from_slice(&v.to_be_bytes());
}

fn read_f64(buf: &[u8], pos: &mut usize) -> f64 {
    let val = f64::from_be_bytes([
        buf[*pos],
        buf[*pos + 1],
        buf[*pos + 2],
        buf[*pos + 3],
        buf[*pos + 4],
        buf[*pos + 5],
        buf[*pos + 6],
        buf[*pos + 7],
    ]);
    *pos += 8;
    val
}

fn write_i64(buf: &mut Vec<u8>, v: i64) {
    buf.extend_from_slice(&v.to_be_bytes());
}

fn read_i64(buf: &[u8], pos: &mut usize) -> i64 {
    let val = i64::from_be_bytes([
        buf[*pos],
        buf[*pos + 1],
        buf[*pos + 2],
        buf[*pos + 3],
        buf[*pos + 4],
        buf[*pos + 5],
        buf[*pos + 6],
        buf[*pos + 7],
    ]);
    *pos += 8;
    val
}

// ═══════════════════════════════════════════════════════════════════
//  Message: NewOrderSingle (template_id = 1)
// ═══════════════════════════════════════════════════════════════════

pub struct NewOrderSingleEncoder;
pub struct NewOrderSingleDecoder;

impl NewOrderSingleEncoder {
    pub fn encode(order: &NewOrderSingle) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::NEW_ORDER_SINGLE, 0); // block_length placeholder

        // Fixed fields
        write_str(&mut buf, &order.cl_ord_id);
        buf.push(side_to_u8(&order.side));
        write_f64(&mut buf, order.order_qty.0);

        // price: f64 value + presence flag
        let price = order.price.as_ref().map(|p| p.0).unwrap_or(0.0);
        write_f64(&mut buf, price);
        buf.push(if order.price.is_some() { 1 } else { 0 });

        write_str(&mut buf, &order.symbol);
        buf.push(order_type_to_u8(&order.order_type));
        buf.push(time_in_force_to_u8(&order.time_in_force));

        // account: optional string
        buf.push(if order.account.is_some() { 1 } else { 0 });
        if let Some(ref account) = order.account {
            write_str(&mut buf, account);
        }

        // expire_time: optional i64
        buf.push(if order.expire_time.is_some() { 1 } else { 0 });
        if let Some(expire_time) = order.expire_time {
            write_i64(&mut buf, expire_time);
        }

        // strategy_id: optional string
        buf.push(if order.strategy_id.is_some() { 1 } else { 0 });
        if let Some(ref strategy_id) = order.strategy_id {
            write_str(&mut buf, strategy_id);
        }

        buf
    }
}

impl NewOrderSingleDecoder {
    pub fn decode(buf: &[u8]) -> Result<NewOrderSingle, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::NEW_ORDER_SINGLE {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let cl_ord_id = read_str(buf, &mut pos);
        let side = side_from_u8(buf[pos]).ok_or_else(|| format!("invalid side: {}", buf[pos]))?;
        pos += 1;
        let order_qty = Quantity(read_f64(buf, &mut pos));

        let price_val = read_f64(buf, &mut pos);
        let has_price = buf[pos];
        pos += 1;
        let price = if has_price == 1 {
            Some(Price(price_val))
        } else {
            None
        };

        let symbol = read_str(buf, &mut pos);
        let order_type = order_type_from_u8(buf[pos])
            .ok_or_else(|| format!("invalid order_type: {}", buf[pos]))?;
        pos += 1;
        let time_in_force = time_in_force_from_u8(buf[pos])
            .ok_or_else(|| format!("invalid time_in_force: {}", buf[pos]))?;
        pos += 1;

        let has_account = buf[pos];
        pos += 1;
        let account = if has_account == 1 {
            Some(read_str(buf, &mut pos))
        } else {
            None
        };

        let has_expire_time = buf[pos];
        pos += 1;
        let expire_time = if has_expire_time == 1 {
            Some(read_i64(buf, &mut pos))
        } else {
            None
        };

        let has_strategy_id = buf[pos];
        pos += 1;
        let strategy_id = if has_strategy_id == 1 {
            Some(read_str(buf, &mut pos))
        } else {
            None
        };

        Ok(NewOrderSingle {
            cl_ord_id,
            side,
            order_qty,
            price,
            symbol,
            order_type,
            time_in_force,
            expire_time,
            account,
            strategy_id,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Message: CancelRequest (template_id = 3)
// ═══════════════════════════════════════════════════════════════════

pub struct CancelRequestEncoder;
pub struct CancelRequestDecoder;

impl CancelRequestEncoder {
    pub fn encode(cancel: &CancelRequest) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::CANCEL_REQUEST, 0);

        write_str(&mut buf, &cancel.cl_ord_id);
        write_str(&mut buf, &cancel.orig_cl_ord_id);
        write_str(&mut buf, &cancel.symbol);
        buf.push(side_to_u8(&cancel.side));

        buf
    }
}

impl CancelRequestDecoder {
    pub fn decode(buf: &[u8]) -> Result<CancelRequest, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::CANCEL_REQUEST {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let cl_ord_id = read_str(buf, &mut pos);
        let orig_cl_ord_id = read_str(buf, &mut pos);
        let symbol = read_str(buf, &mut pos);
        let side = side_from_u8(buf[pos]).ok_or_else(|| format!("invalid side: {}", buf[pos]))?;
        // pos advanced past side byte; no further fields to decode

        Ok(CancelRequest {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Message: CancelReplaceRequest (template_id = 4)
// ═══════════════════════════════════════════════════════════════════

pub struct CancelReplaceRequestEncoder;
pub struct CancelReplaceRequestDecoder;

impl CancelReplaceRequestEncoder {
    pub fn encode(msg: &CancelReplaceRequest) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::CANCEL_REPLACE_REQUEST, 0);

        write_str(&mut buf, &msg.cl_ord_id);
        write_str(&mut buf, &msg.orig_cl_ord_id);
        write_str(&mut buf, &msg.symbol);
        buf.push(side_to_u8(&msg.side));
        write_f64(&mut buf, msg.order_qty.0);

        // price: optional f64
        buf.push(if msg.price.is_some() { 1 } else { 0 });
        if let Some(ref price) = msg.price {
            write_f64(&mut buf, price.0);
        }

        buf
    }
}

impl CancelReplaceRequestDecoder {
    pub fn decode(buf: &[u8]) -> Result<CancelReplaceRequest, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::CANCEL_REPLACE_REQUEST {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let cl_ord_id = read_str(buf, &mut pos);
        let orig_cl_ord_id = read_str(buf, &mut pos);
        let symbol = read_str(buf, &mut pos);
        let side = side_from_u8(buf[pos]).ok_or_else(|| format!("invalid side: {}", buf[pos]))?;
        pos += 1;
        let order_qty = Quantity(read_f64(buf, &mut pos));

        let has_price = buf[pos];
        pos += 1;
        let price = if has_price == 1 {
            Some(Price(read_f64(buf, &mut pos)))
        } else {
            None
        };

        Ok(CancelReplaceRequest {
            cl_ord_id,
            orig_cl_ord_id,
            symbol,
            side,
            order_qty,
            price,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Message: ExecutionReport (template_id = 2)
// ═══════════════════════════════════════════════════════════════════

pub struct ExecutionReportEncoder;
pub struct ExecutionReportDecoder;

impl ExecutionReportEncoder {
    pub fn encode(report: &ExecutionReport) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::EXECUTION_REPORT, 0);

        write_str(&mut buf, &report.cl_ord_id);
        write_str(&mut buf, &report.order_id);
        write_str(&mut buf, &report.exec_id);
        buf.push(exec_type_to_u8(&report.exec_type));
        buf.push(ord_status_to_u8(&report.ord_status));
        buf.push(side_to_u8(&report.side));

        // last_qty: optional f64
        buf.push(if report.last_qty.is_some() { 1 } else { 0 });
        if let Some(ref qty) = report.last_qty {
            write_f64(&mut buf, qty.0);
        }

        // last_price: optional f64
        buf.push(if report.last_price.is_some() { 1 } else { 0 });
        if let Some(ref price) = report.last_price {
            write_f64(&mut buf, price.0);
        }

        write_f64(&mut buf, report.leaves_qty.0);
        write_f64(&mut buf, report.cum_qty.0);
        write_f64(&mut buf, report.avg_price.0);
        write_str(&mut buf, &report.symbol);
        write_i64(&mut buf, report.transact_time);

        buf
    }
}

impl ExecutionReportDecoder {
    pub fn decode(buf: &[u8]) -> Result<ExecutionReport, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::EXECUTION_REPORT {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let cl_ord_id = read_str(buf, &mut pos);
        let order_id = read_str(buf, &mut pos);
        let exec_id = read_str(buf, &mut pos);
        let exec_type = exec_type_from_u8(buf[pos])
            .ok_or_else(|| format!("invalid exec_type: {}", buf[pos]))?;
        pos += 1;
        let ord_status = ord_status_from_u8(buf[pos])
            .ok_or_else(|| format!("invalid ord_status: {}", buf[pos]))?;
        pos += 1;
        let side = side_from_u8(buf[pos]).ok_or_else(|| format!("invalid side: {}", buf[pos]))?;
        pos += 1;

        let has_last_qty = buf[pos];
        pos += 1;
        let last_qty = if has_last_qty == 1 {
            Some(Quantity(read_f64(buf, &mut pos)))
        } else {
            None
        };

        let has_last_price = buf[pos];
        pos += 1;
        let last_price = if has_last_price == 1 {
            Some(Price(read_f64(buf, &mut pos)))
        } else {
            None
        };

        let leaves_qty = Quantity(read_f64(buf, &mut pos));
        let cum_qty = Quantity(read_f64(buf, &mut pos));
        let avg_price = Price(read_f64(buf, &mut pos));
        let symbol = read_str(buf, &mut pos);
        let transact_time = read_i64(buf, &mut pos);

        Ok(ExecutionReport {
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

// ═══════════════════════════════════════════════════════════════════
//  Message: CancelReject (template_id = 5)
// ═══════════════════════════════════════════════════════════════════

pub struct CancelRejectEncoder;
pub struct CancelRejectDecoder;

impl CancelRejectEncoder {
    pub fn encode(msg: &CancelReject) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::CANCEL_REJECT, 0);

        write_str(&mut buf, &msg.cl_ord_id);
        write_str(&mut buf, &msg.orig_cl_ord_id);
        buf.push(cancel_reject_reason_to_u8(&msg.reject_reason));
        write_str(&mut buf, &msg.symbol);

        buf
    }
}

impl CancelRejectDecoder {
    pub fn decode(buf: &[u8]) -> Result<CancelReject, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::CANCEL_REJECT {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let cl_ord_id = read_str(buf, &mut pos);
        let orig_cl_ord_id = read_str(buf, &mut pos);
        let reject_reason = cancel_reject_reason_from_u8(buf[pos])
            .ok_or_else(|| format!("invalid reject_reason: {}", buf[pos]))?;
        pos += 1;
        let symbol = read_str(buf, &mut pos);

        Ok(CancelReject {
            cl_ord_id,
            orig_cl_ord_id,
            reject_reason,
            symbol,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Message: MarketDataSnapshot (template_id = 6)
// ═══════════════════════════════════════════════════════════════════

pub struct MarketDataSnapshotEncoder;
pub struct MarketDataSnapshotDecoder;

// ── PriceLevel encoder/decoder (inline) ──────────────────────────

fn write_price_level(buf: &mut Vec<u8>, pl: &PriceLevel) {
    write_f64(buf, pl.price.0);
    write_f64(buf, pl.qty.0);
    buf.push(if pl.order_count.is_some() { 1 } else { 0 });
    if let Some(oc) = pl.order_count {
        buf.extend_from_slice(&oc.to_be_bytes());
    }
}

fn read_price_level(buf: &[u8], pos: &mut usize) -> PriceLevel {
    let price = Price(read_f64(buf, pos));
    let qty = Quantity(read_f64(buf, pos));
    let has_oc = buf[*pos];
    *pos += 1;
    let order_count = if has_oc == 1 {
        let val = u32::from_be_bytes([buf[*pos], buf[*pos + 1], buf[*pos + 2], buf[*pos + 3]]);
        *pos += 4;
        Some(val)
    } else {
        None
    };
    PriceLevel {
        price,
        qty,
        order_count,
    }
}

impl MarketDataSnapshotEncoder {
    pub fn encode(msg: &MarketDataSnapshot) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::MARKET_DATA_SNAPSHOT, 0);

        write_str(&mut buf, &msg.symbol);
        write_str(&mut buf, &msg.exchange);

        // bids: repeating group
        buf.extend_from_slice(&(msg.bids.len() as u32).to_be_bytes());
        for bid in &msg.bids {
            write_price_level(&mut buf, bid);
        }

        // asks: repeating group
        buf.extend_from_slice(&(msg.asks.len() as u32).to_be_bytes());
        for ask in &msg.asks {
            write_price_level(&mut buf, ask);
        }

        write_i64(&mut buf, msg.timestamp);

        buf
    }
}

impl MarketDataSnapshotDecoder {
    pub fn decode(buf: &[u8]) -> Result<MarketDataSnapshot, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::MARKET_DATA_SNAPSHOT {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let symbol = read_str(buf, &mut pos);
        let exchange = read_str(buf, &mut pos);

        // bids
        let bids_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut bids = Vec::with_capacity(bids_count);
        for _ in 0..bids_count {
            bids.push(read_price_level(buf, &mut pos));
        }

        // asks
        let asks_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut asks = Vec::with_capacity(asks_count);
        for _ in 0..asks_count {
            asks.push(read_price_level(buf, &mut pos));
        }

        let timestamp = read_i64(buf, &mut pos);

        Ok(MarketDataSnapshot {
            symbol,
            exchange,
            bids,
            asks,
            timestamp,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Message: MarketDataIncrementalRefresh (template_id = 7)
// ═══════════════════════════════════════════════════════════════════

pub struct MarketDataIncrementalRefreshEncoder;
pub struct MarketDataIncrementalRefreshDecoder;

// ── MarketDataUpdate encoder/decoder (inline) ────────────────────

fn write_market_data_update(buf: &mut Vec<u8>, upd: &MarketDataUpdate) {
    buf.push(side_to_u8(&upd.side));
    buf.push(market_data_action_to_u8(&upd.action));
    write_f64(buf, upd.price.0);
    write_f64(buf, upd.qty.0);
}

fn read_market_data_update(buf: &[u8], pos: &mut usize) -> MarketDataUpdate {
    let side = side_from_u8(buf[*pos]).unwrap();
    *pos += 1;
    let action = market_data_action_from_u8(buf[*pos]).unwrap();
    *pos += 1;
    let price = Price(read_f64(buf, pos));
    let qty = Quantity(read_f64(buf, pos));
    MarketDataUpdate {
        side,
        action,
        price,
        qty,
    }
}

impl MarketDataIncrementalRefreshEncoder {
    pub fn encode(msg: &MarketDataIncrementalRefresh) -> Vec<u8> {
        let mut buf = Vec::new();
        write_header(&mut buf, template_id::MARKET_DATA_INCREMENTAL_REFRESH, 0);

        write_str(&mut buf, &msg.symbol);

        // updates: repeating group
        buf.extend_from_slice(&(msg.updates.len() as u32).to_be_bytes());
        for upd in &msg.updates {
            write_market_data_update(&mut buf, upd);
        }

        write_i64(&mut buf, msg.timestamp);

        buf
    }
}

impl MarketDataIncrementalRefreshDecoder {
    pub fn decode(buf: &[u8]) -> Result<MarketDataIncrementalRefresh, String> {
        let (schema_id, template_id, _version, _block_length) = read_header(buf)?;
        if schema_id != SCHEMA_ID {
            return Err(format!("invalid schema_id: {}", schema_id));
        }
        if template_id != template_id::MARKET_DATA_INCREMENTAL_REFRESH {
            return Err(format!("invalid template_id: {}", template_id));
        }

        let mut pos: usize = 8;

        let symbol = read_str(buf, &mut pos);

        let updates_count =
            u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;
        let mut updates = Vec::with_capacity(updates_count);
        for _ in 0..updates_count {
            updates.push(read_market_data_update(buf, &mut pos));
        }

        let timestamp = read_i64(buf, &mut pos);

        Ok(MarketDataIncrementalRefresh {
            symbol,
            updates,
            timestamp,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════
//  Tests
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbe as handwritten;

    // ── Round-trip tests ──────────────────────────────────────────

    #[test]
    fn test_new_order_single_round_trip() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-001".to_string(),
            side: Side::Buy,
            order_qty: Quantity(100.0),
            price: Some(Price(50.25)),
            symbol: "AAPL".to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: Some("ACCT-123".to_string()),
            strategy_id: None,
        };

        let encoded = NewOrderSingleEncoder::encode(&order);
        let decoded = NewOrderSingleDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-001");
        assert_eq!(decoded.side, Side::Buy);
        assert_eq!(decoded.order_qty, Quantity(100.0));
        assert_eq!(decoded.price, Some(Price(50.25)));
        assert_eq!(decoded.symbol, "AAPL");
        assert_eq!(decoded.order_type, OrderType::Limit);
        assert_eq!(decoded.time_in_force, TimeInForce::Day);
        assert_eq!(decoded.account.as_deref(), Some("ACCT-123"));
        assert_eq!(decoded.expire_time, None);
        assert_eq!(decoded.strategy_id, None);
    }

    #[test]
    fn test_execution_report_round_trip() {
        let report = ExecutionReport {
            cl_ord_id: "ORD-001".to_string(),
            order_id: "OX-001".to_string(),
            exec_id: "EX-001".to_string(),
            exec_type: ExecType::Fill,
            ord_status: OrdStatus::Filled,
            side: Side::Buy,
            last_qty: Some(Quantity(100.0)),
            last_price: Some(Price(50.25)),
            leaves_qty: Quantity(0.0),
            cum_qty: Quantity(100.0),
            avg_price: Price(50.25),
            symbol: "AAPL".to_string(),
            transact_time: 1700000000000000000,
        };

        let encoded = ExecutionReportEncoder::encode(&report);
        let decoded = ExecutionReportDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-001");
        assert_eq!(decoded.order_id, "OX-001");
        assert_eq!(decoded.exec_id, "EX-001");
        assert_eq!(decoded.exec_type, ExecType::Fill);
        assert_eq!(decoded.ord_status, OrdStatus::Filled);
        assert_eq!(decoded.side, Side::Buy);
        assert_eq!(decoded.last_qty, Some(Quantity(100.0)));
        assert_eq!(decoded.last_price, Some(Price(50.25)));
        assert_eq!(decoded.leaves_qty, Quantity(0.0));
        assert_eq!(decoded.cum_qty, Quantity(100.0));
        assert_eq!(decoded.avg_price, Price(50.25));
        assert_eq!(decoded.symbol, "AAPL");
        assert_eq!(decoded.transact_time, 1700000000000000000);
    }

    #[test]
    fn test_cancel_request_round_trip() {
        let cancel = CancelRequest {
            cl_ord_id: "ORD-002".to_string(),
            orig_cl_ord_id: "ORD-001".to_string(),
            symbol: "MSFT".to_string(),
            side: Side::Sell,
            order_qty: None,
        };

        let encoded = CancelRequestEncoder::encode(&cancel);
        let decoded = CancelRequestDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-002");
        assert_eq!(decoded.orig_cl_ord_id, "ORD-001");
        assert_eq!(decoded.symbol, "MSFT");
        assert_eq!(decoded.side, Side::Sell);
    }

    #[test]
    fn test_cancel_replace_request_round_trip() {
        let msg = CancelReplaceRequest {
            cl_ord_id: "ORD-003".to_string(),
            orig_cl_ord_id: "ORD-001".to_string(),
            symbol: "GOOG".to_string(),
            side: Side::SellShort,
            order_qty: Quantity(200.0),
            price: Some(Price(150.50)),
        };

        let encoded = CancelReplaceRequestEncoder::encode(&msg);
        let decoded = CancelReplaceRequestDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-003");
        assert_eq!(decoded.orig_cl_ord_id, "ORD-001");
        assert_eq!(decoded.symbol, "GOOG");
        assert_eq!(decoded.side, Side::SellShort);
        assert_eq!(decoded.order_qty, Quantity(200.0));
        assert_eq!(decoded.price, Some(Price(150.50)));
    }

    #[test]
    fn test_cancel_reject_round_trip() {
        let msg = CancelReject {
            cl_ord_id: "ORD-004".to_string(),
            orig_cl_ord_id: "ORD-001".to_string(),
            reject_reason: CancelRejectReason::AlreadyFilled,
            symbol: "TSLA".to_string(),
        };

        let encoded = CancelRejectEncoder::encode(&msg);
        let decoded = CancelRejectDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-004");
        assert_eq!(decoded.orig_cl_ord_id, "ORD-001");
        assert_eq!(decoded.reject_reason, CancelRejectReason::AlreadyFilled);
        assert_eq!(decoded.symbol, "TSLA");
    }

    #[test]
    fn test_market_data_snapshot_round_trip() {
        let msg = MarketDataSnapshot {
            symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
            bids: vec![
                PriceLevel {
                    price: Price(150.0),
                    qty: Quantity(100.0),
                    order_count: Some(5),
                },
                PriceLevel {
                    price: Price(149.5),
                    qty: Quantity(200.0),
                    order_count: None,
                },
            ],
            asks: vec![PriceLevel {
                price: Price(151.0),
                qty: Quantity(50.0),
                order_count: Some(3),
            }],
            timestamp: 1700000000000000000,
        };

        let encoded = MarketDataSnapshotEncoder::encode(&msg);
        let decoded = MarketDataSnapshotDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.symbol, "AAPL");
        assert_eq!(decoded.exchange, "NASDAQ");
        assert_eq!(decoded.bids.len(), 2);
        assert_eq!(decoded.asks.len(), 1);
        assert_eq!(decoded.bids[0].price, Price(150.0));
        assert_eq!(decoded.bids[0].qty, Quantity(100.0));
        assert_eq!(decoded.bids[0].order_count, Some(5));
        assert_eq!(decoded.bids[1].price, Price(149.5));
        assert_eq!(decoded.bids[1].qty, Quantity(200.0));
        assert_eq!(decoded.bids[1].order_count, None);
        assert_eq!(decoded.asks[0].price, Price(151.0));
        assert_eq!(decoded.asks[0].qty, Quantity(50.0));
        assert_eq!(decoded.asks[0].order_count, Some(3));
        assert_eq!(decoded.timestamp, 1700000000000000000);
    }

    #[test]
    fn test_market_data_incremental_refresh_round_trip() {
        let msg = MarketDataIncrementalRefresh {
            symbol: "AAPL".to_string(),
            updates: vec![
                MarketDataUpdate {
                    side: Side::Buy,
                    action: MarketDataAction::New,
                    price: Price(150.0),
                    qty: Quantity(100.0),
                },
                MarketDataUpdate {
                    side: Side::Sell,
                    action: MarketDataAction::Change,
                    price: Price(151.0),
                    qty: Quantity(50.0),
                },
            ],
            timestamp: 1700000000000000000,
        };

        let encoded = MarketDataIncrementalRefreshEncoder::encode(&msg);
        let decoded = MarketDataIncrementalRefreshDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.symbol, "AAPL");
        assert_eq!(decoded.updates.len(), 2);
        assert_eq!(decoded.updates[0].side, Side::Buy);
        assert_eq!(decoded.updates[0].action, MarketDataAction::New);
        assert_eq!(decoded.updates[0].price, Price(150.0));
        assert_eq!(decoded.updates[0].qty, Quantity(100.0));
        assert_eq!(decoded.updates[1].side, Side::Sell);
        assert_eq!(decoded.updates[1].action, MarketDataAction::Change);
        assert_eq!(decoded.updates[1].price, Price(151.0));
        assert_eq!(decoded.updates[1].qty, Quantity(50.0));
        assert_eq!(decoded.timestamp, 1700000000000000000);
    }

    // ── Cross-validation tests (hand-written encode, generated decode) ──

    #[test]
    fn test_new_order_single_cross_validation_hw_to_gen() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-CV1".to_string(),
            side: Side::SellShort,
            order_qty: Quantity(500.0),
            price: Some(Price(75.50)),
            symbol: "MSFT".to_string(),
            order_type: OrderType::Stop,
            time_in_force: TimeInForce::Gtc,
            expire_time: Some(1700000000000000000),
            account: Some("ACCT-X".to_string()),
            strategy_id: Some("STRAT-Y".to_string()),
        };

        // Encode with hand-written
        let encoded = handwritten::encode_new_order_single(&order);

        // Decode with generated
        let decoded = NewOrderSingleDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, order.cl_ord_id);
        assert_eq!(decoded.side, order.side);
        assert_eq!(decoded.order_qty, order.order_qty);
        assert_eq!(decoded.price, order.price);
        assert_eq!(decoded.symbol, order.symbol);
        assert_eq!(decoded.order_type, order.order_type);
        assert_eq!(decoded.time_in_force, order.time_in_force);
        assert_eq!(decoded.expire_time, order.expire_time);
        assert_eq!(decoded.account, order.account);
        assert_eq!(decoded.strategy_id, order.strategy_id);
    }

    #[test]
    fn test_new_order_single_cross_validation_gen_to_hw() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-CV2".to_string(),
            side: Side::Buy,
            order_qty: Quantity(1000.0),
            price: None,
            symbol: "TSLA".to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Ioc,
            expire_time: None,
            account: None,
            strategy_id: None,
        };

        // Encode with generated
        let encoded = NewOrderSingleEncoder::encode(&order);

        // Decode with hand-written
        let decoded = handwritten::decode_new_order_single(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, order.cl_ord_id);
        assert_eq!(decoded.side, order.side);
        assert_eq!(decoded.order_qty, order.order_qty);
        assert_eq!(decoded.price, order.price);
        assert_eq!(decoded.symbol, order.symbol);
        assert_eq!(decoded.order_type, order.order_type);
        assert_eq!(decoded.time_in_force, order.time_in_force);
        assert_eq!(decoded.expire_time, order.expire_time);
        assert_eq!(decoded.account, order.account);
        assert_eq!(decoded.strategy_id, order.strategy_id);
    }

    #[test]
    fn test_execution_report_cross_validation_hw_to_gen() {
        let report = ExecutionReport {
            cl_ord_id: "ORD-CV3".to_string(),
            order_id: "OX-CV3".to_string(),
            exec_id: "EX-CV3".to_string(),
            exec_type: ExecType::PartialFill,
            ord_status: OrdStatus::PartiallyFilled,
            side: Side::Sell,
            last_qty: Some(Quantity(50.0)),
            last_price: Some(Price(100.0)),
            leaves_qty: Quantity(50.0),
            cum_qty: Quantity(50.0),
            avg_price: Price(100.0),
            symbol: "GOOG".to_string(),
            transact_time: 1700000000000000000,
        };

        let encoded = handwritten::encode_execution_report(&report);
        let decoded = ExecutionReportDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, report.cl_ord_id);
        assert_eq!(decoded.order_id, report.order_id);
        assert_eq!(decoded.exec_id, report.exec_id);
        assert_eq!(decoded.exec_type, report.exec_type);
        assert_eq!(decoded.ord_status, report.ord_status);
        assert_eq!(decoded.side, report.side);
        assert_eq!(decoded.last_qty, report.last_qty);
        assert_eq!(decoded.last_price, report.last_price);
        assert_eq!(decoded.leaves_qty, report.leaves_qty);
        assert_eq!(decoded.cum_qty, report.cum_qty);
        assert_eq!(decoded.avg_price, report.avg_price);
        assert_eq!(decoded.symbol, report.symbol);
        assert_eq!(decoded.transact_time, report.transact_time);
    }

    #[test]
    fn test_execution_report_cross_validation_gen_to_hw() {
        let report = ExecutionReport {
            cl_ord_id: "ORD-CV4".to_string(),
            order_id: "OX-CV4".to_string(),
            exec_id: "EX-CV4".to_string(),
            exec_type: ExecType::New,
            ord_status: OrdStatus::New,
            side: Side::Buy,
            last_qty: None,
            last_price: None,
            leaves_qty: Quantity(100.0),
            cum_qty: Quantity(0.0),
            avg_price: Price(0.0),
            symbol: "AAPL".to_string(),
            transact_time: 1700000000000000000,
        };

        let encoded = ExecutionReportEncoder::encode(&report);
        let decoded = handwritten::decode_execution_report(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, report.cl_ord_id);
        assert_eq!(decoded.exec_type, report.exec_type);
        assert_eq!(decoded.ord_status, report.ord_status);
        assert_eq!(decoded.side, report.side);
        assert_eq!(decoded.last_qty, report.last_qty);
        assert_eq!(decoded.last_price, report.last_price);
        assert_eq!(decoded.leaves_qty, report.leaves_qty);
        assert_eq!(decoded.cum_qty, report.cum_qty);
        assert_eq!(decoded.avg_price, report.avg_price);
        assert_eq!(decoded.symbol, report.symbol);
        assert_eq!(decoded.transact_time, report.transact_time);
    }

    #[test]
    fn test_cancel_request_cross_validation_hw_to_gen() {
        let cancel = CancelRequest {
            cl_ord_id: "ORD-CV5".to_string(),
            orig_cl_ord_id: "ORD-OLD".to_string(),
            symbol: "GOOG".to_string(),
            side: Side::SellShort,
            order_qty: None,
        };

        let encoded = handwritten::encode_cancel_request(&cancel);
        let decoded = CancelRequestDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, cancel.cl_ord_id);
        assert_eq!(decoded.orig_cl_ord_id, cancel.orig_cl_ord_id);
        assert_eq!(decoded.symbol, cancel.symbol);
        assert_eq!(decoded.side, cancel.side);
    }

    #[test]
    fn test_cancel_request_cross_validation_gen_to_hw() {
        let cancel = CancelRequest {
            cl_ord_id: "ORD-CV6".to_string(),
            orig_cl_ord_id: "ORD-OLD2".to_string(),
            symbol: "MSFT".to_string(),
            side: Side::Buy,
            order_qty: None,
        };

        let encoded = CancelRequestEncoder::encode(&cancel);
        let decoded = handwritten::decode_cancel_request(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, cancel.cl_ord_id);
        assert_eq!(decoded.orig_cl_ord_id, cancel.orig_cl_ord_id);
        assert_eq!(decoded.symbol, cancel.symbol);
        assert_eq!(decoded.side, cancel.side);
    }

    // ── Edge cases ────────────────────────────────────────────────

    #[test]
    fn test_new_order_no_optionals() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-MIN".to_string(),
            side: Side::Buy,
            order_qty: Quantity(50.0),
            price: None,
            symbol: "TEST".to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Ioc,
            expire_time: None,
            account: None,
            strategy_id: None,
        };

        let encoded = NewOrderSingleEncoder::encode(&order);
        let decoded = NewOrderSingleDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.price, None);
        assert_eq!(decoded.account, None);
        assert_eq!(decoded.expire_time, None);
        assert_eq!(decoded.strategy_id, None);
    }

    #[test]
    fn test_all_side_values() {
        let sides = vec![
            Side::Buy,
            Side::Sell,
            Side::SellShort,
            Side::SellShortExempt,
        ];
        for side in &sides {
            let v = side_to_u8(side);
            let back = side_from_u8(v).unwrap();
            assert_eq!(*side, back);
        }
    }

    #[test]
    fn test_all_order_types() {
        let types = vec![
            OrderType::Market,
            OrderType::Limit,
            OrderType::Stop,
            OrderType::StopLimit,
        ];
        for ot in &types {
            let v = order_type_to_u8(ot);
            let back = order_type_from_u8(v).unwrap();
            assert_eq!(*ot, back);
        }
    }

    #[test]
    fn test_empty_market_data() {
        let msg = MarketDataSnapshot {
            symbol: "EMPTY".to_string(),
            exchange: "NONE".to_string(),
            bids: vec![],
            asks: vec![],
            timestamp: 0,
        };

        let encoded = MarketDataSnapshotEncoder::encode(&msg);
        let decoded = MarketDataSnapshotDecoder::decode(&encoded).unwrap();

        assert_eq!(decoded.symbol, "EMPTY");
        assert_eq!(decoded.bids.len(), 0);
        assert_eq!(decoded.asks.len(), 0);
    }
}
