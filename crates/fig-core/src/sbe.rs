//! SBE (Simple Binary Encoding) module for FIG protocol.
//!
//! SBE is a FIX standard for binary encoding with fixed-offset fields.
//! This module provides zero-alloc encoding/decoding for FIG trading messages
//! as an alternative to the CBOR codec.
//!
//! # Format
//!
//! Each SBE message consists of:
//! - **Message Header** (8 bytes): schema_id (u16), template_id (u16), version (u16), block_length (u16)
//! - **Fields**: encoded sequentially at offsets after the header
//! - **Primitives**: fixed-size big-endian encoding
//! - **Strings**: length-prefixed (u16 length + UTF-8 bytes)
//! - **Optional fields**: presence flag byte (0 or 1), followed by value if present

use crate::messages::{
    CancelRequest, ExecType, ExecutionReport, NewOrderSingle, OrdStatus, OrderType, Price,
    Quantity, SecurityIdSource, Side, TimeInForce,
};

// ─── Error Types ─────────────────────────────────────────────────

/// Errors that can occur during SBE encoding or decoding.
#[derive(Debug, thiserror::Error)]
pub enum SbeError {
    #[error("buffer too short: need {needed} bytes, have {have}")]
    BufferTooShort { needed: usize, have: usize },

    #[error("invalid template id: {0}")]
    InvalidTemplateId(u16),

    #[error("invalid schema id: {0}")]
    InvalidSchemaId(u16),

    #[error("invalid UTF-8 string")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("invalid enum value for {field}: {value}")]
    InvalidEnumValue { field: &'static str, value: u8 },
}

// ─── SBE Encoder ─────────────────────────────────────────────────

/// SBE message encoder.
///
/// Writes fields sequentially into an internal buffer.
/// The 8-byte SBE header is written on construction.
pub struct SbeEncoder {
    buf: Vec<u8>,
    offset: usize,
}

impl SbeEncoder {
    /// Create a new encoder with the given header values.
    pub fn new(template_id: u16, schema_id: u16, version: u16) -> Self {
        let mut buf = Vec::new();
        // Header: schema_id, template_id, version, block_length (placeholder=0)
        buf.extend_from_slice(&schema_id.to_be_bytes());
        buf.extend_from_slice(&template_id.to_be_bytes());
        buf.extend_from_slice(&version.to_be_bytes());
        buf.extend_from_slice(&0u16.to_be_bytes()); // block_length
        Self { buf, offset: 8 }
    }

    /// Write raw bytes to the buffer.
    fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
        self.offset += bytes.len();
    }

    /// Write a u16 (big-endian).
    fn write_u16(&mut self, v: u16) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write a u8.
    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
        self.offset += 1;
    }

    /// Write an i64 (big-endian).
    pub fn write_i64(&mut self, v: i64) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write a u64 (big-endian).
    pub fn write_u64(&mut self, v: u64) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write an i32 (big-endian).
    pub fn write_i32(&mut self, v: i32) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write a u32 (big-endian).
    pub fn write_u32(&mut self, v: u32) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write an f64 (big-endian).
    pub fn write_f64(&mut self, v: f64) {
        self.write_bytes(&v.to_be_bytes());
    }

    /// Write a length-prefixed string.
    /// Format: u16 length + UTF-8 bytes.
    pub fn write_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u16(bytes.len() as u16);
        self.write_bytes(bytes);
    }

    /// Finalize encoding and return the byte buffer.
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

// ─── SBE Decoder ─────────────────────────────────────────────────

