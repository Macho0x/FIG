//! Standalone FIG gateway process.
//!
//! Accepts legacy FIX, REST, WebSocket, and opt-in OUCH/ITCH connections and
//! translates them to FIG frames. Intended as a migration bridge alongside
//! native FIG clients.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    self, CancelReplaceRequest, CancelRequest, ExecutionReport, MassCancelRequest, NewOrderSingle,
    SecurityStatus, SecurityStatusRequest, TradingSessionStatus, TradingSessionStatusRequest,
};
use fig_gateways::backend::{connect_backend, proxy_frame, BackendSession};
use fig_gateways::fix::{
    fig_to_fix_business_message_reject, fig_to_fix_execution_report, fig_to_fix_mass_cancel_report,
    fig_to_fix_security_status, fig_to_fix_trading_session_status, fix_to_fig_cancel,
    fix_to_fig_cancel_replace, fix_to_fig_mass_cancel, fix_to_fig_order,
    fix_to_fig_security_status_request, fix_to_fig_trading_session_status_request,
    logon_to_stream_open, split_fix_messages, BusinessMessageReject, BusinessRejectReason,
    FixMessage,
};
use fig_gateways::fix_seq_store::{build_seq_store, FixSeqStoreBackend};
use fig_gateways::fix_session::{FixAction, FixSession};
use fig_gateways::fix_tls::{accept_tls, build_tls_acceptor, FixGatewayStream, FixTlsAcceptor};
use fig_gateways::rest::{
    frames_to_http_response, http_to_fig_frame, parse_http_request, serialize_http_response,
};
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws::{fig_to_ws_frame, serialize_ws_frame, WsOpcode};
use fig_gateways::ws_catalog::{
    fig_frame_to_legacy_ws_json, fig_subscribe_frame, fig_unsubscribe_frame, legacy_ws_command,
    LegacyWsCommand,
};
use fig_gateways::ws_listener::{
    accept_websocket, is_websocket_upgrade, read_ws_text_or_binary, write_ws_json, write_ws_pong,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "fig-gateway", about = "FIG legacy protocol gateway")]
struct Args {
    /// REST/HTTP listen address
    #[arg(long, default_value = "127.0.0.1:8080")]
    rest_addr: SocketAddr,
    /// WebSocket listen address (HTTP upgrade)
    #[arg(long, default_value = "127.0.0.1:8090")]
    ws_addr: SocketAddr,
    /// FIX TCP listen address
    #[arg(long, default_value = "127.0.0.1:9876")]
    fix_addr: SocketAddr,
    /// Enable TLS on the FIX acceptor
    #[arg(long)]
    fix_tls: bool,
    /// FIX sequence store backend: memory, file
    #[arg(long, default_value = "memory")]
    fix_seq_store: String,
    /// Directory for file-backed FIX sequence store
    #[arg(long, default_value = "/tmp/fig-fix-seq")]
    fix_seq_path: PathBuf,
    /// Optional FIG backend to proxy translated frames (host:port)
    #[arg(long)]
    fig_backend: Option<SocketAddr>,
    /// HTTP listen address for `/healthz` liveness
    #[arg(long)]
    health_addr: Option<SocketAddr>,
    /// HTTP listen address for Prometheus `/metrics`
    #[arg(long)]
    metrics_addr: Option<SocketAddr>,
    /// After FIX Logon, SUBSCRIBE executions for this account and egress 35=8
    #[arg(long)]
    fix_dropcopy_account: Option<String>,
    /// SoupBinTCP + OUCH 4.2-shaped listener (off unless set)
    #[arg(long)]
    ouch_addr: Option<SocketAddr>,
    /// SoupBinTCP ITCH L2 egress (off unless set)
    #[arg(long)]
    itch_addr: Option<SocketAddr>,
    /// MoldUDP64 ITCH L2 datagram destination (off unless set)
    #[arg(long)]
    itch_multicast: Option<SocketAddr>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fig_core::observability::init_tracing("info,fig_gateway=debug");
    let args = Args::parse();

    let seq_backend = FixSeqStoreBackend::parse(&args.fix_seq_store)
        .ok_or_else(|| anyhow::anyhow!("invalid --fix-seq-store: {}", args.fix_seq_store))?;
    let seq_store = build_seq_store(seq_backend, &args.fix_seq_path);

