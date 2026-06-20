//! Gateway adapters for FIG — translate legacy protocols to/from FIG frames.
//!
//! This crate provides backwards-compatibility adapters:
//!
//! - **FIX** — Parse/serialize FIX 4.4 messages, map to/from FIG trading messages
//! - **REST** — Parse HTTP/1.1 requests, map to/from FIG request/response frames
//! - **WebSocket** — Parse RFC 6455 frames, map to/from FIG stream frames
//! - **SSE** — Parse Server-Sent Events, map to/from FIG STREAM_ITEM frames
//!
//! # Example
//!
//! ```ignore
//! use fig_gateways::fix::{parse_fix_message, fix_to_fig_order};
//! use fig_gateways::rest::{parse_http_request, http_to_fig_frame};
//! use fig_gateways::ws::{parse_ws_frame, ws_to_fig_frame};
//! use fig_gateways::sse::{parse_sse_chunk, sse_to_fig_stream_item};
//! ```

pub mod fix;
pub mod fix_seq_store;
pub mod fix_session;
pub mod fix_tls;
pub mod rest;
pub mod sse;
pub mod ws;
