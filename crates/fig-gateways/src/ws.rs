//! WebSocket protocol adapter (RFC 6455).
//!
//! Parses and serializes WebSocket frames, and maps between WebSocket semantics
//! and FIG Stream frames.
//!
//! # WebSocket Frame Format (RFC 6455)
//!
//! ```text
//!  0                   1                   2                   3
//!  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//! +-+-+-+-+-------+-+-------------+-------------------------------+
//! |F|R|R|R| opcode|M| Payload len |    Extended payload length    |
//! |I|S|S|S|  (4)  |A|     (7)     |          (16/64)              |
//! |N|V|V|V|       |S|             | (if payload len == 126/127)   |
//! | |1|2|3|       |K|             |                               |
//! +-+-+-+-+-------+-+-------------+- - - - - - - - - - - - - - - +
//! |     Extended payload length continued, if payload len == 127  |
//! + - - - - - - - - - - - - - - - +-------------------------------+
//! |                               |  Masking key (if MASK set)    |
//! +-------------------------------+-------------------------------+
//! |  Masking key (continued)      |          Payload Data         |
//! +-------------------------------+ - - - - - - - - - - - - - - - +
//! :                     Payload Data continued ...                :
//! +---------------------------------------------------------------+
//! ```

use thiserror::Error;

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};

// ─── WebSocket Types ──────────────────────────────────────────────

/// WebSocket frame opcodes (RFC 6455 §5.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsOpcode {
    /// Continuation frame (0x0)
    Continuation,
    /// Text frame (0x1)
    Text,
    /// Binary frame (0x2)
    Binary,
    /// Connection close (0x8)
    Close,
    /// Ping (0x9)
    Ping,
    /// Pong (0xA)
    Pong,
}

impl WsOpcode {
    /// Get the numeric opcode value.
    pub fn code(&self) -> u8 {
        match self {
            WsOpcode::Continuation => 0x0,
            WsOpcode::Text => 0x1,
            WsOpcode::Binary => 0x2,
            WsOpcode::Close => 0x8,
            WsOpcode::Ping => 0x9,
            WsOpcode::Pong => 0xA,
        }
    }

    /// Create an opcode from its numeric value.
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0x0 => Some(WsOpcode::Continuation),
            0x1 => Some(WsOpcode::Text),
            0x2 => Some(WsOpcode::Binary),
            0x8 => Some(WsOpcode::Close),
            0x9 => Some(WsOpcode::Ping),
            0xA => Some(WsOpcode::Pong),
            _ => None,
        }
    }
}

/// A parsed WebSocket frame.
#[derive(Debug, Clone, PartialEq)]
pub struct WsFrame {
    /// FIN bit: indicates the final fragment in a message.
    pub fin: bool,
    /// Frame opcode.
    pub opcode: WsOpcode,
    /// Whether the payload is masked (client→server frames must be masked).
    pub masked: bool,
    /// Payload data (unmasked).
    pub payload: Vec<u8>,
}

// ─── Error ───────────────────────────────────────────────────────

/// Errors that can occur when parsing or converting WebSocket frames.
#[derive(Error, Debug)]
pub enum WsError {
    #[error("buffer too short: need at least {need} bytes, have {have}")]
    BufferTooShort { need: usize, have: usize },

    #[error("invalid WebSocket opcode: 0x{0:02x}")]
    InvalidOpcode(u8),

    #[error("RSV bits must be zero (RFC 6455 §5.2)")]
    RsvBitsNonZero,

    #[error("control frame must not be fragmented (RFC 6455 §5.5)")]
    ControlFrameFragmented,

    #[error("payload too large: {0} bytes")]
    PayloadTooLarge(u64),

    #[error("payload length mismatch: expected {expected}, got {actual}")]
    PayloadLengthMismatch { expected: usize, actual: usize },

    #[error("cannot map WebSocket opcode {0:?} to FIG frame")]
    UnmappableOpcode(WsOpcode),

    #[error("cannot map FIG frame type {0} to WebSocket")]
    UnmappableFrameType(String),
}

