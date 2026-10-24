//! FFI merge handle smoke tests for perps stream state.

use std::ffi::{CStr, CString};

use fig_core::codec::encode_cbor;
use fig_core::messages::{
    AggregateTrade, AggregateTradeEvent, AllMidsBatch, BestBidOffer, CandleBar, CandleBarEvent,
    ExecType, ExecutionReport, FundingPayment, LedgerUpdate, LedgerUpdateKind, LiquidationTrade,
    LiquidationTradeEvent, MarkPriceUpdate, MarketDataAction, MarketDataUpdate, MiniTicker,
    OpenOrdersSnapshot, OrdStatus, OrderBookDelta, OrderBookSnapshot, Price, PriceLevel,
    PublicTrade, PublicTradeEvent, Quantity, Side, UserLiquidation,
};
use fig_ffi::candles::{
    fig_candle_apply, fig_candle_close, fig_candle_free, fig_candle_interval, fig_candle_is_final,
    fig_candle_new, fig_candle_symbol,
};
use fig_ffi::order_book::{
    fig_order_book_apply_delta, fig_order_book_apply_snapshot, fig_order_book_best_bid,
    fig_order_book_free, fig_order_book_new,
};
use fig_ffi::stream_state::{
    fig_agg_trades_apply, fig_agg_trades_free, fig_agg_trades_latest_price, fig_agg_trades_len,
    fig_agg_trades_new, fig_funding_apply, fig_funding_free, fig_funding_latest_amount,
    fig_funding_len, fig_funding_new, fig_ledger_apply, fig_ledger_free, fig_ledger_len,
    fig_ledger_new, fig_liquidation_apply_public, fig_liquidation_apply_user, fig_liquidation_free,
    fig_liquidation_new, fig_liquidation_user_count,
};
use fig_ffi::stream_state::{
    fig_bbo_apply, fig_bbo_best_ask, fig_bbo_best_bid, fig_bbo_free, fig_bbo_implied_mid,
    fig_bbo_new, fig_mark_price_apply, fig_mark_price_free, fig_mark_price_get, fig_mark_price_new,
    fig_mids_apply_batch, fig_mids_apply_ticker, fig_mids_free, fig_mids_mid, fig_mids_new,
    fig_orders_apply_execution, fig_orders_apply_snapshot, fig_orders_execution_count,
    fig_orders_free, fig_orders_new, fig_orders_open_count, fig_trade_tape_free,
    fig_trade_tape_latest_price, fig_trade_tape_len, fig_trade_tape_new, fig_trade_tape_push,
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
    let payload = encode_cbor(&AggregateTradeEvent {
        trade: trade.clone(),
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_agg_trades_apply(handle, payload.as_ptr(), payload.len()) },
        0
    );
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
    assert_eq!(
        unsafe { fig_funding_apply(handle, payload.as_ptr(), payload.len()) },
        0
    );
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
    assert_eq!(
        unsafe { fig_ledger_apply(handle, payload.as_ptr(), payload.len()) },
        0
    );
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
        unsafe {
            fig_liquidation_apply_public(handle, public_payload.as_ptr(), public_payload.len())
        },
        0
    );
    unsafe { fig_liquidation_free(handle) };
}

fn level(price: f64, qty: f64) -> PriceLevel {
    PriceLevel {
        price: Price(price),
        qty: Quantity(qty),
        order_count: None,
    }
}

fn execution(order_id: &str, status: OrdStatus) -> ExecutionReport {
    ExecutionReport {
        cl_ord_id: "ORD-1".into(),
        order_id: order_id.into(),
        exec_id: "EX-1".into(),
        exec_type: if status == OrdStatus::Filled {
            ExecType::Fill
        } else {
            ExecType::New
        },
        ord_status: status,
        side: Side::Buy,
        last_qty: None,
        last_price: None,
        leaves_qty: Quantity(1.0),
        cum_qty: Quantity(0.0),
        avg_price: Price(10.0),
        symbol: "AAPL".into(),
        transact_time: 1,
    }
}

#[test]
fn order_book_handle_round_trip() {
    let handle = fig_order_book_new();
    let snap = encode_cbor(&OrderBookSnapshot {
        symbol: "AAPL".into(),
        exchange: "FIG".into(),
        bids: vec![level(10.0, 2.0)],
        asks: vec![level(11.0, 1.0)],
        timestamp: 1,
        sequence: Some(1),
        is_snapshot: Some(true),
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_order_book_apply_snapshot(handle, snap.as_ptr(), snap.len()) },
        0
    );
    assert_eq!(unsafe { fig_order_book_best_bid(handle) }, 10.0);
    let delta = encode_cbor(&OrderBookDelta {
        symbol: "AAPL".into(),
        updates: vec![MarketDataUpdate {
            side: Side::Buy,
            action: MarketDataAction::Change,
            price: Price(10.0),
            qty: Quantity(3.0),
        }],
        timestamp: 2,
        sequence: Some(2),
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_order_book_apply_delta(handle, delta.as_ptr(), delta.len()) },
        0
    );
    assert_eq!(unsafe { fig_order_book_best_bid(handle) }, 10.0);
    unsafe { fig_order_book_free(handle) };
}

