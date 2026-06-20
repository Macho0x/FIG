//! Standalone FIG gateway process.
//!
//! Accepts legacy FIX and REST connections and translates them to FIG frames.
//! Intended as a migration bridge alongside native FIG clients.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use fig_gateways::fix::{
    fig_to_fix_business_message_reject, fix_to_fig_cancel, fix_to_fig_cancel_replace,
    fix_to_fig_order, logon_to_stream_open, split_fix_messages, BusinessMessageReject,
    BusinessRejectReason, FixMessage, FixOutboundContext,
};
use fig_gateways::fix_seq_store::{build_seq_store, FixSeqStoreBackend};
use fig_gateways::fix_session::{FixAction, FixSession};
use fig_gateways::fix_tls::{accept_tls, build_tls_acceptor, FixGatewayStream, FixTlsAcceptor};
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
    /// Enable TLS on the FIX acceptor
    #[arg(long)]
    fix_tls: bool,
    /// FIX sequence store backend: memory, file, redis, etcd
    #[arg(long, default_value = "memory")]
    fix_seq_store: String,
    /// Directory for file-backed FIX sequence store
    #[arg(long, default_value = "/tmp/fig-fix-seq")]
    fix_seq_path: PathBuf,
    /// Redis/etcd URL for shared FIX sequence store
    #[arg(long, default_value = "redis://127.0.0.1:6379/fig")]
    fix_seq_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fig_core::observability::init_tracing("info,fig_gateway=debug");
    let args = Args::parse();

    let seq_backend = FixSeqStoreBackend::parse(&args.fix_seq_store)
        .ok_or_else(|| anyhow::anyhow!("invalid --fix-seq-store: {}", args.fix_seq_store))?;
    let seq_store = build_seq_store(seq_backend, &args.fix_seq_path, &args.fix_seq_url);

    info!("FIG gateway starting");
    info!("  REST: {}", args.rest_addr);
    info!(
        "  FIX:  {} (tls={}, seq_store={:?})",
        args.fix_addr, args.fix_tls, seq_backend
    );

    let tls_acceptor = if args.fix_tls {
        let (cert, key) = fig_core::transport::generate_self_signed_cert()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Some(build_tls_acceptor(cert, key).map_err(|e| anyhow::anyhow!("{e}"))?)
    } else {
        None
    };

    let rest = tokio::spawn(run_rest_gateway(args.rest_addr));
    let fix = tokio::spawn(run_fix_gateway(args.fix_addr, tls_acceptor, seq_store));

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

async fn run_fix_gateway(
    addr: SocketAddr,
    tls_acceptor: Option<FixTlsAcceptor>,
    seq_store: Arc<dyn fig_gateways::fix_seq_store::FixSeqStore>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("FIX gateway listening on {}", addr);

    loop {
        let (stream, peer) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let store = seq_store.clone();
        tokio::spawn(async move {
            let gateway_stream = match acceptor {
                Some(acceptor) => {
                    match accept_tls(&acceptor, stream).await {
                        Ok(tls) => FixGatewayStream::from_tls(tls),
                        Err(e) => {
                            warn!("FIX TLS handshake from {} failed: {}", peer, e);
                            return;
                        }
                    }
                }
                None => FixGatewayStream::from_plain(stream),
            };
            if let Err(e) = handle_fix_connection(gateway_stream, store).await {
                warn!("FIX connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_fix_connection(
    mut stream: FixGatewayStream,
    seq_store: Arc<dyn fig_gateways::fix_seq_store::FixSeqStore>,
) -> anyhow::Result<()> {
    let mut session = FixSession::new("FIG-GW".into(), "CLIENT".into()).with_seq_store(seq_store);
    let mut read_buf = Vec::new();
    let mut tmp = vec![0u8; 65536];

    loop {
        let n = stream.read(&mut tmp).await?;
        if n == 0 {
            break;
        }
        read_buf.extend_from_slice(&tmp[..n]);

        let messages = split_fix_messages(&read_buf);
        if messages.is_empty() {
            continue;
        }

        let consumed = messages.iter().map(|m| m.len()).sum::<usize>();
        read_buf.drain(..consumed);

        for raw in messages {
            let fix_msg = FixMessage::from_bytes(&raw)?;
            let msg_type = fix_msg.msg_type().unwrap_or("?").to_string();
            let ref_seq_num = fix_msg.get_tag(34).and_then(|s| s.parse().ok());
            info!("FIX message received: MsgType={}", msg_type);

            if msg_type == "A" {
                let frame = logon_to_stream_open(&fix_msg)?;
                info!(
                    "Translated FIX Logon → FIG {:?} ch={}",
                    frame.frame_type, frame.channel_id
                );
            } else if session.is_logged_in() {
                let translate_result = match msg_type.as_str() {
                    "D" => fix_to_fig_order(&fix_msg.tags).map(|order| {
                        info!(
                            "Translated FIX NewOrderSingle → FIG {} {:?} {}",
                            order.cl_ord_id, order.side, order.symbol
                        );
                    }),
                    "F" => fix_to_fig_cancel(&fix_msg.tags).map(|cancel| {
                        info!(
                            "Translated FIX CancelRequest → FIG {} -> {}",
                            cancel.cl_ord_id, cancel.orig_cl_ord_id
                        );
                    }),
                    "G" => fix_to_fig_cancel_replace(&fix_msg.tags).map(|replace| {
                        info!(
                            "Translated FIX CancelReplace → FIG {} -> {}",
                            replace.cl_ord_id, replace.orig_cl_ord_id
                        );
                    }),
                    _ => Ok(()),
                };

                if let Err(err) = translate_result {
                    warn!("FIX translation failed for MsgType={}: {}", msg_type, err);
                    let ctx = FixOutboundContext {
                        sender_comp_id: session.sender_comp_id().to_string(),
                        target_comp_id: session.target_comp_id().to_string(),
                        msg_seq_num: session.next_send_seq(),
                    };
                    let reject = BusinessMessageReject {
                        ref_msg_type: msg_type.clone(),
                        ref_seq_num,
                        reason: BusinessRejectReason::ConditionallyRequiredFieldMissing,
                        text: Some(err.to_string()),
                    };
                    stream
                        .write_all(&fig_to_fix_business_message_reject(&reject, &ctx))
                        .await?;
                } else if msg_type == "D" || msg_type == "F" || msg_type == "G" {
                    fig_core::observability::Metrics::inc(
                        &fig_core::observability::METRICS.gateway_translations,
                    );
                }
            }

            let actions = session.process_incoming(&fix_msg)?;
            for action in actions {
                match action {
                    FixAction::SendMessage(response) => {
                        stream.write_all(&response.to_bytes()).await?;
                    }
                    FixAction::CloseConnection => return Ok(()),
                    FixAction::SendResendRequest { .. } | FixAction::NoAction => {}
                }
            }
        }
    }

    Ok(())
}
