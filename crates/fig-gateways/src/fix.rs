//! FIX 4.4 protocol adapter.
//!
//! Parses and serializes FIX messages (tag=value pairs separated by SOH `0x01`,
//! terminated by a checksum at tag 10), and maps between FIX and FIG trading
//! message types.
//!
//! # FIX message format
//!
//! ```text
//! 8=FIX.4.4\x019=...\x0135=D\x01...10=XXX\x01
//! ```
//!
//! The checksum (tag 10) is the sum of all bytes preceding it, modulo 256,
//! formatted as a zero-padded 3-digit string.

use thiserror::Error;

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    CancelReject, CancelRejectReason, CancelReplaceRequest, CancelRequest, ExecType,
    ExecutionReport, NewOrderSingle, OrdStatus, OrderType, Price, Quantity, Side, TimeInForce,
};

/// SOH separator character (ASCII 0x01).
const SOH: u8 = 0x01;

// ─── FixMessage ──────────────────────────────────────────────────

/// A parsed FIX message containing tag-value pairs.
///
/// This wraps the output of [`parse_fix_message`] and provides
/// convenience accessors for common operations. It can also be
/// serialized back to wire format via [`FixMessage::to_bytes`].
#[derive(Debug, Clone, PartialEq)]
pub struct FixMessage {
    /// Parsed tag-value pairs (tag, value).
    pub tags: Vec<(u32, String)>,
}

impl FixMessage {
    /// Create a `FixMessage` from pre-parsed tag-value pairs.
    pub fn new(tags: Vec<(u32, String)>) -> Self {
        Self { tags }
    }

    /// Parse a `FixMessage` from raw FIX wire-format bytes.
    pub fn from_bytes(bytes: &[u8]) -> FixResult<Self> {
        let tags = parse_fix_message(bytes)?;
        Ok(Self { tags })
    }

    /// Serialize this message to FIX wire-format bytes (including checksum).
    pub fn to_bytes(&self) -> Vec<u8> {
        serialize_fix_message(&self.tags)
    }

    /// Get the value for a specific tag, if present.
    pub fn get_tag(&self, tag: u32) -> Option<&str> {
        self.tags
            .iter()
            .find(|(t, _)| *t == tag)
            .map(|(_, v)| v.as_str())
    }

    /// Get the MsgType (tag 35) for this message.
    pub fn msg_type(&self) -> Option<&str> {
        self.get_tag(35)
    }
}

// ─── Error ───────────────────────────────────────────────────────

/// Errors that can occur when parsing or converting FIX messages.
#[derive(Error, Debug)]
pub enum FixError {
    #[error("empty FIX message")]
    EmptyMessage,

    #[error("invalid FIX message: missing BeginString (tag 8)")]
    MissingBeginString,

    #[error("invalid FIX message: missing MsgType (tag 35)")]
    MissingMsgType,

    #[error("invalid FIX message: missing checksum (tag 10)")]
    MissingChecksum,

    #[error("checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: u8, actual: u8 },

    #[error("invalid tag-value pair at position {pos}: {reason}")]
    InvalidTagValue { pos: usize, reason: String },

    #[error("missing required tag {tag} for {msg_type}")]
    MissingTag { tag: u32, msg_type: String },

    #[error("invalid value for tag {tag}: {value}")]
    InvalidTagValueData { tag: u32, value: String },

    #[error("unsupported MsgType: {0}")]
    UnsupportedMsgType(String),

    #[error("unknown FIX side: {0}")]
    UnknownSide(String),

    #[error("unknown FIX order type: {0}")]
    UnknownOrderType(String),

    #[error("unknown FIX time in force: {0}")]
    UnknownTimeInForce(String),

    #[error("unknown FIX exec type: {0}")]
    UnknownExecType(String),

    #[error("unknown FIX ord status: {0}")]
    UnknownOrdStatus(String),
}

/// Convenience type alias for FIX operations.
pub type FixResult<T> = Result<T, FixError>;

/// Errors that can occur when converting between FIX messages and FIG frames.
#[derive(Error, Debug)]
pub enum FixConvertError {
    #[error("missing required FIX tag {tag} in logon message")]
    MissingTag { tag: u32 },

    #[error("missing AUTH_TOKEN extension in frame")]
    MissingAuthToken,

    #[error("unsupported FIX MsgType for conversion: {0}")]
    UnsupportedMsgType(String),

    #[error("invalid sequence number in tag {tag}: {value}")]
    InvalidSeqNumber { tag: u32, value: String },
}

/// Convenience type alias for FIX↔FIG conversion operations.
pub type FixConvertResult<T> = Result<T, FixConvertError>;

/// Session header fields required on outbound FIX application messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixOutboundContext {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub msg_seq_num: u32,
}

impl Default for FixOutboundContext {
    fn default() -> Self {
        Self {
            sender_comp_id: "FIG".to_string(),
            target_comp_id: "CLIENT".to_string(),
            msg_seq_num: 1,
        }
    }
}

/// Response target for a FIX OrderCancelReject (tag 434).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixCxlRejResponseTo {
    OrderCancelRequest = 1,
    OrderCancelReplaceRequest = 2,
}

/// Split a byte buffer into complete FIX messages (each ending with `10=NNN\x01`).
pub fn split_fix_messages(input: &[u8]) -> Vec<Vec<u8>> {
    let mut messages = Vec::new();
    let mut start = 0usize;
    let mut i = 0usize;
    while i + 4 <= input.len() {
        if input[i..].starts_with(b"10=") {
            let end = (i + 4..input.len()).find(|&j| input[j] == SOH).map(|j| j + 1);
            if let Some(end) = end {
                messages.push(input[start..end].to_vec());
                start = end;
                i = end;
                continue;
            }
        }
        i += 1;
    }
    messages
}

/// Format a nanosecond epoch timestamp as FIX UTCTimestamp (`YYYYMMDD-HH:MM:SS.sss`).
pub fn format_fix_utc_timestamp(nanos: i64) -> String {
    let secs = nanos / 1_000_000_000;
    let millis = ((nanos % 1_000_000_000).abs() / 1_000_000) as u32;
    let datetime = chrono_from_epoch_secs(secs);
    format!(
        "{:04}{:02}{:02}-{:02}:{:02}:{:02}.{:03}",
        datetime.year,
        datetime.month,
        datetime.day,
        datetime.hour,
        datetime.minute,
        datetime.second,
        millis
    )
}