/// SBE message decoder.
///
/// Reads fields sequentially from a byte slice.
/// The 8-byte SBE header is parsed on construction.
pub struct SbeDecoder<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> SbeDecoder<'a> {
    /// Create a new decoder from a byte buffer.
    ///
    /// Parses the 8-byte SBE header and returns the decoder along with
    /// the header fields: (decoder, template_id, schema_id, version).
    pub fn new(buf: &'a [u8]) -> Result<(Self, u16, u16, u16), SbeError> {
        if buf.len() < 8 {
            return Err(SbeError::BufferTooShort {
                needed: 8,
                have: buf.len(),
            });
        }

        let schema_id = u16::from_be_bytes([buf[0], buf[1]]);
        let template_id = u16::from_be_bytes([buf[2], buf[3]]);
        let version = u16::from_be_bytes([buf[4], buf[5]]);
        // block_length: u16::from_be_bytes([buf[6], buf[7]])

        let decoder = Self { buf, offset: 8 };
        Ok((decoder, template_id, schema_id, version))
    }

    /// Check that we have at least `needed` bytes remaining.
    #[allow(dead_code)]
    fn ensure_remaining(&self, needed: usize) -> Result<(), SbeError> {
        if self.buf.len() - self.offset < needed {
            return Err(SbeError::BufferTooShort {
                needed: needed + self.offset,
                have: self.buf.len(),
            });
        }
        Ok(())
    }

    /// Read a u16 (big-endian) and advance offset.
    fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes([self.buf[self.offset], self.buf[self.offset + 1]]);
        self.offset += 2;
        val
    }

    /// Read a u8 and advance offset.
    pub fn read_u8(&mut self) -> u8 {
        let val = self.buf[self.offset];
        self.offset += 1;
        val
    }

    /// Read an i64 (big-endian) and advance offset.
    pub fn read_i64(&mut self) -> i64 {
        let val = i64::from_be_bytes([
            self.buf[self.offset],
            self.buf[self.offset + 1],
            self.buf[self.offset + 2],
            self.buf[self.offset + 3],
            self.buf[self.offset + 4],
            self.buf[self.offset + 5],
            self.buf[self.offset + 6],
            self.buf[self.offset + 7],
        ]);
        self.offset += 8;
        val
    }

    /// Read a u64 (big-endian) and advance offset.
    pub fn read_u64(&mut self) -> u64 {
        let val = u64::from_be_bytes([
            self.buf[self.offset],
            self.buf[self.offset + 1],
            self.buf[self.offset + 2],
            self.buf[self.offset + 3],
            self.buf[self.offset + 4],
            self.buf[self.offset + 5],
            self.buf[self.offset + 6],
            self.buf[self.offset + 7],
        ]);
        self.offset += 8;
        val
    }

    /// Read an i32 (big-endian) and advance offset.
    pub fn read_i32(&mut self) -> i32 {
        let val = i32::from_be_bytes([
            self.buf[self.offset],
            self.buf[self.offset + 1],
            self.buf[self.offset + 2],
            self.buf[self.offset + 3],
        ]);
        self.offset += 4;
        val
    }

    /// Read a u32 (big-endian) and advance offset.
    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes([
            self.buf[self.offset],
            self.buf[self.offset + 1],
            self.buf[self.offset + 2],
            self.buf[self.offset + 3],
        ]);
        self.offset += 4;
        val
    }

    /// Read an f64 (big-endian) and advance offset.
    pub fn read_f64(&mut self) -> f64 {
        let val = f64::from_be_bytes([
            self.buf[self.offset],
            self.buf[self.offset + 1],
            self.buf[self.offset + 2],
            self.buf[self.offset + 3],
            self.buf[self.offset + 4],
            self.buf[self.offset + 5],
            self.buf[self.offset + 6],
            self.buf[self.offset + 7],
        ]);
        self.offset += 8;
        val
    }

    /// Read a length-prefixed string and advance offset.
    ///
    /// Returns a borrowed `&str` from the underlying buffer.
    /// Panics if the string data is not valid UTF-8.
    pub fn read_str(&mut self) -> &str {
        let len = self.read_u16() as usize;
        let bytes = &self.buf[self.offset..self.offset + len];
        self.offset += len;
        std::str::from_utf8(bytes).expect("invalid UTF-8 in SBE string field")
    }
}

// ─── Enum Mapping Helpers ────────────────────────────────────────

fn side_to_u8(side: &Side) -> u8 {
    match side {
        Side::Buy => 1,
        Side::Sell => 2,
        Side::SellShort => 3,
        Side::SellShortExempt => 4,
    }
}

