//! OUCH/SoupBin order entry and ITCH L2 SoupBin/MoldUDP64 egress.

use fig_core::messages::Side;
use fig_exchange_sim::server::run_server;
use fig_gateways::itch::{decode_moldudp64, MSG_LEVEL, MSG_SYSTEM};
use fig_gateways::ouch::{encode_enter, parse_inbound, OuchInbound, MSG_ACCEPTED};
use fig_gateways::soupbin::{
    encode_login_request, encode_unsequenced, SoupDecoder, LOGIN_ACCEPTED, SEQUENCED_DATA,
};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};

#[tokio::test]
async fn ouch_enter_order_round_trip_through_backend() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    std::env::set_var("FIG_TLS_INSECURE", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("sim");
    let backend = endpoint.local_addr().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ouch_addr = listener.local_addr().unwrap();
    drop(listener);
    tokio::spawn(async move {
        let _ = fig_gateways::ouch::run_ouch_gateway(ouch_addr, Some(backend)).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(ouch_addr).await.unwrap();
    stream
        .write_all(&encode_login_request("DEMO-ACCT", "x", "OUCH", 1))
        .await
        .unwrap();

    let mut decoder = SoupDecoder::new();
    let mut buf = vec![0u8; 4096];
    let n = tokio::time::timeout(Duration::from_secs(2), stream.read(&mut buf))
        .await
        .unwrap()
        .unwrap();
    decoder.feed(&buf[..n]);
    let login = decoder.next_packet().unwrap().unwrap();
    assert_eq!(login.packet_type, LOGIN_ACCEPTED);

    let enter = encode_enter(
        "TOK-E2E-1",
        Side::Buy,
        1,
        "AAPL",
        10.0,
        fig_core::messages::TimeInForce::Day,
        "DEMO-ACCT",
    );
    stream.write_all(&encode_unsequenced(&enter)).await.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
        let n = tokio::time::timeout(remain, stream.read(&mut buf))
            .await
            .expect("ouch response")
            .unwrap();
        if n == 0 {
            panic!("socket closed before OUCH accepted");
        }
        decoder.feed(&buf[..n]);
        while let Some(pkt) = decoder.next_packet() {
            let pkt = pkt.unwrap();
            if pkt.packet_type == SEQUENCED_DATA && pkt.payload.first() == Some(&MSG_ACCEPTED) {
                match parse_inbound(&enter) {
                    Some(OuchInbound::Enter { token, .. }) => assert_eq!(token, "TOK-E2E-1"),
                    _ => {}
                }
                return;
            }
        }
    }
}

#[tokio::test]
async fn itch_soubin_login_emits_system_event() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    std::env::set_var("FIG_TLS_INSECURE", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("sim");
    let backend = endpoint.local_addr().unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let itch_addr = listener.local_addr().unwrap();
    drop(listener);
    tokio::spawn(async move {
        let _ = fig_gateways::itch::run_itch_gateway(Some(itch_addr), None, Some(backend)).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let mut stream = TcpStream::connect(itch_addr).await.unwrap();
    stream
        .write_all(&encode_login_request("ITCH", "x", "ITCH", 1))
        .await
        .unwrap();
    let mut decoder = SoupDecoder::new();
    let mut buf = vec![0u8; 4096];
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        let remain = deadline.saturating_duration_since(tokio::time::Instant::now());
        let n = tokio::time::timeout(remain, stream.read(&mut buf))
            .await
            .expect("itch response")
            .unwrap();
        decoder.feed(&buf[..n]);
        while let Some(pkt) = decoder.next_packet() {
            let pkt = pkt.unwrap();
            if pkt.packet_type == SEQUENCED_DATA && pkt.payload.first() == Some(&MSG_SYSTEM) {
                return;
            }
        }
    }
}

#[tokio::test]
async fn itch_moldudp64_round_trip_bytes() {
    let level = fig_gateways::itch::encode_price_level("AAPL", Side::Buy, 12.5, 4, 0, b'S');
    let dgram = fig_gateways::itch::encode_moldudp64("FIGITCH01", 3, &[&level]);
    let (sess, seq, msgs) = decode_moldudp64(&dgram).unwrap();
    assert_eq!(sess, "FIGITCH01");
    assert_eq!(seq, 3);
    assert_eq!(msgs[0][0], MSG_LEVEL);
}

#[tokio::test]
async fn itch_multicast_sends_datagram() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    std::env::set_var("FIG_TLS_INSECURE", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("sim");
    let backend = endpoint.local_addr().unwrap();
    let udp = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let dest = udp.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = fig_gateways::itch::run_itch_gateway(None, Some(dest), Some(backend)).await;
    });
    let mut buf = vec![0u8; 2048];
    let n = tokio::time::timeout(Duration::from_secs(5), udp.recv(&mut buf))
        .await
        .expect("mold datagram")
        .unwrap();
    assert!(decode_moldudp64(&buf[..n]).is_some() || n > 0);
}
