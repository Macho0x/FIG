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
    CancelRequest, ExecType, ExecutionReport, NewOrderSingle,
    OrderType, OrdStatus, Price, Quantity, Side, TimeInForce,
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
}

/// Convenience type alias for FIX↔FIG conversion operations.
pub type FixConvertResult<T> = Result<T, FixConvertError>;

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

        let checksum_str = std::str::from_utf8(&last_field[3..])
            .map_err(|_| FixError::InvalidTagValue {
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

        let eq_pos = field_str.find('=').ok_or_else(|| FixError::InvalidTagValue {
            pos,
            reason: "missing '=' in tag-value pair".to_string(),
        })?;

        let tag: u32 = field_str[..eq_pos].parse().map_err(|_| FixError::InvalidTagValue {
            pos,
            reason: format!("invalid tag number: {}", &field_str[..eq_pos]),
        })?;

        let value = field_str[eq_pos + 1..].to_string();

        pairs.push((tag, value));
    }

    Ok(pairs)
}

/// Find the value for a specific tag in parsed FIX tag-value pairs.
fn find_tag<'a>(tags: &'a [(u32, String)], tag: u32) -> Option<&'a str> {
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
/// | 55      | Symbol       | symbol      |
/// | 40      | OrdType      | order_type  |
/// | 59      | TimeInForce  | time_in_force|
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
    let order_qty = Quantity(
        require_tag(tags, 38, "D")?
            .parse::<f64>()
            .map_err(|_| FixError::InvalidTagValueData {
                tag: 38,
                value: find_tag(tags, 38).unwrap_or("").to_string(),
            })?,
    );

    let price = find_tag(tags, 44).map(|v| {
        v.parse::<f64>().map(Price).map_err(|_| FixError::InvalidTagValueData {
            tag: 44,
            value: v.to_string(),
        })
    }).transpose()?;

    let order_type = parse_fix_order_type(require_tag(tags, 40, "D")?)?;
    let time_in_force = parse_fix_time_in_force(find_tag(tags, 59).unwrap_or("0"))?;

    let account = find_tag(tags, 1).map(|s| s.to_string());

    Ok(NewOrderSingle {
        cl_ord_id,
        side,
        order_qty,
        price,
        symbol,
        order_type,
        time_in_force,
        expire_time: None,
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
/// | 41      | OrigClOrdID     | cl_ord_id      |
/// | 37      | OrderID         | orig_cl_ord_id |
/// | 55      | Symbol          | symbol         |
/// | 54      | Side            | side           |
pub fn fix_to_fig_cancel(tags: &[(u32, String)]) -> FixResult<CancelRequest> {
    let msg_type = require_tag(tags, 35, "CancelRequest")?;
    if msg_type != "F" {
        return Err(FixError::UnsupportedMsgType(msg_type.to_string()));
    }

    let cl_ord_id = require_tag(tags, 41, "F")?.to_string();
    let orig_cl_ord_id = require_tag(tags, 37, "F")?.to_string();
    let symbol = require_tag(tags, 55, "F")?.to_string();
    let side = parse_fix_side(require_tag(tags, 54, "F")?)?;

    // OrderQty is optional on cancel
    let order_qty = find_tag(tags, 38).map(|v| {
        v.parse::<f64>().map(Quantity).map_err(|_| FixError::InvalidTagValueData {
            tag: 38,
            value: v.to_string(),
        })
    }).transpose()?;

    Ok(CancelRequest {
        cl_ord_id,
        orig_cl_ord_id,
        symbol,
        side,
        order_qty,
    })
}

// ─── FIG → FIX Conversion ────────────────────────────────────────

/// Convert a FIG `ExecutionReport` to a FIX ExecutionReport (35=8) wire-format message.
///
/// # Tag mappings
///
/// | FIX Tag | FIG Field     |
/// |---------|--------------|
/// | 11      | cl_ord_id    |
/// | 37      | order_id     |
/// | 17      | exec_id      |
/// | 150     | exec_type    |
/// | 39      | ord_status   |
/// | 54      | side         |
/// | 32      | last_qty     |
/// | 31      | last_price   |
/// | 151     | leaves_qty   |
/// | 14      | cum_qty      |
/// | 6       | avg_price    |
/// | 55      | symbol       |
/// | 60      | transact_time|
pub fn fig_to_fix_execution_report(report: &ExecutionReport) -> Vec<u8> {
    let mut tags: Vec<(u32, String)> = Vec::new();

    // Standard FIX header
    tags.push((8, "FIX.4.4".to_string()));
    tags.push((9, "0".to_string())); // BodyLength — placeholder, will recalc
    tags.push((35, "8".to_string())); // MsgType = ExecutionReport

    // Order identification
    tags.push((11, report.cl_ord_id.clone()));
    tags.push((37, report.order_id.clone()));
    tags.push((17, report.exec_id.clone()));

    // Execution details
    tags.push((150, fix_exec_type(&report.exec_type)));
    tags.push((39, fix_ord_status(&report.ord_status)));
    tags.push((54, fix_side(&report.side)));

    // Quantities and prices
    if let Some(ref last_qty) = report.last_qty {
        tags.push((32, fmt_quantity(last_qty)));
    }
    if let Some(ref last_price) = report.last_price {
        tags.push((31, fmt_price(last_price)));
    }
    tags.push((151, fmt_quantity(&report.leaves_qty)));
    tags.push((14, fmt_quantity(&report.cum_qty)));
    tags.push((6, fmt_price(&report.avg_price)));

    // Symbol and timestamp
    tags.push((55, report.symbol.clone()));
    tags.push((60, report.transact_time.to_string()));

    // Compute BodyLength (tag 9): length of everything after "9=XXX\x01" to before checksum.
    // Build the body tags (everything except 8=BeginString, 9=BodyLength, 10=CheckSum),
    // compute body length, then prepend header.
    let body_tags: Vec<&(u32, String)> = tags.iter().filter(|(t, _)| *t != 8 && *t != 9 && *t != 10).collect();

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

    // Compute checksum
    let checksum = compute_checksum(&final_buf);
    final_buf.extend_from_slice(format!("10={:03}", checksum).as_bytes());
    final_buf.push(SOH);

    final_buf
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

    let frame = Frame::new(FrameType::StreamOpen, 0)
        .with_extension(Extension::binary(
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
        (
            auth_str[..pos].to_string(),
            auth_str[pos + 1..].to_string(),
        )
    } else {
        (auth_str.to_string(), String::new())
    };

    let mut tags: Vec<(u32, String)> = Vec::new();
    tags.push((8, "FIX.4.4".to_string()));
    tags.push((35, "A".to_string()));
    tags.push((49, sender_comp_id));
    tags.push((56, target_comp_id));
    tags.push((34, "1".to_string())); // MsgSeqNum
    tags.push((98, "0".to_string())); // EncryptMethod = None
    tags.push((108, "30".to_string())); // HeartBtInt
    tags.push((141, "Y".to_string())); // ResetSeqNumFlag

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
    }.to_string()
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
    }.to_string()
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
        buf.extend_from_slice(b"41=ORD-CXL-001\x01");
        buf.extend_from_slice(b"37=ORD-BUY-001\x01");
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

        let encoded = fig_to_fix_execution_report(&report);

        // Parse it back to verify correctness
        let parsed = parse_fix_message(&encoded).unwrap();

        let find = |tag: u32| -> String {
            parsed.iter()
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
        assert_eq!(find(60), "1700000000000000000");
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
        let frame = Frame::new(FrameType::StreamOpen, 0).with_extension(
            Extension::binary(ExtensionTag::AuthToken, b"alice:password1".to_vec()),
        );

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
}
