//! High-level FIG client helpers: frame builders, stream merge, query wrappers.

pub mod account;
pub mod agg_trades;
pub mod bbo;
pub mod candles;
pub mod client;
pub mod frames;
pub mod funding;
pub mod ledger;
pub mod liquidations;
pub mod mark_price;
pub mod mids;
pub mod order_book;
pub mod orders;
pub mod trades;

pub use account::AccountCache;
pub use agg_trades::AggTradeState;
pub use bbo::BboState;
pub use candles::CandleState;
pub use client::{FigSdkClient, LiveSubscription};
pub use funding::FundingState;
pub use ledger::LedgerState;
pub use liquidations::LiquidationState;
pub use mark_price::MarkPriceState;
pub use mids::MidsState;
pub use order_book::{BookGap, OrderBookState};
pub use orders::OrdersState;
pub use trades::TradeTape;

/// Dev harness auth token for the reference simulator (`fig-dev-{account}`).
pub fn dev_auth_token(account: &str) -> String {
    format!("fig-dev-{account}")
}
