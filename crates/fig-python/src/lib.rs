//! Python bindings for FIG — Tier 1–2 reference SDK (CBOR + frames).

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce};

#[pyfunction]
fn version() -> &'static str {
    "0.1.0"
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
    let side = parse_side(&side)?;
    let order = NewOrderSingle {
        cl_ord_id,
        side,
        order_qty: Quantity(order_qty),
        price: price.map(Price),
        stop_price: None,
        symbol,
        order_type: if price.is_some() {
            OrderType::Limit
        } else {
            OrderType::Market
        },
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    };
    encode_cbor(&order).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn decode_cbor_order_cl_ord_id(data: &[u8]) -> PyResult<String> {
    let order: NewOrderSingle =
        decode_cbor(data).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(order.cl_ord_id)
}

#[pyfunction]
#[pyo3(signature = (channel_id, stream_seq, schema_id, payload=None))]
fn encode_request_frame(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    payload: Option<&[u8]>,
) -> PyResult<Vec<u8>> {
    let frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id)
        .with_payload(payload.unwrap_or_default().to_vec());
    frame
        .encode()
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
fn channel_stream_id(channel_id: u16, is_server: bool) -> u64 {
    if is_server {
        channel_id as u64 * 4 + 1
    } else {
        channel_id as u64 * 4
    }
}

#[pyfunction]
fn frame_decode_header(data: &[u8]) -> PyResult<(u16, u32, u8)> {
    let (frame, _) = Frame::decode(data).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok((frame.channel_id, frame.stream_seq, frame.schema_id))
}

#[pymodule]
fn fig(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(encode_cbor_order, m)?)?;
    m.add_function(wrap_pyfunction!(decode_cbor_order_cl_ord_id, m)?)?;
    m.add_function(wrap_pyfunction!(encode_request_frame, m)?)?;
    m.add_function(wrap_pyfunction!(frame_decode_header, m)?)?;
    m.add_function(wrap_pyfunction!(channel_stream_id, m)?)?;
    Ok(())
}

fn parse_side(s: &str) -> PyResult<Side> {
    match s {
        "Buy" | "buy" => Ok(Side::Buy),
        "Sell" | "sell" => Ok(Side::Sell),
        "SellShort" | "sell_short" => Ok(Side::SellShort),
        "SellShortExempt" | "sell_short_exempt" => Ok(Side::SellShortExempt),
        _ => Err(PyValueError::new_err(format!("unknown side: {s}"))),
    }
}

#[cfg(test)]
mod tests {
    use fig_core::codec::{decode_cbor, encode_cbor};
    use fig_core::messages::{NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce};

    #[test]
    fn test_encode_cbor_order_round_trip() {
        let order = NewOrderSingle {
            cl_ord_id: "PY-001".into(),
            side: Side::Buy,
            order_qty: Quantity(100.0),
            price: Some(Price(50.25)),
            stop_price: None,
            symbol: "AAPL".into(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
        };
        let bytes = encode_cbor(&order).unwrap();
        let decoded: NewOrderSingle = decode_cbor(&bytes).unwrap();
        assert_eq!(decoded.cl_ord_id, "PY-001");
    }
}
