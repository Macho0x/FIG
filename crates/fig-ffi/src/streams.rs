//! Typed stream helpers — decode STREAM_ITEM payloads via fig-core messages.

use std::slice;

use fig_core::codec::decode_cbor;
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{BalanceSnapshot, ExecutionReport, MarketDataSnapshot, SymbolTicker};

use crate::{into_buffer, FigBuffer};

/// Extract payload bytes from an encoded FIG frame.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_payload(
    frame_bytes: *const u8,
    frame_len: usize,
    out: *mut FigBuffer,
    out_frame_type: *mut u8,
    out_schema_id: *mut u8,
) -> i32 {
    if out.is_null() || frame_bytes.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(frame_bytes, frame_len);
    let (frame, _) = match Frame::decode(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    if !out_frame_type.is_null() {
        *out_frame_type = frame.frame_type as u8;
    }
    if !out_schema_id.is_null() {
        *out_schema_id = frame.schema_id;
    }
    *out = into_buffer(frame.payload);
    0
}

/// Return 1 when frame is STREAM_ITEM, else 0. Returns -1 on decode error.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_is_stream_item(frame_bytes: *const u8, frame_len: usize) -> i32 {
    if frame_bytes.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(frame_bytes, frame_len);
    match Frame::decode(bytes) {
        Ok((frame, _)) => i32::from(frame.frame_type == FrameType::StreamItem),
        Err(_) => -1,
    }
}

/// Decode ExecutionReport CBOR payload; write cl_ord_id to newly allocated C string.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_execution_report_cl_ord_id(
    data: *const u8,
    data_len: usize,
    out_cl_ord_id: *mut *mut std::os::raw::c_char,
) -> i32 {
    if data.is_null() || out_cl_ord_id.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let report: ExecutionReport = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    match std::ffi::CString::new(report.cl_ord_id) {
        Ok(s) => {
            *out_cl_ord_id = s.into_raw();
            0
        }
        Err(_) => -3,
    }
}

/// Decode SymbolTicker CBOR; returns last price in `out_price` (0 on success).
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_symbol_ticker_price(
    data: *const u8,
    data_len: usize,
    out_price: *mut f64,
) -> i32 {
    if data.is_null() || out_price.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let ticker: SymbolTicker = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    *out_price = ticker.last_price.0;
    0
}

/// Decode BalanceSnapshot CBOR; returns account id via allocated C string.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_balance_snapshot_account(
    data: *const u8,
    data_len: usize,
    out_account: *mut *mut std::os::raw::c_char,
) -> i32 {
    if data.is_null() || out_account.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let snap: BalanceSnapshot = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    match std::ffi::CString::new(snap.account) {
        Ok(s) => {
            *out_account = s.into_raw();
            0
        }
        Err(_) => -3,
    }
}

/// Decode MarketDataSnapshot CBOR; returns symbol via allocated C string.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_market_data_snapshot_symbol(
    data: *const u8,
    data_len: usize,
    out_symbol: *mut *mut std::os::raw::c_char,
) -> i32 {
    if data.is_null() || out_symbol.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let snap: MarketDataSnapshot = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    match std::ffi::CString::new(snap.symbol) {
        Ok(s) => {
            *out_symbol = s.into_raw();
            0
        }
        Err(_) => -3,
    }
}
