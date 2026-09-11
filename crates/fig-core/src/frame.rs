//! FIG frame wire format: encode, decode, and streaming parsing.
//!
//! The frame is the fundamental unit of communication in FIG. Every frame
//! has a fixed 16-byte header followed by optional TLV extensions and an
//! optional payload.
//!
//! ## Wire Layout
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                            Length (32)                        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |   Type (8)   |  Flags (8)   |        Channel ID (16)         |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |                        Stream Seq (32)                        |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! | HeaderCount  |  SchemaID    |          reserved (16)         |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |   Extension Block (variable: HeaderCount × TLV entries)       |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! |   Payload (variable)                                          |
//! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//! ```

use std::io;
use std::sync::atomic::Ordering;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::error::FrameError;
use crate::ext::{self, Extension, ExtensionTag};
use crate::observability::{span_decode, span_encode, METRICS};
use crate::FrameResult;

// ─── Constants ───────────────────────────────────────────────────

/// Size of the fixed frame header in bytes.
pub const HEADER_SIZE: usize = 16;

/// Minimum frame size (header only, no extensions, no payload).
pub const MIN_FRAME_SIZE: usize = HEADER_SIZE;

/// Reserved channel ID for connection-level control frames.
pub const CONTROL_CHANNEL: u16 = 0;

/// Maximum number of channels per connection.
pub const MAX_CHANNELS: u16 = u16::MAX;

// ─── Frame Type ──────────────────────────────────────────────────

/// Frame type codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameType {
    /// Connection-level control (PING, PONG, GOAWAY, SETTINGS, etc.)
    Control = 0x00,
    /// Expects exactly one RESPONSE on this channel
    Request = 0x01,
    /// Final response to a REQUEST; channel closes after this
    Response = 0x02,
    /// Opens a new stream; carries channel metadata
    StreamOpen = 0x03,
    /// One item in an open stream (bidirectional data)
    StreamItem = 0x04,
    /// Graceful close of a stream
    StreamClose = 0x05,
    /// Error that terminates the stream
    StreamError = 0x06,
    /// Fire-and-forget; no response expected
    OneWay = 0x07,
    /// Subscribe to a topic/routing key
    Subscribe = 0x08,
    /// Unsubscribe from a topic/routing key
    Unsubscribe = 0x09,
    /// Acknowledge receipt of a range of Stream Seq numbers
    AckRange = 0x0A,
    /// Message-level backpressure: "accept at most N more messages"
    FlowControl = 0x0B,
    /// Channel migrated to a different server; reconnect with token
    Redirect = 0x0C,
}

impl FrameType {
    /// Get the numeric code for this frame type.
    pub fn code(&self) -> u8 {
        *self as u8
    }

    /// Convert a numeric code to a FrameType, or None if invalid.
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x00 => Some(FrameType::Control),
            0x01 => Some(FrameType::Request),
            0x02 => Some(FrameType::Response),
            0x03 => Some(FrameType::StreamOpen),
            0x04 => Some(FrameType::StreamItem),
            0x05 => Some(FrameType::StreamClose),
            0x06 => Some(FrameType::StreamError),
            0x07 => Some(FrameType::OneWay),
            0x08 => Some(FrameType::Subscribe),
            0x09 => Some(FrameType::Unsubscribe),
            0x0A => Some(FrameType::AckRange),
            0x0B => Some(FrameType::FlowControl),
            0x0C => Some(FrameType::Redirect),
            _ => None,
        }
    }

    /// Check if this frame type can carry a payload.
    pub fn has_payload(&self) -> bool {
        !matches!(self, FrameType::Control | FrameType::Unsubscribe)
    }
}

impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::Control => write!(f, "CONTROL"),
            FrameType::Request => write!(f, "REQUEST"),
            FrameType::Response => write!(f, "RESPONSE"),
            FrameType::StreamOpen => write!(f, "STREAM_OPEN"),
            FrameType::StreamItem => write!(f, "STREAM_ITEM"),
            FrameType::StreamClose => write!(f, "STREAM_CLOSE"),
            FrameType::StreamError => write!(f, "STREAM_ERROR"),
            FrameType::OneWay => write!(f, "ONE_WAY"),
            FrameType::Subscribe => write!(f, "SUBSCRIBE"),
            FrameType::Unsubscribe => write!(f, "UNSUBSCRIBE"),
            FrameType::AckRange => write!(f, "ACK_RANGE"),
            FrameType::FlowControl => write!(f, "FLOW_CONTROL"),
            FrameType::Redirect => write!(f, "REDIRECT"),
        }
    }
}

