//! Legacy gateway alias round-trips against exchange-sim.
//!
//! Binance WS topics, Hyperliquid subscription JSON, and FIX order wire are
//! **reference alias fixtures** — each maps to native FIG `CHANNEL_PATH` values
//! and proxies through the same backend. See [AGENTS.md](../../../AGENTS.md).

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{schema_id, AccountSummary, CapabilitiesResponse, ExecutionReport};
use fig_exchange_sim::server::run_server;
use fig_gateways::backend::proxy_frame;
use fig_gateways::fix::{fix_to_fig_order, parse_fix_message};
use fig_gateways::rest::parse_http_request;
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws_catalog::legacy_ws_json_to_fig_subscribe;
use std::net::SocketAddr;

/// One legacy WebSocket JSON shape → native FIG subscribe mapping.
struct LegacyWsCase {
    /// Fixture table id (`binance`, `hyperliquid`, …).
    source: &'static str,
    name: &'static str,
    json: String,
    auth_account: Option<&'static str>,
}

/// One FIX NewOrderSingle (35=D) wire → native FIG order request.
struct LegacyFixOrderCase {
    source: &'static str,
    name: &'static str,
    fix_wire: Vec<u8>,
}

fn binance_ws_fixtures() -> Vec<LegacyWsCase> {
    let public_topics = [
        "btcusdt@kline_5m",
        "ethusdt@trade",
        "btcusdt@aggTrade",
        "btcusdt@bookTicker",
        "btcusdt@ticker",
        "btcusdt@miniTicker",
        "btcusdt@markPrice",
        "btcusdt@forceOrder",
        "btcusdt@depth",
    ];
    let mut cases: Vec<LegacyWsCase> = public_topics
        .into_iter()
        .map(|topic| LegacyWsCase {
            source: "binance",
            name: topic,
            json: format!(r#"{{"method":"SUBSCRIBE","params":["{topic}"]}}"#),
            auth_account: None,
        })
        .collect();

    for (topic, account) in [
        ("DEMO-ACCT@executionReport", "DEMO-ACCT"),
        ("DEMO-ACCT@balance", "DEMO-ACCT"),
        ("DEMO@orderlists", "DEMO"),
    ] {
        cases.push(LegacyWsCase {
            source: "binance",
            name: topic,
            json: format!(r#"{{"method":"SUBSCRIBE","params":["{topic}"]}}"#),
            auth_account: Some(account),
        });
    }
    cases
}

fn hyperliquid_ws_fixtures() -> Vec<LegacyWsCase> {
    const ACCOUNT: &str = "DEMO-ACCT";

    fn hl(name: &'static str, sub: serde_json::Value, auth: bool) -> LegacyWsCase {
        LegacyWsCase {
            source: "hyperliquid",
            name,
            json: serde_json::json!({
                "method": "subscribe",
                "subscription": sub,
            })
            .to_string(),
            auth_account: if auth { Some(ACCOUNT) } else { None },
        }
    }

    vec![
        hl(
            "trades",
            serde_json::json!({"type": "trades", "coin": "BTC"}),
            false,
        ),
        hl(
            "candle",
            serde_json::json!({"type": "candle", "coin": "ETH", "interval": "5m"}),
            false,
        ),
        hl(
            "l2Book",
            serde_json::json!({"type": "l2Book", "coin": "BTC"}),
            false,
        ),
        hl(
            "bbo",
            serde_json::json!({"type": "bbo", "coin": "BTC"}),
            false,
        ),
        hl(
            "orderUpdates",
            serde_json::json!({"type": "orderUpdates", "user": ACCOUNT}),
            true,
        ),
        hl(
            "userFills",
            serde_json::json!({"type": "userFills", "user": ACCOUNT}),
            true,
        ),
        hl(
            "openOrders",
            serde_json::json!({"type": "openOrders", "user": ACCOUNT}),
            true,
        ),
        hl(
            "orderState",
            serde_json::json!({"type": "orderState", "user": ACCOUNT}),
            true,
        ),
        hl(
            "orderLists",
            serde_json::json!({"type": "orderLists", "user": ACCOUNT}),
            true,
        ),
        hl(
            "listStatus",
            serde_json::json!({"type": "listStatus", "user": ACCOUNT}),
            true,
        ),
        hl(
            "balanceUpdate",
            serde_json::json!({"type": "balanceUpdate", "user": ACCOUNT}),
            true,
        ),
        hl(
            "spotState",
            serde_json::json!({"type": "spotState", "user": ACCOUNT}),
            true,
        ),
        hl(
            "subscribeBalance",
            serde_json::json!({"type": "subscribeBalance", "user": ACCOUNT}),
            true,
        ),
        hl(
            "clearinghouseState",
            serde_json::json!({"type": "clearinghouseState", "user": ACCOUNT}),
            true,
        ),
        hl(
            "subscribePosition",
            serde_json::json!({"type": "subscribePosition", "user": ACCOUNT}),
            true,
        ),
        hl(
            "userFunding",
            serde_json::json!({"type": "userFunding", "user": ACCOUNT}),
            true,
        ),
        hl(
            "fundingHistory",
            serde_json::json!({"type": "fundingHistory", "user": ACCOUNT}),
            true,
        ),
        hl(
            "ledgerUpdates",
            serde_json::json!({"type": "ledgerUpdates", "user": ACCOUNT}),
            true,
        ),
        hl(
            "userNonFundingLedgerUpdates",
            serde_json::json!({"type": "userNonFundingLedgerUpdates", "user": ACCOUNT}),
            true,
        ),
        hl(
            "liquidation",
            serde_json::json!({"type": "liquidation", "user": ACCOUNT}),
            true,
        ),
        hl(
            "userLiquidation",
            serde_json::json!({"type": "userLiquidation", "user": ACCOUNT}),
            true,
        ),
        hl(
            "activeAssetCtx",
            serde_json::json!({"type": "activeAssetCtx", "coin": "BTC"}),
            false,
        ),
        hl(
            "markPrice",
            serde_json::json!({"type": "markPrice", "coin": "BTC"}),
            false,
        ),
        hl("allMids", serde_json::json!({"type": "allMids"}), false),
        hl(
            "miniTicker",
            serde_json::json!({"type": "miniTicker"}),
            false,
        ),
        hl(
            "aggTrades",
            serde_json::json!({"type": "aggTrades", "coin": "BTC"}),
            false,
        ),
        hl(
            "liquidations",
            serde_json::json!({"type": "liquidations"}),
            false,
        ),
        hl(
            "forceOrder",
            serde_json::json!({"type": "forceOrder"}),
            false,
        ),
        hl(
            "margin",
            serde_json::json!({"type": "margin", "user": ACCOUNT}),
            true,
        ),
        hl(
            "clearinghouseMargin",
            serde_json::json!({"type": "clearinghouseMargin", "user": ACCOUNT}),
            true,
        ),
    ]
}

fn fix_checksum(body: &[u8]) -> u8 {
    body.iter().fold(0u8, |sum, b| sum.wrapping_add(*b))
}

fn build_fix_new_order(body_tags: &[(&str, &str)]) -> Vec<u8> {
    const SOH: u8 = 0x01;
    let mut buf = Vec::new();
    buf.extend_from_slice(b"8=FIX.4.4");
    buf.push(SOH);
    for (tag, value) in body_tags {
        buf.extend_from_slice(format!("{tag}={value}").as_bytes());
        buf.push(SOH);
    }
    let checksum = fix_checksum(&buf);
    buf.extend_from_slice(format!("10={checksum:03}").as_bytes());
    buf.push(SOH);
    buf
}

fn fix_order_fixtures() -> Vec<LegacyFixOrderCase> {
    vec![
        LegacyFixOrderCase {
            source: "fix",
            name: "limit_buy_btc",
            fix_wire: build_fix_new_order(&[
                ("35", "D"),
                ("11", "FIX-E2E-BTC-001"),
                ("54", "1"),
                ("38", "1"),
                ("44", "50000"),
                ("55", "BTC"),
                ("40", "2"),
                ("59", "0"),
                ("1", "DEMO-ACCT"),
            ]),
        },
        LegacyFixOrderCase {
            source: "fix",
            name: "limit_sell_aapl",
            fix_wire: build_fix_new_order(&[
                ("35", "D"),
                ("11", "FIX-E2E-AAPL-001"),
                ("54", "2"),
                ("38", "10"),
                ("44", "150"),
                ("55", "AAPL"),
                ("40", "2"),
                ("59", "0"),
                ("1", "DEMO-ACCT"),
            ]),
        },
    ]
}

fn with_dev_auth(frame: Frame, account: &str) -> Frame {
    frame.with_extension(Extension::text(
        ExtensionTag::AuthToken,
        format!("fig-dev-{account}"),
    ))
}

fn channel_path(frame: &Frame) -> Option<&str> {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
}

fn is_private_path(path: &str) -> bool {
    path.starts_with("accounts/") || path.starts_with("trading/accounts/")
}

fn order_to_fig_request(order: &fig_core::messages::NewOrderSingle) -> Frame {
    let account = order.account.as_deref().unwrap_or("DEFAULT");
    let payload = encode_cbor(order).expect("encode order");
    Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("trading/accounts/{account}/orders"),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(payload)
}

async fn assert_ws_aliases_round_trip(addr: SocketAddr, cases: &[LegacyWsCase]) {
    for (idx, case) in cases.iter().enumerate() {
        let mut frame = legacy_ws_json_to_fig_subscribe(&case.json, (idx as u16) + 10)
            .unwrap_or_else(|e| panic!("[{}] {} map: {e}", case.source, case.name));

        if let Some(path) = channel_path(&frame) {
            if is_private_path(path) {
                let account = case.auth_account.unwrap_or("DEMO-ACCT");
                frame = with_dev_auth(frame, account);
            }
        } else if let Some(account) = case.auth_account {
            frame = with_dev_auth(frame, account);
        }

        let frames = proxy_frame(addr, frame)
            .await
            .unwrap_or_else(|e| panic!("[{}] {} proxy: {e}", case.source, case.name));

        assert!(
            !frames
                .iter()
                .any(|f| f.frame_type == FrameType::StreamError),
            "[{}] {} STREAM_ERROR",
            case.source,
            case.name
        );
        assert!(
            frames.iter().any(|f| {
                f.frame_type == FrameType::StreamItem || f.frame_type == FrameType::Response
            }),
            "[{}] {} expected ack or stream item, got: {:?}",
            case.source,
            case.name,
            frames.iter().map(|f| f.frame_type).collect::<Vec<_>>()
        );
    }
}

#[tokio::test]
async fn legacy_ws_binance_aliases_round_trip_through_backend() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();
    assert_ws_aliases_round_trip(addr, &binance_ws_fixtures()).await;
    drop(endpoint);
}

#[tokio::test]
async fn legacy_ws_hyperliquid_aliases_round_trip_through_backend() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();
    assert_ws_aliases_round_trip(addr, &hyperliquid_ws_fixtures()).await;
    drop(endpoint);
}

