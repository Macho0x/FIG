use std::sync::atomic::{AtomicU16, Ordering};
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use fig_core::frame::Frame;
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};
use tokio::runtime::Runtime;

/// Cold start: new server + connect + one ping/pong per iteration (handshake cost included).
fn tree_ping_pong_cold_start(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config().unwrap();

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

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

/// Steady state: persistent connection; only ping/pong measured inside the loop.
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
        let conn = server.accept().await.unwrap();
        loop {
            if let Ok((ch, frame)) = conn.accept_frame().await {
                if frame.control_subtype() == Some(fig_core::frame::ControlSubtype::Ping) {
                    let _ = conn.send_frame(ch, &Frame::pong()).await;
                }
            } else {
                break;
            }
        }
    });

    let conn = rt.block_on(async {
        let client = FigClient::new(client_cfg).unwrap();
        client.connect(addr, "localhost").await.unwrap()
    });

    // Warm up the connection before Criterion samples.
    let mut channel = 1u16;
    for _ in 0..100 {
        let ch = channel;
        channel = channel.wrapping_add(1);
        if channel == 0 {
            channel = 1;
        }
        rt.block_on(async {
            conn.send_frame(ch, &Frame::ping()).await.unwrap();
            conn.recv_frame(ch).await.unwrap();
        });
    }

    static BENCH_CHANNEL: AtomicU16 = AtomicU16::new(200);

    let mut group = c.benchmark_group("transport");
    group.sampling_mode(SamplingMode::Flat);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("tree_ping_pong_steady_state", |b| {
        b.iter(|| {
            let ch = BENCH_CHANNEL.fetch_add(1, Ordering::Relaxed);
            rt.block_on(async {
                conn.send_frame(ch, &Frame::ping()).await.unwrap();
                let pong = conn.recv_frame(ch).await.unwrap();
                black_box(pong);
            });
        });
    });

    group.finish();
}

criterion_group!(benches, tree_ping_pong_cold_start, tree_ping_pong_steady_state);
criterion_main!(benches);
