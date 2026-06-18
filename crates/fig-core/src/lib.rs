//! FIG — Unified Network Interchange Protocol
//!
//! A schema-native, multiplexed, zero-RTT protocol for trading systems.
//! Unifies and supersedes FIX, REST, and WebSocket.
//!
//! # Overview
//!
//! The core library provides:
//! - **Frame** encoding/decoding with the 16-byte fixed header + TLV extensions
//! - **Extension tags** for protocol-level metadata (URIs, status codes, timestamps, etc.)
//! - **FrameDecoder** for streaming frame parsing over QUIC connections
//! - **Channel** management with QUIC stream ID mapping and sequence numbering
//! - **Session** model with pluggable storage backends
//! - **QUIC Transport** wrapper with TLS certificate generation
//! - **CBOR Codec** for self-describing payload encoding
//! - **SBE Codec** for zero-alloc binary encoding of trading messages
//! - **Error types** for frame, channel, and session operations
//!
//! # Example
//!
//! ```ignore
//! use fig_core::frame::{Frame, FrameType, FrameDecoder};
//! use fig_core::ext::{Extension, ExtensionTag};
//! use fig_core::channel::{ChannelManager, ChannelMode};
//!
//! // Create a request frame
//! let frame = Frame::new(FrameType::Request, 1)
//!     .with_seq(42)
//!     .with_schema_id(0x01)
//!     .with_extension(Extension::text(ExtensionTag::RequestUri, "/orders"))
//!     .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
//!     .with_payload(b"order data".to_vec());
//!
//! // Encode to bytes
//! let encoded = frame.encode().unwrap();
//!
//! // Decode from bytes
//! let (decoded, _) = Frame::decode(&encoded).unwrap();
//! assert_eq!(decoded.frame_type, FrameType::Request);
//!
//! // Streaming decode (for QUIC reads)
//! let mut decoder = FrameDecoder::new();
//! decoder.feed(&partial_chunk);
//! if let Some(result) = decoder.decode_next() {
//!     let frame = result.unwrap();
//! }
//!
//! // Channel management
//! let mut mgr = ChannelManager::new(false); // client
//! let ch = mgr.open_channel(ChannelMode::Stateless, None).unwrap();
//! assert_eq!(mgr.quic_stream_id(ch), 4);
//! ```
//!
//! # Module Index
//!
//! | Module | Purpose |
//! |---|---|
//! | [`frame`] | FIG frame wire format, encoding/decoding, streaming parser |
//! | [`ext`] | Extension tags (TLV) for protocol metadata |
//! | [`channel`] | Channel lifecycle, sequence numbering, QUIC stream ID mapping |
//! | [`session`] | Durable session model with pluggable storage backends |
//! | [`transport`] | QUIC transport wrapper, TLS cert generation, ALPN |
//! | [`codec`] | CBOR encode/decode helpers for self-describing payloads |
//! | [`sbe`] | SBE binary encoder/decoder for trading messages (zero-alloc) |
//! | [`error`] | Error types: `FrameError`, `ChannelError`, `SessionError` |
//! | [`auth`] | Authentication methods: token-based and mTLS |
//! | [`observability`] | Tracing spans and atomic metrics counters |

pub mod auth;
pub mod channel;
pub mod codec;
pub mod error;
pub mod ext;
pub mod frame;
pub mod messages;
pub mod observability;
pub mod sbe;
pub mod session;
pub mod transport;

pub use auth::*;
pub use channel::*;
pub use codec::*;
pub use error::*;
pub use ext::*;
pub use frame::*;
pub use observability::*;
pub use sbe::*;
pub use session::*;
pub use transport::*;