/// Parse FIX UTCTimestamp (tags 60, 432) into nanoseconds since Unix epoch.
pub fn parse_fix_utc_timestamp(value: &str) -> FixResult<i64> {
    if value.len() < 17 {
        return Err(FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        });
    }
    let year: i32 = value[0..4]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let month: u32 = value[4..6]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let day: u32 = value[6..8]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let hour: u32 = value[9..11]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let minute: u32 = value[12..14]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let second: u32 = value[15..17]
        .parse()
        .map_err(|_| FixError::InvalidTagValueData {
            tag: 60,
            value: value.to_string(),
        })?;
    let millis: u32 = if value.len() >= 21 && value.as_bytes().get(17) == Some(&b'.') {
        value[18..21]
            .parse()
            .map_err(|_| FixError::InvalidTagValueData {
                tag: 60,
                value: value.to_string(),
            })?
    } else {
        0
    };

    let secs = epoch_secs_from_utc(year, month, day, hour, minute, second);
    Ok(secs * 1_000_000_000 + millis as i64 * 1_000_000)
}

struct UtcDateTime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

fn days_from_civil(year: i32, month: u32, day: u32) -> i32 {
    let y = year - i32::from(month <= 2);
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if month <= 2 {
        month as i32 + 9
    } else {
        month as i32 - 3
    }) + 2)
        / 5
        + i32::try_from(day).unwrap_or(0)
        - 1
        + yoe * 365
        + yoe / 4
        - yoe / 100;
    era * 146097 + doy - 719468
}

fn civil_from_days(z: i32) -> (i32, u32, u32) {
    let z = z + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let y = y + if m <= 2 { 1 } else { 0 };
    (y, m as u32, d as u32)
}

fn chrono_from_epoch_secs(secs: i64) -> UtcDateTime {
    let days = (secs / 86400) as i32;
    let rem = ((secs % 86400) + 86400) % 86400;
    let (year, month, day) = civil_from_days(days);
    UtcDateTime {
        year,
        month,
        day: day as u32,
        hour: (rem / 3600) as u32,
        minute: ((rem % 3600) / 60) as u32,
        second: (rem % 60) as u32,
    }
}

fn epoch_secs_from_utc(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> i64 {
    i64::from(days_from_civil(year, month, day)) * 86400
        + i64::from(hour) * 3600
        + i64::from(minute) * 60
        + i64::from(second)
}

fn build_outbound_fix_message(
    ctx: &FixOutboundContext,
    msg_type: &str,
    body_tags: Vec<(u32, String)>,
) -> Vec<u8> {
    let sending_time = format_fix_utc_timestamp(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64,
    );

    let mut tags: Vec<(u32, String)> = vec![
        (8, "FIX.4.4".to_string()),
        (9, "0".to_string()),
        (35, msg_type.to_string()),
        (49, ctx.sender_comp_id.clone()),
        (56, ctx.target_comp_id.clone()),
        (34, ctx.msg_seq_num.to_string()),
        (52, sending_time),
    ];
    tags.extend(body_tags);

    let body_tags: Vec<&(u32, String)> = tags
        .iter()
        .filter(|(t, _)| *t != 8 && *t != 9 && *t != 10)
        .collect();

    let mut body_buf = Vec::new();
    for (tag, value) in &body_tags {
        body_buf.extend_from_slice(format!("{}={}", tag, value).as_bytes());
        body_buf.push(SOH);
    }
    let body_length = body_buf.len();

    let mut final_buf = Vec::new();
    final_buf.extend_from_slice(b"8=FIX.4.4\x01");
    final_buf.extend_from_slice(format!("9={}", body_length).as_bytes());
    final_buf.push(SOH);
    final_buf.extend_from_slice(&body_buf);

    let checksum = compute_checksum(&final_buf);
    final_buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
    final_buf.push(SOH);

    final_buf
}

// ─── Parsing ─────────────────────────────────────────────────────

/// Parse a raw FIX message into a list of `(tag, value)` pairs.
///
/// FIX messages consist of `tag=value` pairs separated by the SOH byte
/// (0x01). The last pair is the checksum at tag 10.
///
/// # Errors
///
/// Returns `FixError` if the message is empty, has no checksum, or
/// contains invalid tag-value pairs.
pub fn parse_fix_message(input: &[u8]) -> FixResult<Vec<(u32, String)>> {
    if input.is_empty() {
        return Err(FixError::EmptyMessage);
    }

    // Strip trailing SOH if present
    let data = if input.last() == Some(&SOH) {
        &input[..input.len() - 1]
    } else {
        input
    };

    let body = data.to_vec();

    // Split on SOH
    let fields: Vec<&[u8]> = body.split(|b| *b == SOH).collect();

    // Verify checksum before parsing.
    // The checksum is computed over all bytes in the message up to but NOT
    // including the "10=XXX" field (the SOH before 10= is included).
    if let Some(last_field) = fields.last() {
        if !last_field.starts_with(b"10=") {
            return Err(FixError::MissingChecksum);
        }
        // Find the position of "10=" in the body to compute checksum over
        // everything before it.
        let checksum_pos = body
            .windows(3)
            .rposition(|w| w == b"10=")
            .unwrap_or(body.len());
        let expected = compute_checksum(&body[..checksum_pos]);

        let checksum_str =
            std::str::from_utf8(&last_field[3..]).map_err(|_| FixError::InvalidTagValue {
                pos: body.len() - last_field.len(),
                reason: "checksum not valid UTF-8".to_string(),
            })?;
        let actual: u8 = checksum_str
            .parse()
            .map_err(|_| FixError::InvalidTagValue {
                pos: body.len() - last_field.len(),
                reason: format!("invalid checksum value: {}", checksum_str),
            })?;
        if expected != actual {
            return Err(FixError::ChecksumMismatch { expected, actual });
        }
    } else {
        return Err(FixError::MissingChecksum);
    }

    // Parse all tag=value pairs
    let mut pairs = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let pos = fields[..i].iter().map(|f| f.len() + 1).sum::<usize>();

        let field_str = std::str::from_utf8(field).map_err(|_| FixError::InvalidTagValue {
            pos,
            reason: "field not valid UTF-8".to_string(),
        })?;

        let eq_pos = field_str
            .find('=')
            .ok_or_else(|| FixError::InvalidTagValue {
                pos,
                reason: "missing '=' in tag-value pair".to_string(),
            })?;

        let tag: u32 = field_str[..eq_pos]
            .parse()
            .map_err(|_| FixError::InvalidTagValue {
                pos,
                reason: format!("invalid tag number: {}", &field_str[..eq_pos]),
            })?;

        let value = field_str[eq_pos + 1..].to_string();

        pairs.push((tag, value));
    }

    Ok(pairs)
}

