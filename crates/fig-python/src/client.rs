//! Tier 1–2 Python client wrapping `fig_core::transport`.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use fig_client::{FigSdkClient, LiveSubscription};
use fig_core::frame::Frame;
use fig_core::transport::{client_config, FigClient, FigConnection};

use crate::codec;

#[pyclass]
pub struct FigPyClient {
    rt: Arc<Runtime>,
    conn: Arc<Mutex<Option<FigConnection>>>,
}

impl FigPyClient {
    pub(crate) fn try_new() -> Result<Self, String> {
        Ok(Self {
            rt: Arc::new(Runtime::new().map_err(|e| e.to_string())?),
            conn: Arc::new(Mutex::new(None)),
        })
    }

    pub(crate) fn connect_inner(
        &self,
        addr: &str,
        server_name: Option<&str>,
    ) -> Result<(), String> {
        let server_name = server_name.unwrap_or("localhost").to_string();
        let addr: SocketAddr = addr.parse().map_err(|e| format!("invalid address: {e}"))?;
        self.rt.block_on(async {
            let client = FigClient::new(client_config().map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let conn = client
                .connect(addr, &server_name)
                .await
                .map_err(|e| e.to_string())?;
            *self.conn.lock().await = Some(conn);
            Ok(())
        })
    }

    pub(crate) fn subscribe_live_inner(
        &self,
        channel_path: &str,
        routing_key: Option<&str>,
        auth_token: Option<&str>,
        channel_id: u16,
        stream_seq: u32,
    ) -> Result<(Vec<Vec<u8>>, LiveSubscription), String> {
        let routing_key = routing_key.unwrap_or(channel_path);
        let encoded = codec::encode_subscribe_frame(
            channel_id,
            stream_seq,
            routing_key,
            channel_path,
            auth_token,
        )?;
        let frame = Frame::decode(&encoded).map_err(|e| e.to_string())?.0;
        self.rt.block_on(async {
            let guard = self.conn.lock().await;
            let conn = guard.as_ref().ok_or_else(|| "not connected".to_string())?;
            let sdk = FigSdkClient::new(conn.inner());
            let (snapshot, live) = sdk.subscribe_live(frame).await.map_err(|e| e.to_string())?;
            let bytes = snapshot
                .into_iter()
                .map(|f| f.encode().map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((bytes, live))
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_inner(
        &self,
        channel_path: &str,
        method: &str,
        payload: Option<&[u8]>,
        auth_token: Option<&str>,
        channel_id: u16,
        stream_seq: u32,
        schema_id: u8,
    ) -> Result<Vec<Vec<u8>>, String> {
        let encoded = codec::encode_request_frame(
            channel_id,
            stream_seq,
            schema_id,
            Some(channel_path),
            Some(method),
            payload
                .filter(|p| !p.is_empty())
                .map(|_| "application/cbor"),
            payload,
            auth_token,
        )?;
        let frame = Frame::decode(&encoded).map_err(|e| e.to_string())?.0;
        self.rt.block_on(async {
            let guard = self.conn.lock().await;
            let conn = guard.as_ref().ok_or_else(|| "not connected".to_string())?;
            let responses = conn
                .request_and_recv_all(frame)
                .await
                .map_err(|e| e.to_string())?;
            responses
                .into_iter()
                .map(|f| f.encode().map_err(|e| e.to_string()))
                .collect()
        })
    }
}

#[pyclass]
pub struct FigPySubscription {
    rt: Arc<Runtime>,
    live: std::sync::Mutex<Option<LiveSubscription>>,
    snapshot_bytes: Vec<Vec<u8>>,
}

#[pymethods]
impl FigPySubscription {
    #[getter]
    fn snapshot<'py>(&self, py: Python<'py>) -> Vec<Bound<'py, PyBytes>> {
        self.snapshot_bytes
            .iter()
            .map(|b| PyBytes::new(py, b))
            .collect()
    }

    /// Next live frame. `timeout_ms=None` or `0` waits forever. Returns `None` on EOF.
    #[pyo3(signature = (timeout_ms=None))]
    fn next<'py>(
        &self,
        py: Python<'py>,
        timeout_ms: Option<u32>,
    ) -> PyResult<Option<Bound<'py, PyBytes>>> {
        let encoded = py
            .allow_threads(|| -> Result<Option<Vec<u8>>, String> {
                let mut guard = self
                    .live
                    .lock()
                    .map_err(|_| "subscription poisoned".to_string())?;
                let live = guard
                    .as_mut()
                    .ok_or_else(|| "subscription closed".to_string())?;
                let frame = if timeout_ms.unwrap_or(0) == 0 {
                    self.rt
                        .block_on(live.next_frame())
                        .map_err(|e| e.to_string())?
                } else {
                    match self.rt.block_on(async {
                        tokio::time::timeout(
                            Duration::from_millis(u64::from(timeout_ms.unwrap_or(0))),
                            live.next_frame(),
                        )
                        .await
                    }) {
                        Ok(inner) => inner.map_err(|e| e.to_string())?,
                        Err(_) => return Err("timeout".to_string()),
                    }
                };
                match frame {
                    Some(f) => Ok(Some(f.encode().map_err(|e| e.to_string())?)),
                    None => Ok(None),
                }
            })
            .map_err(PyRuntimeError::new_err)?;
        Ok(encoded.map(|b| PyBytes::new(py, &b)))
    }

    fn close(&self) -> PyResult<()> {
        if let Ok(mut guard) = self.live.lock() {
            *guard = None;
        }
        Ok(())
    }
}

#[pymethods]
impl FigPyClient {
    #[new]
    fn new() -> PyResult<Self> {
        Self::try_new().map_err(PyRuntimeError::new_err)
    }