/// Convenience type alias for WebSocket operations.
pub type WsResult<T> = Result<T, WsError>;

// ─── WebSocket Frame Parsing ─────────────────────────────────────

/// Parse a single WebSocket frame from bytes.
///
/// Returns the parsed frame and the number of bytes consumed. The payload
/// is automatically unmasked if the MASK bit is set.
///
/// # RFC 6455 references
/// - §5.1: Overview
/// - §5.2: Base Framing Protocol
/// - §5.3: Client-to-Server Masking
pub fn parse_ws_frame(input: &[u8]) -> WsResult<(WsFrame, usize)> {
    // Minimum frame size: 2 bytes (FIN+opcode+mask+len, no payload, no masking key)
    if input.len() < 2 {
        return Err(WsError::BufferTooShort {
            need: 2,
            have: input.len(),
        });
    }

    let first_byte = input[0];
    let second_byte = input[1];

    // FIN bit (bit 7)
    let fin = (first_byte & 0x80) != 0;

    // RSV bits (bits 4-6) — must be zero
    if (first_byte & 0x70) != 0 {
        return Err(WsError::RsvBitsNonZero);
    }

    // Opcode (bits 0-3)
    let opcode_raw = first_byte & 0x0F;
    let opcode = WsOpcode::from_code(opcode_raw).ok_or(WsError::InvalidOpcode(opcode_raw))?;

    // Control frames (opcode 0x8-0xF) must have FIN set and payload ≤ 125 bytes
    if opcode_raw >= 0x8 && (!fin) {
        return Err(WsError::ControlFrameFragmented);
    }

    // MASK bit (bit 7 of second byte)
    let masked = (second_byte & 0x80) != 0;

    // Payload length (bits 0-6 of second byte)
    let mut payload_len = (second_byte & 0x7F) as u64;
    let mut header_size = 2;

    // Extended payload length
    if payload_len == 126 {
        // Next 2 bytes as u16
        if input.len() < header_size + 2 {
            return Err(WsError::BufferTooShort {
                need: header_size + 2,
                have: input.len(),
            });
        }
        payload_len = u16::from_be_bytes([input[header_size], input[header_size + 1]]) as u64;
        header_size += 2;
    } else if payload_len == 127 {
        // Next 8 bytes as u64
        if input.len() < header_size + 8 {
            return Err(WsError::BufferTooShort {
                need: header_size + 8,
                have: input.len(),
            });
        }
        payload_len = u64::from_be_bytes([
            input[header_size],
            input[header_size + 1],
            input[header_size + 2],
            input[header_size + 3],
            input[header_size + 4],
            input[header_size + 5],
            input[header_size + 6],
            input[header_size + 7],
        ]);
        header_size += 8;
    }

    // Payload size sanity check (2^63 max)
    if payload_len > (1u64 << 63) {
        return Err(WsError::PayloadTooLarge(payload_len));
    }

    // Masking key (4 bytes if MASK is set)
    let mut mask_key = [0u8; 4];
    if masked {
        if input.len() < header_size + 4 {
            return Err(WsError::BufferTooShort {
                need: header_size + 4,
                have: input.len(),
            });
        }
        mask_key.copy_from_slice(&input[header_size..header_size + 4]);
        header_size += 4;
    }

    // Payload
    let total_size = header_size + payload_len as usize;
    if input.len() < total_size {
        return Err(WsError::BufferTooShort {
            need: total_size,
            have: input.len(),
        });
    }

    let mut payload = input[header_size..total_size].to_vec();

    // Unmask payload if masked
    if masked {
        for (i, byte) in payload.iter_mut().enumerate() {
            *byte ^= mask_key[i % 4];
        }
    }

    Ok((
        WsFrame {
            fin,
            opcode,
            masked,
            payload,
        },
        total_size,
    ))
}

// ─── WebSocket Frame Serialization ───────────────────────────────