/// Find the value for a specific tag in parsed FIX tag-value pairs.
fn find_tag(tags: &[(u32, String)], tag: u32) -> Option<&str> {
    tags.iter()
        .find(|(t, _)| *t == tag)
        .map(|(_, v)| v.as_str())
}

/// Require the value for a specific tag, returning an error if not found.
fn require_tag<'a>(tags: &'a [(u32, String)], tag: u32, msg_type: &str) -> FixResult<&'a str> {
    find_tag(tags, tag).ok_or(FixError::MissingTag {
        tag,
        msg_type: msg_type.to_string(),
    })
}

// ─── Serialization ────────────────────────────────────────────────

/// Serialize a list of `(tag, value)` pairs into a FIX wire-format message.
///
/// The returned bytes use SOH (`0x01`) as the field separator and include
/// the checksum (tag 10) as the final field.
pub fn serialize_fix_message(tags: &[(u32, String)]) -> Vec<u8> {
    let mut buf = Vec::new();

    for (tag, value) in tags {
        buf.extend_from_slice(format!("{}={}", tag, value).as_bytes());
        buf.push(SOH);
    }

    // Compute checksum and append
    let checksum = compute_checksum(&buf);
    buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
    buf.push(SOH);

    buf
}

// ─── Checksum ─────────────────────────────────────────────────────

/// Compute a FIX checksum: sum of all bytes modulo 256.
fn compute_checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |sum, b| sum.wrapping_add(*b))
}

// ─── FIX → FIG Conversion ────────────────────────────────────────

/// Convert a FIX NewOrderSingle (35=D) to a FIG `NewOrderSingle`.
///
/// # Tag mappings
///
/// | FIX Tag | Field        | FIG Field    |
/// |---------|-------------|-------------|
/// | 11      | ClOrdID      | cl_ord_id   |
/// | 54      | Side         | side        |
/// | 38      | OrderQty     | order_qty   |
/// | 44      | Price        | price       |
/// | 99      | StopPx       | stop_price  |
/// | 55      | Symbol       | symbol      |
/// | 40      | OrdType      | order_type  |
/// | 59      | TimeInForce  | time_in_force|
/// | 432     | ExpireTime   | expire_time |
/// | 1       | Account      | account     |
pub fn fix_to_fig_order(tags: &[(u32, String)]) -> FixResult<NewOrderSingle> {
    // Validate MsgType
    let msg_type = require_tag(tags, 35, "NewOrderSingle")?;
    if msg_type != "D" {
        return Err(FixError::UnsupportedMsgType(msg_type.to_string()));
    }

    let cl_ord_id = require_tag(tags, 11, "D")?.to_string();
    let symbol = require_tag(tags, 55, "D")?.to_string();

    let side = parse_fix_side(require_tag(tags, 54, "D")?)?;
    let order_qty = Quantity(require_tag(tags, 38, "D")?.parse::<f64>().map_err(|_| {
        FixError::InvalidTagValueData {
            tag: 38,
            value: find_tag(tags, 38).unwrap_or("").to_string(),
        }
    })?);

    let price = find_tag(tags, 44)
        .map(|v| {
            v.parse::<f64>()
                .map(Price)
                .map_err(|_| FixError::InvalidTagValueData {
                    tag: 44,
                    value: v.to_string(),
                })
        })
        .transpose()?;

    let stop_price = find_tag(tags, 99)
        .map(|v| {
            v.parse::<f64>()
                .map(Price)
                .map_err(|_| FixError::InvalidTagValueData {
                    tag: 99,
                    value: v.to_string(),
                })
        })
        .transpose()?;

    let order_type = parse_fix_order_type(require_tag(tags, 40, "D")?)?;
    let time_in_force = parse_fix_time_in_force(find_tag(tags, 59).unwrap_or("0"))?;

    let expire_time = find_tag(tags, 432)
        .map(parse_fix_utc_timestamp)
        .transpose()?;

    let account = find_tag(tags, 1).map(|s| s.to_string());

    match order_type {
        OrderType::Stop | OrderType::StopLimit if stop_price.is_none() => {
            return Err(FixError::MissingTag {
                tag: 99,
                msg_type: "D".to_string(),
            });
        }
        OrderType::Limit | OrderType::StopLimit if price.is_none() => {
            return Err(FixError::MissingTag {
                tag: 44,
                msg_type: "D".to_string(),
            });
        }
        _ => {}
    }

    if time_in_force == TimeInForce::Gtd && expire_time.is_none() {
        return Err(FixError::MissingTag {
            tag: 432,
            msg_type: "D".to_string(),
        });
    }

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
        strategy_id: None,
    })
}

/// Convert a FIX OrderCancelRequest (35=F) to a FIG `CancelRequest`.
///
/// # Tag mappings
///
/// | FIX Tag | Field            | FIG Field       |
/// |---------|-----------------|----------------|
/// | 11      | ClOrdID         | cl_ord_id      |
/// | 41      | OrigClOrdID     | orig_cl_ord_id |
/// | 55      | Symbol          | symbol         |
/// | 54      | Side            | side           |
/// | 38      | OrderQty        | order_qty (opt)|
pub fn fix_to_fig_cancel(tags: &[(u32, String)]) -> FixResult<CancelRequest> {
    let msg_type = require_tag(tags, 35, "CancelRequest")?;
    if msg_type != "F" {
        return Err(FixError::UnsupportedMsgType(msg_type.to_string()));
    }

    let cl_ord_id = require_tag(tags, 11, "F")?.to_string();
    let orig_cl_ord_id = require_tag(tags, 41, "F")?.to_string();
    let symbol = require_tag(tags, 55, "F")?.to_string();
    let side = parse_fix_side(require_tag(tags, 54, "F")?)?;

    // OrderQty is optional on cancel
    let order_qty = find_tag(tags, 38)
        .map(|v| {
            v.parse::<f64>()
                .map(Quantity)
                .map_err(|_| FixError::InvalidTagValueData {
                    tag: 38,
                    value: v.to_string(),
                })
        })
        .transpose()?;

    Ok(CancelRequest {
        cl_ord_id,
        orig_cl_ord_id,
        symbol,
        side,
        order_qty,
    })
}