    info!("FIG gateway starting");
    info!("  REST: {}", args.rest_addr);
    info!("  WS:   {}", args.ws_addr);
    info!(
        "  FIX:  {} (tls={}, seq_store={:?})",
        args.fix_addr, args.fix_tls, seq_backend
    );
    if let Some(backend) = args.fig_backend {
        info!("  FIG backend: {}", backend);
    }

    let tls_acceptor = if args.fix_tls {
        let (cert, key) =
            fig_core::transport::generate_self_signed_cert().map_err(|e| anyhow::anyhow!("{e}"))?;
        Some(build_tls_acceptor(cert, key).map_err(|e| anyhow::anyhow!("{e}"))?)
    } else {
        None
    };

    let rest = tokio::spawn(run_rest_gateway(args.rest_addr, args.fig_backend));
    let ws = tokio::spawn(run_ws_gateway(args.ws_addr, args.fig_backend));
    let fix = tokio::spawn(run_fix_gateway(
        args.fix_addr,
        tls_acceptor,
        seq_store,
        args.fig_backend,
        args.fix_dropcopy_account.clone(),
    ));
    if let Some(addr) = args.ouch_addr {
        info!("  OUCH: {}", addr);
        tokio::spawn(fig_gateways::ouch::run_ouch_gateway(addr, args.fig_backend));
    }
    if args.itch_addr.is_some() || args.itch_multicast.is_some() {
        if let Some(addr) = args.itch_addr {
            info!("  ITCH SoupBin: {}", addr);
        }
        if let Some(addr) = args.itch_multicast {
            info!("  ITCH MoldUDP64: {}", addr);
        }
        tokio::spawn(fig_gateways::itch::run_itch_gateway(
            args.itch_addr,
            args.itch_multicast,
            args.fig_backend,
        ));
    }

    if let Some(addr) = args.health_addr {
        tokio::spawn(run_health_server(addr));
    }
    if let Some(addr) = args.metrics_addr {
        tokio::spawn(run_metrics_server(addr));
    }

    tokio::select! {
        r = rest => r??,
        w = ws => w??,
        f = fix => f??,
    }

    Ok(())
}

async fn run_health_server(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("Health check on http://{}/healthz", addr);
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 256];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]);
            if req.starts_with("GET /healthz") {
                let _ = stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nok")
                    .await;
            } else {
                let _ = stream
                    .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n")
                    .await;
            }
        });
    }
}

