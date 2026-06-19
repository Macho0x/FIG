//! Payload fragmentation and reassembly for FIG frames.
//!
//! Spec §5: large payloads may be split across multiple frames using the
//! `FRAGMENTED` and `LAST_FRAGMENT` flags. All fragments of a message
//! share the same `(channel_id, stream_seq)` key.

use std::collections::HashMap;

use crate::error::FrameError;
use crate::frame::{Flags, Frame, FrameType};

// ─── FragmentReassembler ─────────────────────────────────────────

/// Key identifying a fragmented message in progress.
type FragmentKey = (u16, u32);

/// Buffered state for an in-progress fragmented message.
#[derive(Debug, Clone)]
struct FragmentBuffer {
    frame_type: FrameType,
    schema_id: u8,
    extensions: Vec<crate::ext::Extension>,
    base_flags: Flags,
    parts: Vec<Vec<u8>>,
}

impl FragmentBuffer {
    fn from_first_fragment(frame: &Frame) -> Self {
        let base_flags = frame.flags - Flags::FRAGMENTED - Flags::LAST_FRAGMENT;
        Self {
            frame_type: frame.frame_type,
            schema_id: frame.schema_id,
            extensions: frame.extensions.clone(),
            base_flags,
            parts: Vec::new(),
        }
    }
}

/// Reassembles fragmented FIG frames into complete messages.
///
/// Feed each received frame via [`FragmentReassembler::feed`]. Complete
/// frames (no fragment flags) pass through immediately. Fragmented frames
/// are buffered until the `LAST_FRAGMENT` arrives.
#[derive(Debug, Default)]
pub struct FragmentReassembler {
    pending: HashMap<FragmentKey, FragmentBuffer>,
}

impl FragmentReassembler {
    /// Create a new reassembler with no buffered fragments.
    pub fn new() -> Self {
        Self::default()
    }

    /// Process an incoming frame.
    ///
    /// Returns `Ok(Some(frame))` when a complete message is ready (either
    /// because the frame was not fragmented, or because reassembly finished).
    /// Returns `Ok(None)` when more fragments are expected.
    pub fn feed(&mut self, frame: Frame) -> Result<Option<Frame>, FrameError> {
        let is_fragmented = frame.flags.contains(Flags::FRAGMENTED);
        let is_last = frame.flags.contains(Flags::LAST_FRAGMENT);

        if !is_fragmented && !is_last {
            return Ok(Some(frame));
        }

        let key = (frame.channel_id, frame.stream_seq);

        if is_fragmented && !is_last {
            let entry = self
                .pending
                .entry(key)
                .or_insert_with(|| FragmentBuffer::from_first_fragment(&frame));
            entry.parts.push(frame.payload);
            return Ok(None);
        }

        // LAST_FRAGMENT (with or without FRAGMENTED)
        let mut buffer = if let Some(existing) = self.pending.remove(&key) {
            existing
        } else {
            FragmentBuffer::from_first_fragment(&frame)
        };
        buffer.parts.push(frame.payload);

        let mut payload = Vec::new();
        for part in buffer.parts {
            payload.extend(part);
        }

        Ok(Some(Frame {
            frame_type: buffer.frame_type,
            flags: buffer.base_flags,
            channel_id: frame.channel_id,
            stream_seq: frame.stream_seq,
            schema_id: buffer.schema_id,
            extensions: buffer.extensions,
            payload,
        }))
    }

    /// Number of in-progress fragmented messages.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

// ─── Fragmentation Helpers ───────────────────────────────────────

/// Split a frame's payload into fragment frames of at most `max_payload` bytes.
///
/// All fragments share the original frame's metadata and `stream_seq`.
/// Intermediate fragments set `FRAGMENTED`; the final fragment sets both
/// `FRAGMENTED` and `LAST_FRAGMENT`. Single-chunk payloads produce one
/// frame with no fragment flags.
pub fn split_frame(frame: &Frame, max_payload: usize) -> Result<Vec<Frame>, FrameError> {
    if max_payload == 0 {
        return Err(FrameError::InvalidHeader(0));
    }

    if frame.payload.len() <= max_payload {
        return Ok(vec![frame.clone()]);
    }

    let chunks: Vec<&[u8]> = frame.payload.chunks(max_payload).collect();
    let mut fragments = Vec::with_capacity(chunks.len());

    for (i, chunk) in chunks.iter().enumerate() {
        let is_last = i + 1 == chunks.len();
        let mut flags = frame.flags - Flags::FRAGMENTED - Flags::LAST_FRAGMENT;
        if !is_last {
            flags |= Flags::FRAGMENTED;
        } else if chunks.len() > 1 {
            flags |= Flags::FRAGMENTED | Flags::LAST_FRAGMENT;
        }

        fragments.push(Frame {
            frame_type: frame.frame_type,
            flags,
            channel_id: frame.channel_id,
            stream_seq: frame.stream_seq,
            schema_id: frame.schema_id,
            extensions: if i == 0 {
                frame.extensions.clone()
            } else {
                Vec::new()
            },
            payload: chunk.to_vec(),
        });
    }

    Ok(fragments)
}

/// Reassemble a sequence of fragment frames into the original message.
///
/// Convenience wrapper around [`FragmentReassembler`] for callers that
/// already have all fragments collected.
pub fn reassemble_fragments(fragments: Vec<Frame>) -> Result<Frame, FrameError> {
    let mut reassembler = FragmentReassembler::new();
    let mut result = None;
    for fragment in fragments {
        match reassembler.feed(fragment)? {
            Some(frame) => result = Some(frame),
            None => {}
        }
    }
    result.ok_or(FrameError::BufferTooShort {
        expected: 1,
        actual: 0,
    })
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ext::{Extension, ExtensionTag};

    #[test]
    fn test_non_fragmented_passes_through() {
        let mut reassembler = FragmentReassembler::new();
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"hello".to_vec());
        let result = reassembler.feed(frame.clone()).unwrap();
        assert_eq!(result, Some(frame));
        assert_eq!(reassembler.pending_count(), 0);
    }

