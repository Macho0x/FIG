//! Extension tags and TLV encoding/decoding for FIG frames.
//!
//! Extensions carry metadata in the variable-length header of each frame.
//! Each extension is a Tag-Length-Value (TLV) triple.

use crate::error::FrameError;
use crate::FrameResult;

// ─── Content-Type Constants ─────────────────────────────────────

pub const CONTENT_TYPE_CBOR: &str = "application/cbor";
pub const CONTENT_TYPE_SBE: &str = "application/fig+sbe";
pub const CONTENT_TYPE_JSON: &str = "application/json";
pub const CONTENT_TYPE_PROTOBUF: &str = "application/x-protobuf";

// ─── Extension Tag ──────────────────────────────────────────────

/// Well-known extension tags for FIG frames.
///
/// Each tag identifies the type of metadata carried in the extension.
/// Tags 0x0001–0x001D are defined by this specification.
/// Tags 0x8000–0xFFFF are reserved for private/custom use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionTag {
    /// Resource path: `/accounts/12345/orders`
    RequestUri,
    /// Effective URI after routing
    ResponseUri,
    /// MIME-like content type
    ContentType,
    /// HTTP-compatible status code
    StatusCode,
    /// Links request → response across channels
    CorrelationId,
    /// FIX MsgSeqNum compatibility
    SequenceNum,
    /// Identifies a FIX session or logical session
    SessionId,
    /// Nanoseconds since Unix epoch
    Timestamp,
    /// Message validity window in milliseconds
    TtlMillis,
    /// Pub/sub topic: `marketdata.NYSE.AAPL.quotes`
    RoutingKey,
    /// 64-bit hash of schema definition
    SchemaFingerprint,
    /// Content encoding: `zstd`, `lz4`, `snappy`, `gzip`
    ContentEncoding,
    /// HTTP method: `GET`, `PUT`, `POST`, `DELETE`, `PATCH`
    Method,
    /// Client identification
    UserAgent,
    /// Bearer token, JWT, or custom auth blob
    AuthToken,
    /// Accepted content types for response
    Accept,
    /// Cache control: `no-cache`, `max-age=3600`
    CacheControl,
    /// Safe retry of non-idempotent requests
    IdempotencyKey,
    /// Distributed tracing correlation (16 bytes)
    TraceId,
    /// Machine-readable error code: `UNAUTHORIZED`, `INVALID_ORDER_QTY`
    ErrorCode,
    /// Human-readable error description
    ErrorMessage,
    /// Channel mode: `stateless`, `session`, `affinity`
    ChannelMode,
    /// Unified channel address: `trading/accounts/123/orders`
    ChannelPath,
    /// Target server address for REDIRECT frames
    RedirectTarget,
    /// Session token for redirect reconnection
    RedirectToken,
    /// Start of acknowledged seq range
    AckRangeStart,
    /// End of acknowledged seq range
    AckRangeEnd,
    /// Additional message credits granted
    FlowControlCredit,
    /// Authorization scope: `trading:orders:write`
    Scope,
    /// Authentication method identifier
    AuthMethod,
    /// Unknown/custom extension tag
    Unknown(u16),
}

