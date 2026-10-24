//! Python wrapper for `fig_client::CandleState`.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use fig_client::candles::CandleState;
use fig_core::codec::decode_cbor;
use fig_core::messages::CandleBarEvent;

#[pyclass]
pub struct PyCandleState {
    inner: CandleState,
}

#[pymethods]
impl PyCandleState {
    #[new]
    fn new() -> Self {
        Self {
            inner: CandleState::default(),
        }
    }

    fn apply_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let event: CandleBarEvent =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply_event(&event);
        Ok(())
    }

    fn close_price(&self) -> Option<f64> {
        self.inner.current().map(|bar| bar.close.0)
    }

    fn is_final(&self) -> Option<bool> {
        self.inner.current().map(|bar| bar.is_final)
    }

    fn symbol(&self) -> String {
        self.inner.symbol.clone()
    }

    fn interval(&self) -> String {
        self.inner.interval.clone()
    }
}
