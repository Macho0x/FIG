//! Python wrapper for `fig_client::OrderBookState`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use fig_client::order_book::OrderBookState;
use fig_core::codec::decode_cbor;
use fig_core::messages::{OrderBookDelta, OrderBookSnapshot};

#[pyclass]
pub struct PyOrderBookState {
    inner: OrderBookState,
}

#[pymethods]
impl PyOrderBookState {
    #[new]
    fn new() -> Self {
        Self {
            inner: OrderBookState::default(),
        }
    }

    fn apply_snapshot_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let snap: OrderBookSnapshot =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner
            .apply_snapshot(&snap)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn apply_delta_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let delta: OrderBookDelta =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner
            .apply_delta(&delta)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn best_bid(&self) -> Option<f64> {
        self.inner.bids.first().map(|l| l.price.0)
    }
}