fn side_from_u8(v: u8) -> Result<Side, SbeError> {
    match v {
        1 => Ok(Side::Buy),
        2 => Ok(Side::Sell),
        3 => Ok(Side::SellShort),
        4 => Ok(Side::SellShortExempt),
        _ => Err(SbeError::InvalidEnumValue {
            field: "side",
            value: v,
        }),
    }
}

fn order_type_to_u8(ot: &OrderType) -> u8 {
    match ot {
        OrderType::Market => 1,
        OrderType::Limit => 2,
        OrderType::Stop => 3,
        OrderType::StopLimit => 4,
        OrderType::MarketOnClose => 5,
        OrderType::LimitOnClose => 6,
        OrderType::Pegged => 7,
    }
}

fn order_type_from_u8(v: u8) -> Result<OrderType, SbeError> {
    match v {
        1 => Ok(OrderType::Market),
        2 => Ok(OrderType::Limit),
        3 => Ok(OrderType::Stop),
        4 => Ok(OrderType::StopLimit),
        5 => Ok(OrderType::MarketOnClose),
        6 => Ok(OrderType::LimitOnClose),
        7 => Ok(OrderType::Pegged),
        _ => Err(SbeError::InvalidEnumValue {
            field: "order_type",
            value: v,
        }),
    }
}

fn security_id_source_to_u8(source: &SecurityIdSource) -> u8 {
    match source {
        SecurityIdSource::Cusip => 1,
        SecurityIdSource::Sedol => 2,
        SecurityIdSource::Isin => 4,
        SecurityIdSource::Ric => 5,
        SecurityIdSource::ExchangeSymbol => 8,
    }
}

fn security_id_source_from_u8(v: u8) -> Result<SecurityIdSource, SbeError> {
    match v {
        1 => Ok(SecurityIdSource::Cusip),
        2 => Ok(SecurityIdSource::Sedol),
        4 => Ok(SecurityIdSource::Isin),
        5 => Ok(SecurityIdSource::Ric),
        8 => Ok(SecurityIdSource::ExchangeSymbol),
        _ => Err(SbeError::InvalidEnumValue {
            field: "id_source",
            value: v,
        }),
    }
}

fn time_in_force_to_u8(tif: &TimeInForce) -> u8 {
    match tif {
        TimeInForce::Day => 0,
        TimeInForce::Gtc => 1,
        TimeInForce::Gtd => 2,
        TimeInForce::Ioc => 3,
        TimeInForce::Fok => 4,
    }
}

fn time_in_force_from_u8(v: u8) -> Result<TimeInForce, SbeError> {
    match v {
        0 => Ok(TimeInForce::Day),
        1 => Ok(TimeInForce::Gtc),
        2 => Ok(TimeInForce::Gtd),
        3 => Ok(TimeInForce::Ioc),
        4 => Ok(TimeInForce::Fok),
        _ => Err(SbeError::InvalidEnumValue {
            field: "time_in_force",
            value: v,
        }),
    }
}

fn exec_type_to_u8(et: &ExecType) -> u8 {
    match et {
        ExecType::New => 0,
        ExecType::PartialFill => 1,
        ExecType::Fill => 2,
        ExecType::DoneForDay => 3,
        ExecType::Canceled => 4,
        ExecType::Replaced => 5,
        ExecType::PendingCancel => 6,
        ExecType::Stopped => 7,
        ExecType::Rejected => 8,
        ExecType::Suspended => 9,
        ExecType::PendingNew => 10,
        ExecType::Expired => 11,
    }
}

fn exec_type_from_u8(v: u8) -> Result<ExecType, SbeError> {
    match v {
        0 => Ok(ExecType::New),
        1 => Ok(ExecType::PartialFill),
        2 => Ok(ExecType::Fill),
        3 => Ok(ExecType::DoneForDay),
        4 => Ok(ExecType::Canceled),
        5 => Ok(ExecType::Replaced),
        6 => Ok(ExecType::PendingCancel),
        7 => Ok(ExecType::Stopped),
        8 => Ok(ExecType::Rejected),
        9 => Ok(ExecType::Suspended),
        10 => Ok(ExecType::PendingNew),
        11 => Ok(ExecType::Expired),
        _ => Err(SbeError::InvalidEnumValue {
            field: "exec_type",
            value: v,
        }),
    }
}