/// Serialize a `WsFrame` to WebSocket wire format bytes.
///
/// Server-side frames are sent without masking (RFC 6455 §5.3).
pub fn serialize_ws_frame(frame: &WsFrame) -> Vec<u8> {
    let mut buf = Vec::new();

    // First byte: FIN + RSV (0) + opcode
    let first_byte = if frame.fin { 0x80 } else { 0x00 } | (frame.opcode.code() & 0x0F);
    buf.push(first_byte);

    // Second byte: MASK (0 for server) + payload length
    let len = frame.payload.len();
    if len < 126 {
        buf.push(len as u8);
    } else if len <= 0xFFFF {
        buf.push(126);
        buf.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        buf.push(127);
        buf.extend_from_slice(&(len as u64).to_be_bytes());
    }

    // No masking key for server→client

    // Payload
    buf.extend_from_slice(&frame.payload);

    buf
}

// ─── WebSocket → FIG Conversion ──────────────────────────────────

/// Convert a WebSocket frame to a FIG frame.
///
/// Mappings:
/// - Text frame → STREAM_ITEM with CONTENT_TYPE "text/plain"
/// - Binary frame → STREAM_ITEM with CONTENT_TYPE "application/octet-stream"
/// - Close frame → STREAM_CLOSE
/// - Ping frame → CONTROL with Ping subtype
/// - Pong frame → CONTROL with Pong subtype
pub fn ws_to_fig_frame(ws: &WsFrame) -> WsResult<Frame> {
    match ws.opcode {
        WsOpcode::Text => {
            let mut frame = Frame::new(FrameType::StreamItem, 1);
            frame = frame.with_extension(Extension::text(ExtensionTag::ContentType, "text/plain"));
            frame = frame.with_payload(ws.payload.clone());
            Ok(frame)
        }
        WsOpcode::Binary => {
            let mut frame = Frame::new(FrameType::StreamItem, 1);
            frame = frame.with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/octet-stream",
            ));
            frame = frame.with_payload(ws.payload.clone());
            Ok(frame)
        }
        WsOpcode::Close => Ok(Frame::new(FrameType::StreamClose, 1)),
        WsOpcode::Ping => Ok(Frame::ping()),
        WsOpcode::Pong => Ok(Frame::pong()),
        WsOpcode::Continuation => {
            // Continuation frame: treat as StreamItem with no content type
            let mut frame = Frame::new(FrameType::StreamItem, 1);
            frame = frame.with_payload(ws.payload.clone());
            Ok(frame)
        }
    }
}

// ─── FIG → WebSocket Conversion ──────────────────────────────────

/// Convert a FIG frame to a WebSocket frame.
///
/// Reverse of `ws_to_fig_frame`:
/// - STREAM_ITEM with CONTENT_TYPE "text/plain" → Text frame
/// - STREAM_ITEM with CONTENT_TYPE "application/octet-stream" → Binary frame
/// - STREAM_CLOSE → Close frame
/// - CONTROL with Ping subtype → Ping frame
/// - CONTROL with Pong subtype → Pong frame
pub fn fig_to_ws_frame(frame: &Frame) -> WsResult<WsFrame> {
    match frame.frame_type {
        FrameType::StreamItem => {
            let content_type = frame
                .extensions
                .iter()
                .find(|e| e.tag == ExtensionTag::ContentType)
                .and_then(|e| e.value.as_text());

            let opcode = match content_type {
                Some("application/octet-stream") | Some("application/cbor") => WsOpcode::Binary,
                _ => WsOpcode::Text,
            };

            Ok(WsFrame {
                fin: true,
                opcode,
                masked: false,
                payload: frame.payload.clone(),
            })
        }
        FrameType::StreamClose => Ok(WsFrame {
            fin: true,
            opcode: WsOpcode::Close,
            masked: false,
            payload: Vec::new(),
        }),
        FrameType::Control => {
            // Check the control subtype in the payload
            if frame.channel_id == 0 && !frame.payload.is_empty() {
                let subtype = frame.payload[0];
                match subtype {
                    0x00 => Ok(WsFrame {
                        fin: true,
                        opcode: WsOpcode::Ping,
                        masked: false,
                        payload: Vec::new(),
                    }),
                    0x01 => Ok(WsFrame {
                        fin: true,
                        opcode: WsOpcode::Pong,
                        masked: false,
                        payload: Vec::new(),
                    }),
                    _ => Err(WsError::UnmappableFrameType(format!(
                        "Control({:#04x})",
                        subtype
                    ))),
                }
            } else {
                Err(WsError::UnmappableFrameType("Control".to_string()))
            }
        }
        _ => Err(WsError::UnmappableFrameType(format!(
            "{:?}",
            frame.frame_type
        ))),
    }
}

