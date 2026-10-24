//! Candle bar merge handle for C bindings.

use std::ffi::{c_char, CString};
use std::slice;

use fig_client::candles::CandleState;
use fig_core::codec::decode_cbor;
use fig_core::messages::CandleBarEvent;

/// Opaque candle merge state for C bindings.
pub struct FigCandleHandle {
    state: CandleState,
}

/// Create an empty candle state.
#[no_mangle]
pub extern "C" fn fig_candle_new() -> *mut FigCandleHandle {
    Box::into_raw(Box::new(FigCandleHandle {
        state: CandleState::default(),
    }))
}

/// Free candle state.
///
/// # Safety
/// `handle` must be from `fig_candle_new` and not already freed.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_free(handle: *mut FigCandleHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Apply a CBOR `CandleBarEvent`. Returns 0 on success, -1 on error.
///
/// # Safety
/// `payload` must be valid for `len` bytes.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_apply(
    handle: *mut FigCandleHandle,
    payload: *const u8,
    len: usize,
) -> i32 {
    if handle.is_null() || payload.is_null() {
        return -1;
    }
    let data = slice::from_raw_parts(payload, len);
    let event: CandleBarEvent = match decode_cbor(data) {
        Ok(event) => event,
        Err(_) => return -1,
    };
    (*handle).state.apply_event(&event);
    0
}

/// Close of the current bar, or NaN when empty.
///
/// # Safety
/// `handle` must be valid or null.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_close(handle: *const FigCandleHandle) -> f64 {
    if handle.is_null() {
        return f64::NAN;
    }
    (*handle)
        .state
        .current()
        .map(|bar| bar.close.0)
        .unwrap_or(f64::NAN)
}

/// `1` when the current bar is final, `0` when it is open, `-1` when empty.
///
/// # Safety
/// `handle` must be valid or null.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_is_final(handle: *const FigCandleHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }
    match (*handle).state.current() {
        Some(bar) if bar.is_final => 1,
        Some(_) => 0,
        None => -1,
    }
}

fn write_bar_text(value: Option<&str>, out: *mut *mut c_char) -> i32 {
    if out.is_null() {
        return -1;
    }
    let Some(value) = value.filter(|text| !text.is_empty()) else {
        unsafe { *out = std::ptr::null_mut() };
        return -1;
    };
    match CString::new(value) {
        Ok(text) => {
            unsafe { *out = text.into_raw() };
            0
        }
        Err(_) => -3,
    }
}

/// Symbol of the current bar. Free `out` with `fig_string_free`. Returns -1 when empty.
///
/// # Safety
/// `out` must be writable.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_symbol(
    handle: *const FigCandleHandle,
    out: *mut *mut c_char,
) -> i32 {
    if handle.is_null() {
        return write_bar_text(None, out);
    }
    let symbol = (*handle).state.current().map(|bar| bar.symbol.as_str());
    write_bar_text(symbol, out)
}

/// Interval of the current bar. Free `out` with `fig_string_free`. Returns -1 when empty.
///
/// # Safety
/// `out` must be writable.
#[no_mangle]
pub unsafe extern "C" fn fig_candle_interval(
    handle: *const FigCandleHandle,
    out: *mut *mut c_char,
) -> i32 {
    if handle.is_null() {
        return write_bar_text(None, out);
    }
    let interval = (*handle).state.current().map(|bar| bar.interval.as_str());
    write_bar_text(interval, out)
}
