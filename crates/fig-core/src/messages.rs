//! Trading message types generated from `schemas/orders.fsl`.
//!
//! Regenerate with `cargo xtask codegen`.

#[path = "generated/messages.rs"]
mod generated;
pub use generated::*;

/// Well-known schema IDs for FIG FSL domains (SPEC §9.1).
///
/// All merged FSL artifacts share wire schema `0x01` today; logical domain IDs
/// are advertised via `CapabilitiesResponse.schema_ids` and documentation.
pub mod schema_id {
    pub const TRADING_ORDERS: u8 = super::SCHEMA_ID;
    pub const DOMAIN_TRADING: u8 = 0x01;
    pub const DOMAIN_MARKET_DATA: u8 = 0x02;
    pub const DOMAIN_ACCOUNT: u8 = 0x03;
}

/// Well-known channel paths for the exchange simulator.
pub mod channel_path {
    pub const ORDERS: &str = "trading/accounts/{account}/orders";
    pub const CANCEL: &str = "trading/accounts/{account}/orders/{order_id}/cancel";
    pub const CANCEL_REPLACE: &str = "trading/accounts/{account}/orders/{order_id}/replace";
    pub const EXECUTIONS: &str = "trading/accounts/{account}/executions";
    pub const MARKET_DATA: &str = "marketdata/{symbol}/quotes";
    pub const BOOK: &str = "marketdata/{symbol}/book";
    pub const BBO: &str = "marketdata/{symbol}/bbo";
    pub const TRADES: &str = "marketdata/{symbol}/trades";
    pub const CANDLES: &str = "marketdata/{symbol}/candles/{interval}";
    pub const ACCOUNT: &str = "accounts/{account}";
    pub const BALANCES: &str = "accounts/{account}/balances";
    pub const POSITIONS: &str = "accounts/{account}/positions";
    pub const MARGIN: &str = "accounts/{account}/margin";
    pub const FILLS: &str = "accounts/{account}/fills";
    pub const TICKER: &str = "marketdata/{symbol}/ticker";
    pub const FUNDING: &str = "accounts/{account}/funding";
    pub const LEDGER: &str = "accounts/{account}/ledger";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_cbor_round_trip() {
        let order = NewOrderSingle {
            cl_ord_id: "ORD-001".to_string(),
            side: Side::Buy,
            order_qty: Quantity(100.0),
            price: Some(Price(50.25)),
            stop_price: None,
            symbol: "AAPL".to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: Some("ACCT-123".to_string()),
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        };

        let mut buf = Vec::new();
        ciborium::ser::into_writer(&order, &mut buf).unwrap();
        let decoded: NewOrderSingle = ciborium::de::from_reader(&buf[..]).unwrap();
        assert_eq!(order, decoded);
    }
}
