//! Proxy FIG requests/subscribes to a remote backend over TREE.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use fig_core::frame::Frame;
use fig_core::transport::{FigClient, FigConnection};

/// Connect to a FIG backend (exchange-sim or production broker).
pub async fn connect_backend(addr: SocketAddr) -> anyhow::Result<FigConnection> {
    let config =
        fig_core::transport::client_config().map_err(|e| anyhow::anyhow!("client config: {e}"))?;
    let client = FigClient::new(config)?;
    client.connect(addr, "localhost").await.map_err(Into::into)
}

/// Send a FIG frame and collect all response frames from the backend.
///
/// Half-closes send and waits for EOF. Use for REQUEST/control only — SUBSCRIBE
/// streams stay open, so use [`proxy_subscribe_snapshot`] or [`BackendSession`].
pub async fn proxy_frame(addr: SocketAddr, frame: Frame) -> anyhow::Result<Vec<Frame>> {
    let conn = connect_backend(addr).await?;
    conn.request_and_recv_all(frame)
        .await
        .map_err(|e| anyhow::anyhow!("backend proxy: {e}"))
}

/// SUBSCRIBE without finishing the send stream; read until the first idle gap.
pub async fn proxy_subscribe_snapshot(
    addr: SocketAddr,
    frame: Frame,
) -> anyhow::Result<Vec<Frame>> {
    let conn = connect_backend(addr).await?;
    recv_until_idle(&conn, frame.channel_id, frame).await
}

async fn recv_until_idle(
    conn: &FigConnection,
    channel_id: u16,
    frame: Frame,
) -> anyhow::Result<Vec<Frame>> {
    conn.send_frame(channel_id, &frame)
        .await
        .map_err(|e| anyhow::anyhow!("backend subscribe send: {e}"))?;
    let mut frames = Vec::new();
    loop {
        let idle = if frames.is_empty() {
            Duration::from_secs(5)
        } else {
            Duration::from_millis(150)
        };
        match tokio::time::timeout(idle, conn.recv_frame(channel_id)).await {
            Ok(Ok(f)) => frames.push(f),
            Ok(Err(_)) => break,
            Err(_) if !frames.is_empty() => break,
            Err(_) => anyhow::bail!("backend subscribe timed out"),
        }
    }
    Ok(frames)
}

/// Persistent TREE connection for a legacy WebSocket client.
pub struct BackendSession {
    conn: Arc<FigConnection>,
}

impl BackendSession {
    pub async fn connect(addr: SocketAddr) -> anyhow::Result<Self> {
        Ok(Self {
            conn: Arc::new(connect_backend(addr).await?),
        })
    }

    pub fn connection(&self) -> Arc<FigConnection> {
        self.conn.clone()
    }

    pub async fn send_frame(&self, frame: &Frame) -> anyhow::Result<()> {
        self.conn
            .send_frame(frame.channel_id, frame)
            .await
            .map_err(|e| anyhow::anyhow!("backend send: {e}"))
    }

    pub async fn recv_frame(&self, channel_id: u16) -> anyhow::Result<Frame> {
        self.conn
            .recv_frame(channel_id)
            .await
            .map_err(|e| anyhow::anyhow!("backend recv: {e}"))
    }

    /// SUBSCRIBE and collect snapshot/ack frames without requiring EOF.
    pub async fn subscribe_snapshot(&self, frame: Frame) -> anyhow::Result<Vec<Frame>> {
        recv_until_idle(self.conn.as_ref(), frame.channel_id, frame).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn proxy_requires_running_backend() {
        // Smoke: invalid address fails fast
        let frame = fig_core::frame::Frame::ping();
        let err = proxy_frame("127.0.0.1:1".parse().unwrap(), frame).await;
        assert!(err.is_err());
    }
}
