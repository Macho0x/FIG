//! Stable C ABI over `fig-core` for multi-language SDK bindings.

#![allow(clippy::missing_safety_doc)]

mod advanced;
mod binding;
mod client;
mod jwt;
mod order_book;
mod sbe;
mod stream_state;
mod streams;

pub use advanced::{
    fig_client_connect_0rtt, fig_client_export_resumption_token, fig_client_migration_apply,
    fig_client_migration_prepare, fig_frame_split, fig_frames_reassemble, fig_payload_compress,
    fig_payload_decompress, fig_session_from_resumption_token,
};
pub use binding::run_binding_conformance;
pub use client::{
    fig_client_close, fig_client_connect, fig_client_ping, fig_client_request_and_recv,
    fig_frame_list_free, FigClientHandle, FigFrameList,
};
pub use jwt::{fig_jwt_decode_sub, fig_jwt_encode, fig_jwt_verify_bearer};
pub use sbe::{
    fig_sbe_decode_new_order_single_cl_ord_id, fig_sbe_encode_candle_bar,
    fig_sbe_encode_new_order_single, fig_sbe_encode_symbol_ticker,
};
pub use streams::{
    fig_cbor_decode_balance_snapshot_account, fig_cbor_decode_execution_report_cl_ord_id,
    fig_cbor_decode_market_data_snapshot_symbol, fig_cbor_decode_symbol_ticker_price,
    fig_frame_is_stream_item, fig_frame_payload,
};
pub use stream_state::{
    fig_agg_trades_apply, fig_agg_trades_free, fig_agg_trades_latest_price, fig_agg_trades_len,
    fig_agg_trades_new, fig_funding_apply, fig_funding_free, fig_funding_latest_amount,
    fig_funding_len, fig_funding_new, fig_ledger_apply, fig_ledger_free, fig_ledger_len,
    fig_ledger_new, fig_liquidation_apply_public, fig_liquidation_apply_user,
    fig_liquidation_free, fig_liquidation_new, fig_liquidation_user_count,
};

use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    CandleBar, CapabilitiesResponse, CapabilityPath, CapabilityPathPattern,
    InstrumentCatalogResponse, InstrumentMetadata, MarketDataSnapshot, NewOrderSingle,
    OpenOrdersRequest, OpenOrdersSnapshot, OrderHistoryRequest, OrderType, Price, PriceLevel,
    Quantity, Side, TimeInForce,
};

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

pub(crate) fn into_buffer(bytes: Vec<u8>) -> FigBuffer {
    let mut bytes = bytes;
    let ptr = bytes.as_mut_ptr();
    let len = bytes.len();
    std::mem::forget(bytes);
    FigBuffer { data: ptr, len }
}

fn optional_cstr(s: *const c_char) -> Result<Option<String>, i32> {
    if s.is_null() {
        return Ok(None);
    }
    unsafe {
        CStr::from_ptr(s)
            .to_str()
            .map(|v| Some(v.to_string()))
            .map_err(|_| -2)
    }
}

/// Returns static version string.
#[no_mangle]
pub extern "C" fn fig_version() -> *const c_char {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_char
}

/// Encode a minimal REQUEST frame. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_request(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    payload: *const u8,
    payload_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    fig_frame_encode_request_ex(
        channel_id,
        stream_seq,
        schema_id,
        std::ptr::null(),
        std::ptr::null(),
        std::ptr::null(),
        payload,
        payload_len,
        out,
    )
}

/// Encode REQUEST with optional ChannelPath / Method / ContentType extensions.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_request_ex(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    channel_path: *const c_char,
    method: *const c_char,
    content_type: *const c_char,
    payload: *const u8,
    payload_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let channel_path = match optional_cstr(channel_path) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let method = match optional_cstr(method) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let content_type = match optional_cstr(content_type) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let payload = if payload.is_null() || payload_len == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(payload, payload_len).to_vec()
    };
    let mut frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id);
    if let Some(path) = channel_path {
        frame = frame.with_extension(Extension::text(ExtensionTag::ChannelPath, path));
    }
    if let Some(method) = method {
        frame = frame.with_extension(Extension::text(ExtensionTag::Method, method));
    }
    if let Some(ct) = content_type {
        frame = frame.with_extension(Extension::text(ExtensionTag::ContentType, ct));
    }
    frame = frame.with_payload(payload);
    match frame.encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -3,
    }
}

