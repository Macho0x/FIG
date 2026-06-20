//! FIG Exchange Simulator — Killer Demo Server
//!
//! A native FIG exchange simulator that demonstrates:
//! - Order entry (NewOrderSingle, Cancel, CancelReplace)
//! - Execution reports (streaming)
//! - Market data (streaming)
//! - Account queries (request-response)
//! - Session management (0-RTT resumption)
//!
//! Run with: cargo run -p fig-exchange-sim

use fig_exchange_sim::server::run_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fig_exchange_sim=debug,fig_core=info")
        .init();
    tracing::info!("FIG Exchange Simulator starting...");
    let endpoint = run_server("127.0.0.1:8443").await?;
    tracing::info!("FIG server listening on {}", endpoint.local_addr()?);
    // Stay alive until interrupted (wait_idle returns immediately with no peers).
    tokio::signal::ctrl_c().await?;
    Ok(())
}
