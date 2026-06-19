use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fig_core::frame::Frame;
use fig_core::transport::{
    client_config, generate_self_signed_cert, server_config, FigClient, FigServer,
};
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
}

criterion_group!(benches, tree_round_trip);
criterion_main!(benches);
