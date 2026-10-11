//! Public market-data rate limits (SPEC §9.3) against exchange-sim.

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};
use fig_core::transport::client_config_insecure;
use fig_exchange_sim::md_rate::MdRateConfig;
use fig_exchange_sim::server::{run_server_with, ServerOptions};

async fn connect_client(server_addr: std::net::SocketAddr) -> anyhow::Result<quinn::Connection> {
    let client_ep = quinn::Endpoint::client("127.0.0.1:0".parse()?)?;
    let client_cfg =
        client_config_insecure().map_err(|e| anyhow::anyhow!("client_config: {}", e))?;
    Ok(client_ep
        .connect_with(client_cfg, server_addr, "localhost")?
        .await?)
}

async fn send_and_receive(conn: &quinn::Connection, frame: Frame) -> anyhow::Result<Vec<Frame>> {
    let (mut send, mut recv) = conn.open_bi().await?;
    send.write_all(&frame.encode()?).await?;
    send.finish()?;
    let mut decoder = FrameDecoder::new();
    let mut buf = vec![0u8; 8192];
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

fn book_get() -> Frame {
    Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/book",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
}

fn error_code(frames: &[Frame]) -> Option<&str> {
    frames.iter().find_map(|f| {
        if f.frame_type != FrameType::StreamError {
            return None;
        }
        f.extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ErrorCode)
            .and_then(|e| e.value.as_text())
    })
}

fn status_code(frames: &[Frame]) -> Option<u16> {
    frames.iter().find_map(|f| {
        f.extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::StatusCode)
            .and_then(|e| e.value.as_u16())
    })
}

#[tokio::test]
async fn public_md_bucket_returns_rate_limited() {
    let endpoint = run_server_with(
        "127.0.0.1:0",
        ServerOptions {
            md_rate: MdRateConfig {
                public_per_sec: 2,
                authenticated_per_sec: 32,
                honor_dev_open: false,
            },
            ..Default::default()
        },
    )
    .await
    .unwrap();
    let conn = connect_client(endpoint.local_addr().unwrap())
        .await
        .unwrap();

    for _ in 0..2 {
        let frames = send_and_receive(&conn, book_get()).await.unwrap();
        assert!(
            frames.iter().any(|f| f.frame_type == FrameType::Response),
            "first two public GETs should succeed"
        );
    }
    let frames = send_and_receive(&conn, book_get()).await.unwrap();
    assert_eq!(error_code(&frames), Some("RATE_LIMITED"));
    assert_eq!(status_code(&frames), Some(429));
}

#[tokio::test]
async fn authenticated_md_is_not_the_public_bucket() {
    let endpoint = run_server_with(
        "127.0.0.1:0",
        ServerOptions {
            md_rate: MdRateConfig {
                public_per_sec: 1,
                authenticated_per_sec: 8,
                honor_dev_open: false,
            },
            ..Default::default()
        },
    )
    .await
    .unwrap();
    let conn = connect_client(endpoint.local_addr().unwrap())
        .await
        .unwrap();

    let frames = send_and_receive(&conn, book_get()).await.unwrap();
    assert!(frames.iter().any(|f| f.frame_type == FrameType::Response));
    let frames = send_and_receive(&conn, book_get()).await.unwrap();
    assert_eq!(error_code(&frames), Some("RATE_LIMITED"));

    let authed =
        book_get().with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-TEST"));
    let frames = send_and_receive(&conn, authed).await.unwrap();
    assert!(
        frames.iter().any(|f| f.frame_type == FrameType::Response),
        "AUTH_TOKEN uses the authenticated bucket"
    );
}
