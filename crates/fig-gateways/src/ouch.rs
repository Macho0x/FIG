//! FIG OUCH 4.2-shaped order entry over SoupBinTCP.
//!
//! **Not NASDAQ byte-identical.** Field lengths follow OUCH 4.2 conventions
//! (14-byte tokens, 8-byte stock) with a FIG `account` suffix on Enter Order.
//! See [GATEWAY.md](../../../docs/GATEWAY.md) and ADR 0008.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    schema_id, CancelReplaceRequest, CancelRequest, ExecType, ExecutionReport, NewOrderSingle,
    OrderType, Price, Quantity, Side, TimeInForce,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

use crate::backend::connect_backend;
use crate::soupbin::{
    encode_login_accepted, encode_sequenced, encode_server_heartbeat, parse_login_request,
    SoupDecoder, CLIENT_HEARTBEAT, LOGIN_REQUEST, LOGOUT_REQUEST, UNSEQUENCED_DATA,
};

pub const MSG_ENTER: u8 = b'O';
pub const MSG_CANCEL: u8 = b'X';
pub const MSG_REPLACE: u8 = b'U';
pub const MSG_ACCEPTED: u8 = b'A';
pub const MSG_EXECUTED: u8 = b'E';
pub const MSG_CANCELED: u8 = b'C';
pub const MSG_REJECTED: u8 = b'J';

const TOKEN_LEN: usize = 14;
const STOCK_LEN: usize = 8;
const ACCOUNT_LEN: usize = 16;
const PRICE_SCALE: f64 = 10_000.0;

#[derive(Debug, Clone, PartialEq)]
pub enum OuchInbound {
    Enter {
        token: String,
        side: Side,
        shares: u32,
        stock: String,
        price: f64,
        tif: TimeInForce,
        account: String,
    },
    Cancel {
        token: String,
        shares: u32,
    },
    Replace {
        existing: String,
        replacement: String,
        shares: u32,
        price: f64,
    },
}

#[derive(Debug, Clone)]
struct LiveOrder {
    symbol: String,
    side: Side,
    account: String,
}

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn pad(s: &str, n: usize) -> Vec<u8> {
    crate::soupbin::pad_ascii(s, n)
}

fn take_str(buf: &[u8], off: &mut usize, n: usize) -> Option<String> {
    let end = off.checked_add(n)?;
    let s = crate::soupbin::trim_ascii(buf.get(*off..end)?);
    *off = end;
    Some(s)
}

fn take_u32(buf: &[u8], off: &mut usize) -> Option<u32> {
    let end = off.checked_add(4)?;
    let v = u32::from_be_bytes(buf.get(*off..end)?.try_into().ok()?);
    *off = end;
    Some(v)
}

fn take_u64(buf: &[u8], off: &mut usize) -> Option<u64> {
    let end = off.checked_add(8)?;
    let v = u64::from_be_bytes(buf.get(*off..end)?.try_into().ok()?);
    *off = end;
    Some(v)
}

fn price_from_wire(raw: u64) -> f64 {
    raw as f64 / PRICE_SCALE
}

fn price_to_wire(px: f64) -> u64 {
    (px * PRICE_SCALE).round() as u64
}

/// OUCH buy/sell byte. Short sales are sells (`S`), not bids.
fn side_byte(side: Side) -> u8 {
    match side {
        Side::Buy => b'B',
        Side::Sell | Side::SellShort | Side::SellShortExempt => b'S',
    }
}