async fn run_metrics_server(addr: SocketAddr) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("Prometheus metrics on http://{}/metrics", addr);
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0u8; 512];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            let req = String::from_utf8_lossy(&buf[..n]);
            if req.starts_with("GET /metrics") {
                let body = fig_core::observability::METRICS.render_prometheus();
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

async fn run_rest_gateway(addr: SocketAddr, fig_backend: Option<SocketAddr>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("REST gateway listening on {}", addr);

    loop {
        let (mut stream, peer) = listener.accept().await?;
        let backend = fig_backend;
        tokio::spawn(async move {
            if let Err(e) = handle_rest_connection(&mut stream, backend).await {
                warn!("REST connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_rest_connection(
    stream: &mut TcpStream,
    fig_backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    let mut buf = vec![0u8; 65536];
    let n = stream.read(&mut buf).await?;
    if n == 0 {
        return Ok(());
    }

    if is_websocket_upgrade(&buf[..n]) {
        return handle_ws_connection(stream, &buf[..n], fig_backend).await;
    }

    let request = parse_http_request(&buf[..n])?;
    fig_core::observability::Metrics::inc(&fig_core::observability::METRICS.gateway_translations);

    let http_response = if request.method.eq_ignore_ascii_case("GET") {
        let frame = http_get_to_fig_request(&request)?;
        if let Some(addr) = fig_backend {
            match proxy_frame(addr, frame).await {
                Ok(frames) => frames_to_http_response(&frames)?,
                Err(e) => {
                    warn!("FIG backend query failed: {}", e);
                    demo_translate_response(&http_get_to_fig_request(&request)?)?
                }
            }
        } else {
            demo_translate_response(&http_get_to_fig_request(&request)?)?
        }
    } else {
        let frame = http_to_fig_frame(&request)?;
        demo_translate_response(&frame)?
    };

    stream
        .write_all(&serialize_http_response(&http_response))
        .await?;
    Ok(())
}

async fn run_ws_gateway(addr: SocketAddr, fig_backend: Option<SocketAddr>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket gateway listening on {}", addr);

    loop {
        let (mut stream, peer) = listener.accept().await?;
        let backend = fig_backend;
        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            let n = match stream.read(&mut buf).await {
                Ok(n) => n,
                Err(e) => {
                    warn!("WS read from {} failed: {}", peer, e);
                    return;
                }
            };
            if n == 0 {
                return;
            }
            if let Err(e) = handle_ws_connection(&mut stream, &buf[..n], backend).await {
                warn!("WS connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_ws_connection(
    stream: &mut TcpStream,
    raw: &[u8],
    fig_backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    accept_websocket(stream, raw).await?;
    fig_core::observability::Metrics::inc(&fig_core::observability::METRICS.gateway_translations);

    let backend = if let Some(addr) = fig_backend {
        Some(BackendSession::connect(addr).await?)
    } else {
        None
    };
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Frame>();
    let mut next_channel: u16 = 1;
    let mut topics: HashMap<String, u16> = HashMap::new();

    loop {
        tokio::select! {
            ws = read_ws_text_or_binary(stream) => {
                let ws = ws?;
                match ws.opcode {
                    WsOpcode::Close => break,
                    WsOpcode::Ping => {
                        write_ws_pong(stream, &ws.payload).await?;
                    }
                    WsOpcode::Text | WsOpcode::Binary => {
                        let text = String::from_utf8(ws.payload)?;
                        let command = match legacy_ws_command(&text) {
                            Ok(command) => command,
                            Err(e) => {
                                write_ws_json(
                                    stream,
                                    &serde_json::json!({"error": e.to_string()}).to_string(),
                                )
                                .await?;
                                continue;
                            }
                        };
                        let (fig, listen) = match command {
                            LegacyWsCommand::Subscribe(sub) => {
                                let ch = next_channel;
                                next_channel = next_channel.saturating_add(1);
                                topics.insert(sub.routing_key.clone(), ch);
                                (fig_subscribe_frame(ch, &sub), true)
                            }
                            LegacyWsCommand::Unsubscribe(sub) => {
                                let Some(ch) = topics.get(&sub.routing_key).copied() else {
                                    write_ws_json(stream, r#"{"error":"unknown subscription"}"#)
                                        .await?;
                                    continue;
                                };
                                (fig_unsubscribe_frame(ch, &sub), false)
                            }
                        };
                        if let Some(session) = &backend {
                            session.send_frame(&fig).await?;
                            if listen {
                                let conn = session.connection();
                                let ch = fig.channel_id;
                                let tx = tx.clone();
                                tokio::spawn(async move {
                                    while let Ok(frame) = conn.recv_frame(ch).await {
                                        if tx.send(frame).is_err() {
                                            break;
                                        }
                                    }
                                });
                            }
                        } else {
                            write_ws_json(
                                stream,
                                &format!(
                                    "{{\"status\":\"translated\",\"frame_type\":\"{}\"}}",
                                    fig.frame_type
                                ),
                            )
                            .await?;
                        }
                    }
                    _ => {}
                }
            }
            Some(frame) = rx.recv() => {
                if matches!(
                    frame.frame_type,
                    FrameType::StreamItem | FrameType::Response | FrameType::StreamClose
                ) {
                    let json = fig_frame_to_legacy_ws_json(&frame)?;
                    write_ws_json(stream, &json).await?;
                } else if frame.frame_type == FrameType::Control {
                    let pong = fig_to_ws_frame(&frame)?;
                    stream.write_all(&serialize_ws_frame(&pong)).await?;
                }
            }
        }
    }
    Ok(())
}

fn demo_translate_response(frame: &Frame) -> anyhow::Result<fig_gateways::rest::HttpResponse> {
    let channel_path = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");
    let response_body = format!(
        "FIG frame translated: type={} channel={} path={} payload_len={}\n",
        frame.frame_type,
        frame.channel_id,
        channel_path,
        frame.payload.len()
    );
    Ok(fig_gateways::rest::HttpResponse {
        status_code: 200,
        reason: "OK".to_string(),
        headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
        body: response_body.into_bytes(),
    })
}

async fn run_fix_gateway(
    addr: SocketAddr,
    tls_acceptor: Option<FixTlsAcceptor>,
    seq_store: Arc<dyn fig_gateways::fix_seq_store::FixSeqStore>,
    fig_backend: Option<SocketAddr>,
    dropcopy_account: Option<String>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("FIX gateway listening on {}", addr);

    loop {
        let (stream, peer) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let store = seq_store.clone();
        let backend = fig_backend;
        let dropcopy = dropcopy_account.clone();
        tokio::spawn(async move {
            let gateway_stream = match acceptor {
                Some(acceptor) => match accept_tls(&acceptor, stream).await {
                    Ok(tls) => FixGatewayStream::from_tls(tls),
                    Err(e) => {
                        warn!("FIX TLS handshake from {} failed: {}", peer, e);
                        return;
                    }
                },
                None => FixGatewayStream::from_plain(stream),
            };
            if let Err(e) = handle_fix_connection(gateway_stream, store, backend, dropcopy).await {
                warn!("FIX connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_fix_connection(
    mut stream: FixGatewayStream,
    seq_store: Arc<dyn fig_gateways::fix_seq_store::FixSeqStore>,
    fig_backend: Option<SocketAddr>,
    dropcopy_account: Option<String>,
) -> anyhow::Result<()> {
    let backend_conn = if let Some(addr) = fig_backend {
        Some(connect_backend(addr).await?)
    } else {
        None
    };
    let mut session = FixSession::new("FIG-GW".into(), "CLIENT".into()).with_seq_store(seq_store);
    let mut read_buf = Vec::new();
    let mut tmp = vec![0u8; 65536];
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ExecutionReport>();
    let mut dropcopy_started = false;

    loop {
        tokio::select! {
            n = stream.read(&mut tmp) => {
                let n = n?;
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
                        if let Some(sender) = fix_msg.get_tag(49) {
                            session.set_target_comp_id(sender);
                        }
                        let _frame = logon_to_stream_open(&fix_msg)?;
                    }

                    let actions = session.process_incoming(&fix_msg)?;
                    for action in actions {
                        match action {
                            FixAction::SendMessage(response) => {
                                session.record_outbound(response.clone());
                                stream.write_all(&response.to_bytes()).await?;
                            }
                            FixAction::CloseConnection => return Ok(()),
                            FixAction::SendResendRequest { .. } | FixAction::NoAction => {}
                        }
                    }

                    if msg_type == "A" && session.is_logged_in() {
                        let ack = session.send_logon();
                        stream.write_all(&ack.to_bytes()).await?;
                        if !dropcopy_started {
                            if let (Some(account), Some(addr)) =
                                (dropcopy_account.as_deref(), fig_backend)
                            {
                                start_dropcopy(addr, account, tx.clone());
                                dropcopy_started = true;
                            }
                        }
                    }

                    if !session.is_logged_in() {
                        continue;
                    }

                    let translate_result: Result<Vec<Vec<u8>>, fig_gateways::fix::FixError> =
                        match msg_type.as_str() {
                            "D" => {
                                let order = fix_to_fig_order(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    match forward_new_order(conn, &order).await {
                                        Ok(reports) => {
                                            for report in reports {
                                                let ctx = session.outbound_context();
                                                let bytes = fig_to_fix_execution_report(&report, &ctx);
                                                if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                                    session.record_outbound(msg);
                                                }
                                                out.push(bytes);
                                            }
                                        }
                                        Err(e) => warn!("FIG backend forward failed: {}", e),
                                    }
                                }
                                Ok(out)
                            }
                            "F" => {
                                let cancel = fix_to_fig_cancel(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    match forward_cancel(conn, &cancel, "DEFAULT").await {
                                        Ok(reports) => {
                                            for report in reports {
                                                let ctx = session.outbound_context();
                                                let bytes = fig_to_fix_execution_report(&report, &ctx);
                                                if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                                    session.record_outbound(msg);
                                                }
                                                out.push(bytes);
                                            }
                                        }
                                        Err(e) => warn!("FIG backend forward failed: {}", e),
                                    }
                                }
                                Ok(out)
                            }
                            "G" => {
                                let replace = fix_to_fig_cancel_replace(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    match forward_cancel_replace(conn, &replace).await {
                                        Ok(reports) => {
                                            for report in reports {
                                                let ctx = session.outbound_context();
                                                let bytes = fig_to_fix_execution_report(&report, &ctx);
                                                if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                                    session.record_outbound(msg);
                                                }
                                                out.push(bytes);
                                            }
                                        }
                                        Err(e) => warn!("FIG backend forward failed: {}", e),
                                    }
                                }
                                Ok(out)
                            }
                            "q" => {
                                let req = fix_to_fig_mass_cancel(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    match forward_mass_cancel(conn, &req).await {
                                        Ok(report) => {
                                            let ctx = session.outbound_context();
                                            let bytes =
                                                fig_to_fix_mass_cancel_report(&report, &ctx);
                                            if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                                session.record_outbound(msg);
                                            }
                                            out.push(bytes);
                                        }
                                        Err(e) => warn!("mass cancel forward failed: {e}"),
                                    }
                                }
                                Ok(out)
                            }
                            "g" => {
                                let req = fix_to_fig_trading_session_status_request(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    if let Ok(status) = forward_session_status(conn, &req).await {
                                        let ctx = session.outbound_context();
                                        let bytes = fig_to_fix_trading_session_status(&status, &ctx);
                                        if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                            session.record_outbound(msg);
                                        }
                                        out.push(bytes);
                                    }
                                }
                                Ok(out)
                            }
                            "e" => {
                                let req = fix_to_fig_security_status_request(&fix_msg.tags)?;
                                let mut out = Vec::new();
                                if let Some(conn) = backend_conn.as_ref() {
                                    if let Ok(status) = forward_security_status(conn, &req).await {
                                        let ctx = session.outbound_context();
                                        let bytes = fig_to_fix_security_status(&status, &ctx);
                                        if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                            session.record_outbound(msg);
                                        }
                                        out.push(bytes);
                                    }
                                }
                                Ok(out)
                            }
                            _ => Ok(Vec::new()),
                        };

                    match translate_result {
                        Err(err) => {
                            warn!("FIX translation failed for MsgType={}: {}", msg_type, err);
                            let ctx = session.outbound_context();
                            let reject = BusinessMessageReject {
                                ref_msg_type: msg_type.clone(),
                                ref_seq_num,
                                reason: BusinessRejectReason::ConditionallyRequiredFieldMissing,
                                text: Some(err.to_string()),
                            };
                            let bytes = fig_to_fix_business_message_reject(&reject, &ctx);
                            if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                                session.record_outbound(msg);
                            }
                            stream.write_all(&bytes).await?;
                        }
                        Ok(outbound) => {
                            if matches!(msg_type.as_str(), "D" | "F" | "G" | "q" | "g" | "e") {
                                fig_core::observability::Metrics::inc(
                                    &fig_core::observability::METRICS.gateway_translations,
                                );
                            }
                            for bytes in outbound {
                                stream.write_all(&bytes).await?;
                            }
                        }
                    }
                }
            }
            Some(report) = rx.recv() => {
                let ctx = session.outbound_context();
                let bytes = fig_to_fix_execution_report(&report, &ctx);
                if let Ok(msg) = FixMessage::from_bytes(&bytes) {
                    session.record_outbound(msg);
                }
                stream.write_all(&bytes).await?;
            }
        }
    }

    Ok(())
}

fn start_dropcopy(
    backend: SocketAddr,
    account: &str,
    tx: tokio::sync::mpsc::UnboundedSender<ExecutionReport>,
) {
    let account = account.to_string();
    tokio::spawn(async move {
        let session = match BackendSession::connect(backend).await {
            Ok(s) => s,
            Err(e) => {
                warn!("drop-copy backend connect failed: {e}");
                return;
            }
        };
        let path = format!("trading/accounts/{account}/executions");
        let frame = Frame::new(FrameType::Subscribe, 100)
            .with_seq(1)
            .with_extension(Extension::text(ExtensionTag::ChannelPath, &path))
            .with_extension(Extension::text(ExtensionTag::RoutingKey, &path))
            .with_extension(Extension::text(
                ExtensionTag::AuthToken,
                format!("fig-dev-{account}"),
            ));
        if let Err(e) = session.send_frame(&frame).await {
            warn!("drop-copy subscribe failed: {e}");
            return;
        }
        let conn = session.connection();
        while let Ok(frame) = conn.recv_frame(100).await {
            if let Ok(report) = decode_cbor::<ExecutionReport>(&frame.payload) {
                if tx.send(report).is_err() {
                    break;
                }
            }
        }
    });
}

fn auth_frame(frame: Frame, account: &str) -> Frame {
    frame.with_extension(Extension::text(
        ExtensionTag::AuthToken,
        format!("fig-dev-{account}"),
    ))
}

fn reports_from_frames(frames: &[Frame]) -> Vec<ExecutionReport> {
    frames
        .iter()
        .filter_map(|f| decode_cbor::<ExecutionReport>(&f.payload).ok())
        .collect()
}

async fn forward_new_order(
    conn: &fig_core::transport::FigConnection,
    order: &NewOrderSingle,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let account = order.account.as_deref().unwrap_or("DEFAULT");
    let payload = encode_cbor(order)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 1)
            .with_seq(1)
            .with_schema_id(messages::schema_id::TRADING_ORDERS)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!("trading/accounts/{account}/orders"),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/cbor",
            ))
            .with_payload(payload),
        account,
    );
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(reports_from_frames(&frames))
}

async fn forward_cancel(
    conn: &fig_core::transport::FigConnection,
    cancel: &CancelRequest,
    account: &str,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let payload = encode_cbor(cancel)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 1)
            .with_seq(2)
            .with_schema_id(messages::schema_id::TRADING_ORDERS)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!(
                    "trading/accounts/{account}/orders/{}/cancel",
                    cancel.orig_cl_ord_id
                ),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_payload(payload),
        account,
    );
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(reports_from_frames(&frames))
}