/// Mapping from ExtensionTag variants to their numeric codes.
const TAG_CODES: [(ExtensionTag, u16); 30] = [
    (ExtensionTag::RequestUri, 0x0001),
    (ExtensionTag::ResponseUri, 0x0002),
    (ExtensionTag::ContentType, 0x0003),
    (ExtensionTag::StatusCode, 0x0004),
    (ExtensionTag::CorrelationId, 0x0005),
    (ExtensionTag::SequenceNum, 0x0006),
    (ExtensionTag::SessionId, 0x0007),
    (ExtensionTag::Timestamp, 0x0008),
    (ExtensionTag::TtlMillis, 0x0009),
    (ExtensionTag::RoutingKey, 0x000A),
    (ExtensionTag::SchemaFingerprint, 0x000B),
    (ExtensionTag::ContentEncoding, 0x000C),
    (ExtensionTag::Method, 0x000D),
    (ExtensionTag::UserAgent, 0x000E),
    (ExtensionTag::AuthToken, 0x000F),
    (ExtensionTag::Accept, 0x0010),
    (ExtensionTag::CacheControl, 0x0011),
    (ExtensionTag::IdempotencyKey, 0x0012),
    (ExtensionTag::TraceId, 0x0013),
    (ExtensionTag::ErrorCode, 0x0014),
    (ExtensionTag::ErrorMessage, 0x0015),
    (ExtensionTag::ChannelMode, 0x0016),
    (ExtensionTag::ChannelPath, 0x0017),
    (ExtensionTag::RedirectTarget, 0x0018),
    (ExtensionTag::RedirectToken, 0x0019),
    (ExtensionTag::AckRangeStart, 0x001A),
    (ExtensionTag::AckRangeEnd, 0x001B),
    (ExtensionTag::FlowControlCredit, 0x001C),
    (ExtensionTag::Scope, 0x001D),
    (ExtensionTag::AuthMethod, 0x001E),
];

impl ExtensionTag {
    /// Get the numeric code for this tag.
    pub fn code(&self) -> u16 {
        match self {
            ExtensionTag::Unknown(code) => *code,
            tag => {
                for (known_tag, code) in TAG_CODES {
                    if std::mem::discriminant(tag) == std::mem::discriminant(&known_tag) {
                        return code;
                    }
                }
                0 // fallback (should not happen)
            }
        }
    }

    /// Check if this is a well-known (spec-defined) tag.
    pub fn is_well_known(&self) -> bool {
        !matches!(self, ExtensionTag::Unknown(_))
    }

    /// Check if this is a custom/private tag (0x8000–0xFFFF).
    pub fn is_custom(&self) -> bool {
        matches!(self, ExtensionTag::Unknown(code) if *code >= 0x8000)
    }
}

impl From<u16> for ExtensionTag {
    fn from(code: u16) -> Self {
        match code {
            0x0001 => ExtensionTag::RequestUri,
            0x0002 => ExtensionTag::ResponseUri,
            0x0003 => ExtensionTag::ContentType,
            0x0004 => ExtensionTag::StatusCode,
            0x0005 => ExtensionTag::CorrelationId,
            0x0006 => ExtensionTag::SequenceNum,
            0x0007 => ExtensionTag::SessionId,
            0x0008 => ExtensionTag::Timestamp,
            0x0009 => ExtensionTag::TtlMillis,
            0x000A => ExtensionTag::RoutingKey,
            0x000B => ExtensionTag::SchemaFingerprint,
            0x000C => ExtensionTag::ContentEncoding,
            0x000D => ExtensionTag::Method,
            0x000E => ExtensionTag::UserAgent,
            0x000F => ExtensionTag::AuthToken,
            0x0010 => ExtensionTag::Accept,
            0x0011 => ExtensionTag::CacheControl,
            0x0012 => ExtensionTag::IdempotencyKey,
            0x0013 => ExtensionTag::TraceId,
            0x0014 => ExtensionTag::ErrorCode,
            0x0015 => ExtensionTag::ErrorMessage,
            0x0016 => ExtensionTag::ChannelMode,
            0x0017 => ExtensionTag::ChannelPath,
            0x0018 => ExtensionTag::RedirectTarget,
            0x0019 => ExtensionTag::RedirectToken,
            0x001A => ExtensionTag::AckRangeStart,
            0x001B => ExtensionTag::AckRangeEnd,
            0x001C => ExtensionTag::FlowControlCredit,
            0x001D => ExtensionTag::Scope,
            0x001E => ExtensionTag::AuthMethod,
            other => ExtensionTag::Unknown(other),
        }
    }
}

impl From<ExtensionTag> for u16 {
    fn from(tag: ExtensionTag) -> Self {
        tag.code()
    }
}

impl std::fmt::Display for ExtensionTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionTag::Unknown(code) => write!(f, "Unknown(0x{:04x})", code),
            tag => write!(f, "{:?}", tag),
        }
    }
}