#[test]
fn mids_handle_round_trip() {
    let handle = fig_mids_new();
    let ticker = encode_cbor(&MiniTicker {
        symbol: "BTC".into(),
        last_price: Price(100.0),
        volume: Quantity(1.0),
        timestamp: 1,
        is_snapshot: None,
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_mids_apply_ticker(handle, ticker.as_ptr(), ticker.len()) },
        0
    );
    let btc = CString::new("BTC").unwrap();
    assert_eq!(unsafe { fig_mids_mid(handle, btc.as_ptr()) }, 100.0);
    let batch = encode_cbor(&AllMidsBatch {
        tickers: vec![MiniTicker {
            symbol: "ETH".into(),
            last_price: Price(200.0),
            volume: Quantity(2.0),
            timestamp: 2,
            is_snapshot: None,
        }],
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_mids_apply_batch(handle, batch.as_ptr(), batch.len()) },
        0
    );
    let eth = CString::new("ETH").unwrap();
    assert_eq!(unsafe { fig_mids_mid(handle, eth.as_ptr()) }, 200.0);
    unsafe { fig_mids_free(handle) };
}

#[test]
fn bbo_handle_round_trip() {
    let handle = fig_bbo_new();
    let payload = encode_cbor(&BestBidOffer {
        symbol: "AAPL".into(),
        bid_price: Some(Price(10.0)),
        bid_qty: Some(Quantity(1.0)),
        ask_price: Some(Price(12.0)),
        ask_qty: Some(Quantity(1.0)),
        timestamp: 1,
        is_snapshot: Some(true),
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_bbo_apply(handle, payload.as_ptr(), payload.len()) },
        0
    );
    assert_eq!(unsafe { fig_bbo_best_bid(handle) }, 10.0);
    assert_eq!(unsafe { fig_bbo_best_ask(handle) }, 12.0);
    assert_eq!(unsafe { fig_bbo_implied_mid(handle) }, 11.0);
    unsafe { fig_bbo_free(handle) };
}

#[test]
fn trade_tape_handle_round_trip() {
    let handle = fig_trade_tape_new(4);
    let payload = encode_cbor(&PublicTradeEvent {
        trade: PublicTrade {
            symbol: "AAPL".into(),
            trade_id: "T1".into(),
            price: Price(7.0),
            qty: Quantity(1.0),
            side: Side::Buy,
            timestamp: 1,
        },
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_trade_tape_push(handle, payload.as_ptr(), payload.len()) },
        0
    );
    assert_eq!(unsafe { fig_trade_tape_len(handle) }, 1);
    assert_eq!(unsafe { fig_trade_tape_latest_price(handle) }, 7.0);
    unsafe { fig_trade_tape_free(handle) };
}

#[test]
fn mark_price_handle_round_trip() {
    let handle = fig_mark_price_new();
    let payload = encode_cbor(&MarkPriceUpdate {
        symbol: "AAPL".into(),
        mark_price: Price(50.0),
        index_price: None,
        funding_rate: None,
        timestamp: 1,
        is_snapshot: None,
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_mark_price_apply(handle, payload.as_ptr(), payload.len()) },
        0
    );
    let symbol = CString::new("AAPL").unwrap();
    assert_eq!(unsafe { fig_mark_price_get(handle, symbol.as_ptr()) }, 50.0);
    unsafe { fig_mark_price_free(handle) };
}

#[test]
fn orders_handle_round_trip() {
    let handle = fig_orders_new(4);
    let snap = encode_cbor(&OpenOrdersSnapshot {
        account: "TEST".into(),
        orders: vec![execution("O1", OrdStatus::New)],
        is_snapshot: Some(true),
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_orders_apply_snapshot(handle, snap.as_ptr(), snap.len()) },
        0
    );
    assert_eq!(unsafe { fig_orders_open_count(handle) }, 1);
    let fill = encode_cbor(&execution("O1", OrdStatus::Filled)).unwrap();
    assert_eq!(
        unsafe { fig_orders_apply_execution(handle, fill.as_ptr(), fill.len()) },
        0
    );
    assert_eq!(unsafe { fig_orders_execution_count(handle) }, 1);
    assert_eq!(unsafe { fig_orders_open_count(handle) }, 0);
    unsafe { fig_orders_free(handle) };
}

#[test]
fn candle_handle_round_trip() {
    let handle = fig_candle_new();
    assert_eq!(unsafe { fig_candle_is_final(handle) }, -1);
    assert!(unsafe { fig_candle_close(handle) }.is_nan());
    let open = encode_cbor(&CandleBarEvent {
        bar: CandleBar {
            symbol: "BTC".into(),
            interval: "5m".into(),
            open: Price(1.0),
            high: Price(2.0),
            low: Price(0.5),
            close: Price(1.5),
            volume: Quantity(100.0),
            bar_start: 0,
            bar_end: 300,
            is_final: false,
            is_snapshot: None,
        },
    })
    .unwrap();
    assert_eq!(
        unsafe { fig_candle_apply(handle, open.as_ptr(), open.len()) },
        0
    );
    assert_eq!(unsafe { fig_candle_close(handle) }, 1.5);
    assert_eq!(unsafe { fig_candle_is_final(handle) }, 0);
    let mut symbol = std::ptr::null_mut();
    let mut interval = std::ptr::null_mut();
    assert_eq!(unsafe { fig_candle_symbol(handle, &mut symbol) }, 0);
    assert_eq!(unsafe { fig_candle_interval(handle, &mut interval) }, 0);
    assert_eq!(unsafe { CStr::from_ptr(symbol) }.to_str().unwrap(), "BTC");
    assert_eq!(unsafe { CStr::from_ptr(interval) }.to_str().unwrap(), "5m");
    unsafe {
        fig_ffi::fig_string_free(symbol);
        fig_ffi::fig_string_free(interval);
    }
    unsafe { fig_candle_free(handle) };
}