/// Parse one FIG-OUCH inbound payload (SoupBin unsequenced body).
pub fn parse_inbound(buf: &[u8]) -> Option<OuchInbound> {
    if buf.is_empty() {
        return None;
    }
    let mut off = 1;
    match buf[0] {
        MSG_ENTER => {
            let token = take_str(buf, &mut off, TOKEN_LEN)?;
            let side = match buf.get(off)? {
                b'B' => Side::Buy,
                b'S' => Side::Sell,
                _ => return None,
            };
            off += 1;
            let shares = take_u32(buf, &mut off)?;
            let stock = take_str(buf, &mut off, STOCK_LEN)?;
            let price = price_from_wire(take_u64(buf, &mut off)?);
            let tif = match buf.get(off)? {
                b'3' => TimeInForce::Ioc,
                _ => TimeInForce::Day,
            };
            off += 1;
            let account = buf
                .get(off..off + ACCOUNT_LEN)
                .map(crate::soupbin::trim_ascii)
                .unwrap_or_default();
            Some(OuchInbound::Enter {
                token,
                side,
                shares,
                stock,
                price,
                tif,
                account,
            })
        }
        MSG_CANCEL => {
            let token = take_str(buf, &mut off, TOKEN_LEN)?;
            let shares = take_u32(buf, &mut off).unwrap_or(0);
            Some(OuchInbound::Cancel { token, shares })
        }
        MSG_REPLACE => {
            let existing = take_str(buf, &mut off, TOKEN_LEN)?;
            let replacement = take_str(buf, &mut off, TOKEN_LEN)?;
            let shares = take_u32(buf, &mut off)?;
            let price = price_from_wire(take_u64(buf, &mut off)?);
            Some(OuchInbound::Replace {
                existing,
                replacement,
                shares,
                price,
            })
        }
        _ => None,
    }
}

pub fn encode_enter(
    token: &str,
    side: Side,
    shares: u32,
    stock: &str,
    price: f64,
    tif: TimeInForce,
    account: &str,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_ENTER);
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.push(side_byte(side));
    buf.extend_from_slice(&shares.to_be_bytes());
    buf.extend_from_slice(&pad(stock, STOCK_LEN));
    buf.extend_from_slice(&price_to_wire(price).to_be_bytes());
    buf.push(match tif {
        TimeInForce::Ioc => b'3',
        _ => b'0',
    });
    buf.extend_from_slice(&pad(account, ACCOUNT_LEN));
    buf
}

pub fn encode_cancel(token: &str, shares: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_CANCEL);
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.extend_from_slice(&shares.to_be_bytes());
    buf
}

pub fn encode_accepted(token: &str, side: Side, shares: u32, stock: &str, price: f64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_ACCEPTED);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.push(side_byte(side));
    buf.extend_from_slice(&shares.to_be_bytes());
    buf.extend_from_slice(&pad(stock, STOCK_LEN));
    buf.extend_from_slice(&price_to_wire(price).to_be_bytes());
    buf.push(b'L');
    buf
}

pub fn encode_executed(token: &str, shares: u32, price: f64, match_num: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_EXECUTED);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.extend_from_slice(&shares.to_be_bytes());
    buf.extend_from_slice(&price_to_wire(price).to_be_bytes());
    buf.push(b'R');
    buf.extend_from_slice(&match_num.to_be_bytes());
    buf
}

pub fn encode_canceled(token: &str, shares: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_CANCELED);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.extend_from_slice(&shares.to_be_bytes());
    buf.push(b'U');
    buf
}

pub fn encode_rejected(token: &str, reason: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_REJECTED);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(token, TOKEN_LEN));
    buf.push(reason);
    buf
}

fn outbound_from_reports(token: &str, reports: &[ExecutionReport]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for (i, report) in reports.iter().enumerate() {
        match report.exec_type {
            ExecType::New => out.push(encode_accepted(
                token,
                report.side,
                report.leaves_qty.0.max(0.0) as u32,
                &report.symbol,
                report.avg_price.0,
            )),
            ExecType::Fill | ExecType::PartialFill => {
                let qty = report.last_qty.as_ref().map(|q| q.0).unwrap_or(0.0) as u32;
                let px = report.last_price.as_ref().map(|p| p.0).unwrap_or(0.0);
                out.push(encode_executed(token, qty, px, i as u64 + 1));
            }
            ExecType::Canceled => {
                out.push(encode_canceled(token, report.cum_qty.0 as u32));
            }
            ExecType::Rejected => out.push(encode_rejected(token, b'T')),
            _ => {}
        }
    }
    if out.is_empty() {
        out.push(encode_rejected(token, b'T'));
    }
    out
}

fn auth_frame(frame: Frame, account: &str) -> Frame {
    frame.with_extension(Extension::text(
        ExtensionTag::AuthToken,
        format!("fig-dev-{account}"),
    ))
}