// ─── Extension Value ─────────────────────────────────────────────

/// The value of an extension tag.
///
/// Extension values can be text, binary, or numeric types.
/// The interpretation depends on the tag's defined value type.
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionValue {
    /// UTF-8 string value (for URI, content type, method, etc.)
    Text(String),
    /// Raw binary value (for session IDs, auth tokens, trace IDs, etc.)
    Binary(Vec<u8>),
    /// Unsigned 16-bit integer (for status codes, etc.)
    U16(u16),
    /// Unsigned 32-bit integer (for TTL, ack ranges, flow control credits, etc.)
    U32(u32),
    /// Unsigned 64-bit integer (for sequence numbers, timestamps, fingerprints, etc.)
    U64(u64),
    /// Signed 64-bit integer (for timestamps in nanoseconds, etc.)
    I64(i64),
    /// 128-bit binary value (for UUIDs, correlation IDs, etc.)
    U128([u8; 16]),
}

impl ExtensionValue {
    /// Get the value as a text string, if it is one.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ExtensionValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Get the value as raw bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            ExtensionValue::Text(s) => s.as_bytes().to_vec(),
            ExtensionValue::Binary(b) => b.clone(),
            ExtensionValue::U16(v) => v.to_be_bytes().to_vec(),
            ExtensionValue::U32(v) => v.to_be_bytes().to_vec(),
            ExtensionValue::U64(v) => v.to_be_bytes().to_vec(),
            ExtensionValue::I64(v) => v.to_be_bytes().to_vec(),
            ExtensionValue::U128(v) => v.to_vec(),
        }
    }

    /// Get the value as a u16, if applicable.
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            ExtensionValue::U16(v) => Some(*v),
            _ => None,
        }
    }

    /// Get the value as a u32, if applicable.
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ExtensionValue::U32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get the value as a u64, if applicable.
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            ExtensionValue::U64(v) => Some(*v),
            _ => None,
        }
    }
}

// ─── Extension ──────────────────────────────────────────────────

/// A single TLV extension entry in a frame header.
#[derive(Debug, Clone, PartialEq)]
pub struct Extension {
    pub tag: ExtensionTag,
    pub value: ExtensionValue,
}

impl Extension {
    /// Create a new extension with a text value.
    pub fn text(tag: ExtensionTag, value: impl Into<String>) -> Self {
        Self {
            tag,
            value: ExtensionValue::Text(value.into()),
        }
    }

    /// Create a new extension with a binary value.
    pub fn binary(tag: ExtensionTag, value: Vec<u8>) -> Self {
        Self {
            tag,
            value: ExtensionValue::Binary(value),
        }
    }

    /// Create a new extension with a u16 value.
    pub fn u16(tag: ExtensionTag, value: u16) -> Self {
        Self {
            tag,
            value: ExtensionValue::U16(value),
        }
    }

    /// Create a new extension with a u32 value.
    pub fn u32(tag: ExtensionTag, value: u32) -> Self {
        Self {
            tag,
            value: ExtensionValue::U32(value),
        }
    }

    /// Create a new extension with a u64 value.
    pub fn u64(tag: ExtensionTag, value: u64) -> Self {
        Self {
            tag,
            value: ExtensionValue::U64(value),
        }
    }

    /// Create a new extension with an i64 value.
    pub fn i64(tag: ExtensionTag, value: i64) -> Self {
        Self {
            tag,
            value: ExtensionValue::I64(value),
        }
    }

    /// Create a new extension with a 128-bit value (UUID, etc.).
    pub fn u128(tag: ExtensionTag, value: [u8; 16]) -> Self {
        Self {
            tag,
            value: ExtensionValue::U128(value),
        }
    }
}

// ─── TLV Encoding/Decoding ──────────────────────────────────────