fn ord_status_to_u8(os: &OrdStatus) -> u8 {
    match os {
        OrdStatus::New => 0,
        OrdStatus::PartiallyFilled => 1,
        OrdStatus::Filled => 2,
        OrdStatus::DoneForDay => 3,
        OrdStatus::Canceled => 4,
        OrdStatus::PendingCancel => 5,
        OrdStatus::Stopped => 6,
        OrdStatus::Rejected => 7,
        OrdStatus::Suspended => 8,
        OrdStatus::PendingNew => 9,
        OrdStatus::Expired => 10,
        OrdStatus::Replaced => 11,
    }
}

fn ord_status_from_u8(v: u8) -> Result<OrdStatus, SbeError> {
    match v {
        0 => Ok(OrdStatus::New),
        1 => Ok(OrdStatus::PartiallyFilled),
        2 => Ok(OrdStatus::Filled),
        3 => Ok(OrdStatus::DoneForDay),
        4 => Ok(OrdStatus::Canceled),
        5 => Ok(OrdStatus::PendingCancel),
        6 => Ok(OrdStatus::Stopped),
        7 => Ok(OrdStatus::Rejected),
        8 => Ok(OrdStatus::Suspended),
        9 => Ok(OrdStatus::PendingNew),
        10 => Ok(OrdStatus::Expired),
        11 => Ok(OrdStatus::Replaced),
        _ => Err(SbeError::InvalidEnumValue {
            field: "ord_status",
            value: v,
        }),
    }
}

// ─── Message Encoders ────────────────────────────────────────────

/// Encode a NewOrderSingle into SBE binary format.
///
/// Template ID: 1, Schema ID: 1, Version: 1
pub fn encode_new_order_single(order: &NewOrderSingle) -> Vec<u8> {
    let mut enc = SbeEncoder::new(1, 1, 1);

    // Fixed fields
    enc.write_str(&order.cl_ord_id);
    enc.write_u8(side_to_u8(&order.side));
    enc.write_f64(order.order_qty.0);

    let price = order.price.as_ref().map(|p| p.0).unwrap_or(0.0);
    enc.write_f64(price);
    enc.write_u8(if order.price.is_some() { 1 } else { 0 });

    enc.write_str(&order.symbol);
    enc.write_u8(order_type_to_u8(&order.order_type));
    enc.write_u8(time_in_force_to_u8(&order.time_in_force));

    // Optional: account
    enc.write_u8(if order.account.is_some() { 1 } else { 0 });
    if let Some(ref account) = order.account {
        enc.write_str(account);
    }

    // Optional: expire_time
    enc.write_u8(if order.expire_time.is_some() { 1 } else { 0 });
    if let Some(expire_time) = order.expire_time {
        enc.write_i64(expire_time);
    }

    // Optional: strategy_id
    enc.write_u8(if order.strategy_id.is_some() { 1 } else { 0 });
    if let Some(ref strategy_id) = order.strategy_id {
        enc.write_str(strategy_id);
    }

    // Optional: stop_price
    enc.write_u8(if order.stop_price.is_some() { 1 } else { 0 });
    if let Some(ref stop_price) = order.stop_price {
        enc.write_f64(stop_price.0);
    }

    enc.write_u8(if order.security_id.is_some() { 1 } else { 0 });
    if let Some(ref security_id) = order.security_id {
        enc.write_str(security_id);
    }
    enc.write_u8(if order.id_source.is_some() { 1 } else { 0 });
    if let Some(ref id_source) = order.id_source {
        enc.write_u8(security_id_source_to_u8(id_source));
    }
    enc.write_u8(if order.security_exchange.is_some() {
        1
    } else {
        0
    });
    if let Some(ref security_exchange) = order.security_exchange {
        enc.write_str(security_exchange);
    }

    enc.finish()
}

