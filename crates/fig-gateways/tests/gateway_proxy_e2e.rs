//! Gateway REST/WS proxy round-trip against exchange-sim (§17.6).

use fig_core::codec;
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::CapabilitiesResponse;
use fig_exchange_sim::server::run_server;
use fig_gateways::backend::proxy_frame;
use fig_gateways::rest::parse_http_request;
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws_catalog::legacy_ws_json_to_fig_subscribe;

#[tokio::test]
async fn rest_capabilities_round_trips_through_backend() {
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();

    let raw = b"GET /.well-known/capabilities HTTP/1.1\r\n\r\n";
    let http = parse_http_request(raw).expect("parse http");
    let fig_frame = http_get_to_fig_request(&http).expect("map request");

    let frames = proxy_frame(addr, fig_frame).await.expect("proxy");
    let resp = frames
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("response frame");
    let caps: CapabilitiesResponse = codec::decode_cbor(&resp.payload).expect("decode caps");
    assert!(!caps.paths.is_empty());
}

#[tokio::test]
async fn ws_subscribe_round_trips_through_backend() {
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();

    let json = r#"{"method":"SUBSCRIBE","params":["aapl@bookTicker"]}"#;
    let sub: Frame = legacy_ws_json_to_fig_subscribe(json, 3).expect("ws map");

    let frames = proxy_frame(addr, sub).await.expect("proxy subscribe");
    assert!(
        frames.iter().any(|f| {
            f.frame_type == FrameType::StreamItem || f.frame_type == FrameType::Response
        }),
        "expected subscribe ack or stream item"
    );
}