async fn forward_order(
    backend: SocketAddr,
    order: &NewOrderSingle,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let account = order.account.as_deref().unwrap_or("DEMO-ACCT");
    let payload = encode_cbor(order)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 1)
            .with_seq(1)
            .with_schema_id(schema_id::TRADING_ORDERS)
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
    let conn = connect_backend(backend).await?;
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(frames
        .iter()
        .filter_map(|f| fig_core::codec::decode_cbor::<ExecutionReport>(&f.payload).ok())
        .collect())
}

async fn forward_cancel(
    backend: SocketAddr,
    cancel: &CancelRequest,
    account: &str,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let payload = encode_cbor(cancel)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 2)
            .with_seq(1)
            .with_schema_id(schema_id::TRADING_ORDERS)
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
    let conn = connect_backend(backend).await?;
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(frames
        .iter()
        .filter_map(|f| fig_core::codec::decode_cbor::<ExecutionReport>(&f.payload).ok())
        .collect())
}

async fn forward_replace(
    backend: SocketAddr,
    replace: &CancelReplaceRequest,
    account: &str,
) -> anyhow::Result<Vec<ExecutionReport>> {
    let payload = encode_cbor(replace)?;
    let frame = auth_frame(
        Frame::new(FrameType::Request, 3)
            .with_seq(1)
            .with_schema_id(schema_id::TRADING_ORDERS)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!(
                    "trading/accounts/{account}/orders/{}/replace",
                    replace.orig_cl_ord_id
                ),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_payload(payload),
        account,
    );
    let conn = connect_backend(backend).await?;
    let frames = conn.request_and_recv_all(frame).await?;
    Ok(frames
        .iter()
        .filter_map(|f| fig_core::codec::decode_cbor::<ExecutionReport>(&f.payload).ok())
        .collect())
}

/// Accept SoupBinTCP clients and translate FIG-OUCH into native order REQUESTs.
pub async fn run_ouch_gateway(
    addr: SocketAddr,
    fig_backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("OUCH/SoupBin gateway listening on {}", addr);
    loop {
        let (stream, peer) = listener.accept().await?;
        let backend = fig_backend;
        tokio::spawn(async move {
            if let Err(e) = handle_ouch_connection(stream, backend).await {
                warn!("OUCH connection from {} failed: {}", peer, e);
            }
        });
    }
}