/// Convert a FIX OrderCancelReplaceRequest (35=G) to a FIG `CancelReplaceRequest`.
pub fn fix_to_fig_cancel_replace(tags: &[(u32, String)]) -> FixResult<CancelReplaceRequest> {
    let msg_type = require_tag(tags, 35, "CancelReplaceRequest")?;
    if msg_type != "G" {
        return Err(FixError::UnsupportedMsgType(msg_type.to_string()));
    }

    let cl_ord_id = require_tag(tags, 11, "G")?.to_string();
    let orig_cl_ord_id = require_tag(tags, 41, "G")?.to_string();
    let symbol = require_tag(tags, 55, "G")?.to_string();
    let side = parse_fix_side(require_tag(tags, 54, "G")?)?;
    let order_qty = Quantity(require_tag(tags, 38, "G")?.parse::<f64>().map_err(|_| {
        FixError::InvalidTagValueData {
            tag: 38,
            value: find_tag(tags, 38).unwrap_or("").to_string(),
        }
    })?);

    let price = find_tag(tags, 44)
        .map(|v| {
            v.parse::<f64>()
                .map(Price)
                .map_err(|_| FixError::InvalidTagValueData {
                    tag: 44,
                    value: v.to_string(),
                })
        })
        .transpose()?;

    Ok(CancelReplaceRequest {
        cl_ord_id,
        orig_cl_ord_id,
        symbol,
        side,
        order_qty,
        price,
    })
}

// ─── FIG → FIX Conversion ────────────────────────────────────────

/// Convert a FIG `ExecutionReport` to a FIX ExecutionReport (35=8) wire-format message.
pub fn fig_to_fix_execution_report(
    report: &ExecutionReport,
    ctx: &FixOutboundContext,
) -> Vec<u8> {
    let mut body = vec![
        (11, report.cl_ord_id.clone()),
        (37, report.order_id.clone()),
        (17, report.exec_id.clone()),
        (150, fix_exec_type(&report.exec_type)),
        (39, fix_ord_status(&report.ord_status)),
        (54, fix_side(&report.side)),
    ];

    if let Some(ref last_qty) = report.last_qty {
        body.push((32, fmt_quantity(last_qty)));
    }
    if let Some(ref last_price) = report.last_price {
        body.push((31, fmt_price(last_price)));
    }
    body.push((151, fmt_quantity(&report.leaves_qty)));
    body.push((14, fmt_quantity(&report.cum_qty)));
    body.push((6, fmt_price(&report.avg_price)));
    body.push((55, report.symbol.clone()));
    body.push((60, format_fix_utc_timestamp(report.transact_time)));

    build_outbound_fix_message(ctx, "8", body)
}

/// Convert a FIG `NewOrderSingle` to FIX NewOrderSingle (35=D) wire-format bytes.
pub fn fig_to_fix_new_order_single(order: &NewOrderSingle, ctx: &FixOutboundContext) -> Vec<u8> {
    let mut body = vec![
        (11, order.cl_ord_id.clone()),
        (54, fix_side(&order.side)),
        (38, fmt_quantity(&order.order_qty)),
        (55, order.symbol.clone()),
        (40, fix_order_type(&order.order_type)),
        (59, fix_time_in_force(&order.time_in_force)),
    ];

    if let Some(ref price) = order.price {
        body.push((44, fmt_price(price)));
    }
    if let Some(ref stop_price) = order.stop_price {
        body.push((99, fmt_price(stop_price)));
    }
    if let Some(ref account) = order.account {
        body.push((1, account.clone()));
    }
    if let Some(expire_time) = order.expire_time {
        body.push((432, format_fix_utc_timestamp(expire_time)));
    }

    build_outbound_fix_message(ctx, "D", body)
}

/// Convert a FIG `CancelReject` to FIX OrderCancelReject (35=9) wire-format bytes.
pub fn fig_to_fix_cancel_reject(
    reject: &CancelReject,
    ctx: &FixOutboundContext,
    response_to: FixCxlRejResponseTo,
) -> Vec<u8> {
    let body = vec![
        (11, reject.cl_ord_id.clone()),
        (41, reject.orig_cl_ord_id.clone()),
        (39, fix_ord_status(&OrdStatus::Rejected)),
        (434, (response_to as u8).to_string()),
        (102, fix_cancel_reject_reason(&reject.reject_reason)),
        (55, reject.symbol.clone()),
    ];

    build_outbound_fix_message(ctx, "9", body)
}

// ─── FIX Logon ↔ FIG STREAM_OPEN ─────────────────────────────────

/// Convert a FIX Logon (35=A) message to a FIG STREAM_OPEN frame with
/// an AUTH_TOKEN extension.
///
/// Extracts `SenderCompID` (tag 49), `TargetCompID` (tag 56),
/// `Username` (tag 553), and `Password` (tag 554) from the Logon.
/// If username/password are present, they are used as the auth token
/// in `user:pass` format. Otherwise, `SenderCompID:TargetCompID` is
/// used as the token.
pub fn logon_to_stream_open(logon: &FixMessage) -> FixConvertResult<Frame> {
    // Validate MsgType
    let msg_type = logon.msg_type().unwrap_or("");
    if msg_type != "A" {
        return Err(FixConvertError::UnsupportedMsgType(msg_type.to_string()));
    }

    let sender_comp_id = logon
        .get_tag(49)
        .ok_or(FixConvertError::MissingTag { tag: 49 })?
        .to_string();
    let target_comp_id = logon
        .get_tag(56)
        .ok_or(FixConvertError::MissingTag { tag: 56 })?
        .to_string();

    let username = logon.get_tag(553);
    let password = logon.get_tag(554);

    let auth_token = if let (Some(user), Some(pass)) = (username, password) {
        format!("{}:{}", user, pass)
    } else {
        format!("{}:{}", sender_comp_id, target_comp_id)
    };

    let frame = Frame::new(FrameType::StreamOpen, 0).with_extension(Extension::binary(
        ExtensionTag::AuthToken,
        auth_token.into_bytes(),
    ));

    Ok(frame)
}

/// Convert a FIG STREAM_OPEN frame with an AUTH_TOKEN extension to a
/// FIX Logon (35=A) message.
///
/// The auth token is expected to be in `user:pass` or `sender:target`
/// format. The first component is used as `SenderCompID` (tag 49) and
/// the second as `TargetCompID` (tag 56).
pub fn stream_open_to_logon(frame: &Frame) -> FixConvertResult<FixMessage> {
    if frame.frame_type != FrameType::StreamOpen {
        return Err(FixConvertError::UnsupportedMsgType(format!(
            "{}",
            frame.frame_type
        )));
    }

    let auth_bytes = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::AuthToken)
        .map(|e| e.value.as_bytes())
        .ok_or(FixConvertError::MissingAuthToken)?;

    let auth_str = String::from_utf8_lossy(&auth_bytes);
    let (sender_comp_id, target_comp_id) = if let Some(pos) = auth_str.find(':') {
        (auth_str[..pos].to_string(), auth_str[pos + 1..].to_string())
    } else {
        (auth_str.to_string(), String::new())
    };

    let tags = vec![
        (8, "FIX.4.4".to_string()),
        (35, "A".to_string()),
        (49, sender_comp_id),
        (56, target_comp_id),
        (34, "1".to_string()),   // MsgSeqNum
        (98, "0".to_string()),   // EncryptMethod = None
        (108, "30".to_string()), // HeartBtInt
        (141, "Y".to_string()),  // ResetSeqNumFlag
    ];

    Ok(FixMessage::new(tags))
}

