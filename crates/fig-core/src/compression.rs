//! Payload compression for FIG frames.
//!
//! Spec §5: the `COMPRESSED` flag indicates the payload was compressed.
//! This module uses zstd as the default algorithm (see CONTENT_ENCODING
//! extension tag `0x000C` for explicit algorithm negotiation).

use crate::error::FrameError;
use crate::frame::{Flags, Frame};

/// Default compression algorithm name (Spec §6.1 CONTENT_ENCODING).
pub const DEFAULT_ENCODING: &str = "zstd";

/// Compress payload bytes using zstd.
pub fn compress_payload(data: &[u8]) -> Result<Vec<u8>, FrameError> {
    zstd::encode_all(data, 3).map_err(|e| FrameError::CborEncodeError(e.to_string()))
}

/// Decompress zstd-compressed payload bytes.
pub fn decompress_payload(data: &[u8]) -> Result<Vec<u8>, FrameError> {
    zstd::decode_all(data).map_err(|e| FrameError::CborDecodeError(e.to_string()))
}

/// Compress a frame's payload in place and set the COMPRESSED flag.
pub fn compress_frame(frame: &mut Frame) -> Result<(), FrameError> {
    if frame.flags.contains(Flags::COMPRESSED) {
        return Ok(());
    }
    let compressed = compress_payload(&frame.payload)?;
    frame.payload = compressed;
    frame.flags |= Flags::COMPRESSED;
    Ok(())
}

/// Decompress a frame's payload in place and clear the COMPRESSED flag.
pub fn decompress_frame(frame: &mut Frame) -> Result<(), FrameError> {
    if !frame.flags.contains(Flags::COMPRESSED) {
        return Ok(());
    }
    let decompressed = decompress_payload(&frame.payload)?;
    frame.payload = decompressed;
    frame.flags -= Flags::COMPRESSED;
    Ok(())
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{Flags, FrameType};

    #[test]
    fn test_compress_decompress_round_trip() {
        let original = b"hello world hello world hello world".repeat(20);
        let compressed = compress_payload(&original).unwrap();
        assert!(compressed.len() < original.len());

        let decompressed = decompress_payload(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_compress_frame_sets_flag() {
        let mut frame = Frame::new(FrameType::Request, 1)
            .with_payload(b"compressible payload data here".repeat(50));
        compress_frame(&mut frame).unwrap();

        assert!(frame.flags.contains(Flags::COMPRESSED));
        assert!(frame.payload.len() < 50 * "compressible payload data here".len());
    }

    #[test]
    fn test_decompress_frame_clears_flag() {
        let mut frame = Frame::new(FrameType::Request, 1)
            .with_payload(b"compressible payload data here".repeat(50));
        compress_frame(&mut frame).unwrap();
        decompress_frame(&mut frame).unwrap();

        assert!(!frame.flags.contains(Flags::COMPRESSED));
        assert_eq!(frame.payload, b"compressible payload data here".repeat(50));
    }

    #[test]
    fn test_encode_decode_with_compression() {
        let payload = b"large repetitive payload ".repeat(100);
        let mut frame = Frame::new(FrameType::StreamItem, 2)
            .with_seq(99)
            .with_payload(payload.to_vec());
        compress_frame(&mut frame).unwrap();

        let encoded = frame.encode().unwrap();
        let (mut decoded, _) = Frame::decode(&encoded).unwrap();
        assert!(decoded.flags.contains(Flags::COMPRESSED));

        decompress_frame(&mut decoded).unwrap();
        assert_eq!(decoded.payload, payload);
        assert!(!decoded.flags.contains(Flags::COMPRESSED));
    }

    #[test]
    fn test_decompress_invalid_data() {
        let result = decompress_payload(b"not zstd data");
        assert!(result.is_err());
    }
}
