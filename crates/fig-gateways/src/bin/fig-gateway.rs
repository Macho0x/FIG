//! Standalone FIG gateway process.
//!
//! Accepts legacy FIX and REST connections and translates them to FIG frames.
//! Intended as a migration bridge alongside native FIG clients.

use std::net::SocketAddr;

use clap::Parser;
use fig_gateways::fix::FixMessage;
use fig_gateways::rest::{http_to_fig_frame, parse_http_request};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "fig-gateway", about = "FIG legacy protocol gateway")]
struct Args {
    /// REST/HTTP listen address
    #[arg(long, default_value = "127.0.0.1:8080")]
    rest_addr: SocketAddr,
    /// FIX TCP listen address
    #[arg(long, default_value = "127.0.0.1:9876")]
    fix_addr: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fig_core::observability::init_tracing("info,fig_gateway=debug");
    let args = Args::parse();

    info!("FIG gateway starting");
    info!("  REST: {}", args.rest_addr);
    info!("  FIX:  {}", args.fix_addr);

    let rest = tokio::spawn(run_rest_gateway(args.rest_addr));
    let fix = tokio::spawn(run_fix_gateway(args.fix_addr));

    tokio::select! {
        r = rest => r??,
        f = fix => f??,
    }

    Ok(())
}

async fn run_rest_gateway(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("REST gateway listening on {}", addr);

    loop {
        let (mut stream, peer) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_rest_connection(&mut stream).await {
                warn!("REST connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_rest_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut buf = vec![0u8; 65536];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let request = parse_http_request(&buf[..n])?;
    let frame = http_to_fig_frame(&request)?;
    fig_core::observability::Metrics::inc(&fig_core::observability::METRICS.gateway_translations);

    let response_body = format!(
        "FIG frame translated: type={} channel={} payload_len={}\n",
        frame.frame_type,
        frame.channel_id,
        frame.payload.len()
    );

    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    stream.write_all(http_response.as_bytes()).await?;
    Ok(())
}

async fn run_fix_gateway(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("FIX gateway listening on {}", addr);

    loop {
        let (mut stream, peer) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_fix_connection(&mut stream).await {
                warn!("FIX connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_fix_connection(stream: &mut TcpStream) -> anyhow::Result<()> {
    let mut buf = vec![0u8; 65536];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    let fix_msg = FixMessage::from_bytes(&buf[..n])?;
    let msg_type = fix_msg.msg_type().unwrap_or("?");
    info!("FIX message received: MsgType={}", msg_type);
    fig_core::observability::Metrics::inc(&fig_core::observability::METRICS.gateway_translations);

    if fix_msg.msg_type() == Some("A") {
        let frame = fig_gateways::fix::logon_to_stream_open(&fix_msg)?;
        info!(
            "Translated FIX Logon → FIG {:?} ch={}",
            frame.frame_type, frame.channel_id
        );
    }

    let ack = format!(
        "8=FIX.4.4\x0135=0\x0149=FIG-GW\x0156=CLIENT\x0134=1\x0152={}\x0110=000\x01",
        chrono_lite_timestamp()
    );
    stream.write_all(ack.as_bytes()).await?;
    Ok(())
}

fn chrono_lite_timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}", secs)
}
