//! Sustained throughput harness (msgs/sec).
//!
//! Criterion reports mean time per iteration. [`crate::latency`] reports
//! per-operation percentiles. This module fills the remaining gap: wall-clock
//! **operations per second** over a fixed duration, which is what the
//! throughput TODO row asked for.
//!
//! Set `FIG_THROUGHPUT_SECS` (default 3) and `FIG_THROUGHPUT_WARMUP_MS`
//! (default 200). CI smoke uses `FIG_THROUGHPUT_SECS=1`.
//!
//! TREE scenarios lift [`fig_core::dos::DoSGuard`] / per-channel rate limits
//! so the numbers measure the codec and QUIC path, not the default 10_000
//! msgs/sec/channel cap. Production connections keep that cap.

use std::hint::black_box;
use std::sync::Arc;
use std::time::{Duration, Instant};

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::dos::DoSGuard;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    CancelRequest, NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce,
};
use fig_core::rate_limit::ChannelRateLimiter;
use fig_core::sbe::{decode_new_order_single, encode_new_order_single};
use fig_core::transport::{
    client_config_insecure, generate_self_signed_cert, server_config, FigClient, FigConnection,
    FigServer,
};
use fig_exchange_sim::matching::MatchingEngine;
use fig_gateways::fix::{parse_fix_message, serialize_fix_message};
use tokio::runtime::Runtime;

/// Inner CPU ops between clock checks so `Instant::now` is not on the 300 ns path.
const CPU_BATCH: u64 = 1_024;

/// Resolve measurement duration from `FIG_THROUGHPUT_SECS`.
pub fn throughput_secs() -> Duration {
    Duration::from_secs(resolve_positive_u64(
        3,
        std::env::var("FIG_THROUGHPUT_SECS").ok().as_deref(),
    ))
}

/// Resolve warmup from `FIG_THROUGHPUT_WARMUP_MS`.
pub fn throughput_warmup() -> Duration {
    Duration::from_millis(resolve_positive_u64(
        200,
        std::env::var("FIG_THROUGHPUT_WARMUP_MS").ok().as_deref(),
    ))
}

/// Pure resolution (no environment access) so tests cannot race on process env.
pub(crate) fn resolve_positive_u64(default: u64, raw: Option<&str>) -> u64 {
    raw.and_then(|s| s.parse().ok())
        .filter(|&n: &u64| n > 0)
        .unwrap_or(default)
}

/// One scenario's wall-clock result.
#[derive(Debug, Clone)]
pub struct ThroughputReport {
    pub name: &'static str,
    pub ops: u64,
    pub elapsed: Duration,
    pub notes: &'static str,
}

impl ThroughputReport {
    pub fn msgs_per_sec(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs <= 0.0 {
            0.0
        } else {
            self.ops as f64 / secs
        }
    }

    pub fn print_row(&self) {
        println!(
            "  {:<32} {:>12} ops  {:>12.0} /s    {}",
            self.name,
            self.ops,
            self.msgs_per_sec(),
            self.notes
        );
    }
}

fn run_cpu_loop(warmup: Duration, measure: Duration, mut op: impl FnMut()) -> (u64, Duration) {
    let warm_until = Instant::now() + warmup;
    while Instant::now() < warm_until {
        for _ in 0..CPU_BATCH {
            op();
        }
    }

    let start = Instant::now();
    let until = start + measure;
    let mut ops = 0u64;
    while Instant::now() < until {
        for _ in 0..CPU_BATCH {
            op();
            ops += 1;
        }
    }
    (ops, start.elapsed())
}

fn make_order() -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: "ORD-THR-001".to_string(),
        side: Side::Buy,
        order_qty: Quantity(100.0),
        price: Some(Price(150.25)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some("ACCT-123".to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: None,
        reduce_only: None,
    }
}