// ─── Control Sub-Types ───────────────────────────────────────────

/// Control frame sub-types (carried in the first byte of the payload
/// for CONTROL frames on channel 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlSubtype {
    Ping = 0x00,
    Pong = 0x01,
    Goaway = 0x02,
    Settings = 0x03,
    AuthRefresh = 0x04,
    SeqReset = 0x05,
    Resend = 0x06,
}

impl ControlSubtype {
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x00 => Some(ControlSubtype::Ping),
            0x01 => Some(ControlSubtype::Pong),
            0x02 => Some(ControlSubtype::Goaway),
            0x03 => Some(ControlSubtype::Settings),
            0x04 => Some(ControlSubtype::AuthRefresh),
            0x05 => Some(ControlSubtype::SeqReset),
            0x06 => Some(ControlSubtype::Resend),
            _ => None,
        }
    }

    pub fn code(&self) -> u8 {
        *self as u8
    }
}

// ─── Flags ───────────────────────────────────────────────────────

bitflags::bitflags! {
    /// Frame header flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Flags: u8 {
        /// Extension block present (HeaderCount > 0)
        const EXTENSIONS = 0x01;
        /// Payload compressed (algorithm from SETTINGS negotiation)
        const COMPRESSED = 0x02;
        /// High-priority message — expedite delivery
        const PRIORITY = 0x04;
        /// Payload encrypted at application layer (E2E)
        const ENCRYPTED = 0x08;
        /// Message continues in subsequent frame(s)
        const FRAGMENTED = 0x10;
        /// This is the last fragment of a fragmented message
        const LAST_FRAGMENT = 0x20;
        /// Sender requests an ACK_RANGE for this message
        const ACK_REQUESTED = 0x40;
        // Bit 7 reserved — must be 0
    }
}

impl Flags {
    /// Create flags with no bits set.
    ///
    /// This is equivalent to `Flags::from_bits_truncate(0)`.
    pub fn none() -> Self {
        Flags::from_bits_truncate(0)
    }
}

// ─── Frame ───────────────────────────────────────────────────────

/// A FIG frame — the fundamental unit of communication.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// Frame type (REQUEST, RESPONSE, STREAM_ITEM, etc.)
    pub frame_type: FrameType,
    /// Frame flags (EXTENSIONS, COMPRESSED, PRIORITY, etc.)
    pub flags: Flags,
    /// Channel ID (0 = control, 1–65535 = data channels)
    pub channel_id: u16,
    /// Monotonically increasing per-channel sequence number
    pub stream_seq: u32,
    /// Schema ID for the payload (0 = no schema/self-describing)
    pub schema_id: u8,
    /// TLV extension entries
    pub extensions: Vec<Extension>,
    /// Payload bytes (schema-defined or raw)
    pub payload: Vec<u8>,
}

impl Frame {
    /// Create a new frame with the given type and channel ID.
    pub fn new(frame_type: FrameType, channel_id: u16) -> Self {
        Self {
            frame_type,
            flags: Flags::none(),
            channel_id,
            stream_seq: 0,
            schema_id: 0,
            extensions: Vec::new(),
            payload: Vec::new(),
        }
    }

    /// Create a control frame on channel 0.
    pub fn control(subtype: ControlSubtype) -> Self {
        let mut frame = Self::new(FrameType::Control, CONTROL_CHANNEL);
        frame.payload = vec![subtype.code()];
        frame
    }

    /// Create a PING control frame.
    pub fn ping() -> Self {
        Self::control(ControlSubtype::Ping)
    }

    /// Create a PONG control frame.
    pub fn pong() -> Self {
        Self::control(ControlSubtype::Pong)
    }

    /// Create a GOAWAY control frame with an optional reason.
    pub fn goaway(reason: impl Into<String>) -> Self {
        let mut frame = Self::control(ControlSubtype::Goaway);
        // Append reason string after the subtype byte
        let reason_bytes = reason.into().into_bytes();
        frame.payload.extend_from_slice(&reason_bytes);
        frame
    }

    /// Create a SETTINGS control frame.
    pub fn settings() -> Self {
        Self::control(ControlSubtype::Settings)
    }

    /// Create an AUTH_REFRESH control frame with a new auth token.
    pub fn auth_refresh(token: Vec<u8>) -> Self {
        let mut frame = Self::control(ControlSubtype::AuthRefresh);
        frame.payload.extend_from_slice(&token);
        frame
    }

