use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fig_core::frame::{Frame, FrameType};
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::runtime::Runtime;

fn tree_round_trip(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config().unwrap();

    c.bench_function("tree_ping_pong_round_trip", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg.clone())
                    .await
                    .unwrap();
                let addr = server.local_addr().unwrap();

                let server_handle = tokio::spawn(async move {
                    let conn = server.accept().await.unwrap();
                    let (_ch, frame) = conn.accept_frame().await.unwrap();
                    black_box(frame);
                    conn.send_frame(0, &Frame::pong()).await.unwrap();
                });

                let client = FigClient::new(client_cfg.clone()).unwrap();
                let conn = client.connect(addr, "localhost").await.unwrap();
                let ch = conn
                    .open_channel(fig_core::channel::ChannelMode::Stateless, None)
                    .await
                    .unwrap();
                conn.send_frame(ch, &Frame::ping()).await.unwrap();
                let pong = conn.recv_frame(ch).await.unwrap();
                black_box(pong);

                server_handle.await.unwrap();
            });
        });
    });
}

fn tree_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let (cert, key) = generate_self_signed_cert().unwrap();
    let server_cfg = server_config(cert, key).unwrap();
    let client_cfg = client_config().unwrap();

    let mut group = c.benchmark_group("tree_throughput");
    group.throughput(Throughput::Elements(100));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("100_frames", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = FigServer::bind("127.0.0.1:0".parse().unwrap(), server_cfg.clone())
                    .await
                    .unwrap();
                let addr: SocketAddr = server.local_addr().unwrap();

                let server_handle = tokio::spawn(async move {
                    let conn = server.accept().await.unwrap();
                    let (ch, _first) = conn.accept_frame().await.unwrap();
                    for _ in 0..99 {
                        let frame = conn.recv_frame(ch).await.unwrap();
                        black_box(frame);
                    }
                });

                let client = FigClient::new(client_cfg.clone()).unwrap();
                let conn = client.connect(addr, "localhost").await.unwrap();
                let ch = conn
                    .open_channel(fig_core::channel::ChannelMode::Stateless, None)
                    .await
                    .unwrap();
                let frame = Frame::new(FrameType::Request, ch).with_payload(b"x".repeat(64));
                for _ in 0..100 {
                    conn.send_frame(ch, &frame).await.unwrap();
                }

                server_handle.await.unwrap();
            });
        });
    });

    group.finish();
}

criterion_group!(benches, tree_round_trip, tree_throughput);
criterion_main!(benches);