/// Encode an ExecutionReport into SBE binary format.
///
/// Template ID: 2, Schema ID: 1, Version: 1
pub fn encode_execution_report(report: &ExecutionReport) -> Vec<u8> {
    let mut enc = SbeEncoder::new(2, 1, 1);

    enc.write_str(&report.cl_ord_id);
    enc.write_str(&report.order_id);
    enc.write_str(&report.exec_id);
    enc.write_u8(exec_type_to_u8(&report.exec_type));
    enc.write_u8(ord_status_to_u8(&report.ord_status));
    enc.write_u8(side_to_u8(&report.side));

    // Optional: last_qty
    enc.write_u8(if report.last_qty.is_some() { 1 } else { 0 });
    if let Some(ref qty) = report.last_qty {
        enc.write_f64(qty.0);
    }

    // Optional: last_price
    enc.write_u8(if report.last_price.is_some() { 1 } else { 0 });
    if let Some(ref price) = report.last_price {
        enc.write_f64(price.0);
    }

    enc.write_f64(report.leaves_qty.0);
    enc.write_f64(report.cum_qty.0);
    enc.write_f64(report.avg_price.0);
    enc.write_str(&report.symbol);
    enc.write_i64(report.transact_time);

    enc.finish()
}

/// Encode a CancelRequest into SBE binary format.
///
/// Template ID: 3, Schema ID: 1, Version: 1
pub fn encode_cancel_request(cancel: &CancelRequest) -> Vec<u8> {
    let mut enc = SbeEncoder::new(3, 1, 1);

    enc.write_str(&cancel.cl_ord_id);
    enc.write_str(&cancel.orig_cl_ord_id);
    enc.write_str(&cancel.symbol);
    enc.write_u8(side_to_u8(&cancel.side));

    enc.finish()
}

// ─── Message Decoders ────────────────────────────────────────────

