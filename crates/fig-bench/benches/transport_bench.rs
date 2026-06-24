use std::cell::Cell;

use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use fig_bench::criterion_config::criterion_slow;
use fig_core::frame::Frame;
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};
use tokio::runtime::Runtime;

const MAX_STREAMS_PER_CONN: u16 = 64;

/// Cold start: new server + connect + one ping/pong per iteration (handshake cost included).
fn tree_ping_pong_cold_start(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config().unwrap();

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);

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
    let client_cfg = client_config().unwrap();

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
    let client_cfg = client_config().unwrap();

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

criterion_group! {
    name = benches;
    config = criterion_slow();
    targets = tree_ping_pong_cold_start,
        tree_ping_pong_steady_state,
        tree_ping_pong_throughput,
}
criterion_main!(benches);
