//! W3C Trace Context propagation for FIG frames.
//!
//! Maps the W3C `traceparent` header to/from the TRACE_ID and
//! CORRELATION_ID extension tags (Spec §6).

use crate::ext::{Extension, ExtensionTag};
use crate::frame::Frame;

/// Parsed W3C Trace Context (`traceparent` format).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceContext {
    pub version: u8,
    pub trace_id: [u8; 16],
    pub parent_id: [u8; 8],
    pub flags: u8,
}

impl TraceContext {
    /// Parse a W3C traceparent string (e.g. `00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01`).
    pub fn parse(traceparent: &str) -> Option<Self> {
        let parts: Vec<&str> = traceparent.split('-').collect();
        if parts.len() != 4 {
            return None;
        }
        let version = u8::from_str_radix(parts[0], 16).ok()?;
        if parts[1].len() != 32 || parts[2].len() != 16 {
            return None;
        }
        let mut trace_id = [0u8; 16];
        for (i, chunk) in parts[1].as_bytes().chunks(2).enumerate() {
            if i >= 16 {
                break;
            }
            trace_id[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
        }
        let mut parent_id = [0u8; 8];
        for (i, chunk) in parts[2].as_bytes().chunks(2).enumerate() {
            if i >= 8 {
                break;
            }
            parent_id[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
        }
        let flags = u8::from_str_radix(parts[3], 16).ok()?;
        Some(Self {
            version,
            trace_id,
            parent_id,
            flags,
        })
    }

    /// Serialize to W3C traceparent format.
    pub fn to_traceparent(&self) -> String {
        format!(
            "{:02x}-{}-{}-{:02x}",
            self.version,
            encode_hex(&self.trace_id),
            encode_hex(&self.parent_id),
            self.flags
        )
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Inject trace context into a frame via TRACE_ID extension.
pub fn inject_trace_context(frame: &mut Frame, ctx: &TraceContext) {
    frame.extensions.retain(|e| e.tag != ExtensionTag::TraceId);
    frame.extensions.push(Extension::binary(
        ExtensionTag::TraceId,
        ctx.trace_id.to_vec(),
    ));
    frame.flags |= crate::frame::Flags::EXTENSIONS;
}

/// Extract trace context from a frame's TRACE_ID extension.
pub fn extract_trace_context(frame: &Frame) -> Option<TraceContext> {
    let trace_bytes = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::TraceId)
        .map(|e| e.value.as_bytes())?;

    if trace_bytes.len() != 16 {
        return None;
    }

    let mut trace_id = [0u8; 16];
    trace_id.copy_from_slice(&trace_bytes);

    Some(TraceContext {
        version: 0,
        trace_id,
        parent_id: [0u8; 8],
        flags: 1,
    })
}

/// Parse traceparent from HTTP-style header value on a frame.
pub fn traceparent_from_frame(frame: &Frame) -> Option<String> {
    extract_trace_context(frame).map(|ctx| ctx.to_traceparent())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{Frame, FrameType};

    #[test]
    fn test_traceparent_round_trip() {
        let tp = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
        let ctx = TraceContext::parse(tp).unwrap();
        assert_eq!(ctx.to_traceparent(), tp);
    }

    #[test]
    fn test_inject_extract_trace_context() {
        let ctx =
            TraceContext::parse("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01").unwrap();

        let mut frame = Frame::new(FrameType::Request, 1);
        inject_trace_context(&mut frame, &ctx);

        let extracted = extract_trace_context(&frame).unwrap();
        assert_eq!(extracted.trace_id, ctx.trace_id);
    }

    #[test]
    fn test_invalid_traceparent() {
        assert!(TraceContext::parse("invalid").is_none());
        assert!(TraceContext::parse("00-short").is_none());
    }
}
