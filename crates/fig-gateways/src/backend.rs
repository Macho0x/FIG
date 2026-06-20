//! Proxy FIG requests/subscribes to a remote backend over TREE.

use std::net::SocketAddr;

use fig_core::frame::Frame;
use fig_core::transport::{FigClient, FigConnection};

/// Connect to a FIG backend (exchange-sim or production broker).
pub async fn connect_backend(addr: SocketAddr) -> anyhow::Result<FigConnection> {
    let config = fig_core::transport::client_config()
        .map_err(|e| anyhow::anyhow!("client config: {e}"))?;
    let client = FigClient::new(config)?;
    client.connect(addr, "localhost").await.map_err(Into::into)
}

/// Send a FIG frame and collect all response frames from the backend.
pub async fn proxy_frame(
    addr: SocketAddr,
    frame: Frame,
) -> anyhow::Result<Vec<Frame>> {
    let conn = connect_backend(addr).await?;
    conn.request_and_recv_all(frame)
        .await
        .map_err(|e| anyhow::anyhow!("backend proxy: {e}"))
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