// ─── FIX ResendRequest ↔ FIG CONTROL(RESEND) ───────────────────

/// Default channel ID for FIX session traffic mapped to FIG.
pub const FIX_SESSION_CHANNEL_ID: u16 = 1;

/// Convert a FIX ResendRequest (35=2) to a FIG CONTROL(RESEND) frame.
///
/// Maps FIX BeginSeqNo (tag 7) and EndSeqNo (tag 16, default 0) to the
/// RESEND payload. The target FIG channel defaults to
/// [`FIX_SESSION_CHANNEL_ID`] unless overridden.
pub fn resend_request_to_control(msg: &FixMessage, channel_id: u16) -> FixConvertResult<Frame> {
    let msg_type = msg.msg_type().unwrap_or("");
    if msg_type != "2" {
        return Err(FixConvertError::UnsupportedMsgType(msg_type.to_string()));
    }

    let begin_seq: u32 = msg
        .get_tag(7)
        .ok_or(FixConvertError::MissingTag { tag: 7 })?
        .parse()
        .map_err(|_| FixConvertError::InvalidSeqNumber {
            tag: 7,
            value: msg.get_tag(7).unwrap_or("").to_string(),
        })?;

    let end_seq: u32 = match msg.get_tag(16) {
        Some(v) => v.parse().map_err(|_| FixConvertError::InvalidSeqNumber {
            tag: 16,
            value: v.to_string(),
        })?,
        None => 0,
    };

    Ok(Frame::resend(channel_id, begin_seq, end_seq))
}

/// Convert a FIG CONTROL(RESEND) frame to a FIX ResendRequest (35=2).
pub fn control_to_resend_request(
    frame: &Frame,
    sender_comp_id: &str,
    target_comp_id: &str,
    msg_seq_num: u32,
) -> FixConvertResult<FixMessage> {
    let (channel_id, begin_seq, end_seq) =
        frame
            .resend_range()
            .ok_or(FixConvertError::UnsupportedMsgType(format!(
                "{}",
                frame.frame_type
            )))?;
    let _ = channel_id; // FIX session-level; channel encoded in FIG payload only

    let mut tags: Vec<(u32, String)> = vec![
        (8, "FIX.4.4".to_string()),
        (35, "2".to_string()),
        (49, sender_comp_id.to_string()),
        (56, target_comp_id.to_string()),
        (34, msg_seq_num.to_string()),
        (7, begin_seq.to_string()),
    ];
    if end_seq != 0 {
        tags.push((16, end_seq.to_string()));
    }

    Ok(FixMessage::new(tags))
}

// ─── FIX Value Converters ─────────────────────────────────────────

/// Parse a FIX side value (tag 54).
fn parse_fix_side(value: &str) -> FixResult<Side> {
    match value {
        "1" => Ok(Side::Buy),
        "2" => Ok(Side::Sell),
        "5" => Ok(Side::SellShort),
        "6" => Ok(Side::SellShortExempt),
        _ => Err(FixError::UnknownSide(value.to_string())),
    }
}

/// Convert a FIG Side to a FIX side value.
fn fix_side(side: &Side) -> String {
    match side {
        Side::Buy => "1".to_string(),
        Side::Sell => "2".to_string(),
        Side::SellShort => "5".to_string(),
        Side::SellShortExempt => "6".to_string(),
    }
}

/// Parse a FIX order type (tag 40).
fn parse_fix_order_type(value: &str) -> FixResult<OrderType> {
    match value {
        "1" => Ok(OrderType::Market),
        "2" => Ok(OrderType::Limit),
        "3" => Ok(OrderType::Stop),
        "4" => Ok(OrderType::StopLimit),
        _ => Err(FixError::UnknownOrderType(value.to_string())),
    }
}

/// Convert a FIG OrderType to FIX order type (tag 40).
fn fix_order_type(order_type: &OrderType) -> String {
    match order_type {
        OrderType::Market => "1",
        OrderType::Limit => "2",
        OrderType::Stop => "3",
        OrderType::StopLimit => "4",
    }
    .to_string()
}

/// Parse a FIX time-in-force (tag 59).
fn parse_fix_time_in_force(value: &str) -> FixResult<TimeInForce> {
    match value {
        "0" => Ok(TimeInForce::Day),
        "1" => Ok(TimeInForce::Gtc),
        "3" => Ok(TimeInForce::Ioc),
        "4" => Ok(TimeInForce::Fok),
        "6" => Ok(TimeInForce::Gtd),
        _ => Err(FixError::UnknownTimeInForce(value.to_string())),
    }
}

/// Convert a FIG TimeInForce to FIX time-in-force (tag 59).
fn fix_time_in_force(tif: &TimeInForce) -> String {
    match tif {
        TimeInForce::Day => "0",
        TimeInForce::Gtc => "1",
        TimeInForce::Ioc => "3",
        TimeInForce::Fok => "4",
        TimeInForce::Gtd => "6",
    }
    .to_string()
}

/// Convert a FIG CancelRejectReason to FIX CxlRejReason (tag 102).
fn fix_cancel_reject_reason(reason: &CancelRejectReason) -> String {
    match reason {
        CancelRejectReason::TooLateToCancel => "0",
        CancelRejectReason::OrderNotFound => "1",
        CancelRejectReason::AlreadyFilled => "2",
        CancelRejectReason::AlreadyCanceled => "6",
    }
    .to_string()
}

/// Convert a FIG ExecType to a FIX exec type value (tag 150).
fn fix_exec_type(et: &ExecType) -> String {
    match et {
        ExecType::New => "0",
        ExecType::PartialFill => "1",
        ExecType::Fill => "2",
        ExecType::DoneForDay => "3",
        ExecType::Canceled => "4",
        ExecType::Replaced => "5",
        ExecType::PendingCancel => "6",
        ExecType::Stopped => "7",
        ExecType::Rejected => "8",
        ExecType::Suspended => "9",
        ExecType::PendingNew => "A",
        ExecType::Expired => "C",
    }
    .to_string()
}

