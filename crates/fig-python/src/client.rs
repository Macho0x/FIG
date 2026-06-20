//! Tier 1–2 Python client wrapping `fig_core::transport`.

use std::net::SocketAddr;
use std::sync::Arc;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use fig_core::frame::Frame;
use fig_core::transport::{client_config, FigClient, FigConnection};

use crate::codec;

#[pyclass]
pub struct FigPyClient {
    rt: Runtime,
    conn: Arc<Mutex<Option<FigConnection>>>,
}

#[pymethods]
impl FigPyClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            rt: Runtime::new().map_err(|e| PyRuntimeError::new_err(e.to_string()))?,
            conn: Arc::new(Mutex::new(None)),
        })
    }

    /// Connect to a FIG server (`host:port`, e.g. `127.0.0.1:4433`).
    #[pyo3(signature = (addr, server_name=None))]
    fn connect(&self, addr: &str, server_name: Option<&str>) -> PyResult<()> {
        let server_name = server_name.unwrap_or("localhost").to_string();
        let addr: SocketAddr = addr
            .parse()
            .map_err(|e| PyRuntimeError::new_err(format!("invalid address: {e}")))?;
        self.rt
            .block_on(async {
                let client = FigClient::new(client_config().map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
                let conn = client
                    .connect(addr, &server_name)
                    .await
                    .map_err(|e| e.to_string())?;
                *self.conn.lock().await = Some(conn);
                Ok::<(), String>(())
            })
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
        )
        .map_err(PyRuntimeError::new_err)?;
        let frame = Frame::decode(&encoded)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            .0;
        let responses = self
            .rt
            .block_on(async {
                let guard = self.conn.lock().await;
                let conn = guard.as_ref().ok_or_else(|| "not connected".to_string())?;
                conn.request_and_recv_all(frame)
                    .await
                    .map_err(|e| e.to_string())
            })
            .map_err(PyRuntimeError::new_err)?;
        Ok(responses
            .into_iter()
            .map(|f| PyBytes::new(py, &f.encode().unwrap_or_default()))
            .collect())
    }

    /// SUBSCRIBE to a native channel path and return initial stream frames.
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
        let routing_key = routing_key.unwrap_or(channel_path);
        let encoded = codec::encode_subscribe_frame(
            channel_id,
            stream_seq,
            routing_key,
            channel_path,
            auth_token,
        )
        .map_err(PyRuntimeError::new_err)?;
        let frame = Frame::decode(&encoded)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
            .0;
        let responses = self
            .rt
            .block_on(async {
                let guard = self.conn.lock().await;
                let conn = guard.as_ref().ok_or_else(|| "not connected".to_string())?;
                conn.request_and_recv_all(frame)
                    .await
                    .map_err(|e| e.to_string())
            })
            .map_err(PyRuntimeError::new_err)?;
        Ok(responses
            .into_iter()
            .map(|f| PyBytes::new(py, &f.encode().unwrap_or_default()))
            .collect())
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
