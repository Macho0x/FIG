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

use std::path::PathBuf;

use clap::Parser;
use fig_exchange_sim::server::run_server;

#[derive(Parser)]
#[command(name = "fig-exchange-sim", about = "FIG reference exchange simulator")]
struct Args {
    /// UDP listen address (QUIC/TREE)
    #[arg(long, default_value = "127.0.0.1:8443")]
    addr: String,
    /// Durable session backend: file or redis (requires `session-redis` feature)
    #[arg(long, default_value = "file")]
    session_store: String,
    /// Redis URL when `--session-store redis`
    #[arg(long)]
    redis_url: Option<String>,
    /// Directory for file-backed sessions
    #[arg(long)]
    session_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fig_exchange_sim=debug,fig_core=info")
        .init();

    let args = Args::parse();
    std::env::set_var("FIG_SESSION_STORE", &args.session_store);
    if let Some(url) = &args.redis_url {
        std::env::set_var("REDIS_URL", url);
    }
    if let Some(path) = &args.session_path {
        std::env::set_var("FIG_SESSION_PATH", path.to_string_lossy().into_owned());
    }

    tracing::info!("FIG Exchange Simulator starting...");
    let endpoint = run_server(&args.addr).await?;
    tracing::info!("FIG server listening on {}", endpoint.local_addr()?);
    tokio::signal::ctrl_c().await?;
    Ok(())
}
