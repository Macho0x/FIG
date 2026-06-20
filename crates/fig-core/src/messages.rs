//! Trading message types generated from `schemas/orders.fsl`.
//!
//! Regenerate with `cargo xtask codegen`.

#[path = "generated/messages.rs"]
mod generated;
pub use generated::*;

/// Well-known schema IDs for the trading messages.
pub mod schema_id {
    pub const TRADING_ORDERS: u8 = super::SCHEMA_ID;
}

/// Well-known channel paths for the exchange simulator.
pub mod channel_path {
    pub const ORDERS: &str = "trading/accounts/{account}/orders";
    pub const CANCEL: &str = "trading/accounts/{account}/orders/{order_id}/cancel";
    pub const CANCEL_REPLACE: &str = "trading/accounts/{account}/orders/{order_id}/replace";
    pub const EXECUTIONS: &str = "trading/accounts/{account}/executions";
    pub const MARKET_DATA: &str = "marketdata/{symbol}/quotes";
    pub const ACCOUNT: &str = "accounts/{account}";
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
        };

        let mut buf = Vec::new();
        ciborium::ser::into_writer(&order, &mut buf).unwrap();
        let decoded: NewOrderSingle = ciborium::de::from_reader(&buf[..]).unwrap();
        assert_eq!(order, decoded);
    }
}
