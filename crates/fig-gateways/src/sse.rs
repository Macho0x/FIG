//! REST Server-Sent Events (SSE) adapter (Spec §12.2).
//!
//! Maps SSE event streams to FIG `STREAM_ITEM` frames for REST gateway
//! streaming endpoints.

use thiserror::Error;

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};

/// A parsed Server-Sent Event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<String>,
}

/// Errors when parsing or converting SSE payloads.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SseError {
    #[error("empty SSE chunk")]
    EmptyChunk,

    #[error("missing data field in SSE event")]
    MissingData,

    #[error("invalid STREAM_ITEM frame for SSE conversion")]
    InvalidStreamItem,
}

/// Parse one or more SSE events from a text chunk.
pub fn parse_sse_chunk(chunk: &str) -> Vec<SseEvent> {
    let mut events = Vec::new();
    let mut event: Option<String> = None;
    let mut data_lines: Vec<String> = Vec::new();
    let mut id: Option<String> = None;

    let flush = |event: &mut Option<String>,
                 data_lines: &mut Vec<String>,
                 id: &mut Option<String>,
                 events: &mut Vec<SseEvent>| {
        if data_lines.is_empty() {
            event.take();
            id.take();
            return;
        }
        events.push(SseEvent {
            event: event.take(),
            data: data_lines.join("\n"),
            id: id.take(),
        });
        data_lines.clear();
    };

    for line in chunk.lines() {
        if line.is_empty() {
            flush(&mut event, &mut data_lines, &mut id, &mut events);
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            event = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("data:") {
            data_lines.push(value.trim_start().to_string());
        } else if let Some(value) = line.strip_prefix("id:") {
            id = Some(value.trim().to_string());
        }
    }

    flush(&mut event, &mut data_lines, &mut id, &mut events);
    events
}

/// Convert an SSE event to a FIG STREAM_ITEM frame.
pub fn sse_to_fig_stream_item(channel_id: u16, seq: u32, event: &SseEvent) -> Frame {
    let mut frame = Frame::new(FrameType::StreamItem, channel_id)
        .with_seq(seq)
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "text/event-stream",
        ))
        .with_payload(event.data.as_bytes().to_vec());

    if let Some(name) = &event.event {
        frame = frame.with_extension(Extension::text(ExtensionTag::RoutingKey, name));
    }
    if let Some(id) = &event.id {
        frame = frame.with_extension(Extension::text(ExtensionTag::CorrelationId, id));
    }
    frame
}

/// Convert a FIG STREAM_ITEM frame back to an SSE event block.
pub fn fig_stream_item_to_sse(frame: &Frame) -> Result<SseEvent, SseError> {
    if frame.frame_type != FrameType::StreamItem {
        return Err(SseError::InvalidStreamItem);
    }
    let data = String::from_utf8_lossy(&frame.payload).to_string();
    if data.is_empty() {
        return Err(SseError::MissingData);
    }

    let event = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::RoutingKey)
        .and_then(|e| e.value.as_text())
        .map(String::from);

    let id = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::CorrelationId)
        .and_then(|e| e.value.as_text())
        .map(String::from);

    Ok(SseEvent { event, data, id })
}

/// Serialize an SSE event to wire format (`event:` / `data:` / blank line).
pub fn serialize_sse_event(event: &SseEvent) -> String {
    let mut out = String::new();
    if let Some(name) = &event.event {
        out.push_str(&format!("event: {}\n", name));
    }
    if let Some(id) = &event.id {
        out.push_str(&format!("id: {}\n", id));
    }
    for line in event.data.lines() {
        out.push_str(&format!("data: {}\n", line));
    }
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sse_chunk_single_event() {
        let chunk = "event: quote\ndata: {\"symbol\":\"AAPL\"}\nid: 1\n\n";
        let events = parse_sse_chunk(chunk);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event.as_deref(), Some("quote"));
        assert_eq!(events[0].data, "{\"symbol\":\"AAPL\"}");
        assert_eq!(events[0].id.as_deref(), Some("1"));
    }

    #[test]
    fn test_sse_fig_round_trip() {
        let event = SseEvent {
            event: Some("trade".into()),
            data: "fill:100".into(),
            id: Some("42".into()),
        };
        let frame = sse_to_fig_stream_item(3, 7, &event);
        assert_eq!(frame.frame_type, FrameType::StreamItem);
        assert_eq!(frame.channel_id, 3);
        assert_eq!(frame.stream_seq, 7);

        let back = fig_stream_item_to_sse(&frame).unwrap();
        assert_eq!(back, event);
    }

    #[test]
    fn test_serialize_sse_event() {
        let event = SseEvent {
            event: Some("ping".into()),
            data: "ok".into(),
            id: None,
        };
        let wire = serialize_sse_event(&event);
        assert!(wire.contains("event: ping\n"));
        assert!(wire.contains("data: ok\n"));
    }
}
