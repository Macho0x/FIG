//! Tail-latency harness using HDR Histogram.
//!
//! Records per-operation wall-clock samples and reports p50/p90/p99/p99.9.
//! Set `FIG_LATENCY_ITERS` to override default iteration counts (e.g. `1000` in CI).

use std::sync::Arc;
use std::time::Instant;

use fig_core::frame::Frame;
use fig_core::messages::{NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce};
use fig_core::sbe::{decode_new_order_single, encode_new_order_single};
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};
use hdrhistogram::Histogram;
use tokio::runtime::Runtime;

/// Resolve iteration count from `FIG_LATENCY_ITERS` or the per-test default.
pub fn latency_iters(default: u64) -> u64 {
    std::env::var("FIG_LATENCY_ITERS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n| n > 0)
        .unwrap_or(default)
}

/// Aggregated latency distribution with percentile reporting.
pub struct LatencyReport {
    name: String,
    hist: Histogram<u64>,
}

impl LatencyReport {
    /// Create a histogram for nanosecond samples up to one minute.
    pub fn new_ns(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            hist: Histogram::<u64>::new_with_bounds(1, 60_000_000_000, 3)
                .expect("valid histogram bounds"),
        }
    }

    pub fn record_ns(&mut self, nanos: u64) {
        let _ = self.hist.record(nanos);
    }

    pub fn len(&self) -> u64 {
        self.hist.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hist.is_empty()
    }

    pub fn merge_histogram(&mut self, other: &Histogram<u64>) {
        self.hist.add(other).expect("merge histogram");
    }

    pub fn print_summary(&self) {
        if self.hist.is_empty() {
            println!("=== {} === (no samples)", self.name);
            return;
        }

        println!("=== {} ===", self.name);
        println!("samples: {}", self.hist.len());
        print_ns("min", self.hist.min());
        print_ns("mean", self.hist.mean() as u64);
        print_ns("max", self.hist.max());
        for (label, q) in [
            ("p50", 0.50),
            ("p90", 0.90),
            ("p99", 0.99),
            ("p99.9", 0.999),
            ("p99.99", 0.9999),
        ] {
            print_ns(label, self.hist.value_at_quantile(q));
        }
        println!();
    }
}

fn print_ns(label: &str, nanos: u64) {
    println!(
        "{label:>7}: {nanos:>12} ns  ({:.3} µs)",
        nanos as f64 / 1_000.0
    );
}

fn make_sbe_order_bytes() -> Vec<u8> {
    let order = NewOrderSingle {
        cl_ord_id: "ORD-LAT-001".to_string(),
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
    };
    encode_new_order_single(&order)
}

