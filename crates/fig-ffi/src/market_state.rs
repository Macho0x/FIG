//! CBOR and client helpers for market-state messages (session, security, mass cancel).

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::slice;
use std::sync::Arc;

use fig_client::FigSdkClient;
use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::messages::{
    MassCancelReport, MassCancelRequest, SecurityStatus, Side, TradingSessionStatus,
};
use fig_core::transport::FigConnection;

use crate::client::FigClientHandle;
use crate::{into_buffer, FigBuffer};

fn set_cstring(value: String, out: *mut *mut c_char) -> i32 {
    if out.is_null() {
        return -1;
    }
    match CString::new(value) {
        Ok(s) => {
            unsafe { *out = s.into_raw() };
            0
        }
        Err(_) => -3,
    }
}

fn set_optional_cstring(value: Option<String>, out: *mut *mut c_char) -> i32 {
    if out.is_null() {
        return -1;
    }
    match value {
        None => {
            unsafe { *out = std::ptr::null_mut() };
            0
        }
        Some(s) => set_cstring(s, out),
    }
}

fn optional_text(ptr: *const c_char) -> Result<Option<String>, i32> {
    if ptr.is_null() {
        return Ok(None);
    }
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|_| -2)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.to_string()))
    }
}

fn required_text(ptr: *const c_char) -> Result<String, i32> {
    if ptr.is_null() {
        return Err(-1);
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| -2)
}

fn side_from_wire(side: u8) -> Result<Option<Side>, i32> {
    if side == 0 {
        Ok(None)
    } else {
        Side::from_value(side).map(Some).ok_or(-3)
    }
}

fn bytes<'a>(data: *const u8, data_len: usize) -> Result<&'a [u8], i32> {
    if data.is_null() {
        return Err(-1);
    }
    Ok(unsafe { slice::from_raw_parts(data, data_len) })
}