// ─── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Parse Text Frame ──────────────────────────────────────

    #[test]
    fn test_parse_text_frame() {
        // Build a text frame: FIN=1, opcode=1 (text), MASK=0, len=5, payload="Hello"
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Text,
            masked: false,
            payload: b"Hello".to_vec(),
        };
        let serialized = serialize_ws_frame(&frame);
        let (parsed, consumed) = parse_ws_frame(&serialized).unwrap();

        assert_eq!(consumed, serialized.len());
        assert!(parsed.fin);
        assert_eq!(parsed.opcode, WsOpcode::Text);
        assert!(!parsed.masked);
        assert_eq!(parsed.payload, b"Hello");
    }

    #[test]
    fn test_parse_binary_frame() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Binary,
            masked: false,
            payload: vec![0x00, 0x01, 0x02, 0x03],
        };
        let serialized = serialize_ws_frame(&frame);
        let (parsed, _) = parse_ws_frame(&serialized).unwrap();

        assert!(parsed.fin);
        assert_eq!(parsed.opcode, WsOpcode::Binary);
        assert_eq!(parsed.payload, vec![0x00, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_parse_close_frame() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Close,
            masked: false,
            payload: vec![0x03, 0xE8], // status code 1000
        };
        let serialized = serialize_ws_frame(&frame);
        let (parsed, _) = parse_ws_frame(&serialized).unwrap();

        assert_eq!(parsed.opcode, WsOpcode::Close);
        assert_eq!(parsed.payload, vec![0x03, 0xE8]);
    }

    #[test]
    fn test_parse_masked_frame() {
        // Client frames must be masked per RFC 6455
        // Build manually with masking
        let payload = b"Hello";
        let mask_key = [0x12, 0x34, 0x56, 0x78];
        let masked_payload: Vec<u8> = payload
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ mask_key[i % 4])
            .collect();

        let mut buf = Vec::new();
        buf.push(0x81); // FIN=1, opcode=Text(1)
        buf.push(0x80 | 5); // MASK=1, len=5
        buf.extend_from_slice(&mask_key);
        buf.extend_from_slice(&masked_payload);

        let (parsed, _) = parse_ws_frame(&buf).unwrap();
        assert!(parsed.masked);
        assert_eq!(parsed.opcode, WsOpcode::Text);
        assert_eq!(parsed.payload, b"Hello");
    }

    // ── Round-trip ────────────────────────────────────────────

    #[test]
    fn test_ws_round_trip() {
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Binary,
            masked: false,
            payload: b"binary payload data".to_vec(),
        };

        let serialized = serialize_ws_frame(&frame);
        let (parsed, _) = parse_ws_frame(&serialized).unwrap();

        assert_eq!(parsed, frame);
    }

    // ── Large Payload ─────────────────────────────────────────

    #[test]
    fn test_large_payload_126() {
        // Payload length exactly 126 bytes (triggers extended length format)
        let payload = vec![b'x'; 126];
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Binary,
            masked: false,
            payload: payload.clone(),
        };

        let serialized = serialize_ws_frame(&frame);
        let (parsed, _) = parse_ws_frame(&serialized).unwrap();

        assert_eq!(parsed.payload, payload);
    }

    #[test]
    fn test_large_payload_65536() {
        // Payload length > 65535 (triggers 64-bit extended length format)
        let payload = vec![b'y'; 65536];
        let frame = WsFrame {
            fin: true,
            opcode: WsOpcode::Binary,
            masked: false,
            payload: payload.clone(),
        };

        let serialized = serialize_ws_frame(&frame);
        let (parsed, _) = parse_ws_frame(&serialized).unwrap();

        assert_eq!(parsed.payload.len(), 65536);
        assert_eq!(parsed.payload, payload);
    }

    // ── WS → FIG Conversion ───────────────────────────────────

    #[test]
    fn test_ws_to_fig_text() {
        let ws = WsFrame {
            fin: true,
            opcode: WsOpcode::Text,
            masked: false,
            payload: b"Hello, World!".to_vec(),
        };

        let fig = ws_to_fig_frame(&ws).unwrap();
        assert_eq!(fig.frame_type, FrameType::StreamItem);

        let ct = fig
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ContentType)
            .and_then(|e| e.value.as_text());
        assert_eq!(ct, Some("text/plain"));
        assert_eq!(fig.payload, b"Hello, World!");
    }

    #[test]
    fn test_ws_to_fig_close() {
        let ws = WsFrame {
            fin: true,
            opcode: WsOpcode::Close,
            masked: false,
            payload: Vec::new(),
        };

        let fig = ws_to_fig_frame(&ws).unwrap();
        assert_eq!(fig.frame_type, FrameType::StreamClose);
    }

    #[test]
    fn test_ws_to_fig_ping_pong() {
        let ping_ws = WsFrame {
            fin: true,
            opcode: WsOpcode::Ping,
            masked: false,
            payload: Vec::new(),
        };
        let ping_fig = ws_to_fig_frame(&ping_ws).unwrap();
        assert_eq!(ping_fig.frame_type, FrameType::Control);
        assert_eq!(ping_fig.channel_id, 0);
        assert_eq!(ping_fig.payload, vec![0x00]); // Ping subtype

        let pong_ws = WsFrame {
            fin: true,
            opcode: WsOpcode::Pong,
            masked: false,
            payload: Vec::new(),
        };
        let pong_fig = ws_to_fig_frame(&pong_ws).unwrap();
        assert_eq!(pong_fig.frame_type, FrameType::Control);
        assert_eq!(pong_fig.payload, vec![0x01]); // Pong subtype
    }

    // ── FIG → WS Conversion ───────────────────────────────────

    #[test]
    fn test_fig_to_ws_stream_item() {
        let fig = Frame::new(FrameType::StreamItem, 1)
            .with_extension(Extension::text(ExtensionTag::ContentType, "text/plain"))
            .with_payload(b"stream data".to_vec());

        let ws = fig_to_ws_frame(&fig).unwrap();
        assert_eq!(ws.opcode, WsOpcode::Text);
        assert_eq!(ws.payload, b"stream data");
    }

    #[test]
    fn test_fig_to_ws_binary() {
        let binary_data = vec![0x00, 0xFF, 0xAA, 0x55];
        let fig = Frame::new(FrameType::StreamItem, 1)
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/octet-stream",
            ))
            .with_payload(binary_data.clone());

        let ws = fig_to_ws_frame(&fig).unwrap();
        assert_eq!(ws.opcode, WsOpcode::Binary);
        assert_eq!(ws.payload, binary_data);
    }

    #[test]
    fn test_fig_to_ws_stream_close() {
        let fig = Frame::new(FrameType::StreamClose, 1);
        let ws = fig_to_ws_frame(&fig).unwrap();
        assert_eq!(ws.opcode, WsOpcode::Close);
    }

    // ── FIG ↔ WS round-trip ───────────────────────────────────

    #[test]
    fn test_fig_ws_round_trip() {
        // FIG → WS → FIG
        let original_fig = Frame::new(FrameType::StreamItem, 1)
            .with_extension(Extension::text(ExtensionTag::ContentType, "text/plain"))
            .with_payload(b"round trip test".to_vec());

        let ws = fig_to_ws_frame(&original_fig).unwrap();
        let round_trip_fig = ws_to_fig_frame(&ws).unwrap();

        assert_eq!(round_trip_fig.frame_type, original_fig.frame_type);
        assert_eq!(round_trip_fig.payload, original_fig.payload);

        let ct = round_trip_fig
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ContentType)
            .and_then(|e| e.value.as_text());
        assert_eq!(ct, Some("text/plain"));
    }
}
