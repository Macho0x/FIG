//! SBE message FFI over `fig_core::sbe`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

use fig_core::messages::{
    CandleBar, NewOrderSingle, OrderType, Price, Quantity, Side, SymbolTicker, TimeInForce,
};
use fig_core::sbe::{
    decode_new_order_single, encode_candle_bar, encode_new_order_single, encode_symbol_ticker,
};

use crate::{into_buffer, FigBuffer};

#[no_mangle]
pub unsafe extern "C" fn fig_sbe_encode_new_order_single(
    cl_ord_id: *const c_char,
    symbol: *const c_char,
    side_buy: u8,
    qty: f64,
    price: f64,
    post_only: i8,
    reduce_only: i8,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || cl_ord_id.is_null() || symbol.is_null() {
        return -1;
    }
    let cl_ord_id = match CStr::from_ptr(cl_ord_id).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };
    let symbol = match CStr::from_ptr(symbol).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };
    let post_only = match crate::tri_state_flag(post_only) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let reduce_only = match crate::tri_state_flag(reduce_only) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let order = NewOrderSingle {
        cl_ord_id,
        side: if side_buy != 0 { Side::Buy } else { Side::Sell },
        order_qty: Quantity(qty),
        price: Some(Price(price)),
        stop_price: None,
        symbol,
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only,
        reduce_only,
    };
    *out = into_buffer(encode_new_order_single(&order));
    0
}

#[no_mangle]
pub unsafe extern "C" fn fig_sbe_decode_new_order_single_cl_ord_id(
    data: *const u8,
    len: usize,
    out_cl_ord_id: *mut *mut c_char,
) -> i32 {
    if data.is_null() || out_cl_ord_id.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, len);
    let order = match decode_new_order_single(bytes) {
        Ok(o) => o,
        Err(_) => return -2,
    };
    match CString::new(order.cl_ord_id) {
        Ok(s) => {
            *out_cl_ord_id = s.into_raw();
            0
        }
        Err(_) => -3,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_sbe_encode_candle_bar(
    symbol: *const c_char,
    interval: *const c_char,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    bar_start: i64,
    bar_end: i64,
    is_final: u8,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || symbol.is_null() || interval.is_null() {
        return -1;
    }
    let symbol = match CStr::from_ptr(symbol).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };
    let interval = match CStr::from_ptr(interval).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };
    let bar = CandleBar {
        symbol,
        interval,
        open: Price(open),
        high: Price(high),
        low: Price(low),
        close: Price(close),
        volume: Quantity(volume),
        bar_start,
        bar_end,
        is_final: is_final != 0,
        is_snapshot: Some(false),
    };
    *out = into_buffer(encode_candle_bar(&bar));
    0
}

#[no_mangle]
pub unsafe extern "C" fn fig_sbe_encode_symbol_ticker(
    symbol: *const c_char,
    last_price: f64,
    price_change: f64,
    price_change_pct: f64,
    volume: f64,
    high: f64,
    low: f64,
    open: f64,
    timestamp: i64,
    is_snapshot: u8,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || symbol.is_null() {
        return -1;
    }
    let symbol = match CStr::from_ptr(symbol).to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return -2,
    };
    let ticker = SymbolTicker {
        symbol,
        last_price: Price(last_price),
        price_change,
        price_change_pct,
        volume: Quantity(volume),
        high: Price(high),
        low: Price(low),
        open: Price(open),
        timestamp,
        is_snapshot: if is_snapshot != 0 {
            Some(true)
        } else {
            Some(false)
        },
    };
    *out = into_buffer(encode_symbol_ticker(&ticker));
    0
}