    /// Create a SEQ_RESET control frame to reset sequence numbers for a channel.
    ///
    /// Payload layout (after subtype byte):
    /// - channel_id: 2 bytes BE
    /// - new_send_seq: 4 bytes BE
    /// - new_recv_seq: 4 bytes BE
    pub fn seq_reset(channel_id: u16, new_send_seq: u32, new_recv_seq: u32) -> Self {
        let mut frame = Self::control(ControlSubtype::SeqReset);
        frame.payload.extend_from_slice(&channel_id.to_be_bytes());
        frame.payload.extend_from_slice(&new_send_seq.to_be_bytes());
        frame.payload.extend_from_slice(&new_recv_seq.to_be_bytes());
        frame
    }

    /// Create a RESEND control frame requesting retransmission of a seq range.
    ///
    /// Payload layout (after subtype byte):
    /// - channel_id: 2 bytes BE
    /// - begin_seq: 4 bytes BE
    /// - end_seq: 4 bytes BE (0 = all messages to current)
    pub fn resend(channel_id: u16, begin_seq: u32, end_seq: u32) -> Self {
        let mut frame = Self::control(ControlSubtype::Resend);
        frame.payload.extend_from_slice(&channel_id.to_be_bytes());
        frame.payload.extend_from_slice(&begin_seq.to_be_bytes());
        frame.payload.extend_from_slice(&end_seq.to_be_bytes());
        frame
    }

    /// Extract the resend range from a RESEND control frame.
    ///
    /// Returns `(channel_id, begin_seq, end_seq)` if this is a valid RESEND
    /// frame with a complete payload.
    pub fn resend_range(&self) -> Option<(u16, u32, u32)> {
        if self.control_subtype() != Some(ControlSubtype::Resend) {
            return None;
        }
        if self.payload.len() < 11 {
            return None;
        }
        let channel_id = u16::from_be_bytes([self.payload[1], self.payload[2]]);
        let begin_seq = u32::from_be_bytes([
            self.payload[3],
            self.payload[4],
            self.payload[5],
            self.payload[6],
        ]);
        let end_seq = u32::from_be_bytes([
            self.payload[7],
            self.payload[8],
            self.payload[9],
            self.payload[10],
        ]);
        Some((channel_id, begin_seq, end_seq))
    }

    /// Set the stream sequence number.
    pub fn with_seq(mut self, seq: u32) -> Self {
        self.stream_seq = seq;
        self
    }

    /// Set the schema ID.
    pub fn with_schema_id(mut self, schema_id: u8) -> Self {
        self.schema_id = schema_id;
        self
    }

    /// Add an extension.
    pub fn with_extension(mut self, ext: Extension) -> Self {
        self.extensions.push(ext);
        self.flags |= Flags::EXTENSIONS;
        self
    }

