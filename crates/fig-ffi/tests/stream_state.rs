//! FFI merge handle smoke tests for perps stream state.

use fig_core::codec::encode_cbor;
use fig_core::messages::{
    AggregateTrade, AggregateTradeEvent, FundingPayment, LedgerUpdate, LedgerUpdateKind,
    LiquidationTrade, LiquidationTradeEvent, Price, Quantity, Side, UserLiquidation,
};
use fig_ffi::stream_state::{
    fig_agg_trades_apply, fig_agg_trades_free, fig_agg_trades_latest_price, fig_agg_trades_len,
    fig_agg_trades_new, fig_funding_apply, fig_funding_free, fig_funding_latest_amount,
    fig_funding_len, fig_funding_new, fig_ledger_apply, fig_ledger_free, fig_ledger_len,
    fig_ledger_new, fig_liquidation_apply_public, fig_liquidation_apply_user,
    fig_liquidation_free, fig_liquidation_new, fig_liquidation_user_count,
};

#[test]
fn agg_trades_handle_round_trip() {
    let handle = fig_agg_trades_new(8);
    let trade = AggregateTrade {
        symbol: "BTC".into(),
        agg_trade_id: "1".into(),
        price: Price(42_000.0),
        qty: Quantity(1.0),
        side: Side::Buy,
        timestamp: 1,
        first_trade_id: "1".into(),
        last_trade_id: "1".into(),
    };
    let payload = encode_cbor(&AggregateTradeEvent { trade: trade.clone() }).unwrap();
    assert_eq!(unsafe { fig_agg_trades_apply(handle, payload.as_ptr(), payload.len()) }, 0);
    assert_eq!(unsafe { fig_agg_trades_len(handle) }, 1);
    assert_eq!(unsafe { fig_agg_trades_latest_price(handle) }, 42_000.0);
    unsafe { fig_agg_trades_free(handle) };
}

#[test]
fn funding_handle_round_trip() {
    let handle = fig_funding_new(4);
    let payment = FundingPayment {
        account: "A".into(),
        symbol: Some("BTC".into()),
        amount: -1.5,
        rate: 0.0001,
        timestamp: 2,
    };
    let payload = encode_cbor(&payment).unwrap();
    assert_eq!(unsafe { fig_funding_apply(handle, payload.as_ptr(), payload.len()) }, 0);
    assert_eq!(unsafe { fig_funding_len(handle) }, 1);
    assert_eq!(unsafe { fig_funding_latest_amount(handle) }, -1.5);
    unsafe { fig_funding_free(handle) };
}

#[test]
fn ledger_handle_round_trip() {
    let handle = fig_ledger_new(4);
    let update = LedgerUpdate {
        account: "A".into(),
        asset: "USDC".into(),
        delta: 100.0,
        kind: LedgerUpdateKind::Deposit,
        timestamp: 3,
        reference_id: None,
    };
    let payload = encode_cbor(&update).unwrap();
    assert_eq!(unsafe { fig_ledger_apply(handle, payload.as_ptr(), payload.len()) }, 0);
    assert_eq!(unsafe { fig_ledger_len(handle) }, 1);
    unsafe { fig_ledger_free(handle) };
}

#[test]
fn liquidation_handle_round_trip() {
    let handle = fig_liquidation_new(4);
    let user = UserLiquidation {
        account: "A".into(),
        symbol: "BTC".into(),
        qty: Quantity(1.0),
        price: Price(40_000.0),
        timestamp: 4,
    };
    let user_payload = encode_cbor(&user).unwrap();
    assert_eq!(
        unsafe { fig_liquidation_apply_user(handle, user_payload.as_ptr(), user_payload.len()) },
        0
    );
    assert_eq!(unsafe { fig_liquidation_user_count(handle) }, 1);

    let trade = LiquidationTrade {
        symbol: "BTC".into(),
        price: Price(40_000.0),
        qty: Quantity(1.0),
        side: Side::Sell,
        timestamp: 5,
    };
    let public_payload = encode_cbor(&LiquidationTradeEvent { trade }).unwrap();
    assert_eq!(
        unsafe { fig_liquidation_apply_public(handle, public_payload.as_ptr(), public_payload.len()) },
        0
    );
    unsafe { fig_liquidation_free(handle) };
}