#[tokio::test]
async fn legacy_fix_order_aliases_round_trip_through_backend() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();

    for case in fix_order_fixtures() {
        let tags = parse_fix_message(&case.fix_wire)
            .unwrap_or_else(|e| panic!("[{}] {} parse: {e}", case.source, case.name));
        let order = fix_to_fig_order(&tags)
            .unwrap_or_else(|e| panic!("[{}] {} map: {e}", case.source, case.name));

        let account = order.account.as_deref().unwrap_or("DEFAULT");
        let mut frame = order_to_fig_request(&order);
        frame = with_dev_auth(frame, account);

        let frames = proxy_frame(addr, frame)
            .await
            .unwrap_or_else(|e| panic!("[{}] {} proxy: {e}", case.source, case.name));

        assert!(
            !frames
                .iter()
                .any(|f| f.frame_type == FrameType::StreamError),
            "[{}] {} STREAM_ERROR",
            case.source,
            case.name
        );

        let payload_frame = frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response || f.frame_type == FrameType::StreamItem)
            .unwrap_or_else(|| {
                panic!(
                    "[{}] {} missing order ack, got: {:?}",
                    case.source,
                    case.name,
                    frames.iter().map(|f| f.frame_type).collect::<Vec<_>>()
                )
            });

        let report: ExecutionReport = decode_cbor(&payload_frame.payload)
            .unwrap_or_else(|e| panic!("[{}] {} decode: {e}", case.source, case.name));
        assert_eq!(report.cl_ord_id, order.cl_ord_id);
    }

    drop(endpoint);
}

