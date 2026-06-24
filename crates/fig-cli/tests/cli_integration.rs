//! End-to-end integration test for the fig-cli demo flows.

use fig_cli::{connect, run_demos, run_sbe_order_demo};
use fig_exchange_sim::server::run_server;

#[tokio::test]
async fn cli_demos_complete_successfully() {
    let _ = tracing_subscriber::fmt::try_init();
    std::env::set_var("FIG_DEV_OPEN", "1");

    let endpoint = run_server("127.0.0.1:0").await.expect("server start");
    let server_addr = endpoint.local_addr().unwrap();

    let (_client_ep, conn) = connect(server_addr).await.expect("client connect");
    run_demos(&conn).await.expect("all demos should succeed");

    drop(conn);
    drop(endpoint);
}

#[tokio::test]
async fn cli_sbe_order_demo_successfully() {
    let _ = tracing_subscriber::fmt::try_init();
    std::env::set_var("FIG_DEV_OPEN", "1");

    let endpoint = run_server("127.0.0.1:0").await.expect("server start");
    let server_addr = endpoint.local_addr().unwrap();

    let (_client_ep, conn) = connect(server_addr).await.expect("client connect");
    run_sbe_order_demo(&conn)
        .await
        .expect("SBE order demo should succeed");

    drop(conn);
    drop(endpoint);
}
