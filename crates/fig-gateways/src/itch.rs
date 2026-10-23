//! FIG ITCH L2 egress (price-level, not TotalView) over SoupBinTCP and MoldUDP64.
//!
//! Messages: `S` system event, `R` stock directory, `L` price level, `T` trade.
//! No synthetic order references ([ADR 0009](../../../docs/adr/0009-market-data-plane.md)).

use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fig_core::codec::decode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    MarketDataAction, OrderBookDelta, OrderBookSnapshot, PublicTradeEvent, Side,
    TradingSessionStatus, TradingSessionStatusStatus,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tracing::{info, warn};

use crate::backend::BackendSession;
use crate::soupbin::{
    encode_login_accepted, encode_sequenced, encode_server_heartbeat, parse_login_request,
    SoupDecoder, CLIENT_HEARTBEAT, LOGIN_REQUEST, LOGOUT_REQUEST,
};

pub const MSG_SYSTEM: u8 = b'S';
pub const MSG_DIRECTORY: u8 = b'R';
pub const MSG_LEVEL: u8 = b'L';
pub const MSG_TRADE: u8 = b'T';

const STOCK_LEN: usize = 8;
const PRICE_SCALE: f64 = 10_000.0;
const MOLD_SESSION_LEN: usize = 10;

fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn pad(s: &str, n: usize) -> Vec<u8> {
    crate::soupbin::pad_ascii(s, n)
}

fn price_to_wire(px: f64) -> u64 {
    (px * PRICE_SCALE).round() as u64
}

pub fn encode_system_event(event: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_SYSTEM);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.push(event);
    buf
}

pub fn encode_stock_directory(stock: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_DIRECTORY);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(stock, STOCK_LEN));
    buf
}

/// ITCH buy/sell indicator. Short sales are sells (`S`).
fn side_byte(side: Side) -> u8 {
    match side {
        Side::Buy => b'B',
        Side::Sell | Side::SellShort | Side::SellShortExempt => b'S',
    }
}

/// Price-level update. `action`: `S` snapshot, `U` update, `D` delete.
pub fn encode_price_level(
    stock: &str,
    side: Side,
    price: f64,
    qty: u32,
    level: u8,
    action: u8,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_LEVEL);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(stock, STOCK_LEN));
    buf.push(side_byte(side));
    buf.extend_from_slice(&price_to_wire(price).to_be_bytes());
    buf.extend_from_slice(&qty.to_be_bytes());
    buf.push(level);
    buf.push(action);
    buf
}

pub fn encode_trade(stock: &str, price: f64, qty: u32, side: Side) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(MSG_TRADE);
    buf.extend_from_slice(&now_ns().to_be_bytes());
    buf.extend_from_slice(&pad(stock, STOCK_LEN));
    buf.extend_from_slice(&price_to_wire(price).to_be_bytes());
    buf.extend_from_slice(&qty.to_be_bytes());
    buf.push(side_byte(side));
    buf
}

/// MoldUDP64 datagram wrapping the same ITCH payload bytes as SoupBin.
pub fn encode_moldudp64(session: &str, sequence: u64, payloads: &[&[u8]]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&pad(session, MOLD_SESSION_LEN));
    buf.extend_from_slice(&sequence.to_be_bytes());
    buf.extend_from_slice(&(payloads.len() as u16).to_be_bytes());
    for p in payloads {
        buf.extend_from_slice(&(p.len() as u16).to_be_bytes());
        buf.extend_from_slice(p);
    }
    buf
}

pub fn decode_moldudp64(buf: &[u8]) -> Option<(String, u64, Vec<Vec<u8>>)> {
    if buf.len() < MOLD_SESSION_LEN + 8 + 2 {
        return None;
    }
    let session = crate::soupbin::trim_ascii(&buf[..MOLD_SESSION_LEN]);
    let sequence = u64::from_be_bytes(
        buf[MOLD_SESSION_LEN..MOLD_SESSION_LEN + 8]
            .try_into()
            .ok()?,
    );
    let count = u16::from_be_bytes(
        buf[MOLD_SESSION_LEN + 8..MOLD_SESSION_LEN + 10]
            .try_into()
            .ok()?,
    ) as usize;
    let mut off = MOLD_SESSION_LEN + 10;
    let mut msgs = Vec::with_capacity(count);
    for _ in 0..count {
        if off + 2 > buf.len() {
            return None;
        }
        let len = u16::from_be_bytes(buf[off..off + 2].try_into().ok()?) as usize;
        off += 2;
        if off + len > buf.len() {
            return None;
        }
        msgs.push(buf[off..off + len].to_vec());
        off += len;
    }
    Some((session, sequence, msgs))
}

