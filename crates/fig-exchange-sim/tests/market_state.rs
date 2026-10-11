//! Session status, security status, and mass cancel against exchange-sim.

use std::time::Duration;

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};
use fig_core::messages::*;
use fig_core::transport::client_config_insecure;
use fig_exchange_sim::server::run_server;

async fn connect_client(server_addr: std::net::SocketAddr) -> anyhow::Result<quinn::Connection> {
    let client_ep = quinn::Endpoint::client("127.0.0.1:0".parse()?)?;
    let client_cfg =
        client_config_insecure().map_err(|e| anyhow::anyhow!("client_config: {}", e))?;
    Ok(client_ep
        .connect_with(client_cfg, server_addr, "localhost")?
        .await?)
}

async fn send_and_receive(conn: &quinn::Connection, frame: Frame) -> anyhow::Result<Vec<Frame>> {
    let keep_open = frame.frame_type == FrameType::Subscribe;
    let (mut send, mut recv) = conn.open_bi().await?;
    send.write_all(&frame.encode()?).await?;
    send.finish()?;
    let mut decoder = FrameDecoder::new();
    let mut buf = vec![0u8; 8192];
    if keep_open {
        let mut frames = Vec::new();
        loop {
            let idle = if frames.is_empty() {
                Duration::from_secs(5)
            } else {
                Duration::from_millis(75)
            };
            match tokio::time::timeout(idle, recv.read(&mut buf)).await {
                Ok(Ok(Some(n))) => {
                    decoder.feed(&buf[..n]);
                    while let Some(r) = decoder.decode_next() {
                        frames.push(r?);
                    }
                }
                Ok(Ok(None)) => break,
                Ok(Err(e)) => return Err(e.into()),
                Err(_) if !frames.is_empty() => break,
                Err(_) => anyhow::bail!("subscribe snapshot timed out"),
            }
        }
        Ok(frames)
    } else {
        loop {
            match recv.read(&mut buf).await? {
                Some(n) => decoder.feed(&buf[..n]),
                None => break,
            }
        }
        Ok(decoder
            .decode_all()
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?)
    }
}

fn auth(account: &str) -> Extension {
    Extension::text(ExtensionTag::AuthToken, format!("fig-dev-{account}"))
}

fn order(id: &str, account: &str, symbol: &str, side: Side, px: f64) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: id.to_string(),
        side,
        order_qty: Quantity(1.0),
        price: Some(Price(px)),
        stop_price: None,
        symbol: symbol.to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some(account.to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: None,
        reduce_only: None,
    }
}

#[tokio::test]
async fn session_and_security_status_query() {
    let endpoint = run_server("127.0.0.1:0").await.unwrap();
    let conn = connect_client(endpoint.local_addr().unwrap())
        .await
        .unwrap();

    let session_frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/session",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let frames = send_and_receive(&conn, session_frame).await.unwrap();
    let status: TradingSessionStatus = decode_cbor(
        &frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response)
            .unwrap()
            .payload,
    )
    .unwrap();
    assert_eq!(status.status, TradingSessionStatusStatus::Open);
    assert_eq!(status.trading_session_id, "REG");

    let sec_frame = Frame::new(FrameType::Request, 2)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/status",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let frames = send_and_receive(&conn, sec_frame).await.unwrap();
    let sec: SecurityStatus = decode_cbor(
        &frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response)
            .unwrap()
            .payload,
    )
    .unwrap();
    assert_eq!(sec.symbol, "AAPL");
    assert_eq!(sec.status, SecurityStatusStatus::Trading);
}

#[tokio::test]
async fn subscribe_session_snapshot() {
    let endpoint = run_server("127.0.0.1:0").await.unwrap();
    let conn = connect_client(endpoint.local_addr().unwrap())
        .await
        .unwrap();
    let frame = Frame::new(FrameType::Subscribe, 3)
        .with_seq(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/session",
        ))
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata/session",
        ));
    let frames = send_and_receive(&conn, frame).await.unwrap();
    assert!(frames.iter().any(|f| {
        f.frame_type == FrameType::StreamItem
            && decode_cbor::<TradingSessionStatus>(&f.payload)
                .is_ok_and(|s| s.status == TradingSessionStatusStatus::Open)
    }));
}

#[tokio::test]
async fn mass_cancel_resting_orders() {
    let endpoint = run_server("127.0.0.1:0").await.unwrap();
    let conn = connect_client(endpoint.local_addr().unwrap())
        .await
        .unwrap();
    let account = "TEST";

    for (id, px) in [("MC-A", 10.0), ("MC-B", 9.0)] {
        let payload = encode_cbor(&order(id, account, "AAPL", Side::Buy, px)).unwrap();
        let frame = Frame::new(FrameType::Request, 1)
            .with_seq(1)
            .with_schema_id(1)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                format!("trading/accounts/{account}/orders"),
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"))
            .with_extension(auth(account))
            .with_payload(payload);
        let _ = send_and_receive(&conn, frame).await.unwrap();
    }

    let req = MassCancelRequest {
        cl_ord_id: "MC-ALL".to_string(),
        account: account.to_string(),
        symbol: Some("AAPL".to_string()),
        side: None,
    };
    let frame = Frame::new(FrameType::Request, 4)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("trading/accounts/{account}/orders/masscancel"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(auth(account))
        .with_payload(encode_cbor(&req).unwrap());
    let frames = send_and_receive(&conn, frame).await.unwrap();
    let report: MassCancelReport = decode_cbor(
        &frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response)
            .unwrap()
            .payload,
    )
    .unwrap();
    assert_eq!(report.cl_ord_id, "MC-ALL");
    assert!(report.total_affected >= 2);
    assert_eq!(report.status, MassCancelReportStatus::Completed);
}
