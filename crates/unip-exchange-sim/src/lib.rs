//! UNIP Exchange Simulator
//!
//! A native UNIP exchange simulator that demonstrates the protocol's
//! capabilities: order entry, market data streaming, and session management.

pub mod matching;
pub mod orderbook;

pub use matching::MatchingEngine;
pub use orderbook::OrderBook;