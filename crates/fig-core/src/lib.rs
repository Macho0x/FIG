//! FIG — Fast Interchange Gateway
//!
//! A schema-native, multiplexed, zero-RTT protocol for trading systems.
//! Unifies and supersedes FIX, REST, and WebSocket.
//!
//! # Overview
//!
//! The core library provides:
//! - **Frame** encoding/decoding with the 16-byte fixed header + TLV extensions
//! - **Extension tags** for protocol-level metadata (URIs, status codes, timestamps, etc.)
//! - **FrameDecoder** for streaming frame parsing over TREE connections
//! - **Channel** management with TREE stream ID mapping and sequence numbering
//! - **Session** model with pluggable storage backends
//! - **TREE Transport** wrapper with TLS certificate generation
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
//! // Streaming decode (for TREE reads)
//! let mut decoder = FrameDecoder::new();
//! decoder.feed(&partial_chunk);
//! if let Some(result) = decoder.decode_next() {
//!     let frame = result.unwrap();
//! }
//!
//! // Channel management
//! let mut mgr = ChannelManager::new(false); // client
//! let ch = mgr.open_channel(ChannelMode::Stateless, None).unwrap();
//! assert_eq!(mgr.tree_stream_id(ch), 4);
//! ```
//!
//! # Module Index
//!
//! | Module | Purpose |
//! |---|---|
//! | [`frame`] | FIG frame wire format, encoding/decoding, streaming parser |
//! | [`ext`] | Extension tags (TLV) for protocol metadata |
//! | [`channel`] | Channel lifecycle, sequence numbering, TREE stream ID mapping |
//! | [`session`] | Durable session model with pluggable storage backends |
//! | [`transport`] | TREE transport wrapper, TLS cert generation, ALPN |
//! | [`codec`] | CBOR encode/decode helpers for self-describing payloads |
//! | [`sbe`] | SBE binary encoder/decoder for trading messages (zero-alloc) |
//! | [`error`] | Error types: `FrameError`, `ChannelError`, `SessionError` |
//! | [`auth`] | Authentication methods: token-based and mTLS |
//! | [`control`] | Control frame dispatcher: PING/PONG, AUTH_REFRESH, SEQ_RESET |
//! | [`observability`] | Tracing spans and atomic metrics counters |

pub mod auth;
pub mod channel;
pub mod channel_auth;
pub mod codec;
pub mod compression;
pub mod control;
pub mod dos;
pub mod error;
pub mod ext;
pub mod fragment;
pub mod frame;
pub mod jwt;
pub mod messages;
pub mod migration;
pub mod oauth;
pub mod observability;
pub mod protobuf;
pub mod rate_limit;
pub mod sbe;
pub mod sbe_generated;
pub mod session;
pub mod tcp;
pub mod trace;
pub mod transport;

pub use auth::*;
pub use channel::*;
pub use channel_auth::*;
pub use codec::*;
pub use compression::*;
pub use control::*;
pub use dos::*;
pub use error::*;
pub use ext::*;
pub use fragment::*;
pub use frame::*;
pub use jwt::*;
pub use migration::*;
pub use oauth::*;
pub use observability::*;
pub use protobuf::*;
pub use rate_limit::*;
pub use sbe::*;
pub use session::*;
pub use tcp::*;
pub use trace::*;
pub use transport::*;