fn make_request_frame() -> Frame {
    Frame::new(FrameType::Request, 42)
        .with_seq(100)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::RequestUri,
            "/trading/accounts/123/orders",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(b"order payload data here".to_vec())
        .with_priority()
        .with_ack_requested()
}

fn make_fix_new_order_bytes() -> Vec<u8> {
    let tags = vec![
        (8u32, "FIX.4.4".to_string()),
        (9, "200".to_string()),
        (35, "D".to_string()),
        (49, "SENDER".to_string()),
        (56, "TARGET".to_string()),
        (34, "1".to_string()),
        (52, "20240101-00:00:00".to_string()),
        (11, "ORD-001".to_string()),
        (21, "1".to_string()),
        (55, "AAPL".to_string()),
        (54, "1".to_string()),
        (38, "100".to_string()),
        (44, "150.25".to_string()),
        (40, "2".to_string()),
        (59, "0".to_string()),
        (1, "ACCT-123".to_string()),
    ];
    serialize_fix_message(&tags)
}

/// Lift inbound DoS / channel caps so TREE benches measure the path, not the limiter.
fn lift_dos(conn: FigConnection) -> FigConnection {
    conn.with_limits(
        Arc::new(DoSGuard::new(65_535, 1024 * 1024, u64::MAX / 4)),
        Arc::new(ChannelRateLimiter::new(u64::MAX / 4)),
    )
}

pub fn frame_encode(warmup: Duration, measure: Duration) -> ThroughputReport {
    let frame = make_request_frame();
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(frame.encode().expect("encode"));
    });
    ThroughputReport {
        name: "frame_encode",
        ops,
        elapsed,
        notes: "16-byte header + TLV + payload",
    }
}

pub fn frame_decode(warmup: Duration, measure: Duration) -> ThroughputReport {
    let encoded = make_request_frame().encode().expect("encode");
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        let (decoded, _) = Frame::decode(black_box(&encoded)).expect("decode");
        black_box(decoded);
    });
    ThroughputReport {
        name: "frame_decode",
        ops,
        elapsed,
        notes: "Frame::decode of a Request",
    }
}

pub fn sbe_encode_new_order(warmup: Duration, measure: Duration) -> ThroughputReport {
    let order = make_order();
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(encode_new_order_single(black_box(&order)));
    });
    ThroughputReport {
        name: "sbe_encode_new_order",
        ops,
        elapsed,
        notes: "NewOrderSingle SBE encode",
    }
}

pub fn sbe_decode_new_order(warmup: Duration, measure: Duration) -> ThroughputReport {
    let encoded = encode_new_order_single(&make_order());
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(decode_new_order_single(black_box(&encoded)).expect("decode"));
    });
    ThroughputReport {
        name: "sbe_decode_new_order",
        ops,
        elapsed,
        notes: "NewOrderSingle SBE decode",
    }
}

pub fn cbor_encode_new_order(warmup: Duration, measure: Duration) -> ThroughputReport {
    let order = make_order();
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(encode_cbor(black_box(&order)).expect("encode"));
    });
    ThroughputReport {
        name: "cbor_encode_new_order",
        ops,
        elapsed,
        notes: "NewOrderSingle CBOR encode",
    }
}

pub fn cbor_decode_new_order(warmup: Duration, measure: Duration) -> ThroughputReport {
    let encoded = encode_cbor(&make_order()).expect("encode");
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        let decoded: NewOrderSingle = decode_cbor(black_box(&encoded)).expect("decode");
        black_box(decoded);
    });
    ThroughputReport {
        name: "cbor_decode_new_order",
        ops,
        elapsed,
        notes: "NewOrderSingle CBOR decode",
    }
}

pub fn fix_parse_new_order(warmup: Duration, measure: Duration) -> ThroughputReport {
    let msg = make_fix_new_order_bytes();
    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(parse_fix_message(black_box(&msg)).expect("parse"));
    });
    ThroughputReport {
        name: "fix_parse_new_order",
        ops,
        elapsed,
        notes: "in-repo FIX 4.4 NewOrderSingle parse",
    }
}