/// Encode a list of extensions into TLV binary format.
///
/// Each extension is encoded as:
/// - Tag: 2 bytes (big-endian u16)
/// - Length: 2 bytes (big-endian u16)
/// - Value: `Length` bytes
pub fn encode_extensions(extensions: &[Extension]) -> Vec<u8> {
    let mut buf = Vec::new();
    for ext in extensions {
        // Tag (2 bytes, big-endian)
        buf.extend_from_slice(&ext.tag.code().to_be_bytes());
        // Value as bytes
        let value_bytes = ext.value.as_bytes();
        // Length (2 bytes, big-endian)
        buf.extend_from_slice(&(value_bytes.len() as u16).to_be_bytes());
        // Value
        buf.extend_from_slice(&value_bytes);
    }
    buf
}

/// Decode TLV extensions from binary data.
///
/// `count` is the number of TLV entries expected (from the HeaderCount field).
pub fn decode_extensions(data: &[u8], count: u8) -> FrameResult<Vec<Extension>> {
    let mut extensions = Vec::new();
    let mut offset = 0;

    for _ in 0..count {
        // Need at least 4 bytes for tag + length
        if data.len() < offset + 4 {
            return Err(FrameError::BufferTooShort {
                expected: offset + 4,
                actual: data.len(),
            });
        }

        // Tag (2 bytes, big-endian)
        let tag_code = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let tag = ExtensionTag::from(tag_code);
        offset += 2;

        // Length (2 bytes, big-endian)
        let length = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;

        // Value
        if data.len() < offset + length {
            return Err(FrameError::InvalidExtensionLength {
                tag: tag_code,
                length: length as u16,
                available: data.len() - offset,
            });
        }

        let value_bytes = &data[offset..offset + length];
        offset += length;

        // Determine the value type based on the tag's expected type
        let value = interpret_extension_value(tag_code, value_bytes);
        extensions.push(Extension { tag, value });
    }

    Ok(extensions)
}