/// Encode SUBSCRIBE frame with routing key and channel path.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_subscribe(
    channel_id: u16,
    stream_seq: u32,
    routing_key: *const c_char,
    channel_path: *const c_char,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || routing_key.is_null() || channel_path.is_null() {
        return -1;
    }
    let routing_key = match CStr::from_ptr(routing_key).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let channel_path = match CStr::from_ptr(channel_path).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let frame = Frame::new(FrameType::Subscribe, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(0x01)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
        .with_extension(Extension::text(ExtensionTag::RoutingKey, routing_key))
        .with_extension(Extension::text(
            ExtensionTag::CorrelationId,
            stream_seq.to_string(),
        ));
    match frame.encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -3,
    }
}

/// Encode REQUEST with optional ChannelPath, Method, ContentType, AuthToken.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_request_auth(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    channel_path: *const c_char,
    method: *const c_char,
    content_type: *const c_char,
    auth_token: *const c_char,
    payload: *const u8,
    payload_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let channel_path = match optional_cstr(channel_path) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let method = match optional_cstr(method) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let content_type = match optional_cstr(content_type) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let auth_token = match optional_cstr(auth_token) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let payload = if payload.is_null() || payload_len == 0 {
        Vec::new()
    } else {
        slice::from_raw_parts(payload, payload_len).to_vec()
    };
    let mut frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id);
    if let Some(path) = channel_path {
        frame = frame.with_extension(Extension::text(ExtensionTag::ChannelPath, path));
    }
    if let Some(method) = method {
        frame = frame.with_extension(Extension::text(ExtensionTag::Method, method));
    }
    if let Some(ct) = content_type {
        frame = frame.with_extension(Extension::text(ExtensionTag::ContentType, ct));
    }
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    frame = frame.with_payload(payload);
    match frame.encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -3,
    }
}

/// Encode SUBSCRIBE with optional auth token (CorrelationId = stream_seq).
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_subscribe_auth(
    channel_id: u16,
    stream_seq: u32,
    routing_key: *const c_char,
    channel_path: *const c_char,
    auth_token: *const c_char,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || routing_key.is_null() || channel_path.is_null() {
        return -1;
    }
    let routing_key = match CStr::from_ptr(routing_key).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let channel_path = match CStr::from_ptr(channel_path).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let auth_token = match optional_cstr(auth_token) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mut frame = Frame::new(FrameType::Subscribe, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(0x01)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
        .with_extension(Extension::text(ExtensionTag::RoutingKey, routing_key))
        .with_extension(Extension::text(
            ExtensionTag::CorrelationId,
            stream_seq.to_string(),
        ));
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    match frame.encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -3,
    }
}

/// Encode PING control frame.
#[no_mangle]
pub unsafe extern "C" fn fig_frame_encode_ping(out: *mut FigBuffer) -> i32 {
    if out.is_null() {
        return -1;
    }
    match Frame::ping().encode() {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -2,
    }
}

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

pub(crate) fn tri_state_flag(v: i8) -> Result<Option<bool>, i32> {
    match v {
        -1 => Ok(None),
        0 => Ok(Some(false)),
        1 => Ok(Some(true)),
        _ => Err(-3),
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_new_order_single(
    cl_ord_id: *const c_char,
    symbol: *const c_char,
    side_buy: u8,
    order_qty: f64,
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
    let post_only = match tri_state_flag(post_only) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let reduce_only = match tri_state_flag(reduce_only) {
        Ok(v) => v,
        Err(e) => return e,
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
        post_only,
        reduce_only,
    };
    encode_cbor_out(&order, out)
}

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
    match std::ffi::CString::new(order.cl_ord_id) {
        Ok(s) => {
            *out_cl_ord_id = s.into_raw();
            0
        }
        Err(_) => -3,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_candle_bar(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_candle_bar(), out)
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_capabilities(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_capabilities(), out)
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_open_orders_snapshot(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_open_orders(), out)
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_market_data_snapshot(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_market_data(), out)
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_order_history_request(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_order_history_request(), out)
}

#[no_mangle]
pub unsafe extern "C" fn fig_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(std::ffi::CString::from_raw(s));
    }
}