pub fn snapshot_to_levels(snap: &OrderBookSnapshot) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for (i, lvl) in snap.bids.iter().enumerate() {
        out.push(encode_price_level(
            &snap.symbol,
            Side::Buy,
            lvl.price.0,
            lvl.qty.0.max(0.0) as u32,
            i as u8,
            b'S',
        ));
    }
    for (i, lvl) in snap.asks.iter().enumerate() {
        out.push(encode_price_level(
            &snap.symbol,
            Side::Sell,
            lvl.price.0,
            lvl.qty.0.max(0.0) as u32,
            i as u8,
            b'S',
        ));
    }
    out
}

pub fn delta_to_levels(delta: &OrderBookDelta) -> Vec<Vec<u8>> {
    delta
        .updates
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let qty = u.qty.0.max(0.0) as u32;
            let action = match u.action {
                MarketDataAction::Delete => b'D',
                MarketDataAction::New => b'S',
                MarketDataAction::Change => {
                    if qty == 0 {
                        b'D'
                    } else {
                        b'U'
                    }
                }
            };
            encode_price_level(&delta.symbol, u.side, u.price.0, qty, i as u8, action)
        })
        .collect()
}

fn frames_to_itch(frame: &Frame, seen_dir: &mut HashSet<String>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    if let Ok(snap) = decode_cbor::<OrderBookSnapshot>(&frame.payload) {
        if seen_dir.insert(snap.symbol.clone()) {
            out.push(encode_stock_directory(&snap.symbol));
        }
        out.extend(snapshot_to_levels(&snap));
    } else if let Ok(delta) = decode_cbor::<OrderBookDelta>(&frame.payload) {
        out.extend(delta_to_levels(&delta));
    } else if let Ok(trade) = decode_cbor::<PublicTradeEvent>(&frame.payload) {
        out.push(encode_trade(
            &trade.trade.symbol,
            trade.trade.price.0,
            trade.trade.qty.0.max(0.0) as u32,
            trade.trade.side,
        ));
    } else if let Ok(session) = decode_cbor::<TradingSessionStatus>(&frame.payload) {
        let ev = match session.status {
            TradingSessionStatusStatus::Open | TradingSessionStatusStatus::Auction => b'S',
            TradingSessionStatusStatus::Halt => b'H',
            TradingSessionStatusStatus::Close => b'C',
            _ => b'O',
        };
        out.push(encode_system_event(ev));
    }
    out
}

/// SoupBin ITCH snapshot/replay plus optional MoldUDP64 fan-out.
pub async fn run_itch_gateway(
    soup_addr: Option<SocketAddr>,
    multicast: Option<SocketAddr>,
    fig_backend: Option<SocketAddr>,
) -> anyhow::Result<()> {
    let udp = if let Some(dest) = multicast {
        let sock = UdpSocket::bind("0.0.0.0:0").await?;
        Some((Arc::new(sock), dest))
    } else {
        None
    };

    if let Some(addr) = soup_addr {
        let listener = TcpListener::bind(addr).await?;
        info!("ITCH/SoupBin gateway listening on {}", addr);
        if let Some((_, dest)) = &udp {
            info!("ITCH MoldUDP64 egress to {}", dest);
        }
        loop {
            let (stream, peer) = listener.accept().await?;
            let backend = fig_backend;
            let udp = udp.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_itch_connection(stream, backend, udp).await {
                    warn!("ITCH connection from {} failed: {}", peer, e);
                }
            });
        }
    } else if let Some((sock, dest)) = udp {
        info!("ITCH MoldUDP64-only egress to {}", dest);
        // Without SoupBin clients, still subscribe once and fan out.
        if let Some(backend) = fig_backend {
            mold_only_loop(sock, dest, backend).await?;
        } else {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let hb = encode_moldudp64("FIGITCH01", 0, &[]);
                let _ = sock.send_to(&hb, dest).await;
            }
        }
    }
    Ok(())
}

async fn mold_only_loop(
    sock: Arc<UdpSocket>,
    dest: SocketAddr,
    backend: SocketAddr,
) -> anyhow::Result<()> {
    let mut rx = subscribe_md(backend).await?;
    let mut seq = 1u64;
    let mut seen = HashSet::new();
    while let Some(frame) = rx.recv().await {
        for payload in frames_to_itch(&frame, &mut seen) {
            let dgram = encode_moldudp64("FIGITCH01", seq, &[&payload]);
            sock.send_to(&dgram, dest).await?;
            seq += 1;
        }
    }
    Ok(())
}