/// Decode a NewOrderSingle from SBE binary format.
pub fn decode_new_order_single(buf: &[u8]) -> Result<NewOrderSingle, SbeError> {
    let (mut dec, template_id, schema_id, _version) = SbeDecoder::new(buf)?;
    if template_id != 1 {
        return Err(SbeError::InvalidTemplateId(template_id));
    }
    if schema_id != 1 {
        return Err(SbeError::InvalidSchemaId(schema_id));
    }

    let cl_ord_id = dec.read_str().to_string();
    let side = side_from_u8(dec.read_u8())?;
    let order_qty = Quantity(dec.read_f64());

    let price_val = dec.read_f64();
    let has_price = dec.read_u8();
    let price = if has_price == 1 {
        Some(Price(price_val))
    } else {
        None
    };

    let symbol = dec.read_str().to_string();
    let order_type = order_type_from_u8(dec.read_u8())?;
    let time_in_force = time_in_force_from_u8(dec.read_u8())?;

    let has_account = dec.read_u8();
    let account = if has_account == 1 {
        Some(dec.read_str().to_string())
    } else {
        None
    };

    let has_expire_time = dec.read_u8();
    let expire_time = if has_expire_time == 1 {
        Some(dec.read_i64())
    } else {
        None
    };

    let has_strategy_id = dec.read_u8();
    let strategy_id = if has_strategy_id == 1 {
        Some(dec.read_str().to_string())
    } else {
        None
    };

    let has_stop_price = dec.read_u8();
    let stop_price = if has_stop_price == 1 {
        Some(Price(dec.read_f64()))
    } else {
        None
    };

    let has_security_id = dec.read_u8();
    let security_id = if has_security_id == 1 {
        Some(dec.read_str().to_string())
    } else {
        None
    };
    let has_id_source = dec.read_u8();
    let id_source = if has_id_source == 1 {
        Some(security_id_source_from_u8(dec.read_u8())?)
    } else {
        None
    };
    let has_security_exchange = dec.read_u8();
    let security_exchange = if has_security_exchange == 1 {
        Some(dec.read_str().to_string())
    } else {
        None
    };

    Ok(NewOrderSingle {
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

/// Decode an ExecutionReport from SBE binary format.
pub fn decode_execution_report(buf: &[u8]) -> Result<ExecutionReport, SbeError> {
    let (mut dec, template_id, schema_id, _version) = SbeDecoder::new(buf)?;
    if template_id != 2 {
        return Err(SbeError::InvalidTemplateId(template_id));
    }
    if schema_id != 1 {
        return Err(SbeError::InvalidSchemaId(schema_id));
    }

    let cl_ord_id = dec.read_str().to_string();
    let order_id = dec.read_str().to_string();
    let exec_id = dec.read_str().to_string();
    let exec_type = exec_type_from_u8(dec.read_u8())?;
    let ord_status = ord_status_from_u8(dec.read_u8())?;
    let side = side_from_u8(dec.read_u8())?;

    let has_last_qty = dec.read_u8();
    let last_qty = if has_last_qty == 1 {
        Some(Quantity(dec.read_f64()))
    } else {
        None
    };

    let has_last_price = dec.read_u8();
    let last_price = if has_last_price == 1 {
        Some(Price(dec.read_f64()))
    } else {
        None
    };

    let leaves_qty = Quantity(dec.read_f64());
    let cum_qty = Quantity(dec.read_f64());
    let avg_price = Price(dec.read_f64());
    let symbol = dec.read_str().to_string();
    let transact_time = dec.read_i64();

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

/// Decode a CancelRequest from SBE binary format.
pub fn decode_cancel_request(buf: &[u8]) -> Result<CancelRequest, SbeError> {
    let (mut dec, template_id, schema_id, _version) = SbeDecoder::new(buf)?;
    if template_id != 3 {
        return Err(SbeError::InvalidTemplateId(template_id));
    }
    if schema_id != 1 {
        return Err(SbeError::InvalidSchemaId(schema_id));
    }

    let cl_ord_id = dec.read_str().to_string();
    let orig_cl_ord_id = dec.read_str().to_string();
    let symbol = dec.read_str().to_string();
    let side = side_from_u8(dec.read_u8())?;

    Ok(CancelRequest {
        cl_ord_id,
        orig_cl_ord_id,
        symbol,
        side,
        order_qty: None,
    })
}

// ─── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Encoder Basic Types ─────────────────────────────────────

    #[test]
    fn test_encoder_basic_types() {
        let mut enc = SbeEncoder::new(42, 7, 3);

        enc.write_u8(0xFF);
        enc.write_u16(0x1234);
        enc.write_u32(0xDEAD_BEEF);
        enc.write_u64(0x0123_4567_89AB_CDEF);
        enc.write_i32(-42);
        enc.write_i64(-999_999_999_999i64);
        enc.write_f64(std::f64::consts::PI);

        let buf = enc.finish();

        // Header is 8 bytes: schema_id=7, template_id=42, version=3, block_length=0
        assert_eq!(&buf[0..2], &[0x00, 0x07]); // schema_id
        assert_eq!(&buf[2..4], &[0x00, 0x2A]); // template_id (42)
        assert_eq!(&buf[4..6], &[0x00, 0x03]); // version
        assert_eq!(&buf[6..8], &[0x00, 0x00]); // block_length

        // u8 at offset 8
        assert_eq!(buf[8], 0xFF);

        // u16 at offset 9-10
        assert_eq!(u16::from_be_bytes([buf[9], buf[10]]), 0x1234);

        // u32 at offset 11-14
        assert_eq!(
            u32::from_be_bytes([buf[11], buf[12], buf[13], buf[14]]),
            0xDEAD_BEEF
        );

        // u64 at offset 15-22
        assert_eq!(
            u64::from_be_bytes([
                buf[15], buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22]
            ]),
            0x0123_4567_89AB_CDEF
        );

        // i32 at offset 23-26
        assert_eq!(
            i32::from_be_bytes([buf[23], buf[24], buf[25], buf[26]]),
            -42
        );

        // i64 at offset 27-34
        assert_eq!(
            i64::from_be_bytes([
                buf[27], buf[28], buf[29], buf[30], buf[31], buf[32], buf[33], buf[34]
            ]),
            -999_999_999_999i64
        );

        // f64 at offset 35-42
        let pi_roundtrip = f64::from_be_bytes([
            buf[35], buf[36], buf[37], buf[38], buf[39], buf[40], buf[41], buf[42],
        ]);
        assert!((pi_roundtrip - std::f64::consts::PI).abs() < 1e-15);

        assert_eq!(buf.len(), 43);
    }

    // ── Decoder Basic Types ─────────────────────────────────────

    #[test]
    fn test_decoder_basic_types() {
        let mut enc = SbeEncoder::new(10, 20, 5);
        enc.write_i64(12345);
        enc.write_u64(67890);
        enc.write_i32(-100);
        enc.write_u32(0xABCD);
        enc.write_f64(2.718281828);
        enc.write_u8(42);
        enc.write_str("hello");

        let buf = enc.finish();

        let (mut dec, template_id, schema_id, version) = SbeDecoder::new(&buf).unwrap();
        assert_eq!(template_id, 10);
        assert_eq!(schema_id, 20);
        assert_eq!(version, 5);

        assert_eq!(dec.read_i64(), 12345);
        assert_eq!(dec.read_u64(), 67890);
        assert_eq!(dec.read_i32(), -100);
        assert_eq!(dec.read_u32(), 0xABCD);
        assert!((dec.read_f64() - 2.718281828).abs() < 1e-15);
        assert_eq!(dec.read_u8(), 42);
        assert_eq!(dec.read_str(), "hello");
    }

    // ── NewOrderSingle Round-Trip ───────────────────────────────

    #[test]
    fn test_new_order_single_round_trip() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-001".to_string(),
            side: Side::Buy,
            order_qty: Quantity(100.0),
            price: Some(Price(50.25)),
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
        };

        let encoded = encode_new_order_single(&order);
        let decoded = decode_new_order_single(&encoded).unwrap();

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

    // ── ExecutionReport Round-Trip ──────────────────────────────

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

        let encoded = encode_execution_report(&report);
        let decoded = decode_execution_report(&encoded).unwrap();

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

    // ── CancelRequest Round-Trip ────────────────────────────────

    #[test]
    fn test_cancel_request_round_trip() {
        let cancel = CancelRequest {
            cl_ord_id: "ORD-002".to_string(),
            orig_cl_ord_id: "ORD-001".to_string(),
            symbol: "MSFT".to_string(),
            side: Side::Sell,
            order_qty: None,
        };

        let encoded = encode_cancel_request(&cancel);
        let decoded = decode_cancel_request(&encoded).unwrap();

        assert_eq!(decoded.cl_ord_id, "ORD-002");
        assert_eq!(decoded.orig_cl_ord_id, "ORD-001");
        assert_eq!(decoded.symbol, "MSFT");
        assert_eq!(decoded.side, Side::Sell);
    }

    // ── Optional Fields ─────────────────────────────────────────

    #[test]
    fn test_optional_fields() {
        // Order with no optional fields
        let order_no_optional = NewOrderSingle {
            cl_ord_id: "ORD-MIN".to_string(),
            side: Side::Buy,
            order_qty: Quantity(50.0),
            price: None,
            stop_price: None,
            symbol: "TEST".to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Ioc,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
        };

        let encoded = encode_new_order_single(&order_no_optional);
        let decoded = decode_new_order_single(&encoded).unwrap();

        assert_eq!(decoded.price, None);
        assert_eq!(decoded.account, None);
        assert_eq!(decoded.expire_time, None);
        assert_eq!(decoded.strategy_id, None);

        // Order with all optional fields
        let order_all_optional = NewOrderSingle {
            cl_ord_id: "ORD-FULL".to_string(),
            side: Side::SellShort,
            order_qty: Quantity(200.0),
            price: Some(Price(99.99)),
            stop_price: None,
            symbol: "GOOG".to_string(),
            order_type: OrderType::StopLimit,
            time_in_force: TimeInForce::Gtd,
            expire_time: Some(1700000000000000000),
            account: Some("ACCT-999".to_string()),
            strategy_id: Some("STRAT-001".to_string()),
            security_id: Some("037833100".to_string()),
            id_source: Some(SecurityIdSource::Cusip),
            security_exchange: Some("XNAS".to_string()),
        };

        let encoded = encode_new_order_single(&order_all_optional);
        let decoded = decode_new_order_single(&encoded).unwrap();

        assert_eq!(decoded.price, Some(Price(99.99)));
        assert_eq!(decoded.order_type, OrderType::StopLimit);
        assert_eq!(decoded.time_in_force, TimeInForce::Gtd);
        assert_eq!(decoded.expire_time, Some(1700000000000000000));
        assert_eq!(decoded.account.as_deref(), Some("ACCT-999"));
        assert_eq!(decoded.strategy_id.as_deref(), Some("STRAT-001"));
    }

    // ── String Encoding ─────────────────────────────────────────

    #[test]
    fn test_string_encoding() {
        let mut enc = SbeEncoder::new(1, 1, 1);
        enc.write_str("hello world");
        enc.write_str("");
        enc.write_str("🦀 Rust is fast!"); // multi-byte UTF-8

        let buf = enc.finish();
        let (mut dec, _, _, _) = SbeDecoder::new(&buf).unwrap();

        assert_eq!(dec.read_str(), "hello world");
        assert_eq!(dec.read_str(), "");
        assert_eq!(dec.read_str(), "🦀 Rust is fast!");
    }

    // ── Header Validation ───────────────────────────────────────

    #[test]
    fn test_header_validation() {
        // Valid header
        let mut enc = SbeEncoder::new(5, 3, 1);
        enc.write_u8(0);
        let buf = enc.finish();

        let result = SbeDecoder::new(&buf);
        assert!(result.is_ok());
        let (_, template_id, schema_id, version) = result.unwrap();
        assert_eq!(template_id, 5);
        assert_eq!(schema_id, 3);
        assert_eq!(version, 1);

        // Buffer too short
        let short_buf = vec![0u8; 3];
        let result = SbeDecoder::new(&short_buf);
        assert!(matches!(
            result,
            Err(SbeError::BufferTooShort { needed: 8, have: 3 })
        ));
    }

    // ── Enum Encoding / Decoding ────────────────────────────────

    #[test]
    fn test_enum_encoding() {
        // All Side variants
        let sides = vec![
            (Side::Buy, 1u8),
            (Side::Sell, 2),
            (Side::SellShort, 3),
            (Side::SellShortExempt, 4),
        ];
        for (side, expected) in sides {
            assert_eq!(side_to_u8(&side), expected);
            assert_eq!(side_from_u8(expected).unwrap(), side);
        }

        // All OrderType variants
        let order_types = vec![
            (OrderType::Market, 1u8),
            (OrderType::Limit, 2),
            (OrderType::Stop, 3),
            (OrderType::StopLimit, 4),
        ];
        for (ot, expected) in order_types {
            assert_eq!(order_type_to_u8(&ot), expected);
            assert_eq!(order_type_from_u8(expected).unwrap(), ot);
        }

        // All TimeInForce variants
        let tifs = vec![
            (TimeInForce::Day, 0u8),
            (TimeInForce::Gtc, 1),
            (TimeInForce::Gtd, 2),
            (TimeInForce::Ioc, 3),
            (TimeInForce::Fok, 4),
        ];
        for (tif, expected) in tifs {
            assert_eq!(time_in_force_to_u8(&tif), expected);
            assert_eq!(time_in_force_from_u8(expected).unwrap(), tif);
        }

        // Invalid enum values
        assert!(side_from_u8(0).is_err());
        assert!(side_from_u8(5).is_err());
        assert!(order_type_from_u8(0).is_err());
        assert!(time_in_force_from_u8(5).is_err());
    }

    // ── NewOrderSingle with no price ────────────────────────────

    #[test]
    fn test_new_order_single_no_price() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-NP".to_string(),
            side: Side::Sell,
            order_qty: Quantity(1000.0),
            price: None, // No price (market order)
            stop_price: None,
            symbol: "TSLA".to_string(),
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Ioc,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
        };

        let encoded = encode_new_order_single(&order);
        let decoded = decode_new_order_single(&encoded).unwrap();

        assert_eq!(decoded.price, None);
        assert_eq!(decoded.order_type, OrderType::Market);
    }
}
