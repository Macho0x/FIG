//! Python wrappers for `fig-client` stream merge states.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use fig_client::{BboState, MarkPriceState, MidsState, OrdersState, TradeTape};
use fig_core::codec::decode_cbor;
use fig_core::messages::{
    AllMidsBatch, BestBidOffer, ExecutionReport, MarkPriceUpdate, MiniTicker, OpenOrdersSnapshot,
    PublicTradeEvent,
};

#[pyclass]
pub struct PyMidsState {
    inner: MidsState,
}

#[pymethods]
impl PyMidsState {
    #[new]
    fn new() -> Self {
        Self {
            inner: MidsState::default(),
        }
    }

    fn apply_ticker_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let t: MiniTicker =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply_ticker(&t);
        Ok(())
    }

    fn apply_batch_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let b: AllMidsBatch =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply_batch(&b);
        Ok(())
    }

    fn mid(&self, symbol: &str) -> Option<f64> {
        self.inner.mid(symbol)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[pyclass]
pub struct PyBboState {
    inner: BboState,
}

#[pymethods]
impl PyBboState {
    #[new]
    fn new() -> Self {
        Self {
            inner: BboState::default(),
        }
    }

    fn apply_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let bbo: BestBidOffer =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply(&bbo);
        Ok(())
    }

    fn implied_mid(&self) -> Option<f64> {
        self.inner.implied_mid()
    }

    fn best_bid(&self) -> Option<f64> {
        self.inner.best_bid()
    }

    fn best_ask(&self) -> Option<f64> {
        self.inner.best_ask()
    }
}

#[pyclass]
pub struct PyTradeTape {
    inner: TradeTape,
}

#[pymethods]
impl PyTradeTape {
    #[new]
    #[pyo3(signature = (capacity=256))]
    fn new(capacity: usize) -> Self {
        Self {
            inner: TradeTape::with_capacity(capacity),
        }
    }

    fn push_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let ev: PublicTradeEvent =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.push_event(&ev);
        Ok(())
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn latest_price(&self) -> Option<f64> {
        self.inner.latest().map(|t| t.price.0)
    }
}

#[pyclass]
pub struct PyMarkPriceState {
    inner: MarkPriceState,
}

#[pymethods]
impl PyMarkPriceState {
    #[new]
    fn new() -> Self {
        Self {
            inner: MarkPriceState::default(),
        }
    }

    fn apply_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let upd: MarkPriceUpdate =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply(&upd);
        Ok(())
    }

    fn mark(&self, symbol: &str) -> Option<f64> {
        self.inner.mark(symbol)
    }

    fn funding_rate(&self, symbol: &str) -> Option<f64> {
        self.inner.funding_rate(symbol)
    }
}

#[pyclass]
pub struct PyOrdersState {
    inner: OrdersState,
}

#[pymethods]
impl PyOrdersState {
    #[new]
    #[pyo3(signature = (exec_capacity=256))]
    fn new(exec_capacity: usize) -> Self {
        Self {
            inner: OrdersState::with_exec_capacity(exec_capacity),
        }
    }

    fn apply_snapshot_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let snap: OpenOrdersSnapshot =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply_open_orders_snapshot(&snap);
        Ok(())
    }

    fn apply_execution_bytes(&mut self, payload: &[u8]) -> PyResult<()> {
        let report: ExecutionReport =
            decode_cbor(payload).map_err(|e| PyValueError::new_err(e.to_string()))?;
        self.inner.apply_execution_report(&report);
        Ok(())
    }

    fn open_count(&self) -> usize {
        self.inner.open_count()
    }

    fn execution_count(&self) -> usize {
        self.inner.executions().len()
    }
}
