//! Prometheus metrics HTTP exporter for FIG.
//!
//! Serves `Metrics::snapshot()` on `/metrics` in Prometheus text format.

use std::net::SocketAddr;

use clap::Parser;
use fig_core::observability::METRICS;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "fig-observability", about = "FIG Prometheus metrics exporter")]
struct Args {
    #[arg(long, default_value = "127.0.0.1:9090")]
    listen: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fig_core::observability::init_tracing("info");
    let args = Args::parse();
    let listener = TcpListener::bind(args.listen).await?;
    tracing::info!("Prometheus metrics on http://{}/metrics", args.listen);

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]);
            if req.starts_with("GET /metrics") {
                let body = METRICS.render_prometheus();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes()).await;
            } else {
                let _ = stream
                    .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
                    .await;
            }
        });
    }
}