/// Convert a FIG OrdStatus to a FIX ord status value (tag 39).
fn fix_ord_status(status: &OrdStatus) -> String {
    match status {
        OrdStatus::New => "0",
        OrdStatus::PartiallyFilled => "1",
        OrdStatus::Filled => "2",
        OrdStatus::DoneForDay => "3",
        OrdStatus::Canceled => "4",
        OrdStatus::Replaced => "5",
        OrdStatus::PendingCancel => "6",
        OrdStatus::Stopped => "7",
        OrdStatus::Rejected => "8",
        OrdStatus::Suspended => "9",
        OrdStatus::PendingNew => "A",
        OrdStatus::Expired => "C",
    }
    .to_string()
}

/// Format a Quantity for FIX.
fn fmt_quantity(q: &Quantity) -> String {
    q.0.to_string()
}

/// Format a Price for FIX.
fn fmt_price(p: &Price) -> String {
    p.0.to_string()
}

// ─── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Parse round-trip ──────────────────────────────────────

    #[test]
    fn test_parse_round_trip() {
        // Build a message with proper checksum using serialize
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (9, "100".to_string()),
            (35, "D".to_string()),
            (49, "SENDER".to_string()),
            (56, "TARGET".to_string()),
            (34, "1".to_string()),
            (52, "20240101-00:00:00".to_string()),
            (11, "ORD-001".to_string()),
            (54, "1".to_string()),
            (38, "100".to_string()),
            (44, "50.25".to_string()),
            (55, "AAPL".to_string()),
            (40, "2".to_string()),
            (59, "0".to_string()),
            (1, "ACCT-123".to_string()),
        ];
        let msg = serialize_fix_message(&tags);
        let parsed = parse_fix_message(&msg).unwrap();

        assert_eq!(parsed.len(), tags.len() + 1); // +1 for checksum
        assert_eq!(parsed[0], (8, "FIX.4.4".to_string()));
        assert_eq!(parsed[2], (35, "D".to_string()));
        assert_eq!(parsed[7], (11, "ORD-001".to_string()));
        assert_eq!(parsed[8], (54, "1".to_string()));
        assert_eq!(parsed[9], (38, "100".to_string()));
        assert_eq!(parsed[10], (44, "50.25".to_string()));
        assert_eq!(parsed[11], (55, "AAPL".to_string()));
        assert_eq!(parsed[12], (40, "2".to_string()));
        assert_eq!(parsed[13], (59, "0".to_string()));
        assert_eq!(parsed[14], (1, "ACCT-123".to_string()));
    }

    #[test]
    fn test_serialize_round_trip() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "D".to_string()),
            (11, "ORD-001".to_string()),
            (54, "1".to_string()),
            (38, "100".to_string()),
            (55, "AAPL".to_string()),
        ];

        let serialized = serialize_fix_message(&tags);
        // Should be valid FIX
        let parsed = parse_fix_message(&serialized).unwrap();
        // parsed includes checksum (tag 10) as an extra pair
        assert_eq!(parsed.len(), tags.len() + 1);
        for (i, (tag, value)) in tags.iter().enumerate() {
            assert_eq!(parsed[i].0, *tag);
            assert_eq!(parsed[i].1, *value);
        }
        // Last entry should be the checksum
        assert_eq!(parsed.last().unwrap().0, 10);
    }

    // ── Checksum ──────────────────────────────────────────────

    #[test]
    fn test_checksum_validation() {
        // Build a known-valid FIX message and verify checksum
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "D".to_string()),
            (11, "TEST-001".to_string()),
        ];

        let serialized = serialize_fix_message(&tags);

        // Parse should validate checksum
        let parsed = parse_fix_message(&serialized).unwrap();
        assert_eq!(parsed[0], (8, "FIX.4.4".to_string()));
    }

    #[test]
    fn test_checksum_mismatch() {
        // A message with a deliberately wrong checksum (correct=231, wrong=042)
        let msg = b"8=FIX.4.4\x019=50\x0135=D\x0110=042\x01";
        let result = parse_fix_message(msg);
        assert!(matches!(result, Err(FixError::ChecksumMismatch { .. })));
    }

    // ── FIX → FIG Order ──────────────────────────────────────

    #[test]
    fn test_fix_to_fig_order() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=100\x0135=D\x01");
        buf.extend_from_slice(b"11=ORD-BUY-001\x01");
        buf.extend_from_slice(b"54=1\x01");
        buf.extend_from_slice(b"38=500\x01");
        buf.extend_from_slice(b"44=150.25\x01");
        buf.extend_from_slice(b"55=MSFT\x01");
        buf.extend_from_slice(b"40=2\x01");
        buf.extend_from_slice(b"59=0\x01");
        buf.extend_from_slice(b"1=ACC-777\x01");

        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        let order = fix_to_fig_order(&tags).unwrap();

        assert_eq!(order.cl_ord_id, "ORD-BUY-001");
        assert_eq!(order.side, Side::Buy);
        assert_eq!(order.order_qty, Quantity(500.0));
        assert_eq!(order.price, Some(Price(150.25)));
        assert_eq!(order.symbol, "MSFT");
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.time_in_force, TimeInForce::Day);
        assert_eq!(order.account, Some("ACC-777".to_string()));
    }

    #[test]
    fn test_fix_to_fig_order_sell_short() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=100\x0135=D\x01");
        buf.extend_from_slice(b"11=ORD-SELL-002\x01");
        buf.extend_from_slice(b"54=5\x01");
        buf.extend_from_slice(b"38=100\x01");
        buf.extend_from_slice(b"55=TSLA\x01");
        buf.extend_from_slice(b"40=1\x01");
        buf.extend_from_slice(b"59=1\x01");

        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        let order = fix_to_fig_order(&tags).unwrap();

        assert_eq!(order.cl_ord_id, "ORD-SELL-002");
        assert_eq!(order.side, Side::SellShort);
        assert_eq!(order.order_type, OrderType::Market);
        assert_eq!(order.time_in_force, TimeInForce::Gtc);
        assert_eq!(order.price, None);
    }

    // ── FIX → FIG Cancel ─────────────────────────────────────

    #[test]
    fn test_fix_to_fig_cancel() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=80\x0135=F\x01");
        buf.extend_from_slice(b"11=ORD-CXL-001\x01");
        buf.extend_from_slice(b"41=ORD-BUY-001\x01");
        buf.extend_from_slice(b"55=MSFT\x01");
        buf.extend_from_slice(b"54=2\x01");

        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        let cancel = fix_to_fig_cancel(&tags).unwrap();

        assert_eq!(cancel.cl_ord_id, "ORD-CXL-001");
        assert_eq!(cancel.orig_cl_ord_id, "ORD-BUY-001");
        assert_eq!(cancel.symbol, "MSFT");
        assert_eq!(cancel.side, Side::Sell);
    }

    #[test]
    fn test_fix_to_fig_stop_order_requires_stop_px() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=80\x0135=D\x01");
        buf.extend_from_slice(b"11=STOP-001\x01");
        buf.extend_from_slice(b"54=1\x01");
        buf.extend_from_slice(b"38=100\x01");
        buf.extend_from_slice(b"55=AAPL\x01");
        buf.extend_from_slice(b"40=3\x01");
        buf.extend_from_slice(b"59=0\x01");
        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        assert!(matches!(
            fix_to_fig_order(&tags),
            Err(FixError::MissingTag { tag: 99, .. })
        ));
    }

    #[test]
    fn test_fix_to_fig_gtd_requires_expire_time() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=100\x0135=D\x01");
        buf.extend_from_slice(b"11=GTD-001\x01");
        buf.extend_from_slice(b"54=1\x01");
        buf.extend_from_slice(b"38=100\x01");
        buf.extend_from_slice(b"44=50\x01");
        buf.extend_from_slice(b"55=AAPL\x01");
        buf.extend_from_slice(b"40=2\x01");
        buf.extend_from_slice(b"59=6\x01");
        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        assert!(matches!(
            fix_to_fig_order(&tags),
            Err(FixError::MissingTag { tag: 432, .. })
        ));
    }

    #[test]
    fn test_fix_to_fig_cancel_replace() {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"8=FIX.4.4\x019=100\x0135=G\x01");
        buf.extend_from_slice(b"11=REP-001\x01");
        buf.extend_from_slice(b"41=ORD-001\x01");
        buf.extend_from_slice(b"55=AAPL\x01");
        buf.extend_from_slice(b"54=1\x01");
        buf.extend_from_slice(b"38=200\x01");
        buf.extend_from_slice(b"44=151\x01");
        let checksum = compute_checksum(&buf);
        buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
        buf.push(SOH);

        let tags = parse_fix_message(&buf).unwrap();
        let replace = fix_to_fig_cancel_replace(&tags).unwrap();
        assert_eq!(replace.cl_ord_id, "REP-001");
        assert_eq!(replace.orig_cl_ord_id, "ORD-001");
        assert_eq!(replace.order_qty, Quantity(200.0));
        assert_eq!(replace.price, Some(Price(151.0)));
    }

    #[test]
    fn test_fix_utc_timestamp_round_trip() {
        let nanos = 1_700_000_000_000_000_000i64;
        let formatted = format_fix_utc_timestamp(nanos);
        let parsed = parse_fix_utc_timestamp(&formatted).unwrap();
        assert!((parsed - nanos).abs() < 1_000_000_000);
    }

    // ── FIG → FIX ExecutionReport ────────────────────────────

    #[test]
    fn test_fig_to_fix_execution_report() {
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

        let encoded = fig_to_fix_execution_report(&report, &FixOutboundContext::default());

        // Parse it back to verify correctness
        let parsed = parse_fix_message(&encoded).unwrap();

        let find = |tag: u32| -> String {
            parsed
                .iter()
                .find(|(t, _)| *t == tag)
                .map(|(_, v)| v.clone())
                .unwrap_or_default()
        };

        assert_eq!(find(35), "8"); // ExecutionReport
        assert_eq!(find(11), "ORD-001");
        assert_eq!(find(37), "OX-001");
        assert_eq!(find(17), "EX-001");
        assert_eq!(find(150), "2"); // Fill
        assert_eq!(find(39), "2"); // Filled
        assert_eq!(find(54), "1"); // Buy
        assert_eq!(find(32), "100");
        assert_eq!(find(31), "50.25");
        assert_eq!(find(151), "0");
        assert_eq!(find(14), "100");
        assert_eq!(find(6), "50.25");
        assert_eq!(find(55), "AAPL");
        assert!(find(60).contains('-'));
        assert_eq!(find(49), "FIG");
        assert_eq!(find(56), "CLIENT");
        assert_eq!(find(34), "1");
    }

    #[test]
    fn test_fig_to_fix_new_order_single_round_trip() {
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
            account: Some("ACC".to_string()),
            strategy_id: None,
        };

        let encoded = fig_to_fix_new_order_single(&order, &FixOutboundContext::default());
        let tags = parse_fix_message(&encoded).unwrap();
        let decoded = fix_to_fig_order(&tags).unwrap();
        assert_eq!(decoded, order);
    }

    #[test]
    fn test_fig_to_fix_cancel_reject() {
        let reject = CancelReject {
            cl_ord_id: "CXL-1".to_string(),
            orig_cl_ord_id: "ORD-1".to_string(),
            reject_reason: CancelRejectReason::OrderNotFound,
            symbol: "AAPL".to_string(),
        };
        let encoded = fig_to_fix_cancel_reject(
            &reject,
            &FixOutboundContext::default(),
            FixCxlRejResponseTo::OrderCancelRequest,
        );
        let tags = parse_fix_message(&encoded).unwrap();
        let find = |tag: u32| -> String {
            tags.iter()
                .find(|(t, _)| *t == tag)
                .map(|(_, v)| v.clone())
                .unwrap_or_default()
        };
        assert_eq!(find(35), "9");
        assert_eq!(find(11), "CXL-1");
        assert_eq!(find(41), "ORD-1");
        assert_eq!(find(102), "1");
        assert_eq!(find(434), "1");
    }

    #[test]
    fn test_split_fix_messages() {
        let msg1 = serialize_fix_message(&[(8, "FIX.4.4".into()), (35, "0".into())]);
        let msg2 = serialize_fix_message(&[(8, "FIX.4.4".into()), (35, "A".into())]);
        let mut buf = msg1.clone();
        buf.extend_from_slice(&msg2);
        let split = split_fix_messages(&buf);
        assert_eq!(split.len(), 2);
        assert_eq!(parse_fix_message(&split[0]).unwrap()[1].1, "0");
        assert_eq!(parse_fix_message(&split[1]).unwrap()[1].1, "A");
    }

    // ── Logon ↔ STREAM_OPEN Conversion ────────────────────────

    #[test]
    fn test_logon_with_credentials_to_stream_open() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "A".to_string()),
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
            (553, "trader1".to_string()),
            (554, "secret123".to_string()),
            (34, "1".to_string()),
        ];
        let logon = FixMessage::new(tags);
        let frame = logon_to_stream_open(&logon).unwrap();

        assert_eq!(frame.frame_type, FrameType::StreamOpen);
        assert_eq!(frame.channel_id, 0);
        assert_eq!(frame.extensions.len(), 1);
        assert_eq!(frame.extensions[0].tag, ExtensionTag::AuthToken);
        let auth_bytes = frame.extensions[0].value.as_bytes();
        let auth = String::from_utf8_lossy(&auth_bytes);
        assert_eq!(auth, "trader1:secret123");
    }

    #[test]
    fn test_logon_without_credentials_to_stream_open() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "A".to_string()),
            (49, "SENDER".to_string()),
            (56, "TARGET".to_string()),
            (34, "1".to_string()),
        ];
        let logon = FixMessage::new(tags);
        let frame = logon_to_stream_open(&logon).unwrap();

        assert_eq!(frame.frame_type, FrameType::StreamOpen);
        assert_eq!(frame.channel_id, 0);
        assert_eq!(frame.extensions.len(), 1);
        assert_eq!(frame.extensions[0].tag, ExtensionTag::AuthToken);
        let auth_bytes = frame.extensions[0].value.as_bytes();
        let auth = String::from_utf8_lossy(&auth_bytes);
        assert_eq!(auth, "SENDER:TARGET");
    }

    #[test]
    fn test_logon_to_stream_open_round_trip() {
        // Logon with credentials → Frame → back to Logon
        let original_tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "A".to_string()),
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
            (553, "trader1".to_string()),
            (554, "secret123".to_string()),
            (34, "2".to_string()),
        ];
        let logon = FixMessage::new(original_tags);

        let frame = logon_to_stream_open(&logon).unwrap();
        let logon2 = stream_open_to_logon(&frame).unwrap();

        // Verify key fields are preserved
        assert_eq!(logon2.msg_type(), Some("A"));
        assert_eq!(logon2.get_tag(49), Some("trader1"));
        assert_eq!(logon2.get_tag(56), Some("secret123"));
    }

    #[test]
    fn test_stream_open_to_logon() {
        let frame = Frame::new(FrameType::StreamOpen, 0).with_extension(Extension::binary(
            ExtensionTag::AuthToken,
            b"alice:password1".to_vec(),
        ));

        let logon = stream_open_to_logon(&frame).unwrap();
        assert_eq!(logon.msg_type(), Some("A"));
        assert_eq!(logon.get_tag(49), Some("alice"));
        assert_eq!(logon.get_tag(56), Some("password1"));
        assert_eq!(logon.get_tag(98), Some("0")); // EncryptMethod
        assert_eq!(logon.get_tag(141), Some("Y")); // ResetSeqNumFlag
    }

    #[test]
    fn test_logon_to_stream_open_rejects_non_logon() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "D".to_string()), // NewOrderSingle, not Logon
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
        ];
        let msg = FixMessage::new(tags);
        let result = logon_to_stream_open(&msg);
        assert!(matches!(
            result,
            Err(FixConvertError::UnsupportedMsgType(_))
        ));
    }

    #[test]
    fn test_stream_open_to_logon_rejects_non_stream_open() {
        let frame = Frame::ping();
        let result = stream_open_to_logon(&frame);
        assert!(matches!(
            result,
            Err(FixConvertError::UnsupportedMsgType(_))
        ));
    }

    #[test]
    fn test_stream_open_to_logon_missing_auth_token() {
        let frame = Frame::new(FrameType::StreamOpen, 0);
        let result = stream_open_to_logon(&frame);
        assert!(matches!(result, Err(FixConvertError::MissingAuthToken)));
    }

    // ── ResendRequest ↔ CONTROL(RESEND) ────────────────────────

    #[test]
    fn test_resend_request_to_control() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "2".to_string()),
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
            (34, "5".to_string()),
            (7, "10".to_string()),
            (16, "25".to_string()),
        ];
        let msg = FixMessage::new(tags);
        let frame = resend_request_to_control(&msg, FIX_SESSION_CHANNEL_ID).unwrap();

        assert_eq!(
            frame.control_subtype(),
            Some(fig_core::frame::ControlSubtype::Resend)
        );
        assert_eq!(frame.resend_range(), Some((FIX_SESSION_CHANNEL_ID, 10, 25)));
    }

    #[test]
    fn test_resend_request_to_control_default_end_seq() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "2".to_string()),
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
            (34, "5".to_string()),
            (7, "3".to_string()),
        ];
        let msg = FixMessage::new(tags);
        let frame = resend_request_to_control(&msg, 2).unwrap();
        assert_eq!(frame.resend_range(), Some((2, 3, 0)));
    }

    #[test]
    fn test_resend_request_to_control_missing_begin_seq() {
        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "2".to_string()),
            (49, "CLIENT".to_string()),
            (56, "BROKER".to_string()),
        ];
        let msg = FixMessage::new(tags);
        let result = resend_request_to_control(&msg, 1);
        assert!(matches!(
            result,
            Err(FixConvertError::MissingTag { tag: 7 })
        ));
    }

    #[test]
    fn test_control_to_resend_request() {
        let frame = Frame::resend(FIX_SESSION_CHANNEL_ID, 10, 25);
        let fix_msg = control_to_resend_request(&frame, "CLIENT", "BROKER", 5).unwrap();

        assert_eq!(fix_msg.msg_type(), Some("2"));
        assert_eq!(fix_msg.get_tag(7), Some("10"));
        assert_eq!(fix_msg.get_tag(16), Some("25"));
        assert_eq!(fix_msg.get_tag(49), Some("CLIENT"));
        assert_eq!(fix_msg.get_tag(56), Some("BROKER"));
        assert_eq!(fix_msg.get_tag(34), Some("5"));
    }

    #[test]
    fn test_resend_round_trip() {
        let original_tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "2".to_string()),
            (49, "SENDER".to_string()),
            (56, "TARGET".to_string()),
            (34, "8".to_string()),
            (7, "100".to_string()),
            (16, "150".to_string()),
        ];
        let original = FixMessage::new(original_tags);

        let frame = resend_request_to_control(&original, FIX_SESSION_CHANNEL_ID).unwrap();
        let converted = control_to_resend_request(&frame, "SENDER", "TARGET", 8).unwrap();

        assert_eq!(converted.msg_type(), original.msg_type());
        assert_eq!(converted.get_tag(7), original.get_tag(7));
        assert_eq!(converted.get_tag(16), original.get_tag(16));
        assert_eq!(converted.get_tag(49), original.get_tag(49));
        assert_eq!(converted.get_tag(56), original.get_tag(56));
    }

    #[test]
    fn test_control_to_resend_request_rejects_non_resend() {
        let frame = Frame::ping();
        let result = control_to_resend_request(&frame, "A", "B", 1);
        assert!(matches!(
            result,
            Err(FixConvertError::UnsupportedMsgType(_))
        ));
    }
}
