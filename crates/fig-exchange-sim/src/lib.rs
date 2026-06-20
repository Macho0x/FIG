//! FIG Exchange Simulator
//!
//! A native FIG exchange simulator that demonstrates the protocol's
//! capabilities: order entry, market data streaming, and session management.

pub mod account_state;
pub mod auth;
pub mod broker_api;
pub mod broker_session;
pub mod market_data;
pub mod matching;
pub mod orderbook;
pub mod server;

pub use matching::MatchingEngine;
pub use orderbook::OrderBook;
pub use server::*;
