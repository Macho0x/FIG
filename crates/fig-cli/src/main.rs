//! FIG CLI binary — runs the seven native FIG demo flows.

use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use tracing::info;

use fig_cli::{connect, run_demos, DEFAULT_SERVER};

#[derive(Parser)]
#[command(
    name = "fig-cli",
    about = "Native FIG demo client (orders, streams, queries)"
)]
struct Args {
    /// FIG server address (host:port). Overrides FIG_SERVER when set.
    #[arg(long, env = "FIG_SERVER", default_value = DEFAULT_SERVER)]
    server: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("fig_cli=info,fig_core=warn")
        .init();

    let args = Args::parse();
    info!("Connecting to {}...", args.server);
    let (endpoint, conn) = connect(args.server).await?;
    info!("Connected.");

    run_demos(&conn).await?;

    conn.close(0u32.into(), b"done");
    endpoint.wait_idle().await;
    Ok(())
}
