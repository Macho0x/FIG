//! Standalone FIG gateway process.
//!
//! Accepts legacy FIX, REST, and WebSocket connections and translates them to FIG frames.
//! Intended as a migration bridge alongside native FIG clients.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{self, CancelReplaceRequest, CancelRequest, NewOrderSingle};
use fig_gateways::backend::{connect_backend, proxy_frame};
use fig_gateways::fix::{
    fig_to_fix_business_message_reject, fix_to_fig_cancel, fix_to_fig_cancel_replace,
    fix_to_fig_order, logon_to_stream_open, split_fix_messages, BusinessMessageReject,
    BusinessRejectReason, FixMessage, FixOutboundContext,
};
use fig_gateways::fix_seq_store::{build_seq_store, FixSeqStoreBackend};
use fig_gateways::fix_session::{FixAction, FixSession};
use fig_gateways::fix_tls::{accept_tls, build_tls_acceptor, FixGatewayStream, FixTlsAcceptor};
use fig_gateways::rest::{
    fig_to_http_response, http_to_fig_frame, parse_http_request, serialize_http_response,
};
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws::{fig_to_ws_frame, serialize_ws_frame, WsOpcode};
use fig_gateways::ws_catalog::{fig_stream_item_to_legacy_json, legacy_ws_json_to_fig_subscribe};
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
    /// FIX sequence store backend: memory, file, redis, etcd
    #[arg(long, default_value = "memory")]
    fix_seq_store: String,
    /// Directory for file-backed FIX sequence store
    #[arg(long, default_value = "/tmp/fig-fix-seq")]
    fix_seq_path: PathBuf,
    /// Redis/etcd URL for shared FIX sequence store
    #[arg(long, default_value = "redis://127.0.0.1:6379/fig")]
    fix_seq_url: String,
    /// Optional FIG backend to proxy translated frames (host:port)
    #[arg(long)]
    fig_backend: Option<SocketAddr>,
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
    ));

    tokio::select! {
        r = rest => r??,
        w = ws => w??,
        f = fix => f??,
    }

    Ok(())
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

    loop {
        let ws = read_ws_text_or_binary(stream).await?;
        match ws.opcode {
            WsOpcode::Close => break,
            WsOpcode::Ping => {
                write_ws_pong(stream, &ws.payload).await?;
                continue;
            }
            WsOpcode::Text | WsOpcode::Binary => {
                let text = String::from_utf8(ws.payload)?;
                let fig = legacy_ws_json_to_fig_subscribe(&text, 1)?;
                if let Some(addr) = fig_backend {
                    let responses = proxy_frame(addr, fig).await?;
                    for resp in responses {
                        if resp.frame_type == FrameType::StreamItem
                            || resp.frame_type == FrameType::Response
                        {
                            let json = fig_stream_item_to_legacy_json(&resp)?;
                            write_ws_json(stream, &json).await?;
                        } else if resp.frame_type == FrameType::Control {
                            let pong = fig_to_ws_frame(&resp)?;
                            stream.write_all(&serialize_ws_frame(&pong)).await?;
                        }
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
    Ok(())
}

fn frames_to_http_response(frames: &[Frame]) -> anyhow::Result<fig_gateways::rest::HttpResponse> {
    let response = frames
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .or_else(|| frames.first())
        .ok_or_else(|| anyhow::anyhow!("backend returned no frames"))?;
    fig_to_http_response(response).map_err(|e| anyhow::anyhow!("{e}"))
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
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("FIX gateway listening on {}", addr);

    loop {
        let (stream, peer) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let store = seq_store.clone();
        let backend = fig_backend;
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
            if let Err(e) = handle_fix_connection(gateway_stream, store, backend).await {
                warn!("FIX connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_fix_connection(
    mut stream: FixGatewayStream,
    seq_store: Arc<dyn fig_gateways::fix_seq_store::FixSeqStore>,
    fig_backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    let backend_conn = if let Some(addr) = fig_backend {
        Some(connect_backend(addr).await?)
    } else {
        None
    };
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
                let translate_result: Result<(), fig_gateways::fix::FixError> =
                    match msg_type.as_str() {
                        "D" => {
                            let order = fix_to_fig_order(&fix_msg.tags)?;
                            if let Some(conn) = backend_conn.as_ref() {
                                if let Err(e) = forward_new_order(conn, &order).await {
                                    warn!("FIG backend forward failed: {}", e);
                                }
                            }
                            Ok(())
                        }
                        "F" => {
                            let cancel = fix_to_fig_cancel(&fix_msg.tags)?;
                            if let Some(conn) = backend_conn.as_ref() {
                                if let Err(e) = forward_cancel(conn, &cancel).await {
                                    warn!("FIG backend forward failed: {}", e);
                                }
                            }
                            Ok(())
                        }
                        "G" => {
                            let replace = fix_to_fig_cancel_replace(&fix_msg.tags)?;
                            if let Some(conn) = backend_conn.as_ref() {
                                if let Err(e) = forward_cancel_replace(conn, &replace).await {
                                    warn!("FIG backend forward failed: {}", e);
                                }
                            }
                            Ok(())
                        }
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

async fn forward_new_order(
    conn: &fig_core::transport::FigConnection,
    order: &NewOrderSingle,
) -> anyhow::Result<()> {
    let account = order.account.as_deref().unwrap_or("DEFAULT");
    let payload = encode_cbor(order)?;
    let frame = Frame::new(FrameType::Request, 1)
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
        .with_payload(payload);
    conn.request_and_recv_all(frame).await?;
    Ok(())
}

async fn forward_cancel(
    conn: &fig_core::transport::FigConnection,
    cancel: &CancelRequest,
) -> anyhow::Result<()> {
    let payload = encode_cbor(cancel)?;
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(2)
        .with_schema_id(messages::schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEFAULT/orders/cancel",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "DELETE"))
        .with_payload(payload);
    conn.request_and_recv_all(frame).await?;
    Ok(())
}

async fn forward_cancel_replace(
    conn: &fig_core::transport::FigConnection,
    replace: &CancelReplaceRequest,
) -> anyhow::Result<()> {
    let payload = encode_cbor(replace)?;
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(3)
        .with_schema_id(messages::schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEFAULT/orders/replace",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "PUT"))
        .with_payload(payload);
    conn.request_and_recv_all(frame).await?;
    Ok(())
}
