//! Shared §16.1 binding conformance checks for FFI encoders.

use fig_conformance::{load_suite, run_vector, ConformanceVector};

use crate::{
    fig_cbor_encode_candle_bar, fig_cbor_encode_capabilities,
    fig_cbor_encode_instrument_catalog_response, fig_cbor_encode_market_data_snapshot,
    fig_cbor_encode_new_order_single, fig_cbor_encode_open_orders_request,
    fig_cbor_encode_open_orders_snapshot, fig_cbor_encode_order_history_request,
    fig_frame_encode_request_ex, fig_frame_encode_subscribe, fig_sbe_encode_new_order_single,
    FigBuffer,
};

/// Run conformance vectors against `fig-ffi` encode paths.
pub fn run_binding_conformance(vectors_path: &std::path::Path) -> Result<(), String> {
    let suite = load_suite(vectors_path).map_err(|e| e.to_string())?;
    for vector in &suite.vectors {
        run_vector(vector).map_err(|e| e.to_string())?;
        binding_check_vector(vector)?;
    }
    Ok(())
}

fn binding_check_vector(vector: &ConformanceVector) -> Result<(), String> {
    match (vector.category.as_str(), vector.message_type.as_str()) {
        ("cbor", "NewOrderSingle") => {
            let (post_only, reduce_only) = if vector.id == "cbor.new_order_single.post_only" {
                (1_i8, -1_i8)
            } else {
                (-1_i8, -1_i8)
            };
            let mut out = empty_buf();
            let rc = unsafe {
                fig_cbor_encode_new_order_single(
                    c"CONF-001".as_ptr(),
                    c"AAPL".as_ptr(),
                    1,
                    100.0,
                    50.25,
                    post_only,
                    reduce_only,
                    &mut out,
                )
            };
            if rc != 0 {
                return Err(format!("fig_cbor_encode_new_order_single rc={rc}"));
            }
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "InstrumentCatalogResponse") => {
            let mut out = empty_buf();
            assert_eq!(unsafe { fig_cbor_encode_instrument_catalog_response(&mut out) }, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "CandleBar") => {
            let mut out = empty_buf();
            assert_eq!(unsafe { fig_cbor_encode_candle_bar(&mut out) }, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("sbe", "NewOrderSingle") => {
            let mut out = empty_buf();
            let rc = unsafe {
                fig_sbe_encode_new_order_single(
                    c"CONF-001".as_ptr(),
                    c"AAPL".as_ptr(),
                    1,
                    100.0,
                    50.25,
                    -1,
                    -1,
                    &mut out,
                )
            };
            if rc != 0 {
                return Err(format!("fig_sbe_encode_new_order_single rc={rc}"));
            }
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "CapabilitiesResponse") => {
            let mut out = empty_buf();
            assert_eq!(unsafe { fig_cbor_encode_capabilities(&mut out) }, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "OpenOrdersSnapshot") => {
            let mut out = empty_buf();
            assert_eq!(unsafe { fig_cbor_encode_open_orders_snapshot(&mut out) }, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "MarketDataSnapshot") => {
            let mut out = empty_buf();
            assert_eq!(unsafe { fig_cbor_encode_market_data_snapshot(&mut out) }, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("cbor", "OrderHistoryRequest") => {
            let mut out = empty_buf();
            assert_eq!(
                unsafe { fig_cbor_encode_order_history_request(&mut out) },
                0
            );
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("frame", _) if vector.id.contains("capabilities") => {
            let path = std::ffi::CString::new(".well-known/capabilities").unwrap();
            let method = std::ffi::CString::new("GET").unwrap();
            let mut out = empty_buf();
            let rc = unsafe {
                fig_frame_encode_request_ex(
                    3,
                    1,
                    1,
                    path.as_ptr(),
                    method.as_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    0,
                    &mut out,
                )
            };
            assert_eq!(rc, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        ("frame", _) if vector.id.contains("open_orders") => {
            let mut payload = empty_buf();
            assert_eq!(
                unsafe { fig_cbor_encode_open_orders_request(&mut payload) },
                0
            );
            let path = std::ffi::CString::new("trading/accounts/DEMO/orders/open").unwrap();
            let method = std::ffi::CString::new("GET").unwrap();
            let ct = std::ffi::CString::new("application/cbor").unwrap();
            let mut out = empty_buf();
            let rc = unsafe {
                fig_frame_encode_request_ex(
                    4,
                    1,
                    1,
                    path.as_ptr(),
                    method.as_ptr(),
                    ct.as_ptr(),
                    payload.data,
                    payload.len,
                    &mut out,
                )
            };
            assert_eq!(rc, 0);
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
                free_buf(payload);
            }
        }
        ("frame", _) if vector.id.contains("subscribe") => {
            let rk = std::ffi::CString::new("marketdata.AAPL.quotes").unwrap();
            let cp = std::ffi::CString::new("marketdata/AAPL/quotes").unwrap();
            let mut out = empty_buf();
            assert_eq!(
                unsafe { fig_frame_encode_subscribe(1, 1, rk.as_ptr(), cp.as_ptr(), &mut out) },
                0
            );
            unsafe {
                check_hex(&vector.expected_hex, &buf_slice(&out))?;
                free_buf(out);
            }
        }
        _ => {}
    }
    Ok(())
}

fn empty_buf() -> FigBuffer {
    FigBuffer {
        data: std::ptr::null_mut(),
        len: 0,
    }
}

unsafe fn buf_slice(buf: &FigBuffer) -> Vec<u8> {
    if buf.data.is_null() || buf.len == 0 {
        return Vec::new();
    }
    std::slice::from_raw_parts(buf.data, buf.len).to_vec()
}

unsafe fn free_buf(buf: FigBuffer) {
    crate::fig_buffer_free(buf);
}

fn check_hex(expected_hex: &str, actual: &[u8]) -> Result<(), String> {
    let expected = hex::decode(expected_hex.trim()).map_err(|e| e.to_string())?;
    if expected != actual {
        return Err(format!(
            "hex mismatch\nexpected: {expected_hex}\nactual:   {}",
            hex::encode(actual)
        ));
    }
    Ok(())
}