/// Encode `MassCancelRequest` as CBOR.
///
/// `symbol` NULL (or empty) omits the field. `side` 0 omits the field; otherwise it is the
/// 1-based `Side` wire value (1 buy, 2 sell, 3 sell short, 4 sell short exempt).
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_encode_mass_cancel_request(
    cl_ord_id: *const c_char,
    account: *const c_char,
    symbol: *const c_char,
    side: u8,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let cl_ord_id = match required_text(cl_ord_id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let account = match required_text(account) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let symbol = match optional_text(symbol) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let side = match side_from_wire(side) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let req = MassCancelRequest {
        cl_ord_id,
        account,
        symbol,
        side,
    };
    match encode_cbor(&req) {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -2,
    }
}

/// Decode `MassCancelRequest` CBOR. `out_symbol` is NULL when absent. `out_side` is 0 when absent.
/// Non-NULL strings must be released with `fig_string_free`.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_mass_cancel_request(
    data: *const u8,
    data_len: usize,
    out_cl_ord_id: *mut *mut c_char,
    out_account: *mut *mut c_char,
    out_symbol: *mut *mut c_char,
    out_side: *mut u8,
) -> i32 {
    if out_cl_ord_id.is_null()
        || out_account.is_null()
        || out_symbol.is_null()
        || out_side.is_null()
    {
        return -1;
    }
    let bytes = match bytes(data, data_len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    let req: MassCancelRequest = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    *out_side = req.side.map(Side::to_value).unwrap_or(0);
    if set_cstring(req.cl_ord_id, out_cl_ord_id) != 0 {
        return -3;
    }
    if set_cstring(req.account, out_account) != 0 {
        return -3;
    }
    set_optional_cstring(req.symbol, out_symbol)
}

/// Decode `TradingSessionStatus` CBOR. `out_status` is the 1-based session status wire value.
/// `out_session_id` must be released with `fig_string_free`.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_trading_session_status(
    data: *const u8,
    data_len: usize,
    out_status: *mut u8,
    out_session_id: *mut *mut c_char,
) -> i32 {
    if out_status.is_null() || out_session_id.is_null() {
        return -1;
    }
    let bytes = match bytes(data, data_len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    let msg: TradingSessionStatus = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    *out_status = msg.status.to_value();
    set_cstring(msg.trading_session_id, out_session_id)
}

/// Decode `SecurityStatus` CBOR. `out_status` is the 1-based security status wire value.
/// `out_symbol` must be released with `fig_string_free`.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_security_status(
    data: *const u8,
    data_len: usize,
    out_symbol: *mut *mut c_char,
    out_status: *mut u8,
) -> i32 {
    if out_symbol.is_null() || out_status.is_null() {
        return -1;
    }
    let bytes = match bytes(data, data_len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    let msg: SecurityStatus = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    *out_status = msg.status.to_value();
    set_cstring(msg.symbol, out_symbol)
}

/// Decode `MassCancelReport` CBOR. `out_status` is the 1-based report status wire value.
/// `out_cl_ord_id` must be released with `fig_string_free`.
#[no_mangle]
pub unsafe extern "C" fn fig_cbor_decode_mass_cancel_report(
    data: *const u8,
    data_len: usize,
    out_cl_ord_id: *mut *mut c_char,
    out_total_affected: *mut u32,
    out_status: *mut u8,
) -> i32 {
    if out_cl_ord_id.is_null() || out_total_affected.is_null() || out_status.is_null() {
        return -1;
    }
    let bytes = match bytes(data, data_len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    let msg: MassCancelReport = match decode_cbor(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    *out_total_affected = msg.total_affected;
    *out_status = msg.status.to_value();
    set_cstring(msg.cl_ord_id, out_cl_ord_id)
}

fn connection(handle: &FigClientHandle) -> Result<Arc<FigConnection>, i32> {
    let guard = handle.conn.lock().map_err(|_| -5)?;
    guard.as_ref().cloned().ok_or(-5)
}

/// GET `marketdata/session`. `out_status` is the 1-based session status. Free `out_session_id`.
#[no_mangle]
pub unsafe extern "C" fn fig_client_request_session_status(
    handle: *mut FigClientHandle,
    channel_id: u16,
    out_status: *mut u8,
    out_session_id: *mut *mut c_char,
) -> i32 {
    if handle.is_null() || out_status.is_null() || out_session_id.is_null() {
        return -1;
    }
    let handle = &*handle;
    let conn = match connection(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let status = match handle.rt.block_on(async move {
        FigSdkClient::new(conn.inner())
            .request_session_status(channel_id)
            .await
    }) {
        Ok(s) => s,
        Err(_) => return -6,
    };
    *out_status = status.status.to_value();
    set_cstring(status.trading_session_id, out_session_id)
}

/// GET `marketdata/{symbol}/status`. `out_status` is the 1-based security status. Free `out_symbol`.
#[no_mangle]
pub unsafe extern "C" fn fig_client_request_security_status(
    handle: *mut FigClientHandle,
    symbol: *const c_char,
    channel_id: u16,
    out_symbol: *mut *mut c_char,
    out_status: *mut u8,
) -> i32 {
    if handle.is_null() || out_symbol.is_null() || out_status.is_null() {
        return -1;
    }
    let symbol = match required_text(symbol) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let handle = &*handle;
    let conn = match connection(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let status = match handle.rt.block_on(async move {
        FigSdkClient::new(conn.inner())
            .request_security_status(&symbol, channel_id)
            .await
    }) {
        Ok(s) => s,
        Err(_) => return -6,
    };
    *out_status = status.status.to_value();
    set_cstring(status.symbol, out_symbol)
}

/// POST `trading/accounts/{account}/orders/masscancel` with the dev auth token.
///
/// `symbol` NULL omits the filter. `side` 0 omits the filter. `out_status` is the 1-based
/// `MassCancelReport` status. Free `out_cl_ord_id`.
#[no_mangle]
pub unsafe extern "C" fn fig_client_mass_cancel(
    handle: *mut FigClientHandle,
    cl_ord_id: *const c_char,
    account: *const c_char,
    symbol: *const c_char,
    side: u8,
    channel_id: u16,
    out_cl_ord_id: *mut *mut c_char,
    out_total_affected: *mut u32,
    out_status: *mut u8,
) -> i32 {
    if handle.is_null()
        || out_cl_ord_id.is_null()
        || out_total_affected.is_null()
        || out_status.is_null()
    {
        return -1;
    }
    let cl_ord_id = match required_text(cl_ord_id) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let account = match required_text(account) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let symbol = match optional_text(symbol) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let side = match side_from_wire(side) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let req = MassCancelRequest {
        cl_ord_id,
        account,
        symbol,
        side,
    };
    let handle = &*handle;
    let conn = match connection(handle) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let report = match handle.rt.block_on(async move {
        FigSdkClient::new(conn.inner())
            .mass_cancel(&req, channel_id)
            .await
    }) {
        Ok(r) => r,
        Err(_) => return -6,
    };
    *out_total_affected = report.total_affected;
    *out_status = report.status.to_value();
    set_cstring(report.cl_ord_id, out_cl_ord_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    use fig_core::messages::{
        MassCancelReportStatus, SecurityStatusStatus, TradingSessionStatusStatus,
    };

    fn round_trip_request() {
        let cl = CString::new("MC-001").unwrap();
        let account = CString::new("DEMO").unwrap();
        let symbol = CString::new("AAPL").unwrap();
        let mut out = FigBuffer {
            data: ptr::null_mut(),
            len: 0,
        };
        unsafe {
            assert_eq!(
                fig_cbor_encode_mass_cancel_request(
                    cl.as_ptr(),
                    account.as_ptr(),
                    symbol.as_ptr(),
                    1,
                    &mut out
                ),
                0
            );
            let mut got_cl = ptr::null_mut();
            let mut got_account = ptr::null_mut();
            let mut got_symbol = ptr::null_mut();
            let mut got_side = 0u8;
            assert_eq!(
                fig_cbor_decode_mass_cancel_request(
                    out.data,
                    out.len,
                    &mut got_cl,
                    &mut got_account,
                    &mut got_symbol,
                    &mut got_side
                ),
                0
            );
            assert_eq!(CStr::from_ptr(got_cl).to_str().unwrap(), "MC-001");
            assert_eq!(CStr::from_ptr(got_account).to_str().unwrap(), "DEMO");
            assert_eq!(CStr::from_ptr(got_symbol).to_str().unwrap(), "AAPL");
            assert_eq!(got_side, 1);
            crate::fig_string_free(got_cl);
            crate::fig_string_free(got_account);
            crate::fig_string_free(got_symbol);
            crate::fig_buffer_free(out);
        }
    }

    #[test]
    fn mass_cancel_request_cbor_round_trip() {
        round_trip_request();
    }

    #[test]
    fn response_decoders_match_wire_values() {
        let session = TradingSessionStatus {
            symbol: None,
            trading_session_id: "REG".into(),
            status: TradingSessionStatusStatus::Open,
            halt_reason: None,
            timestamp: 1,
            is_snapshot: Some(true),
        };
        let security = SecurityStatus {
            symbol: "AAPL".into(),
            status: SecurityStatusStatus::Trading,
            halt_reason: None,
            timestamp: 1,
            is_snapshot: Some(true),
        };
        let report = MassCancelReport {
            cl_ord_id: "MC-001".into(),
            account: "DEMO".into(),
            symbol: Some("AAPL".into()),
            total_affected: 2,
            status: MassCancelReportStatus::Completed,
            timestamp: 1,
        };
        unsafe {
            let session_bytes = encode_cbor(&session).unwrap();
            let mut status = 0u8;
            let mut session_id = ptr::null_mut();
            assert_eq!(
                fig_cbor_decode_trading_session_status(
                    session_bytes.as_ptr(),
                    session_bytes.len(),
                    &mut status,
                    &mut session_id
                ),
                0
            );
            assert_eq!(status, TradingSessionStatusStatus::Open.to_value());
            assert_eq!(CStr::from_ptr(session_id).to_str().unwrap(), "REG");
            crate::fig_string_free(session_id);

            let sec_bytes = encode_cbor(&security).unwrap();
            let mut symbol = ptr::null_mut();
            assert_eq!(
                fig_cbor_decode_security_status(
                    sec_bytes.as_ptr(),
                    sec_bytes.len(),
                    &mut symbol,
                    &mut status
                ),
                0
            );
            assert_eq!(status, SecurityStatusStatus::Trading.to_value());
            assert_eq!(CStr::from_ptr(symbol).to_str().unwrap(), "AAPL");
            crate::fig_string_free(symbol);

            let report_bytes = encode_cbor(&report).unwrap();
            let mut cl = ptr::null_mut();
            let mut total = 0u32;
            assert_eq!(
                fig_cbor_decode_mass_cancel_report(
                    report_bytes.as_ptr(),
                    report_bytes.len(),
                    &mut cl,
                    &mut total,
                    &mut status
                ),
                0
            );
            assert_eq!(CStr::from_ptr(cl).to_str().unwrap(), "MC-001");
            assert_eq!(total, 2);
            assert_eq!(status, MassCancelReportStatus::Completed.to_value());
            crate::fig_string_free(cl);
        }
    }

    #[test]
    fn client_helpers_against_sim() {
        std::env::set_var("FIG_TLS_INSECURE", "1");
        std::env::set_var("FIG_DEV_OPEN", "1");
        let server_rt = tokio::runtime::Runtime::new().unwrap();
        let endpoint = server_rt
            .block_on(fig_exchange_sim::server::run_server("127.0.0.1:0"))
            .unwrap();
        let addr = CString::new(endpoint.local_addr().unwrap().to_string()).unwrap();
        let name = CString::new("localhost").unwrap();
        unsafe {
            let mut handle = ptr::null_mut();
            assert_eq!(
                crate::fig_client_connect(addr.as_ptr(), name.as_ptr(), &mut handle),
                0
            );
            let mut status = 0u8;
            let mut session_id = ptr::null_mut();
            assert_eq!(
                fig_client_request_session_status(handle, 10, &mut status, &mut session_id),
                0
            );
            assert_eq!(status, TradingSessionStatusStatus::Open.to_value());
            assert_eq!(CStr::from_ptr(session_id).to_str().unwrap(), "REG");
            crate::fig_string_free(session_id);

            let symbol_in = CString::new("AAPL").unwrap();
            let mut symbol = ptr::null_mut();
            assert_eq!(
                fig_client_request_security_status(
                    handle,
                    symbol_in.as_ptr(),
                    11,
                    &mut symbol,
                    &mut status
                ),
                0
            );
            assert_eq!(status, SecurityStatusStatus::Trading.to_value());
            assert_eq!(CStr::from_ptr(symbol).to_str().unwrap(), "AAPL");
            crate::fig_string_free(symbol);

            let cl = CString::new("MC-FFI").unwrap();
            let account = CString::new("TEST").unwrap();
            let mut out_cl = ptr::null_mut();
            let mut total = 0u32;
            assert_eq!(
                fig_client_mass_cancel(
                    handle,
                    cl.as_ptr(),
                    account.as_ptr(),
                    symbol_in.as_ptr(),
                    0,
                    12,
                    &mut out_cl,
                    &mut total,
                    &mut status
                ),
                0
            );
            assert_eq!(CStr::from_ptr(out_cl).to_str().unwrap(), "MC-FFI");
            assert_eq!(status, MassCancelReportStatus::Completed.to_value());
            crate::fig_string_free(out_cl);
            crate::fig_client_close(handle);
        }
    }
}
