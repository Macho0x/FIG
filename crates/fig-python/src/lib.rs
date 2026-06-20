//! Python bindings for FIG — Tier 1–2 reference SDK (CBOR + frames + client).

mod client;
mod codec;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use fig_conformance::{load_suite, run_vector};

pub use client::FigPyClient;

#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pyfunction]
#[pyo3(signature = (cl_ord_id, symbol, side, order_qty, price=None))]
fn encode_cbor_order(
    cl_ord_id: String,
    symbol: String,
    side: String,
    order_qty: f64,
    price: Option<f64>,
) -> PyResult<Vec<u8>> {
    let side = codec::parse_side(&side).map_err(PyValueError::new_err)?;
    codec::encode_new_order_single(&cl_ord_id, &symbol, side, order_qty, price)
        .map_err(PyValueError::new_err)
}

#[pyfunction]
fn decode_cbor_order_cl_ord_id(data: &[u8]) -> PyResult<String> {
    codec::decode_new_order_cl_ord_id(data).map_err(PyValueError::new_err)
}

#[pyfunction]
#[pyo3(signature = (channel_id, stream_seq, schema_id, payload=None, channel_path=None, method=None))]
fn encode_request_frame(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    payload: Option<&[u8]>,
    channel_path: Option<&str>,
    method: Option<&str>,
) -> PyResult<Vec<u8>> {
    codec::encode_request_frame(
        channel_id,
        stream_seq,
        schema_id,
        channel_path,
        method,
        payload
            .filter(|p| !p.is_empty())
            .map(|_| "application/cbor"),
        payload,
    )
    .map_err(PyValueError::new_err)
}

#[pyfunction]
#[pyo3(signature = (channel_id, stream_seq, routing_key, channel_path))]
fn encode_subscribe_frame(
    channel_id: u16,
    stream_seq: u32,
    routing_key: &str,
    channel_path: &str,
) -> PyResult<Vec<u8>> {
    codec::encode_subscribe_frame(channel_id, stream_seq, routing_key, channel_path)
        .map_err(PyValueError::new_err)
}

#[pyfunction]
fn encode_ping_frame() -> PyResult<Vec<u8>> {
    codec::encode_ping_frame().map_err(PyValueError::new_err)
}

#[pyfunction]
fn channel_stream_id(channel_id: u16, is_server: bool) -> u64 {
    codec::channel_stream_id(channel_id, is_server)
}

#[pyfunction]
fn frame_decode_header(data: &[u8]) -> PyResult<(u16, u32, u8)> {
    codec::frame_decode_header(data).map_err(PyValueError::new_err)
}

#[pyfunction]
fn encode_cbor_capabilities() -> PyResult<Vec<u8>> {
    fig_core::codec::encode_cbor(&codec::sample_capabilities_response())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn encode_cbor_open_orders_snapshot() -> PyResult<Vec<u8>> {
    fig_core::codec::encode_cbor(&codec::sample_open_orders_snapshot())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn encode_cbor_market_data_snapshot() -> PyResult<Vec<u8>> {
    fig_core::codec::encode_cbor(&codec::sample_market_data_snapshot())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn encode_cbor_order_history_request() -> PyResult<Vec<u8>> {
    fig_core::codec::encode_cbor(&codec::sample_order_history_request())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Run binding-specific conformance checks (Rust reference + Python encode paths).
pub fn run_binding_conformance(vectors_path: &std::path::Path) -> Result<(), String> {
    let suite = load_suite(vectors_path).map_err(|e| e.to_string())?;
    for vector in &suite.vectors {
        run_vector(vector).map_err(|e| e.to_string())?;
        binding_check_vector(vector)?;
    }
    Ok(())
}

/// Run FIG conformance vectors (Rust reference + binding encode paths).
#[pyfunction]
fn run_conformance(vectors_path: &str) -> PyResult<()> {
    run_binding_conformance(std::path::Path::new(vectors_path)).map_err(PyValueError::new_err)
}

fn binding_check_vector(vector: &fig_conformance::ConformanceVector) -> Result<(), String> {
    use fig_core::codec::encode_cbor;
    match (vector.category.as_str(), vector.message_type.as_str()) {
        ("cbor", "NewOrderSingle") => {
            let bytes = codec::encode_new_order_single(
                "CONF-001",
                "AAPL",
                fig_core::messages::Side::Buy,
                100.0,
                Some(50.25),
            )
            .map_err(|e| e.to_string())?;
            check_hex(&vector.expected_hex, &bytes)?;
        }
        ("cbor", "CapabilitiesResponse") => {
            check_hex(
                &vector.expected_hex,
                &encode_cbor(&codec::sample_capabilities_response()).map_err(|e| e.to_string())?,
            )?;
        }
        ("cbor", "OpenOrdersSnapshot") => {
            check_hex(
                &vector.expected_hex,
                &encode_cbor(&codec::sample_open_orders_snapshot()).map_err(|e| e.to_string())?,
            )?;
        }
        ("cbor", "MarketDataSnapshot") => {
            check_hex(
                &vector.expected_hex,
                &encode_cbor(&codec::sample_market_data_snapshot()).map_err(|e| e.to_string())?,
            )?;
        }
        ("cbor", "OrderHistoryRequest") => {
            check_hex(
                &vector.expected_hex,
                &encode_cbor(&codec::sample_order_history_request()).map_err(|e| e.to_string())?,
            )?;
        }
        ("frame", _) if vector.id.contains("capabilities") => {
            let bytes = codec::encode_request_frame(
                3,
                1,
                1,
                Some(".well-known/capabilities"),
                Some("GET"),
                None,
                None,
            )
            .map_err(|e| e.to_string())?;
            check_hex(&vector.expected_hex, &bytes)?;
        }
        ("frame", _) if vector.id.contains("open_orders") => {
            let payload =
                encode_cbor(&codec::sample_open_orders_request()).map_err(|e| e.to_string())?;
            let bytes = codec::encode_request_frame(
                4,
                1,
                1,
                Some("trading/accounts/DEMO/orders/open"),
                Some("GET"),
                Some("application/cbor"),
                Some(&payload),
            )
            .map_err(|e| e.to_string())?;
            check_hex(&vector.expected_hex, &bytes)?;
        }
        ("frame", _) if vector.id.contains("subscribe") => {
            let bytes = codec::encode_subscribe_frame(
                1,
                1,
                "marketdata.AAPL.quotes",
                "marketdata/AAPL/quotes",
            )
            .map_err(|e| e.to_string())?;
            check_hex(&vector.expected_hex, &bytes)?;
        }
        _ => {}
    }
    Ok(())
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

#[pymodule]
fn fig(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_order, m)?)?;
    m.add_function(wrap_pyfunction!(decode_cbor_order_cl_ord_id, m)?)?;
    m.add_function(wrap_pyfunction!(encode_request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(encode_subscribe_frame, m)?)?;
    m.add_function(wrap_pyfunction!(encode_ping_frame, m)?)?;
    m.add_function(wrap_pyfunction!(frame_decode_header, m)?)?;
    m.add_function(wrap_pyfunction!(channel_stream_id, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_capabilities, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_open_orders_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_market_data_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_order_history_request, m)?)?;
    m.add_function(wrap_pyfunction!(run_conformance, m)?)?;
    m.add_class::<FigPyClient>()?;
    Ok(())
}
