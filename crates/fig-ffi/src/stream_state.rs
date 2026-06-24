//! Stream merge state handles for C / multi-language bindings.

use std::slice;

use fig_client::{BboState, MarkPriceState, MidsState, OrdersState, TradeTape};
use fig_core::codec::decode_cbor;
use fig_core::messages::{
    AllMidsBatch, BestBidOffer, ExecutionReport, MarkPriceUpdate, MiniTicker, OpenOrdersSnapshot,
    PublicTradeEvent,
};

macro_rules! opaque_handle {
    ($name:ident, $inner:ty) => {
        pub struct $name {
            state: $inner,
        }
    };
}

opaque_handle!(FigMidsHandle, MidsState);
opaque_handle!(FigBboHandle, BboState);
opaque_handle!(FigTradeTapeHandle, TradeTape);
opaque_handle!(FigMarkPriceHandle, MarkPriceState);
opaque_handle!(FigOrdersHandle, OrdersState);

fn apply_cbor<T: serde::de::DeserializeOwned>(
    payload: *const u8,
    len: usize,
    apply: impl FnOnce(&T),
) -> i32 {
    if payload.is_null() {
        return -1;
    }
    let data = unsafe { slice::from_raw_parts(payload, len) };
    match decode_cbor::<T>(data) {
        Ok(v) => {
            apply(&v);
            0
        }
        Err(_) => -2,
    }
}

// ─── Mids ───────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn fig_mids_new() -> *mut FigMidsHandle {
    Box::into_raw(Box::new(FigMidsHandle {
        state: MidsState::default(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn fig_mids_free(handle: *mut FigMidsHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_mids_apply_ticker(
    handle: *mut FigMidsHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |t: &MiniTicker| {
        (*handle).state.apply_ticker(t)
    })
}

#[no_mangle]
pub unsafe extern "C" fn fig_mids_apply_batch(
    handle: *mut FigMidsHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |b: &AllMidsBatch| {
        (*handle).state.apply_batch(b)
    })
}

#[no_mangle]
pub unsafe extern "C" fn fig_mids_mid(
    handle: *const FigMidsHandle,
    symbol: *const std::os::raw::c_char,
) -> f64 {
    if handle.is_null() || symbol.is_null() {
        return f64::NAN;
    }
    let sym = match unsafe { std::ffi::CStr::from_ptr(symbol) }.to_str() {
        Ok(s) => s,
        Err(_) => return f64::NAN,
    };
    (*handle).state.mid(sym).unwrap_or(f64::NAN)
}

// ─── BBO ────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn fig_bbo_new() -> *mut FigBboHandle {
    Box::into_raw(Box::new(FigBboHandle {
        state: BboState::default(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn fig_bbo_free(handle: *mut FigBboHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_bbo_apply(
    handle: *mut FigBboHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |b: &BestBidOffer| (*handle).state.apply(b))
}

#[no_mangle]
pub unsafe extern "C" fn fig_bbo_implied_mid(handle: *const FigBboHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle).state.implied_mid().unwrap_or(f64::NAN)
}

#[no_mangle]
pub unsafe extern "C" fn fig_bbo_best_bid(handle: *const FigBboHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle).state.best_bid().unwrap_or(f64::NAN)
}

#[no_mangle]
pub unsafe extern "C" fn fig_bbo_best_ask(handle: *const FigBboHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle).state.best_ask().unwrap_or(f64::NAN)
}

// ─── Trades ─────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn fig_trade_tape_new(capacity: usize) -> *mut FigTradeTapeHandle {
    Box::into_raw(Box::new(FigTradeTapeHandle {
        state: TradeTape::with_capacity(capacity),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn fig_trade_tape_free(handle: *mut FigTradeTapeHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_trade_tape_push(
    handle: *mut FigTradeTapeHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |ev: &PublicTradeEvent| {
        (*handle).state.push_event(ev)
    })
}

#[no_mangle]
pub unsafe extern "C" fn fig_trade_tape_len(handle: *const FigTradeTapeHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    (*handle).state.len()
}

#[no_mangle]
pub unsafe extern "C" fn fig_trade_tape_latest_price(handle: *const FigTradeTapeHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle)
        .state
        .latest()
        .map(|t| t.price.0)
        .unwrap_or(f64::NAN)
}

// ─── Mark price ─────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn fig_mark_price_new() -> *mut FigMarkPriceHandle {
    Box::into_raw(Box::new(FigMarkPriceHandle {
        state: MarkPriceState::default(),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn fig_mark_price_free(handle: *mut FigMarkPriceHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_mark_price_apply(
    handle: *mut FigMarkPriceHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |m: &MarkPriceUpdate| (*handle).state.apply(m))
}

#[no_mangle]
pub unsafe extern "C" fn fig_mark_price_get(
    handle: *const FigMarkPriceHandle,
    symbol: *const std::os::raw::c_char,
) -> f64 {
    if handle.is_null() || symbol.is_null() {
        return f64::NAN;
    }
    let sym = match unsafe { std::ffi::CStr::from_ptr(symbol) }.to_str() {
        Ok(s) => s,
        Err(_) => return f64::NAN,
    };
    (*handle).state.mark(sym).unwrap_or(f64::NAN)
}

// ─── Orders + executions ──────────────────────────────────────────

#[no_mangle]
pub extern "C" fn fig_orders_new(exec_capacity: usize) -> *mut FigOrdersHandle {
    Box::into_raw(Box::new(FigOrdersHandle {
        state: OrdersState::with_exec_capacity(exec_capacity),
    }))
}

#[no_mangle]
pub unsafe extern "C" fn fig_orders_free(handle: *mut FigOrdersHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_orders_apply_snapshot(
    handle: *mut FigOrdersHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |s: &OpenOrdersSnapshot| {
        (*handle).state.apply_open_orders_snapshot(s)
    })
}

#[no_mangle]
pub unsafe extern "C" fn fig_orders_apply_execution(
    handle: *mut FigOrdersHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() {
        return -1;
    }
    apply_cbor(payload, len, |r: &ExecutionReport| {
        (*handle).state.apply_execution_report(r)
    })
}

#[no_mangle]
pub unsafe extern "C" fn fig_orders_open_count(handle: *const FigOrdersHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    (*handle).state.open_count()
}

#[no_mangle]
pub unsafe extern "C" fn fig_orders_execution_count(handle: *const FigOrdersHandle) -> usize {
    if handle.is_null() {
        return 0;
    }
    (*handle).state.executions().len()
}