    /// Set the payload.
    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }

    /// Set the PRIORITY flag.
    pub fn with_priority(mut self) -> Self {
        self.flags |= Flags::PRIORITY;
        self
    }

    /// Set the ACK_REQUESTED flag.
    pub fn with_ack_requested(mut self) -> Self {
        self.flags |= Flags::ACK_REQUESTED;
        self
    }

    /// Get the control subtype if this is a Control frame.
    ///
    /// Returns `None` if the frame is not a Control frame or the payload
    /// does not contain a valid control subtype.
    pub fn control_subtype(&self) -> Option<ControlSubtype> {
        if self.frame_type != FrameType::Control {
            return None;
        }
        if self.payload.is_empty() {
            return None;
        }
        ControlSubtype::from_code(self.payload[0])
    }

    /// Extract the auth refresh token from an AUTH_REFRESH control frame.
    ///
    /// Returns the token bytes after the subtype byte. Returns `None` if
    /// this is not an AUTH_REFRESH frame or the payload has no token.
    pub fn auth_refresh_token(&self) -> Option<&[u8]> {
        if let Some(ControlSubtype::AuthRefresh) = self.control_subtype() {
            if self.payload.len() > 1 {
                return Some(&self.payload[1..]);
            }
        }
        None
    }

    /// Create a FLOW_CONTROL frame granting credits for a channel.
    pub fn flow_credit(channel_id: u16, credits: u32) -> Self {
        Self::new(FrameType::FlowControl, channel_id)
            .with_extension(Extension::u32(ExtensionTag::FlowControlCredit, credits))
    }

    /// Get the total encoded size of this frame in bytes.
    pub fn encoded_size(&self) -> usize {
        let ext_len = if self.extensions.is_empty() {
            0
        } else {
            ext::encode_extensions(&self.extensions).len()
        };
        HEADER_SIZE + ext_len + self.payload.len()
    }

    /// Encode this frame into bytes (big-endian, network byte order).
    pub fn encode(&self) -> FrameResult<Vec<u8>> {
        let _span = span_encode(self.channel_id, &self.frame_type.to_string()).entered();
        METRICS.frames_encoded.fetch_add(1, Ordering::Relaxed);

        // Encode extensions
        let ext_bytes = if !self.extensions.is_empty() {
            ext::encode_extensions(&self.extensions)
        } else {
            Vec::new()
        };

        let total_length = HEADER_SIZE + ext_bytes.len() + self.payload.len();
        if total_length > u32::MAX as usize {
            return Err(FrameError::InvalidHeader(total_length));
        }

        let header_count = self.extensions.len() as u8;
        let flags = if !self.extensions.is_empty() {
            self.flags | Flags::EXTENSIONS
        } else {
            self.flags
        };

        let mut buf = Vec::with_capacity(total_length);

        // Length (4 bytes, big-endian)
        buf.write_u32::<BigEndian>(total_length as u32)?;
        // Type (1 byte)
        buf.write_u8(self.frame_type.code())?;
        // Flags (1 byte)
        buf.write_u8(flags.bits())?;
        // Channel ID (2 bytes, big-endian)
        buf.write_u16::<BigEndian>(self.channel_id)?;
        // Stream Seq (4 bytes, big-endian)
        buf.write_u32::<BigEndian>(self.stream_seq)?;
        // HeaderCount (1 byte)
        buf.write_u8(header_count)?;
        // SchemaID (1 byte)
        buf.write_u8(self.schema_id)?;
        // Reserved (2 bytes)
        buf.write_u16::<BigEndian>(0)?;

        // Extensions
        buf.extend_from_slice(&ext_bytes);

        // Payload
        buf.extend_from_slice(&self.payload);

        Ok(buf)
    }

    /// Decode a frame from bytes.
    ///
    /// Returns the decoded frame and the number of bytes consumed.
    /// If there are not enough bytes for a complete frame, returns
    /// `FrameError::BufferTooShort`.
    pub fn decode(data: &[u8]) -> FrameResult<(Frame, usize)> {
        if data.len() < HEADER_SIZE {
            return Err(FrameError::BufferTooShort {
                expected: HEADER_SIZE,
                actual: data.len(),
            });
        }

        let channel_id = u16::from_be_bytes([data[6], data[7]]);
        let _span = span_decode(channel_id).entered();
        METRICS.frames_decoded.fetch_add(1, Ordering::Relaxed);

        let mut cursor = io::Cursor::new(data);

        // Length (4 bytes, big-endian)
        let length = cursor.read_u32::<BigEndian>()? as usize;
        if length < HEADER_SIZE {
            return Err(FrameError::InvalidHeader(length));
        }
        if data.len() < length {
            return Err(FrameError::BufferTooShort {
                expected: length,
                actual: data.len(),
            });
        }

        // Type (1 byte)
        let type_code = cursor.read_u8()?;
        let frame_type =
            FrameType::from_code(type_code).ok_or(FrameError::InvalidFrameType(type_code))?;

        // Flags (1 byte) — bit 7 is reserved and must be 0 (Spec §5)
        let flags_bits = cursor.read_u8()?;
        if flags_bits & 0x80 != 0 {
            return Err(FrameError::InvalidFlags(flags_bits));
        }
        let flags = Flags::from_bits_truncate(flags_bits);

        // Channel ID (2 bytes, big-endian)
        let channel_id = cursor.read_u16::<BigEndian>()?;

        // Stream Seq (4 bytes, big-endian)
        let stream_seq = cursor.read_u32::<BigEndian>()?;

        // HeaderCount (1 byte)
        let header_count = cursor.read_u8()?;

        // SchemaID (1 byte)
        let schema_id = cursor.read_u8()?;

        // Reserved (2 bytes) — must be 0 (Spec §3.1)
        let reserved = cursor.read_u16::<BigEndian>()?;
        if reserved != 0 {
            return Err(FrameError::NonZeroReserved(reserved));
        }

        // Extensions: parse the extension block to find where payload starts
        // We walk through the TLV entries to determine the extension block size,
        // then decode them all at once.
        let mut ext_block_end = HEADER_SIZE;
        let extensions = if header_count > 0 {
            let mut offset = HEADER_SIZE;
            for _ in 0..header_count {
                // Bound against the declared frame length, not the buffer size.
                // A longer buffer (fuzz / coalesced reads) must not walk past `length`.
                if length < offset + 4 {
                    return Err(FrameError::BufferTooShort {
                        expected: offset + 4,
                        actual: length,
                    });
                }
                // Read tag (2 bytes) and length (2 bytes)
                let tag_code = u16::from_be_bytes([data[offset], data[offset + 1]]);
                let value_len = u16::from_be_bytes([data[offset + 2], data[offset + 3]]) as usize;
                offset += 4;
                if length < offset + value_len {
                    return Err(FrameError::InvalidExtensionLength {
                        tag: tag_code,
                        length: value_len as u16,
                        available: length - offset,
                    });
                }
                offset += value_len;
            }
            ext_block_end = offset;
            let ext_data = &data[HEADER_SIZE..offset];
            ext::decode_extensions(ext_data, header_count)?
        } else {
            Vec::new()
        };

        // Payload starts after the extension block (ext_block_end <= length).
        let payload = data[ext_block_end..length].to_vec();

        Ok((
            Frame {
                frame_type,
                flags,
                channel_id,
                stream_seq,
                schema_id,
                extensions,
                payload,
            },
            length,
        ))
    }
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Frame({} ch={} seq={} schema=0x{:02x} exts={} payload={}B)",
            self.frame_type,
            self.channel_id,
            self.stream_seq,
            self.schema_id,
            self.extensions.len(),
            self.payload.len()
        )
    }
}