async fn subscribe_md(
    backend: SocketAddr,
) -> anyhow::Result<tokio::sync::mpsc::UnboundedReceiver<Frame>> {
    let session = BackendSession::connect(backend).await?;
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let symbols = ["AAPL", "BTC", "ETH", "BTCUSDT"];
    let mut ch = 1u16;
    for sym in symbols {
        for suffix in ["book", "trades"] {
            let path = format!("marketdata/{sym}/{suffix}");
            let frame = Frame::new(FrameType::Subscribe, ch)
                .with_seq(1)
                .with_extension(Extension::text(ExtensionTag::ChannelPath, &path))
                .with_extension(Extension::text(ExtensionTag::RoutingKey, &path));
            session.send_frame(&frame).await?;
            let conn = session.connection();
            let tx = tx.clone();
            let id = ch;
            tokio::spawn(async move {
                while let Ok(frame) = conn.recv_frame(id).await {
                    if tx.send(frame).is_err() {
                        break;
                    }
                }
            });
            ch = ch.saturating_add(1);
        }
    }
    let sess = Frame::new(FrameType::Subscribe, ch)
        .with_seq(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/session",
        ))
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata/session",
        ));
    session.send_frame(&sess).await?;
    let conn = session.connection();
    let tx_sess = tx;
    tokio::spawn(async move {
        while let Ok(frame) = conn.recv_frame(ch).await {
            if tx_sess.send(frame).is_err() {
                break;
            }
        }
    });
    let _keep = session;
    Ok(rx)
}

async fn handle_itch_connection(
    mut stream: TcpStream,
    backend: Option<SocketAddr>,
    udp: Option<(Arc<UdpSocket>, SocketAddr)>,
) -> anyhow::Result<()> {
    let mut decoder = SoupDecoder::new();
    let mut buf = vec![0u8; 65536];
    let mut logged_in = false;
    let mut hb = tokio::time::interval(Duration::from_secs(1));
    let mut seq = 1u64;
    let mut seen_dir = HashSet::new();

    let mut md_rx = if let Some(addr) = backend {
        match subscribe_md(addr).await {
            Ok(rx) => Some(rx),
            Err(e) => {
                warn!("ITCH backend subscribe failed: {e}");
                None
            }
        }
    } else {
        None
    };

    async fn emit(
        stream: &mut TcpStream,
        udp: &Option<(Arc<UdpSocket>, SocketAddr)>,
        seq: &mut u64,
        payload: &[u8],
    ) -> anyhow::Result<()> {
        stream.write_all(&encode_sequenced(payload)).await?;
        if let Some((sock, dest)) = udp {
            let dgram = encode_moldudp64("FIGITCH01", *seq, &[payload]);
            sock.send_to(&dgram, *dest).await?;
        }
        *seq += 1;
        Ok(())
    }

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
                            let (_u, _p, session, _) = parse_login_request(&pkt.payload);
                            let sess = if session.is_empty() { "ITCH" } else { session.as_str() };
                            stream.write_all(&encode_login_accepted(sess, 1)).await?;
                            logged_in = true;
                            emit(&mut stream, &udp, &mut seq, &encode_system_event(b'O')).await?;
                        }
                        LOGOUT_REQUEST => return Ok(()),
                        CLIENT_HEARTBEAT => {}
                        _ => {}
                    }
                }
            }
            Some(frame) = async {
                match md_rx.as_mut() {
                    Some(rx) => rx.recv().await,
                    None => None,
                }
            }, if logged_in && md_rx.is_some() => {
                for payload in frames_to_itch(&frame, &mut seen_dir) {
                    emit(&mut stream, &udp, &mut seq, &payload).await?;
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
    use fig_core::messages::{Price, PriceLevel, Quantity};

    #[test]
    fn mold_round_trip() {
        let l = encode_price_level("AAPL", Side::Buy, 10.5, 3, 0, b'S');
        let dgram = encode_moldudp64("FIGITCH01", 9, &[&l]);
        let (sess, seq, msgs) = decode_moldudp64(&dgram).unwrap();
        assert_eq!(sess, "FIGITCH01");
        assert_eq!(seq, 9);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0][0], MSG_LEVEL);
    }

    #[test]
    fn snapshot_emits_levels() {
        let snap = OrderBookSnapshot {
            symbol: "AAPL".to_string(),
            exchange: "SIM".to_string(),
            bids: vec![PriceLevel {
                price: Price(10.0),
                qty: Quantity(5.0),
                order_count: None,
            }],
            asks: vec![PriceLevel {
                price: Price(11.0),
                qty: Quantity(2.0),
                order_count: None,
            }],
            timestamp: 1,
            sequence: Some(1),
            is_snapshot: Some(true),
        };
        let lvls = snapshot_to_levels(&snap);
        assert_eq!(lvls.len(), 2);
        assert_eq!(lvls[0][0], MSG_LEVEL);
    }
}
