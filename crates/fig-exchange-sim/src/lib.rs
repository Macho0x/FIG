//! FIG Exchange Simulator
//!
//! A native FIG exchange simulator that demonstrates the protocol's
//! capabilities: order entry, market data streaming, and session management.

pub mod matching;
pub mod orderbook;
pub mod server;

pub use matching::MatchingEngine;
pub use orderbook::OrderBook;
pub use server::*;
