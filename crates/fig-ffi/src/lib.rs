//! Stable C ABI over `fig-core` for multi-language SDK bindings.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce};

/// Opaque owned byte buffer returned to callers.
#[repr(C)]
pub struct FigBuffer {
    pub data: *mut u8,
    pub len: usize,
}

/// Free a buffer previously returned by fig_* functions.
#[no_mangle]
pub extern "C" fn fig_buffer_free(buf: FigBuffer) {
    if !buf.data.is_null() && buf.len > 0 {
        unsafe {
            drop(Vec::from_raw_parts(buf.data, buf.len, buf.len));
        }
    }
}

fn into_buffer(bytes: Vec<u8>) -> FigBuffer {
    let mut bytes = bytes;
    let ptr = bytes.as_mut_ptr();
    let len = bytes.len();
    std::mem::forget(bytes);
    FigBuffer { data: ptr, len }
}

/// Returns an owned version string (caller must free with `fig_buffer_free` on the inner bytes
/// via copying — use `fig_version` pointer only, it is static).
#[no_mangle]
pub extern "C" fn fig_version() -> *const c_char {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_char
}

/// Encode a minimal REQUEST frame. Returns 0 on success.
///
/// # Safety
///
/// `out` must be a valid pointer. `payload`/`payload_len` must describe a valid buffer when non-null.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_request(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    payload: *const u8,
    payload_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let payload = if payload.is_null() || payload_len == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(payload, payload_len).to_vec()
    };
    let frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id)
        .with_payload(payload);
    match frame.encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -2,
    }
}

/// Decode frame header fields from encoded bytes. Returns 0 on success.
///
/// # Safety
///
/// Pointers must be valid for reads/writes of the described lengths.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_decode_header(
    data: *const u8,
    data_len: usize,
    out_channel_id: *mut u16,
    out_stream_seq: *mut u32,
    out_schema_id: *mut u8,
    out_frame_type: *mut u8,
) -> i32 {
    if data.is_null() || out_channel_id.is_null() || out_stream_seq.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let Ok((frame, _)) = Frame::decode(bytes) else {
        return -2;
    };
    *out_channel_id = frame.channel_id;
    *out_stream_seq = frame.stream_seq;
    if !out_schema_id.is_null() {
        *out_schema_id = frame.schema_id;
    }
    if !out_frame_type.is_null() {
        *out_frame_type = frame.frame_type as u8;
    }
    0
}

/// Encode a canonical limit-order NewOrderSingle as CBOR.
///
/// # Safety
///
/// String pointers must be valid NUL-terminated UTF-8; `out` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_new_order_single(
    cl_ord_id: *const c_char,
    symbol: *const c_char,
    side_buy: u8,
    order_qty: f64,
    price: f64,
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
    let side = if side_buy != 0 { Side::Buy } else { Side::Sell };
    let order = NewOrderSingle {
        cl_ord_id,
        side,
        order_qty: Quantity(order_qty),
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
    };
    match encode_cbor(&order) {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -3,
    }
}

/// Decode CBOR bytes into cl_ord_id (caller frees with `fig_string_free`).
///
/// # Safety
///
/// `data` must point to `data_len` readable bytes; `out_cl_ord_id` must be non-null.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_new_order_single_cl_ord_id(
    data: *const u8,
    data_len: usize,
    out_cl_ord_id: *mut *mut c_char,
) -> i32 {
    if data.is_null() || out_cl_ord_id.is_null() {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, data_len);
    let order: NewOrderSingle = match decode_cbor(bytes) {
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

/// Free a C string returned by this library.
///
/// # Safety
///
/// `s` must be a pointer previously returned by this crate and not yet freed.
#[no_mangle]
pub unsafe extern "C" fn fig_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[no_mangle]
pub extern "C" fn fig_client_channel_stream_id(channel_id: u16, is_server: u8) -> u64 {
    let offset = if is_server != 0 { 1 } else { 0 };
    channel_id as u64 * 4 + offset
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_frame_encode_ffi() {
        let mut out = FigBuffer {
            data: ptr::null_mut(),
            len: 0,
        };
        unsafe {
            assert_eq!(
                fig_frame_encode_request(1, 42, 1, ptr::null(), 0, &mut out),
                0
            );
            assert!(out.len >= 16);
            fig_buffer_free(out);
        }
    }
}
