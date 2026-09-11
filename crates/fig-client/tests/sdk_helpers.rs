//! Smoke tests for FigSdkClient request/subscribe helpers against exchange-sim.

use fig_client::FigSdkClient;
use fig_core::messages::{FillHistoryRequest, OrderHistoryRequest, TradeHistoryRequest};
use fig_core::transport::client_config;
use fig_exchange_sim::server::run_server;

async fn connect_raw(addr: std::net::SocketAddr) -> (quinn::Endpoint, quinn::Connection) {
    let mut ep = quinn::Endpoint::client("127.0.0.1:0".parse().unwrap()).unwrap();
    ep.set_default_client_config(client_config().unwrap());
    let conn = ep.connect(addr, "localhost").unwrap().await.unwrap();
    (ep, conn)
}

#[tokio::test]
async fn sdk_helpers_request_and_subscribe_smoke() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    let server = run_server("127.0.0.1:0").await.expect("sim");
    let addr = server.local_addr().unwrap();
    let (_ep, conn) = connect_raw(addr).await;
    let sdk = FigSdkClient::new(&conn);

    let caps = sdk.request_capabilities(1).await.expect("capabilities");
    assert!(!caps.paths.is_empty());

    let instruments = sdk.request_instruments(2).await.expect("instruments");
    assert!(!instruments.instruments.is_empty());

    let book = sdk.request_book("AAPL", 3).await.expect("book");
    assert_eq!(book.symbol, "AAPL");

    let trades = sdk
        .request_trades(
            TradeHistoryRequest {
                symbol: "AAPL".into(),
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            4,
        )
        .await
        .expect("trades");
    assert_eq!(trades.symbol, "AAPL");

    let _ = sdk
        .request_order_history(
            "TEST",
            OrderHistoryRequest {
                account: "TEST".into(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            5,
        )
        .await
        .expect("order history");

    let fills = sdk
        .request_fills(
            "TEST",
            FillHistoryRequest {
                account: "TEST".into(),
                symbol: None,
                start_time: None,
                end_time: None,
                limit: Some(10),
                cursor: None,
            },
            6,
        )
        .await
        .expect("fills");
    assert_eq!(fills.account, "TEST");

    let (_cache, pos_frames) = sdk.subscribe_positions("TEST", 7).await.expect("positions");
    assert!(pos_frames
        .iter()
        .any(|f| f.frame_type == fig_core::frame::FrameType::StreamItem));

    let _ = sdk.subscribe_ticker("AAPL", 8).await.expect("ticker");
    let _ = sdk
        .subscribe_orderlists("TEST", 9)
        .await
        .expect("orderlists");
}