/// Non-crossing limit insert + cancel so the book stays at a fixed depth.
pub fn matching_insert_cancel(warmup: Duration, measure: Duration) -> ThroughputReport {
    let mut engine = MatchingEngine::new();
    for i in 0..1_000 {
        let sell = NewOrderSingle {
            cl_ord_id: format!("ASK-{i}"),
            side: Side::Sell,
            order_qty: Quantity(10.0),
            price: Some(Price(150.0 + i as f64 * 0.01)),
            stop_price: None,
            symbol: "AAPL".to_string(),
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            expire_time: None,
            account: None,
            strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
            post_only: None,
            reduce_only: None,
        };
        engine.process_new_order(&sell);
    }

    let order = NewOrderSingle {
        cl_ord_id: "THR-BID".to_string(),
        side: Side::Buy,
        order_qty: Quantity(10.0),
        price: Some(Price(149.99)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: None,
        reduce_only: None,
    };
    let cancel = CancelRequest {
        cl_ord_id: "THR-CXL".to_string(),
        orig_cl_ord_id: "THR-BID".to_string(),
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        order_qty: None,
    };

    let (ops, elapsed) = run_cpu_loop(warmup, measure, || {
        black_box(engine.process_new_order(black_box(&order)));
        black_box(engine.process_cancel(black_box(&cancel)));
    });
    ThroughputReport {
        name: "matching_insert_cancel",
        ops,
        elapsed,
        notes: "1 insert + 1 cancel; 1000-level ask book",
    }
}

fn setup_persistent_ping_server(
    rt: &Runtime,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let (cert, key) = generate_self_signed_cert().expect("cert");
    let server_cfg = server_config(cert, key).expect("server config");
    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .expect("bind");
    let addr = server.local_addr().expect("addr");

    let handle = rt.spawn(async move {
        while let Ok(conn) = server.accept().await {
            let conn = Arc::new(lift_dos(conn));
            tokio::spawn(async move {
                loop {
                    let (ch, frame) = match conn.accept_frame().await {
                        Ok(pair) => pair,
                        Err(_) => break,
                    };
                    if frame.control_subtype() == Some(fig_core::frame::ControlSubtype::Ping) {
                        let _ = conn.send_frame(ch, &Frame::pong()).await;
                    }
                    let conn_ch = Arc::clone(&conn);
                    tokio::spawn(async move {
                        while let Ok(frame) = conn_ch.recv_frame(ch).await {
                            if frame.control_subtype()
                                == Some(fig_core::frame::ControlSubtype::Ping)
                            {
                                let _ = conn_ch.send_frame(ch, &Frame::pong()).await;
                            }
                        }
                    });
                }
            });
        }
    });

    (addr, handle)
}

fn setup_new_stream_ping_server(
    rt: &Runtime,
) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let (cert, key) = generate_self_signed_cert().expect("cert");
    let server_cfg = server_config(cert, key).expect("server config");
    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .expect("bind");
    let addr = server.local_addr().expect("addr");

    let handle = rt.spawn(async move {
        while let Ok(conn) = server.accept().await {
            let conn = lift_dos(conn);
            tokio::spawn(async move {
                while let Ok((ch, frame)) = conn.accept_frame().await {
                    if frame.control_subtype() == Some(fig_core::frame::ControlSubtype::Ping) {
                        let _ = conn.send_frame(ch, &Frame::pong()).await;
                    }
                }
            });
        }
    });

    (addr, handle)
}

fn connect_lifted(rt: &Runtime, addr: std::net::SocketAddr) -> FigConnection {
    let client_cfg = client_config_insecure().expect("client config");
    rt.block_on(async {
        let client = FigClient::new(client_cfg).expect("client");
        lift_dos(client.connect(addr, "localhost").await.expect("connect"))
    })
}

