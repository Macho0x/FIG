//! Order book merge helpers for FFI bindings.

use std::slice;

use fig_client::order_book::OrderBookState;
use fig_core::codec::decode_cbor;
use fig_core::messages::{OrderBookDelta, OrderBookSnapshot};

/// Opaque order book state for C bindings.
pub struct FigOrderBookHandle {
    state: OrderBookState,
}

/// Create an empty order book state.
#[no_mangle]
pub extern "C" fn fig_order_book_new() -> *mut FigOrderBookHandle {
    Box::into_raw(Box::new(FigOrderBookHandle {
        state: OrderBookState::default(),
    }))
}

/// Free order book state.
///
/// # Safety
/// `handle` must be from `fig_order_book_new` and not already freed.
#[no_mangle]
pub unsafe extern "C" fn fig_order_book_free(handle: *mut FigOrderBookHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Apply CBOR `OrderBookSnapshot` payload. Returns 0 on success, -1 on error.
///
/// # Safety
/// `payload` must be valid for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn fig_order_book_apply_snapshot(
    handle: *mut FigOrderBookHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() || payload.is_null() {
        return -1;
    }
    let data = slice::from_raw_parts(payload, len);
    let snap: OrderBookSnapshot = match decode_cbor(data) {
        Ok(s) => s,
        Err(_) => return -1,
    };
    let h = &mut *handle;
    match h.state.apply_snapshot(&snap) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Apply CBOR `OrderBookDelta` payload. Returns 0 on success, -1 on error.
///
/// # Safety
/// `payload` must be valid for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn fig_order_book_apply_delta(
    handle: *mut FigOrderBookHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() || payload.is_null() {
        return -1;
    }
    let data = slice::from_raw_parts(payload, len);
    let delta: OrderBookDelta = match decode_cbor(data) {
        Ok(d) => d,
        Err(_) => return -1,
    };
    let h = &mut *handle;
    match h.state.apply_delta(&delta) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

/// Best bid price, or NaN if empty.
///
/// # Safety
/// `handle` must be valid.
#[no_mangle]
pub unsafe extern "C" fn fig_order_book_best_bid(handle: *const FigOrderBookHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle)
        .state
        .bids
        .first()
        .map(|l| l.price.0)
        .unwrap_or(f64::NAN)
}
