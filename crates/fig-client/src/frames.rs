//! FIG frame builders shared by CLI, Python, and SDK callers.

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::FrameResult;

#[allow(clippy::too_many_arguments)]
pub fn subscribe_frame(
    channel_id: u16,
    stream_seq: u32,
    channel_path: &str,
    routing_key: Option<&str>,
    auth_token: Option<&str>,
) -> FrameResult<Frame> {
    let mut frame = Frame::new(FrameType::Subscribe, channel_id)
        .with_seq(stream_seq)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path));
    if let Some(rk) = routing_key {
        frame = frame.with_extension(Extension::text(ExtensionTag::RoutingKey, rk));
    }
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    Ok(frame)
}

#[allow(clippy::too_many_arguments)]
pub fn request_frame(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    channel_path: &str,
    method: &str,
    payload: Option<Vec<u8>>,
    auth_token: Option<&str>,
) -> FrameResult<Frame> {
    let mut frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
        .with_extension(Extension::text(ExtensionTag::Method, method));
    if let Some(body) = payload {
        frame = frame
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/cbor",
            ))
            .with_payload(body);
    }
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    Ok(frame)
}