async fn recv_frame_timed(conn: &FigConnection, ch: u16) -> Frame {
    tokio::time::timeout(Duration::from_secs(2), conn.recv_frame(ch))
        .await
        .unwrap_or_else(|_| panic!("TREE recv timeout on channel {ch}"))
        .expect("recv")
}

/// Sequential ping/pong on **one** persistent channel (round-trips/sec ≈ 1/RTT).
pub fn tree_round_trip_persistent(warmup: Duration, measure: Duration) -> ThroughputReport {
    let rt = Runtime::new().expect("runtime");
    let (addr, _server) = setup_persistent_ping_server(&rt);
    let conn = connect_lifted(&rt, addr);

    let ping = Frame::ping();
    let ch = 1u16;

    let warm_until = Instant::now() + warmup;
    while Instant::now() < warm_until {
        rt.block_on(async {
            conn.send_frame(ch, &ping).await.expect("send");
            black_box(recv_frame_timed(&conn, ch).await);
        });
    }

    let start = Instant::now();
    let until = start + measure;
    let mut ops = 0u64;
    while Instant::now() < until {
        rt.block_on(async {
            conn.send_frame(ch, &ping).await.expect("send");
            black_box(recv_frame_timed(&conn, ch).await);
        });
        ops += 1;
    }

    ThroughputReport {
        name: "tree_round_trip_persistent",
        ops,
        elapsed: start.elapsed(),
        notes: "1 channel reused; DoS caps lifted; localhost",
    }
}

const PIPELINE: u64 = 32;

/// Pipeline 32 pings then 32 pongs on one persistent channel (not 1/RTT).
pub fn tree_pipeline_persistent(warmup: Duration, measure: Duration) -> ThroughputReport {
    let rt = Runtime::new().expect("runtime");
    let (addr, _server) = setup_persistent_ping_server(&rt);
    let conn = connect_lifted(&rt, addr);
    let ping = Frame::ping();
    let ch = 1u16;

    let burst = || {
        rt.block_on(async {
            for _ in 0..PIPELINE {
                conn.send_frame(ch, &ping).await.expect("send");
            }
            for _ in 0..PIPELINE {
                black_box(recv_frame_timed(&conn, ch).await);
            }
        });
    };

    let warm_until = Instant::now() + warmup;
    while Instant::now() < warm_until {
        burst();
    }

    let start = Instant::now();
    let until = start + measure;
    let mut ops = 0u64;
    while Instant::now() < until {
        burst();
        ops += PIPELINE;
    }

    ThroughputReport {
        name: "tree_pipeline_persistent_x32",
        ops,
        elapsed: start.elapsed(),
        notes: "32 in-flight pings on 1 channel; localhost",
    }
}

const MAX_STREAMS_PER_CONN: u16 = 64;

/// New bidirectional stream per ping (stream-open + RTT).
pub fn tree_round_trip_new_stream(warmup: Duration, measure: Duration) -> ThroughputReport {
    let rt = Runtime::new().expect("runtime");
    let (addr, _server) = setup_new_stream_ping_server(&rt);
    let mut conn = connect_lifted(&rt, addr);
    let ping = Frame::ping();
    let mut channel = 1u16;
    let mut streams_on_conn = 0u16;

    let reconnect = |conn: &mut FigConnection, channel: &mut u16, streams: &mut u16| {
        if *streams >= MAX_STREAMS_PER_CONN {
            *conn = connect_lifted(&rt, addr);
            *channel = 1;
            *streams = 0;
        }
    };

    let warm_until = Instant::now() + warmup;
    while Instant::now() < warm_until {
        reconnect(&mut conn, &mut channel, &mut streams_on_conn);
        let ch = channel;
        channel = channel.wrapping_add(1).max(1);
        streams_on_conn = streams_on_conn.saturating_add(1);
        rt.block_on(async {
            conn.send_frame(ch, &ping).await.expect("send");
            black_box(recv_frame_timed(&conn, ch).await);
        });
    }

    let start = Instant::now();
    let until = start + measure;
    let mut ops = 0u64;
    while Instant::now() < until {
        reconnect(&mut conn, &mut channel, &mut streams_on_conn);
        let ch = channel;
        channel = channel.wrapping_add(1).max(1);
        streams_on_conn = streams_on_conn.saturating_add(1);
        rt.block_on(async {
            conn.send_frame(ch, &ping).await.expect("send");
            black_box(recv_frame_timed(&conn, ch).await);
        });
        ops += 1;
    }

    ThroughputReport {
        name: "tree_round_trip_new_stream",
        ops,
        elapsed: start.elapsed(),
        notes: "new bidi stream per ping; localhost",
    }
}

