//! Error types for the UNIP core library.

use thiserror::Error;

/// Errors that can occur when parsing or encoding UNIP frames.
#[derive(Error, Debug)]
pub enum FrameError {
    #[error("invalid frame header: expected at least 16 bytes, got {0}")]
    InvalidHeader(usize),

    #[error("invalid frame type: 0x{0:02x}")]
    InvalidFrameType(u8),

    #[error("invalid flags: 0x{0:02x}")]
    InvalidFlags(u8),

    #[error("buffer too short: expected {expected} bytes, got {actual}")]
    BufferTooShort { expected: usize, actual: usize },

    #[error("invalid extension: {0}")]
    InvalidExtension(String),

    #[error("invalid extension tag: 0x{0:04x}")]
    InvalidExtensionTag(u16),

    #[error("invalid extension length: tag 0x{tag:04x} has length {length} but only {available} bytes available")]
    InvalidExtensionLength {
        tag: u16,
        length: u16,
        available: usize,
    },

    #[error("invalid channel ID: {0}")]
    InvalidChannelId(u16),

    #[error("invalid stream sequence number")]
    InvalidStreamSeq,

    #[error("CBOR decode error: {0}")]
    CborDecodeError(String),

    #[error("CBOR encode error: {0}")]
    CborEncodeError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Errors related to channel operations.
#[derive(Error, Debug)]
pub enum ChannelError {
    #[error("invalid channel mode: {0}")]
    InvalidChannelMode(String),

    #[error("channel closed: channel {0}")]
    ChannelClosed(u16),

    #[error("channel not found: channel {0}")]
    ChannelNotFound(u16),

    #[error("stream error on channel {0}: {1}")]
    StreamError(u16, String),

    #[error("channel ID exhausted: no available channel IDs")]
    ChannelIdExhausted,
}

/// Errors related to session management.
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("session not found: {0:?}")]
    SessionNotFound(uuid::Uuid),

    #[error("session expired: {0:?}")]
    SessionExpired(uuid::Uuid),

    #[error("invalid session ID")]
    InvalidSessionId,

    #[error("authentication failed: {0}")]
    AuthFailed(String),
}

/// Convenience type alias for frame operations.
pub type FrameResult<T> = Result<T, FrameError>;

/// Convenience type alias for channel operations.
pub type ChannelResult<T> = Result<T, ChannelError>;

/// Convenience type alias for session operations.
pub type SessionResult<T> = Result<T, SessionError>;