/// Interpret extension value bytes based on the tag's expected type.
///
/// Well-known tags have defined value types. Unknown tags are treated as binary.
fn interpret_extension_value(tag_code: u16, bytes: &[u8]) -> ExtensionValue {
    match tag_code {
        // Text tags
        0x0001 | 0x0002 | 0x0003 | 0x000C | 0x000D | 0x000E | 0x0010 | 0x0011 |
        0x0014 | 0x0015 | 0x0016 | 0x0017 | 0x0018 | 0x001E => {
            ExtensionValue::Text(String::from_utf8_lossy(bytes).into_owned())
        }
        // U16 tags
        0x0004 => ExtensionValue::U16(u16::from_be_bytes(
            bytes.try_into().unwrap_or([0, 0]),
        )),
        // U32 tags
        0x0009 | 0x001A | 0x001B | 0x001C => ExtensionValue::U32(u32::from_be_bytes(
            bytes.try_into().unwrap_or([0, 0, 0, 0]),
        )),
        // U64 tags
        0x0006 | 0x000B => ExtensionValue::U64(u64::from_be_bytes(
            bytes.try_into().unwrap_or([0; 8]),
        )),
        // I64 tags (timestamps)
        0x0008 => ExtensionValue::I64(i64::from_be_bytes(
            bytes.try_into().unwrap_or([0; 8]),
        )),
        // 128-bit tags (UUIDs, correlation IDs, trace IDs)
        0x0005 | 0x0007 | 0x0012 | 0x0013 | 0x0019 => {
            let mut arr = [0u8; 16];
            let len = bytes.len().min(16);
            arr[..len].copy_from_slice(&bytes[..len]);
            ExtensionValue::U128(arr)
        }
        // Binary tags (auth tokens, etc.) or unknown
        _ => ExtensionValue::Binary(bytes.to_vec()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_tag_round_trip() {
        // Well-known tags
        assert_eq!(ExtensionTag::from(0x0001), ExtensionTag::RequestUri);
        assert_eq!(u16::from(ExtensionTag::RequestUri), 0x0001);
        assert_eq!(ExtensionTag::from(0x001D), ExtensionTag::Scope);
        assert_eq!(u16::from(ExtensionTag::Scope), 0x001D);

        // Unknown tags
        assert_eq!(ExtensionTag::from(0x8001), ExtensionTag::Unknown(0x8001));
        assert_eq!(u16::from(ExtensionTag::Unknown(0x8001)), 0x8001);
    }

    #[test]
    fn test_extension_tag_display() {
        assert_eq!(format!("{}", ExtensionTag::RequestUri), "RequestUri");
        assert_eq!(format!("{}", ExtensionTag::Unknown(0x8001)), "Unknown(0x8001)");
    }

    #[test]
    fn test_extension_value_accessors() {
        let text_val = ExtensionValue::Text("hello".to_string());
        assert_eq!(text_val.as_text(), Some("hello"));
        assert_eq!(text_val.as_u16(), None);

        let u16_val = ExtensionValue::U16(200);
        assert_eq!(u16_val.as_u16(), Some(200));
        assert_eq!(u16_val.as_text(), None);
    }

    #[test]
    fn test_extension_convenience_constructors() {
        let ext = Extension::text(ExtensionTag::RequestUri, "/orders/123");
        assert_eq!(ext.tag, ExtensionTag::RequestUri);
        assert_eq!(ext.value.as_text(), Some("/orders/123"));

        let ext = Extension::u16(ExtensionTag::StatusCode, 200);
        assert_eq!(ext.tag, ExtensionTag::StatusCode);
        assert_eq!(ext.value.as_u16(), Some(200));

        let ext = Extension::u64(ExtensionTag::SequenceNum, 42);
        assert_eq!(ext.tag, ExtensionTag::SequenceNum);
        assert_eq!(ext.value.as_u64(), Some(42));
    }

    #[test]
    fn test_encode_decode_extensions_round_trip() {
        let extensions = vec![
            Extension::text(ExtensionTag::RequestUri, "/accounts/123/orders"),
            Extension::u16(ExtensionTag::StatusCode, 200),
            Extension::u64(ExtensionTag::SequenceNum, 12345),
            Extension::i64(ExtensionTag::Timestamp, 1700000000000000000),
            Extension::binary(ExtensionTag::SessionId, vec![0xAA; 16]),
        ];

        let encoded = encode_extensions(&extensions);
        let decoded = decode_extensions(&encoded, 5).unwrap();

        assert_eq!(decoded.len(), 5);
        assert_eq!(decoded[0].tag, ExtensionTag::RequestUri);
        assert_eq!(decoded[0].value.as_text(), Some("/accounts/123/orders"));
        assert_eq!(decoded[1].tag, ExtensionTag::StatusCode);
        assert_eq!(decoded[1].value.as_u16(), Some(200));
        assert_eq!(decoded[2].tag, ExtensionTag::SequenceNum);
        assert_eq!(decoded[2].value.as_u64(), Some(12345));
        assert_eq!(decoded[3].tag, ExtensionTag::Timestamp);
        assert_eq!(decoded[3].value, ExtensionValue::I64(1700000000000000000));
        assert_eq!(decoded[4].tag, ExtensionTag::SessionId);
    }

    #[test]
    fn test_decode_extensions_buffer_too_short() {
        let data = [0x00, 0x01]; // Only 2 bytes, need at least 4 for one extension
        let result = decode_extensions(&data, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_extensions_invalid_length() {
        // Tag=0x0001, Length=100, but only 5 bytes of value
        let data: Vec<u8> = vec![
            0x00, 0x01, // tag
            0x00, 0x64, // length = 100
            0x48, 0x49, 0x4C, 0x4C, 0x4F, // "HELLO" (5 bytes, not 100)
        ];
        let result = decode_extensions(&data, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_content_type_constants() {
        assert_eq!(CONTENT_TYPE_CBOR, "application/cbor");
        assert_eq!(CONTENT_TYPE_SBE, "application/fig+sbe");
        assert_eq!(CONTENT_TYPE_JSON, "application/json");
        assert_eq!(CONTENT_TYPE_PROTOBUF, "application/x-protobuf");
    }
}