/// SBE decode hot path — one sample per decode, default 100k iterations.
pub fn sbe_decode_hot_path() -> LatencyReport {
    let iters = latency_iters(100_000);
    let encoded = make_sbe_order_bytes();
    let mut report = LatencyReport::new_ns("sbe_decode_hot_path");

    for _ in 0..iters {
        let t0 = Instant::now();
        let decoded = decode_new_order_single(&encoded).expect("decode");
        std::hint::black_box(decoded);
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

fn setup_ping_pong_server(rt: &Runtime) -> (std::net::SocketAddr, tokio::task::JoinHandle<()>) {
    let (cert, key) = generate_self_signed_cert().expect("cert");
    let server_cfg = server_config(cert, key).expect("server config");
    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .expect("bind");
    let addr = server.local_addr().expect("addr");

    let handle = rt.spawn(async move {
        while let Ok(conn) = server.accept().await {
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

fn connect_client(rt: &Runtime, addr: std::net::SocketAddr) -> fig_core::transport::FigConnection {
    let client_cfg = client_config().expect("client config");
    rt.block_on(async {
        let client = FigClient::new(client_cfg).expect("client");
        client.connect(addr, "localhost").await.expect("connect")
    })
}

const MAX_STREAMS_PER_CONN: u16 = 128;

fn maybe_reconnect(
    rt: &Runtime,
    addr: std::net::SocketAddr,
    conn: &mut fig_core::transport::FigConnection,
    channel: &mut u16,
    streams_on_conn: &mut u16,
) {
    if *streams_on_conn >= MAX_STREAMS_PER_CONN {
        *conn = connect_client(rt, addr);
        *channel = 1;
        *streams_on_conn = 0;
    }
}

fn next_ping_channel(counter: &mut u16) -> u16 {
    let ch = *counter;
    *counter = counter.wrapping_add(1);
    if *counter == 0 {
        *counter = 1;
    }
    ch
}

/// Steady-state TREE ping/pong on a persistent connection (default 10k samples).
///
/// Each sample uses a new channel ID (new bidirectional stream) on the same QUIC
/// connection — matching the server's `accept_frame` loop.
pub fn tree_round_trip_steady_state() -> LatencyReport {
    let iters = latency_iters(10_000);
    let warmup = iters.min(1_000);
    let mut report = LatencyReport::new_ns("tree_round_trip_steady_state");

    let rt = Runtime::new().expect("runtime");
    let (addr, _server_task) = setup_ping_pong_server(&rt);
    let mut conn = connect_client(&rt, addr);
    let mut channel = 1u16;
    let mut streams_on_conn = 0u16;

    for _ in 0..warmup {
        maybe_reconnect(&rt, addr, &mut conn, &mut channel, &mut streams_on_conn);
        let ch = next_ping_channel(&mut channel);
        streams_on_conn = streams_on_conn.saturating_add(1);
        rt.block_on(async {
            conn.send_frame(ch, &Frame::ping()).await.expect("send");
            conn.recv_frame(ch).await.expect("recv");
        });
    }

    for _ in 0..iters {
        maybe_reconnect(&rt, addr, &mut conn, &mut channel, &mut streams_on_conn);
        let ch = next_ping_channel(&mut channel);
        streams_on_conn = streams_on_conn.saturating_add(1);
        let t0 = Instant::now();
        rt.block_on(async {
            conn.send_frame(ch, &Frame::ping()).await.expect("send");
            conn.recv_frame(ch).await.expect("recv");
        });
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

/// Ping/pong on channel 0 while background traffic hammers channel 1 (default 5k samples).
pub fn tree_round_trip_under_load() -> LatencyReport {
    let iters = latency_iters(5_000);
    let warmup = iters.min(500);
    let mut report = LatencyReport::new_ns("tree_round_trip_under_load");

    let rt = Runtime::new().expect("runtime");
    let (addr, _server_task) = setup_ping_pong_server(&rt);
    let mut conn = connect_client(&rt, addr);
    let mut channel = 1u16;
    let mut streams_on_conn = 0u16;

    let load_conn = connect_client(&rt, addr);
    let load_stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let load_stop_bg = Arc::clone(&load_stop);
    rt.spawn(async move {
        let load_conn = load_conn;
        let mut bg_ch = 10_000u16;
        while !load_stop_bg.load(std::sync::atomic::Ordering::Relaxed) {
            bg_ch = bg_ch.wrapping_add(1);
            if bg_ch == 0 {
                bg_ch = 10_000;
            }
            if load_conn.send_frame(bg_ch, &Frame::ping()).await.is_err() {
                break;
            }
            if load_conn.recv_frame(bg_ch).await.is_err() {
                break;
            }
        }
    });

    for _ in 0..warmup {
        maybe_reconnect(&rt, addr, &mut conn, &mut channel, &mut streams_on_conn);
        let ch = next_ping_channel(&mut channel);
        streams_on_conn = streams_on_conn.saturating_add(1);
        rt.block_on(async {
            conn.send_frame(ch, &Frame::ping()).await.expect("send");
            conn.recv_frame(ch).await.expect("recv");
        });
    }

    for _ in 0..iters {
        maybe_reconnect(&rt, addr, &mut conn, &mut channel, &mut streams_on_conn);
        let ch = next_ping_channel(&mut channel);
        streams_on_conn = streams_on_conn.saturating_add(1);
        let t0 = Instant::now();
        rt.block_on(async {
            conn.send_frame(ch, &Frame::ping()).await.expect("send");
            conn.recv_frame(ch).await.expect("recv");
        });
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    load_stop.store(true, std::sync::atomic::Ordering::Relaxed);
    std::thread::sleep(std::time::Duration::from_millis(50));

    report
}

/// Run all tail-latency scenarios and print percentile summaries.
pub fn run_all() {
    use crate::matching_latency::{
        matching_cancel_deep, matching_cancel_top, matching_engine_contended, matching_insert_top,
        matching_replace_qty,
    };

    println!("FIG tail-latency harness (HDR Histogram, release profile recommended)\n");

    sbe_decode_hot_path().print_summary();
    tree_round_trip_steady_state().print_summary();
    tree_round_trip_under_load().print_summary();

    for n in [0_usize, 100, 1_000] {
        matching_insert_top(n).print_summary();
    }
    matching_cancel_top(1_000).print_summary();
    matching_cancel_deep(1_000).print_summary();
    matching_replace_qty(1_000).print_summary();
    matching_engine_contended().print_summary();
}