// ─── FrameDecoder (Streaming) ────────────────────────────────────

/// A streaming frame decoder that handles partial TREE stream reads.
///
/// TREE stream reads return variable-length chunks. A frame may arrive
/// split across multiple reads. The decoder buffers incoming bytes and
/// yields complete frames when enough data is available.
///
/// ## Usage
///
/// ```ignore
/// let mut decoder = FrameDecoder::new();
/// decoder.feed(&partial_chunk_1);
/// // No complete frame yet — decode_next returns None
/// decoder.feed(&partial_chunk_2);
/// // Now we have a complete frame
/// if let Some(result) = decoder.decode_next() {
///     let frame = result?;
///     // process frame
/// }
/// ```
#[derive(Debug)]
pub struct FrameDecoder {
    /// Internal buffer for accumulating partial reads.
    buffer: Vec<u8>,
}

impl FrameDecoder {
    /// Create a new decoder with an empty buffer.
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Create a decoder with a pre-allocated buffer capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Feed bytes into the decoder's internal buffer.
    ///
    /// Call `decode_next()` after feeding to check if a complete
    /// frame is available.
    pub fn feed(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Attempt to decode one complete frame from the buffer.
    ///
    /// Returns:
    /// - `Some(Ok(frame))` — a complete frame was decoded and removed from the buffer
    /// - `Some(Err(e))` — a frame was present but failed to decode
    /// - `None` — not enough data for a complete frame yet
    pub fn decode_next(&mut self) -> Option<FrameResult<Frame>> {
        // Need at least 4 bytes to read the length
        if self.buffer.len() < 4 {
            return None;
        }

        // Read the length field (first 4 bytes, big-endian)
        let length = u32::from_be_bytes([
            self.buffer[0],
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
        ]) as usize;

        // Validate minimum length
        if length < HEADER_SIZE {
            let err = Err(FrameError::InvalidHeader(length));
            // Remove the invalid frame from the buffer
            self.buffer.drain(..4.min(self.buffer.len()));
            return Some(err);
        }

        // Check if we have the complete frame
        if self.buffer.len() < length {
            return None; // Not enough data yet
        }

        // We have a complete frame — decode it
        let frame_data: Vec<u8> = self.buffer.drain(..length).collect();
        let (frame, _consumed) = match Frame::decode(&frame_data) {
            Ok(result) => result,
            Err(e) => return Some(Err(e)),
        };

        Some(Ok(frame))
    }

    /// Attempt to decode all complete frames from the buffer.
    ///
    /// Returns a vector of results. Stops when no more complete frames
    /// are available.
    pub fn decode_all(&mut self) -> Vec<FrameResult<Frame>> {
        let mut results = Vec::new();
        while let Some(result) = self.decode_next() {
            results.push(result);
        }
        results
    }

    /// Get the number of bytes currently buffered.
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    /// Clear the internal buffer.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for FrameDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext::ExtensionTag;

    #[test]
    fn test_frame_type_round_trip() {
        for code in 0x00..=0x0C {
            let ft = FrameType::from_code(code).unwrap();
            assert_eq!(ft.code(), code);
        }
        assert!(FrameType::from_code(0x0D).is_none());
        assert!(FrameType::from_code(0xFF).is_none());
    }

    #[test]
    fn test_frame_type_display() {
        assert_eq!(FrameType::Control.to_string(), "CONTROL");
        assert_eq!(FrameType::Request.to_string(), "REQUEST");
        assert_eq!(FrameType::StreamItem.to_string(), "STREAM_ITEM");
    }

    #[test]
    fn test_control_subtype_round_trip() {
        for code in 0x00..=0x06 {
            let ct = ControlSubtype::from_code(code).unwrap();
            assert_eq!(ct.code(), code);
        }
        assert!(ControlSubtype::from_code(0x07).is_none());
    }

    #[test]
    fn test_flags() {
        let flags = Flags::EXTENSIONS | Flags::PRIORITY | Flags::ACK_REQUESTED;
        assert!(flags.contains(Flags::EXTENSIONS));
        assert!(flags.contains(Flags::PRIORITY));
        assert!(flags.contains(Flags::ACK_REQUESTED));
        assert!(!flags.contains(Flags::COMPRESSED));
        assert!(!flags.contains(Flags::ENCRYPTED));
    }

    #[test]
    fn test_frame_new() {
        let frame = Frame::new(FrameType::Request, 42);
        assert_eq!(frame.frame_type, FrameType::Request);
        assert_eq!(frame.channel_id, 42);
        assert_eq!(frame.stream_seq, 0);
        assert_eq!(frame.schema_id, 0);
        assert!(frame.extensions.is_empty());
        assert!(frame.payload.is_empty());
    }

    #[test]
    fn test_frame_control_shortcuts() {
        let ping = Frame::ping();
        assert_eq!(ping.frame_type, FrameType::Control);
        assert_eq!(ping.channel_id, CONTROL_CHANNEL);
        assert_eq!(ping.payload[0], ControlSubtype::Ping.code());

        let pong = Frame::pong();
        assert_eq!(pong.payload[0], ControlSubtype::Pong.code());

        let goaway = Frame::goaway("shutting down");
        assert_eq!(goaway.payload[0], ControlSubtype::Goaway.code());
        assert_eq!(&goaway.payload[1..], b"shutting down");
    }

    #[test]
    fn test_frame_builder_pattern() {
        let frame = Frame::new(FrameType::Request, 100)
            .with_seq(42)
            .with_schema_id(0x01)
            .with_extension(Extension::text(ExtensionTag::RequestUri, "/orders"))
            .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
            .with_payload(b"hello".to_vec())
            .with_priority()
            .with_ack_requested();

        assert_eq!(frame.stream_seq, 42);
        assert_eq!(frame.schema_id, 0x01);
        assert_eq!(frame.extensions.len(), 2);
        assert_eq!(frame.payload, b"hello");
        assert!(frame.flags.contains(Flags::PRIORITY));
        assert!(frame.flags.contains(Flags::ACK_REQUESTED));
        assert!(frame.flags.contains(Flags::EXTENSIONS));
    }

    #[test]
    fn test_encode_decode_minimal_frame() {
        let frame = Frame::new(FrameType::Control, CONTROL_CHANNEL);
        let encoded = frame.encode().unwrap();
        let (decoded, consumed) = Frame::decode(&encoded).unwrap();

        assert_eq!(consumed, HEADER_SIZE);
        assert_eq!(decoded.frame_type, FrameType::Control);
    }

    #[test]
    fn test_encode_decode_all_frame_types() {
        let frame_types = vec![
            FrameType::Control,
            FrameType::Request,
            FrameType::Response,
            FrameType::StreamOpen,
            FrameType::StreamItem,
            FrameType::StreamClose,
            FrameType::StreamError,
            FrameType::OneWay,
            FrameType::Subscribe,
            FrameType::Unsubscribe,
            FrameType::AckRange,
            FrameType::FlowControl,
            FrameType::Redirect,
        ];

        for ft in frame_types {
            let frame = Frame::new(ft, 1)
                .with_seq(100)
                .with_payload(b"test".to_vec());
            let encoded = frame.encode().unwrap();
            let (decoded, consumed) = Frame::decode(&encoded).unwrap();

            assert_eq!(consumed, encoded.len());
            assert_eq!(decoded.frame_type, ft);
            assert_eq!(decoded.channel_id, 1);
            assert_eq!(decoded.stream_seq, 100);
            assert_eq!(decoded.payload, b"test");
        }
    }

    #[test]
    fn test_encode_decode_with_extensions() {
        let frame = Frame::new(FrameType::Request, 5)
            .with_seq(1)
            .with_extension(Extension::text(
                ExtensionTag::RequestUri,
                "/accounts/123/orders",
            ))
            .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
            .with_extension(Extension::u64(ExtensionTag::SequenceNum, 9999))
            .with_payload(b"order data".to_vec());

        let encoded = frame.encode().unwrap();
        let (decoded, consumed) = Frame::decode(&encoded).unwrap();

        assert_eq!(consumed, encoded.len());
        assert_eq!(decoded.frame_type, FrameType::Request);
        assert_eq!(decoded.channel_id, 5);
        assert_eq!(decoded.stream_seq, 1);
        assert_eq!(decoded.extensions.len(), 3);
        assert_eq!(decoded.extensions[0].tag, ExtensionTag::RequestUri);
        assert_eq!(
            decoded.extensions[0].value.as_text(),
            Some("/accounts/123/orders")
        );
        assert_eq!(decoded.extensions[1].tag, ExtensionTag::StatusCode);
        assert_eq!(decoded.extensions[1].value.as_u16(), Some(200));
        assert_eq!(decoded.extensions[2].tag, ExtensionTag::SequenceNum);
        assert_eq!(decoded.extensions[2].value.as_u64(), Some(9999));
        assert_eq!(decoded.payload, b"order data");
    }

    #[test]
    fn test_encode_decode_with_all_flags() {
        let frame = Frame::new(FrameType::StreamItem, 10)
            .with_seq(42)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                "marketdata/AAPL",
            ))
            .with_payload(b"market data".to_vec());