async fn forward_cancel_replace(
    conn: &fig_core::transport::FigConnection,
    replace: &CancelReplaceRequest,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let payload = encode_cbor(replace)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 1)
            .with_seq(3)
            .with_schema_id(messages::schema_id::TRADING_ORDERS)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!(
                    "trading/accounts/DEFAULT/orders/{}/replace",
                    replace.orig_cl_ord_id
                ),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_payload(payload),
        "DEFAULT",
    );
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(reports_from_frames(&frames))
}

async fn forward_mass_cancel(
    conn: &fig_core::transport::FigConnection,
    req: &MassCancelRequest,
) -> anyhow::Result<fig_core::messages::MassCancelReport> {
    let payload = encode_cbor(req)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 1)
            .with_seq(1)
            .with_schema_id(messages::schema_id::TRADING_ORDERS)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!("trading/accounts/{}/orders/masscancel", req.account),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_payload(payload),
        &req.account,
    );
    let frames = conn.request_and_recv_all(frame).await?;
    frames
        .iter()
        .find_map(|f| decode_cbor::<fig_core::messages::MassCancelReport>(&f.payload).ok())
        .ok_or_else(|| anyhow::anyhow!("no MassCancelReport"))
}

async fn forward_session_status(
    conn: &fig_core::transport::FigConnection,
    req: &TradingSessionStatusRequest,
) -> anyhow::Result<TradingSessionStatus> {
    let payload = encode_cbor(req)?;
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(messages::schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/session",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_payload(payload);
    let frames = conn.request_and_recv_all(frame).await?;
    frames
        .iter()
        .find_map(|f| decode_cbor::<TradingSessionStatus>(&f.payload).ok())
        .ok_or_else(|| anyhow::anyhow!("no TradingSessionStatus"))
}

async fn forward_security_status(
    conn: &fig_core::transport::FigConnection,
    req: &SecurityStatusRequest,
) -> anyhow::Result<SecurityStatus> {
    let payload = encode_cbor(req)?;
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(messages::schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("marketdata/{}/status", req.symbol),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_payload(payload);
    let frames = conn.request_and_recv_all(frame).await?;
    frames
        .iter()
        .find_map(|f| decode_cbor::<SecurityStatus>(&f.payload).ok())
        .ok_or_else(|| anyhow::anyhow!("no SecurityStatus"))
}