#[tokio::test]
async fn legacy_rest_get_aliases_round_trip_through_backend() {
    std::env::set_var("FIG_DEV_OPEN", "1");
    let endpoint = run_server("127.0.0.1:0").await.expect("server");
    let addr = endpoint.local_addr().unwrap();

    let caps_raw = b"GET /.well-known/capabilities HTTP/1.1\r\n\r\n";
    let caps_http = parse_http_request(caps_raw).expect("parse capabilities http");
    let caps_frame = http_get_to_fig_request(&caps_http).expect("map capabilities");
    let caps_frames = proxy_frame(addr, caps_frame)
        .await
        .expect("proxy capabilities");
    let caps_resp = caps_frames
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("capabilities response");
    let caps: CapabilitiesResponse = decode_cbor(&caps_resp.payload).expect("decode caps");
    assert!(!caps.paths.is_empty());

    let acct_raw =
        b"GET /accounts/DEMO-ACCT HTTP/1.1\r\nAuthorization: Bearer fig-dev-DEMO-ACCT\r\n\r\n";
    let acct_http = parse_http_request(acct_raw).expect("parse account http");
    let acct_frame = http_get_to_fig_request(&acct_http).expect("map account");
    let acct_frames = proxy_frame(addr, acct_frame).await.expect("proxy account");
    let acct_resp = acct_frames
        .iter()
        .find(|f| f.frame_type == FrameType::Response)
        .expect("account response");
    let summary: AccountSummary = decode_cbor(&acct_resp.payload).expect("decode account");
    assert_eq!(summary.account, "DEMO-ACCT");

    drop(endpoint);
}