        let mut frame = frame;
        frame.flags = Flags::EXTENSIONS | Flags::PRIORITY | Flags::ACK_REQUESTED;

        let encoded = frame.encode().unwrap();
        let (decoded, _) = Frame::decode(&encoded).unwrap();

        assert!(decoded.flags.contains(Flags::EXTENSIONS));
        assert!(decoded.flags.contains(Flags::PRIORITY));
        assert!(decoded.flags.contains(Flags::ACK_REQUESTED));
    }

    #[test]
    fn test_decode_buffer_too_short() {
        let data = [0u8; 10]; // Less than HEADER_SIZE
        let result = Frame::decode(&data);
        assert!(matches!(result, Err(FrameError::BufferTooShort { .. })));
    }

    #[test]
    fn test_decode_rejects_extensions_past_declared_length() {
        // Declared length 17, header_count 1, but the buffer is longer so a naive
        // walk of `data.len()` would parse a TLV past byte 17 and panic on payload.
        let mut data = vec![0u8; 32];
        data[0..4].copy_from_slice(&17u32.to_be_bytes());
        data[4] = FrameType::Request.code();
        data[12] = 1; // HeaderCount
        data[13] = 1; // SchemaID
                      // TLV at offset 16: tag + value_len=0 would end at 20 > 17
        data[16] = 0x00;
        data[17] = 0x17;
        data[18] = 0x00;
        data[19] = 0x00;
        let result = Frame::decode(&data);
        assert!(
            matches!(
                result,
                Err(FrameError::BufferTooShort { .. })
                    | Err(FrameError::InvalidExtensionLength { .. })
            ),
            "got {result:?}"
        );
    }

    #[test]
    fn test_decode_invalid_frame_type() {
        let mut buf = Vec::new();
        // Length = 16 (header only)
        buf.write_u32::<BigEndian>(16).unwrap();
        // Invalid frame type
        buf.write_u8(0xFF).unwrap();
        // Flags
        buf.write_u8(0).unwrap();
        // Channel ID
        buf.write_u16::<BigEndian>(1).unwrap();
        // Stream Seq
        buf.write_u32::<BigEndian>(0).unwrap();
        // HeaderCount + SchemaID + Reserved
        buf.write_u8(0).unwrap();
        buf.write_u8(0).unwrap();
        buf.write_u16::<BigEndian>(0).unwrap();

        let result = Frame::decode(&buf);
        assert!(matches!(result, Err(FrameError::InvalidFrameType(0xFF))));
    }

    #[test]
    fn test_frame_decoder_basic() {
        let frame = Frame::new(FrameType::Request, 1)
            .with_seq(42)
            .with_payload(b"hello world".to_vec());

        let encoded = frame.encode().unwrap();

        let mut decoder = FrameDecoder::new();

        // Feed the entire frame at once
        decoder.feed(&encoded);
        let result = decoder.decode_next().unwrap().unwrap();

        assert_eq!(result.frame_type, FrameType::Request);
        assert_eq!(result.channel_id, 1);
        assert_eq!(result.stream_seq, 42);
        assert_eq!(result.payload, b"hello world");
    }

    #[test]
    fn test_frame_decoder_partial_reads() {
        let frame = Frame::new(FrameType::Response, 5)
            .with_seq(99)
            .with_payload(b"response data".to_vec());

        let encoded = frame.encode().unwrap();

        let mut decoder = FrameDecoder::new();

        // Feed first 4 bytes (just the length field)
        decoder.feed(&encoded[..4]);
        assert!(decoder.decode_next().is_none()); // Not enough data yet

        // Feed the rest
        decoder.feed(&encoded[4..]);
        let result = decoder.decode_next().unwrap().unwrap();

        assert_eq!(result.frame_type, FrameType::Response);
        assert_eq!(result.channel_id, 5);
        assert_eq!(result.stream_seq, 99);
        assert_eq!(result.payload, b"response data");
    }

    #[test]
    fn test_frame_decoder_multiple_frames() {
        let frame1 = Frame::new(FrameType::Request, 1)
            .with_seq(1)
            .with_payload(b"first".to_vec());
        let frame2 = Frame::new(FrameType::Response, 1)
            .with_seq(2)
            .with_payload(b"second".to_vec());

        let mut data = Vec::new();
        data.extend_from_slice(&frame1.encode().unwrap());
        data.extend_from_slice(&frame2.encode().unwrap());

        let mut decoder = FrameDecoder::new();
        decoder.feed(&data);

        let results = decoder.decode_all();
        assert_eq!(results.len(), 2);

        let f1 = results[0].as_ref().unwrap();
        assert_eq!(f1.frame_type, FrameType::Request);
        assert_eq!(f1.payload, b"first");

        let f2 = results[1].as_ref().unwrap();
        assert_eq!(f2.frame_type, FrameType::Response);
        assert_eq!(f2.payload, b"second");
    }

    #[test]
    fn test_frame_decoder_byte_by_byte() {
        let frame = Frame::new(FrameType::StreamItem, 3)
            .with_seq(7)
            .with_payload(b"stream".to_vec());

        let encoded = frame.encode().unwrap();

        let mut decoder = FrameDecoder::new();

        // Feed one byte at a time
        for i in 0..encoded.len() {
            decoder.feed(&encoded[i..i + 1]);
            if i < encoded.len() - 1 {
                assert!(
                    decoder.decode_next().is_none(),
                    "Should not have complete frame at byte {}",
                    i
                );
            }
        }

        let result = decoder.decode_next().unwrap().unwrap();
        assert_eq!(result.frame_type, FrameType::StreamItem);
        assert_eq!(result.channel_id, 3);
        assert_eq!(result.stream_seq, 7);
        assert_eq!(result.payload, b"stream");
    }

    #[test]
    fn test_frame_decoder_invalid_header() {
        let mut decoder = FrameDecoder::new();

        // Feed a frame with length < HEADER_SIZE (invalid)
        let mut buf = Vec::new();
        buf.write_u32::<BigEndian>(10).unwrap(); // length = 10, less than HEADER_SIZE (16)
        buf.write_u8(0x01).unwrap(); // type
        buf.write_u8(0).unwrap(); // flags
        buf.extend_from_slice(&[0u8; 10]); // padding

        decoder.feed(&buf);
        let result = decoder.decode_next().unwrap();
        assert!(matches!(result, Err(FrameError::InvalidHeader(10))));
    }

    #[test]
    fn test_frame_encoded_size() {
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"hello".to_vec());

        assert_eq!(frame.encoded_size(), HEADER_SIZE + 5); // 16 header + 5 payload
    }

    #[test]
    fn test_frame_encoded_size_with_extensions() {
        let frame = Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(ExtensionTag::RequestUri, "/orders"))
            .with_payload(b"data".to_vec());

        let size = frame.encoded_size();
        let encoded = frame.encode().unwrap();
        assert_eq!(size, encoded.len());
    }

    #[test]
    fn test_decode_rejects_non_zero_reserved() {
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"test".to_vec());
        let mut encoded = frame.encode().unwrap();
        // Set reserved field (bytes 14-15) to non-zero.
        encoded[14] = 0x00;
        encoded[15] = 0x01;

        let result = Frame::decode(&encoded);
        assert!(matches!(result, Err(FrameError::NonZeroReserved(1))));
    }

    #[test]
    fn test_decode_rejects_reserved_flag_bit() {
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"test".to_vec());
        let mut encoded = frame.encode().unwrap();
        // Set reserved flag bit 7 in the flags byte (offset 5).
        encoded[5] |= 0x80;

        let result = Frame::decode(&encoded);
        assert!(matches!(result, Err(FrameError::InvalidFlags(0x80))));
    }

    #[test]
    fn test_frame_display() {
        let frame = Frame::new(FrameType::Request, 42).with_seq(100);
        let display = format!("{}", frame);
        assert!(display.contains("REQUEST"));
        assert!(display.contains("ch=42"));
        assert!(display.contains("seq=100"));
    }
}
