use std::cell::Cell;
use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode, Throughput};
use fig_bench::criterion_config::criterion_slow;
use fig_core::dos::DoSGuard;
use fig_core::frame::Frame;
use fig_core::rate_limit::ChannelRateLimiter;
use fig_core::transport::{
    client_config_insecure, generate_self_signed_cert, server_config, FigClient, FigConnection,
    FigServer,
};
use tokio::runtime::Runtime;

fn lift_dos(conn: FigConnection) -> FigConnection {
    conn.with_limits(
        Arc::new(DoSGuard::new(65_535, 1024 * 1024, u64::MAX / 4)),
        Arc::new(ChannelRateLimiter::new(u64::MAX / 4)),
    )
}

const MAX_STREAMS_PER_CONN: u16 = 64;

/// Cold start: new server + connect + one ping/pong per iteration (handshake cost included).
fn tree_ping_pong_cold_start(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config_insecure().unwrap();

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(1));

    group.bench_function("tree_ping_pong_cold_start", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg.clone())
                    .await
                    .unwrap();
                let addr = server.local_addr().unwrap();

                let client_cfg = client_cfg.clone();
                let client_handle = tokio::spawn(async move {
                    let client = FigClient::new(client_cfg).unwrap();
                    let conn = client.connect(addr, "localhost").await.unwrap();
                    conn.send_frame(0, &Frame::ping()).await.unwrap();
                    let pong = conn.recv_frame(0).await.unwrap();
                    black_box(pong);
                });

                let conn = server.accept().await.unwrap();
                let (ch, frame) = conn.accept_frame().await.unwrap();
                black_box(frame);
                conn.send_frame(ch, &Frame::pong()).await.unwrap();
                client_handle.await.unwrap();
            });
        });
    });

    group.finish();
}

/// Steady state: persistent server; reconnect client every N streams to avoid QUIC exhaustion.
fn tree_ping_pong_steady_state(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config_insecure().unwrap();

    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .unwrap();
    let addr = server.local_addr().unwrap();

    rt.spawn(async move {
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

    let conn = std::cell::RefCell::new(rt.block_on(async {
        let client = FigClient::new(client_cfg.clone()).unwrap();
        client.connect(addr, "localhost").await.unwrap()
    }));

    let channel = Cell::new(1u16);
    let streams_on_conn = Cell::new(0u16);

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(1));

    group.bench_function("tree_ping_pong_steady_state", |b| {
        b.iter(|| {
            if streams_on_conn.get() >= MAX_STREAMS_PER_CONN {
                *conn.borrow_mut() = rt.block_on(async {
                    let client = FigClient::new(client_cfg.clone()).unwrap();
                    client.connect(addr, "localhost").await.unwrap()
                });
                channel.set(1);
                streams_on_conn.set(0);
            }

            let ch = channel.get();
            channel.set(ch.wrapping_add(1).max(1));
            streams_on_conn.set(streams_on_conn.get().saturating_add(1));

            rt.block_on(async {
                conn.borrow().send_frame(ch, &Frame::ping()).await.unwrap();
                let pong = conn.borrow().recv_frame(ch).await.unwrap();
                black_box(pong);
            });
        });
    });

    group.finish();
}

/// Steady-state throughput: 16 ping/pong rounds per iteration on one connection.
fn tree_ping_pong_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config_insecure().unwrap();

    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .unwrap();
    let addr = server.local_addr().unwrap();

    rt.spawn(async move {
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

    let conn = rt.block_on(async {
        let client = FigClient::new(client_cfg).unwrap();
        client.connect(addr, "localhost").await.unwrap()
    });

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(criterion::Throughput::Elements(16));

    group.bench_function("tree_ping_pong_throughput_x16", |b| {
        b.iter(|| {
            rt.block_on(async {
                for ch in 1u16..=16 {
                    conn.send_frame(ch, &Frame::ping()).await.unwrap();
                    let pong = conn.recv_frame(ch).await.unwrap();
                    black_box(pong);
                }
            });
        });
    });

    group.finish();
}

/// Persistent-channel throughput: 32 pipelined ping/pongs on one stream.
///
/// The server keeps reading the accepted channel (`recv_frame` after
/// `accept_frame`) so this is not a new-stream-per-message measurement.
fn tree_ping_pong_pipeline_persistent(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config_insecure().unwrap();

    let server = rt
        .block_on(FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg))
        .unwrap();
    let addr = server.local_addr().unwrap();

    rt.spawn(async move {
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

    let conn = rt.block_on(async {
        let client = FigClient::new(client_cfg).unwrap();
        lift_dos(client.connect(addr, "localhost").await.unwrap())
    });

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(32));

    group.bench_function("tree_ping_pong_pipeline_persistent_x32", |b| {
        b.iter(|| {
            rt.block_on(async {
                for _ in 0..32 {
                    conn.send_frame(1, &Frame::ping()).await.unwrap();
                }
                for _ in 0..32 {
                    black_box(conn.recv_frame(1).await.unwrap());
                }
            });
        });
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion_slow();
    targets = tree_ping_pong_cold_start,
        tree_ping_pong_steady_state,
        tree_ping_pong_throughput,
        tree_ping_pong_pipeline_persistent,
}
criterion_main!(benches);