async fn handle_ouch_connection(
    mut stream: TcpStream,
    backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    let mut decoder = SoupDecoder::new();
    let mut buf = vec![0u8; 65536];
    let mut logged_in = false;
    let mut account = "DEMO-ACCT".to_string();
    let mut live: HashMap<String, LiveOrder> = HashMap::new();
    let mut hb = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            n = stream.read(&mut buf) => {
                let n = n?;
                if n == 0 {
                    break;
                }
                decoder.feed(&buf[..n]);
                while let Some(pkt) = decoder.next_packet() {
                    let pkt = pkt?;
                    match pkt.packet_type {
                        LOGIN_REQUEST => {
                            let (user, _pass, session, _seq) = parse_login_request(&pkt.payload);
                            if !user.is_empty() {
                                account = user;
                            }
                            let sess = if session.is_empty() { "OUCH" } else { session.as_str() };
                            stream.write_all(&encode_login_accepted(sess, 1)).await?;
                            logged_in = true;
                        }
                        LOGOUT_REQUEST => return Ok(()),
                        CLIENT_HEARTBEAT => {}
                        UNSEQUENCED_DATA if logged_in => {
                            let Some(msg) = parse_inbound(&pkt.payload) else {
                                continue;
                            };
                            let Some(addr) = backend else {
                                if let OuchInbound::Enter { token, .. } = &msg {
                                    stream.write_all(&encode_sequenced(&encode_rejected(token, b'T'))).await?;
                                }
                                continue;
                            };
                            match msg {
                                OuchInbound::Enter { token, side, shares, stock, price, tif, account: acct } => {
                                    let acct = if acct.is_empty() { account.clone() } else { acct };
                                    let order = NewOrderSingle {
                                        cl_ord_id: token.clone(),
                                        side,
                                        order_qty: Quantity(shares as f64),
                                        price: if price > 0.0 { Some(Price(price)) } else { None },
                                        stop_price: None,
                                        symbol: stock.clone(),
                                        order_type: if price > 0.0 { OrderType::Limit } else { OrderType::Market },
                                        time_in_force: tif,
                                        expire_time: None,
                                        account: Some(acct.clone()),
                                        strategy_id: None,
                                        security_id: None,
                                        id_source: None,
                                        security_exchange: None,
                                        post_only: None,
                                        reduce_only: None,
                                    };
                                    match forward_order(addr, &order).await {
                                        Ok(reports) => {
                                            live.insert(token.clone(), LiveOrder {
                                                symbol: stock,
                                                side,
                                                account: acct,
                                            });
                                            for payload in outbound_from_reports(&token, &reports) {
                                                stream.write_all(&encode_sequenced(&payload)).await?;
                                            }
                                        }
                                        Err(e) => {
                                            warn!("OUCH enter forward failed: {e}");
                                            stream.write_all(&encode_sequenced(&encode_rejected(&token, b'T'))).await?;
                                        }
                                    }
                                }
                                OuchInbound::Cancel { token, shares: _ } => {
                                    let Some(live_ord) = live.get(&token).cloned() else {
                                        stream.write_all(&encode_sequenced(&encode_rejected(&token, b'T'))).await?;
                                        continue;
                                    };
                                    let cancel = CancelRequest {
                                        cl_ord_id: format!("X-{token}"),
                                        orig_cl_ord_id: token.clone(),
                                        symbol: live_ord.symbol,
                                        side: live_ord.side,
                                        order_qty: None,
                                    };
                                    match forward_cancel(addr, &cancel, &live_ord.account).await {
                                        Ok(reports) => {
                                            for payload in outbound_from_reports(&token, &reports) {
                                                stream.write_all(&encode_sequenced(&payload)).await?;
                                            }
                                            live.remove(&token);
                                        }
                                        Err(e) => warn!("OUCH cancel forward failed: {e}"),
                                    }
                                }
                                OuchInbound::Replace { existing, replacement, shares, price } => {
                                    let Some(live_ord) = live.get(&existing).cloned() else {
                                        stream.write_all(&encode_sequenced(&encode_rejected(&existing, b'T'))).await?;
                                        continue;
                                    };
                                    let replace = CancelReplaceRequest {
                                        cl_ord_id: replacement.clone(),
                                        orig_cl_ord_id: existing.clone(),
                                        symbol: live_ord.symbol.clone(),
                                        side: live_ord.side,
                                        order_qty: Quantity(shares as f64),
                                        price: Some(Price(price)),
                                    };
                                    match forward_replace(addr, &replace, &live_ord.account).await {
                                        Ok(reports) => {
                                            live.remove(&existing);
                                            live.insert(replacement.clone(), live_ord);
                                            for payload in outbound_from_reports(&replacement, &reports) {
                                                stream.write_all(&encode_sequenced(&payload)).await?;
                                            }
                                        }
                                        Err(e) => warn!("OUCH replace forward failed: {e}"),
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ = hb.tick(), if logged_in => {
                stream.write_all(&encode_server_heartbeat()).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enter_order_round_trip() {
        let wire = encode_enter(
            "TOK1",
            Side::Buy,
            10,
            "AAPL",
            150.25,
            TimeInForce::Day,
            "DEMO-ACCT",
        );
        let parsed = parse_inbound(&wire).unwrap();
        match parsed {
            OuchInbound::Enter {
                token,
                side,
                shares,
                stock,
                price,
                account,
                ..
            } => {
                assert_eq!(token, "TOK1");
                assert_eq!(side, Side::Buy);
                assert_eq!(shares, 10);
                assert_eq!(stock, "AAPL");
                assert!((price - 150.25).abs() < 0.0001);
                assert_eq!(account, "DEMO-ACCT");
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn cancel_parse() {
        let wire = encode_cancel("TOK2", 0);
        match parse_inbound(&wire).unwrap() {
            OuchInbound::Cancel { token, shares } => {
                assert_eq!(token, "TOK2");
                assert_eq!(shares, 0);
            }
            other => panic!("{other:?}"),
        }
    }
}