/// CPU + matching only (no TREE). Used by unit tests.
pub fn run_cpu(warmup: Duration, measure: Duration) -> Vec<ThroughputReport> {
    vec![
        frame_encode(warmup, measure),
        frame_decode(warmup, measure),
        sbe_encode_new_order(warmup, measure),
        sbe_decode_new_order(warmup, measure),
        cbor_encode_new_order(warmup, measure),
        cbor_decode_new_order(warmup, measure),
        fix_parse_new_order(warmup, measure),
        matching_insert_cancel(warmup, measure),
    ]
}

/// Print every scenario (CPU, matching, TREE).
pub fn run_all() {
    let warmup = throughput_warmup();
    let measure = throughput_secs();

    println!("FIG throughput harness (msgs/sec, release profile recommended)");
    println!(
        "measure: {:.3}s   warmup: {:.3}s   (FIG_THROUGHPUT_SECS / FIG_THROUGHPUT_WARMUP_MS)\n",
        measure.as_secs_f64(),
        warmup.as_secs_f64()
    );
    println!("CPU / codec");
    for report in run_cpu(warmup, measure) {
        report.print_row();
    }

    println!("\nTREE (localhost loopback; DoS caps lifted for the ceiling)");
    tree_round_trip_persistent(warmup, measure).print_row();
    tree_pipeline_persistent(warmup, measure).print_row();
    tree_round_trip_new_stream(warmup, measure).print_row();

    println!("\nDefault DoSGuard remains 10_000 msgs/sec/channel and 100_000/conn in production.");
    println!("Do not cite localhost TREE as a colo SLA. Re-run on the target host.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_u64_resolution() {
        assert_eq!(resolve_positive_u64(3, None), 3);
        assert_eq!(resolve_positive_u64(3, Some("")), 3);
        assert_eq!(resolve_positive_u64(3, Some("0")), 3);
        assert_eq!(resolve_positive_u64(3, Some("nope")), 3);
        assert_eq!(resolve_positive_u64(3, Some("2")), 2);
    }

    #[test]
    fn msgs_per_sec_is_ops_over_wall_seconds() {
        let report = ThroughputReport {
            name: "t",
            ops: 3_000,
            elapsed: Duration::from_secs(3),
            notes: "",
        };
        assert!((report.msgs_per_sec() - 1_000.0).abs() < f64::EPSILON);
        let zero = ThroughputReport {
            name: "z",
            ops: 1,
            elapsed: Duration::ZERO,
            notes: "",
        };
        assert_eq!(zero.msgs_per_sec(), 0.0);
    }

    #[test]
    fn cpu_scenarios_record_ops() {
        let warmup = Duration::ZERO;
        let measure = Duration::from_millis(8);
        let reports = run_cpu(warmup, measure);
        assert_eq!(reports.len(), 8);
        for r in reports {
            assert!(r.ops > 0, "{} produced no ops", r.name);
            assert!(r.msgs_per_sec() > 0.0, "{} rate was 0", r.name);
        }
    }

    #[test]
    fn tree_persistent_records_ops() {
        let report = tree_round_trip_persistent(Duration::ZERO, Duration::from_millis(250));
        assert!(
            report.ops > 0,
            "persistent TREE ping/pong produced no ops in 250ms"
        );
    }
}