    #[test]
    fn test_two_fragment_reassembly() {
        let base = Frame::new(FrameType::Request, 1)
            .with_seq(42)
            .with_payload(b"hello world".to_vec());

        let fragments = split_frame(&base, 5).unwrap();
        assert_eq!(fragments.len(), 3);
        assert!(fragments[0].flags.contains(Flags::FRAGMENTED));
        assert!(!fragments[0].flags.contains(Flags::LAST_FRAGMENT));
        assert!(fragments[1].flags.contains(Flags::FRAGMENTED));
        assert!(!fragments[1].flags.contains(Flags::LAST_FRAGMENT));
        assert!(fragments[2].flags.contains(Flags::FRAGMENTED));
        assert!(fragments[2].flags.contains(Flags::LAST_FRAGMENT));

        let reassembled = reassemble_fragments(fragments).unwrap();
        assert_eq!(reassembled.payload, b"hello world");
        assert_eq!(reassembled.stream_seq, 42);
        assert_eq!(reassembled.channel_id, 1);
        assert!(!reassembled.flags.contains(Flags::FRAGMENTED));
        assert!(!reassembled.flags.contains(Flags::LAST_FRAGMENT));
    }

    #[test]
    fn test_single_fragment_no_flags() {
        let frame = Frame::new(FrameType::Response, 2).with_payload(b"small".to_vec());
        let fragments = split_frame(&frame, 100).unwrap();
        assert_eq!(fragments.len(), 1);
        assert!(!fragments[0].flags.contains(Flags::FRAGMENTED));
    }

    #[test]
    fn test_reassembler_incremental_feed() {
        let base = Frame::new(FrameType::StreamItem, 3)
            .with_seq(7)
            .with_extension(Extension::text(ExtensionTag::ContentType, "application/cbor"))
            .with_payload(b"abcdefghijklmnop".to_vec());

        let fragments = split_frame(&base, 4).unwrap();
        assert_eq!(fragments.len(), 4);
        let mut reassembler = FragmentReassembler::new();

        assert!(reassembler.feed(fragments[0].clone()).unwrap().is_none());
        assert_eq!(reassembler.pending_count(), 1);
        assert!(reassembler.feed(fragments[1].clone()).unwrap().is_none());
        assert!(reassembler.feed(fragments[2].clone()).unwrap().is_none());

        let result = reassembler.feed(fragments[3].clone()).unwrap().unwrap();
        assert_eq!(result.payload, b"abcdefghijklmnop");
        assert_eq!(result.extensions.len(), 1);
        assert_eq!(reassembler.pending_count(), 0);
    }

    #[test]
    fn test_different_messages_different_keys() {
        let mut reassembler = FragmentReassembler::new();

        let f1 = Frame::new(FrameType::Request, 1)
            .with_seq(10)
            .with_payload(b"aaa".to_vec());
        let f1_frag = split_frame(&f1, 2).unwrap();

        let f2 = Frame::new(FrameType::Request, 2)
            .with_seq(20)
            .with_payload(b"bbb".to_vec());
        let f2_frag = split_frame(&f2, 2).unwrap();

        // Interleave fragments from two messages.
        assert!(reassembler.feed(f1_frag[0].clone()).unwrap().is_none());
        assert!(reassembler.feed(f2_frag[0].clone()).unwrap().is_none());
        let r1 = reassembler.feed(f1_frag[1].clone()).unwrap().unwrap();
        let r2 = reassembler.feed(f2_frag[1].clone()).unwrap().unwrap();

        assert_eq!(r1.payload, b"aaa");
        assert_eq!(r1.channel_id, 1);
        assert_eq!(r2.payload, b"bbb");
        assert_eq!(r2.channel_id, 2);
    }

    #[test]
    fn test_encode_decode_fragment_round_trip() {
        let original = Frame::new(FrameType::Request, 5)
            .with_seq(100)
            .with_schema_id(0x01)
            .with_payload(vec![0xAB; 5000]);

        let fragments = split_frame(&original, 1500).unwrap();
        assert_eq!(fragments.len(), 4);

        for fragment in &fragments {
            let encoded = fragment.encode().unwrap();
            let (decoded, _) = Frame::decode(&encoded).unwrap();
            assert_eq!(decoded.payload, fragment.payload);
            assert_eq!(decoded.flags, fragment.flags);
        }

        let reassembled = reassemble_fragments(fragments).unwrap();
        assert_eq!(reassembled.payload, original.payload);
        assert_eq!(reassembled.frame_type, original.frame_type);
        assert_eq!(reassembled.schema_id, original.schema_id);
    }

    #[test]
    fn test_split_rejects_zero_chunk_size() {
        let frame = Frame::new(FrameType::Request, 1).with_payload(b"x".to_vec());
        assert!(split_frame(&frame, 0).is_err());
    }
}