    /// Connect to a FIG server (`host:port`, e.g. `127.0.0.1:4433`).
    #[pyo3(signature = (addr, server_name=None))]
    fn connect(&self, addr: &str, server_name: Option<&str>) -> PyResult<()> {
        self.connect_inner(addr, server_name)
            .map_err(PyRuntimeError::new_err)
    }

    /// Send native FIG REQUEST and return raw response frame bytes.
    #[pyo3(signature = (channel_path, method, payload=None, auth_token=None, channel_id=1, stream_seq=1, schema_id=1))]
    #[allow(clippy::too_many_arguments)]
    fn request<'py>(
        &self,
        py: Python<'py>,
        channel_path: &str,
        method: &str,
        payload: Option<&[u8]>,
        auth_token: Option<&str>,
        channel_id: u16,
        stream_seq: u32,
        schema_id: u8,
    ) -> PyResult<Vec<Bound<'py, PyBytes>>> {
        let responses = self
            .request_inner(
                channel_path,
                method,
                payload,
                auth_token,
                channel_id,
                stream_seq,
                schema_id,
            )
            .map_err(PyRuntimeError::new_err)?;
        Ok(responses
            .into_iter()
            .map(|b| PyBytes::new(py, &b))
            .collect())
    }

    /// SUBSCRIBE snapshot (does not wait for stream EOF).
    #[pyo3(signature = (channel_path, routing_key=None, auth_token=None, channel_id=1, stream_seq=1))]
    fn subscribe<'py>(
        &self,
        py: Python<'py>,
        channel_path: &str,
        routing_key: Option<&str>,
        auth_token: Option<&str>,
        channel_id: u16,
        stream_seq: u32,
    ) -> PyResult<Vec<Bound<'py, PyBytes>>> {
        let (snapshot, _live) = self
            .subscribe_live_inner(
                channel_path,
                routing_key,
                auth_token,
                channel_id,
                stream_seq,
            )
            .map_err(PyRuntimeError::new_err)?;
        Ok(snapshot.into_iter().map(|b| PyBytes::new(py, &b)).collect())
    }

    /// SUBSCRIBE and keep the recv stream open for later STREAM_ITEMs.
    #[pyo3(signature = (channel_path, routing_key=None, auth_token=None, channel_id=1, stream_seq=1))]
    fn subscribe_live(
        &self,
        channel_path: &str,
        routing_key: Option<&str>,
        auth_token: Option<&str>,
        channel_id: u16,
        stream_seq: u32,
    ) -> PyResult<FigPySubscription> {
        let (snapshot_bytes, live) = self
            .subscribe_live_inner(
                channel_path,
                routing_key,
                auth_token,
                channel_id,
                stream_seq,
            )
            .map_err(PyRuntimeError::new_err)?;
        Ok(FigPySubscription {
            rt: self.rt.clone(),
            live: std::sync::Mutex::new(Some(live)),
            snapshot_bytes,
        })
    }

    /// Send a PING control frame on channel 0.
    fn ping(&self) -> PyResult<()> {
        let frame = Frame::ping();
        self.rt
            .block_on(async {
                let guard = self.conn.lock().await;
                let conn = guard.as_ref().ok_or_else(|| "not connected".to_string())?;
                conn.send_frame(0, &frame).await.map_err(|e| e.to_string())
            })
            .map_err(PyRuntimeError::new_err)
    }

    fn close(&self) -> PyResult<()> {
        self.rt
            .block_on(async {
                *self.conn.lock().await = None;
                Ok::<(), String>(())
            })
            .map_err(PyRuntimeError::new_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::codec;
    use fig_core::frame::FrameType;
    use fig_core::messages::{
        NewOrderSingle, OrderType, PositionUpdate, Price, Quantity, Side, TimeInForce,
    };
    use fig_exchange_sim::server::run_server;

    fn order_payload(
        cl_ord_id: &str,
        side: Side,
        order_type: OrderType,
        price: Option<f64>,
        qty: f64,
    ) -> Vec<u8> {
        let order = NewOrderSingle {
            cl_ord_id: cl_ord_id.into(),
            side,
            order_qty: Quantity(qty),
            price: price.map(Price),
            stop_price: None,
            symbol: "AAPL".into(),
            order_type,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: Some("TEST".into()),
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        };
        codec::encode_cbor(&order).expect("cbor")
    }

    #[test]
    fn python_subscribe_live_positions_on_fill() {
        let server_rt = Runtime::new().expect("server runtime");
        let endpoint = server_rt
            .block_on(run_server("127.0.0.1:0"))
            .expect("server");
        let addr = endpoint.local_addr().expect("local addr").to_string();

        let subscriber = FigPyClient::try_new().expect("sub client");
        subscriber.connect_inner(&addr, None).expect("sub connect");
        let (snapshot, mut live) = subscriber
            .subscribe_live_inner(
                "accounts/TEST/positions",
                Some("accounts/TEST/positions"),
                Some("fig-dev-TEST"),
                1,
                1,
            )
            .expect("subscribe_live");
        assert!(!snapshot.is_empty());

        let trader = FigPyClient::try_new().expect("trader");
        trader.connect_inner(&addr, None).expect("trader connect");
        let sell = order_payload("PY-PD-1", Side::Sell, OrderType::Limit, Some(100.0), 5.0);
        trader
            .request_inner(
                "trading/accounts/TEST/orders",
                "POST",
                Some(&sell),
                None,
                2,
                1,
                1,
            )
            .expect("sell");
        let buy = order_payload("PY-PD-2", Side::Buy, OrderType::Market, None, 5.0);
        trader
            .request_inner(
                "trading/accounts/TEST/orders",
                "POST",
                Some(&buy),
                None,
                2,
                1,
                1,
            )
            .expect("buy");

        let frame = subscriber
            .rt
            .block_on(async {
                tokio::time::timeout(Duration::from_secs(5), live.next_frame())
                    .await
                    .expect("timeout")
            })
            .expect("next")
            .expect("eof");
        assert_eq!(frame.frame_type, FrameType::StreamItem);
        let update: PositionUpdate = codec::decode_cbor(&frame.payload).expect("position");
        assert_eq!(update.account, "TEST");
    }
}