#[no_mangle]
pub extern "C" fn fig_client_channel_stream_id(channel_id: u16, is_server: u8) -> u64 {
    let offset = if is_server != 0 { 1 } else { 0 };
    channel_id as u64 * 4 + offset
}

fn encode_cbor_out<T: serde::Serialize>(value: &T, out: *mut FigBuffer) -> i32 {
    if out.is_null() {
        return -1;
    }
    match encode_cbor(value) {
        Ok(bytes) => unsafe {
            *out = into_buffer(bytes);
            0
        },
        Err(_) => -2,
    }
}

fn sample_capabilities() -> CapabilitiesResponse {
    CapabilitiesResponse {
        schema_ids: vec![1, 2, 3],
        paths: vec![CapabilityPath {
            path: "marketdata/AAPL/candles/5m".to_string(),
            pattern: CapabilityPathPattern::PubSub,
            auth_required: false,
        }],
        symbols: vec!["AAPL".to_string()],
        intervals: vec!["5m".to_string()],
    }
}

fn sample_open_orders() -> OpenOrdersSnapshot {
    OpenOrdersSnapshot {
        account: "DEMO".to_string(),
        orders: vec![],
        is_snapshot: Some(true),
    }
}

fn sample_market_data() -> MarketDataSnapshot {
    MarketDataSnapshot {
        symbol: "AAPL".to_string(),
        exchange: "SIM".to_string(),
        bids: vec![PriceLevel {
            price: Price(100.0),
            qty: Quantity(5.0),
            order_count: Some(1),
        }],
        asks: vec![],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(42),
        is_snapshot: Some(true),
    }
}

fn sample_order_history_request() -> OrderHistoryRequest {
    OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    }
}

fn sample_candle_bar() -> CandleBar {
    CandleBar {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        open: Price(150.0),
        high: Price(151.0),
        low: Price(149.5),
        close: Price(150.5),
        volume: Quantity(1000.0),
        bar_start: 1_700_000_000_000_000_000,
        bar_end: 1_700_000_300_000_000_000,
        is_final: true,
        is_snapshot: Some(false),
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_instrument_catalog_response(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(&sample_instrument_catalog(), out)
}

fn sample_instrument_catalog() -> InstrumentCatalogResponse {
    InstrumentCatalogResponse {
        instruments: vec![InstrumentMetadata {
            instrument_id: "BTC-PERP".to_string(),
            symbol: "BTC".to_string(),
            product_kind: "perp".to_string(),
            margin_asset: Some("USDC".to_string()),
            display_name: Some("Bitcoin Perpetual".to_string()),
            tick_size: Some(0.1),
            lot_size: Some(0.001),
        }],
    }
}

#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_open_orders_request(out: *mut FigBuffer) -> i32 {
    encode_cbor_out(
        &OpenOrdersRequest {
            account: "DEMO".to_string(),
            symbol: None,
        },
        out,
    )
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

    #[test]
    fn test_request_ex_and_subscribe() {
        let mut out = FigBuffer {
            data: ptr::null_mut(),
            len: 0,
        };
        unsafe {
            let path = std::ffi::CString::new(".well-known/capabilities").unwrap();
            let method = std::ffi::CString::new("GET").unwrap();
            assert_eq!(
                fig_frame_encode_request_ex(
                    3,
                    1,
                    1,
                    path.as_ptr(),
                    method.as_ptr(),
                    ptr::null(),
                    ptr::null(),
                    0,
                    &mut out
                ),
                0
            );
            fig_buffer_free(out);
            out = FigBuffer {
                data: ptr::null_mut(),
                len: 0,
            };
            let rk = std::ffi::CString::new("marketdata.AAPL.quotes").unwrap();
            let cp = std::ffi::CString::new("marketdata/AAPL/quotes").unwrap();
            assert_eq!(
                fig_frame_encode_subscribe(1, 1, rk.as_ptr(), cp.as_ptr(), &mut out),
                0
            );
            fig_buffer_free(out);
        }
    }
}
