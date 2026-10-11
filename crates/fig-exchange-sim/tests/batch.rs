//! Atomic batch REQUEST (ADR 0010) against exchange-sim.

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::*;
use fig_exchange_sim::md_rate::MdRateConfig;
use fig_exchange_sim::server::{handle_frame, ExchangeState};

fn auth(account: &str) -> Extension {
    Extension::text(ExtensionTag::AuthToken, format!("fig-dev-{account}"))
}

fn limit(id: &str, account: &str, px: f64) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: id.to_string(),
        side: Side::Buy,
        order_qty: Quantity(1.0),
        price: Some(Price(px)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some(account.to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: Some(true),
        reduce_only: None,
    }
}

fn batch_frame(req: &BatchRequest) -> Frame {
    Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            format!("trading/accounts/{}/batch", req.account),
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(auth(&req.account))
        .with_payload(encode_cbor(req).unwrap())
}

fn error_message(frames: &[Frame]) -> Option<&str> {
    frames.iter().find_map(|f| {
        if f.frame_type != FrameType::StreamError {
            return None;
        }
        f.extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ErrorMessage)
            .and_then(|e| e.value.as_text())
    })
}

#[tokio::test]
async fn empty_batch_is_stream_error() {
    let state = ExchangeState::for_test_with(
        MdRateConfig {
            honor_dev_open: false,
            ..MdRateConfig::default()
        },
        Default::default(),
    );
    let req = BatchRequest {
        batch_id: "E".into(),
        account: "TEST".into(),
        actions: vec![],
    };
    let frames = handle_frame(batch_frame(&req), &state, uuid::Uuid::nil(), None).await;
    assert_eq!(error_message(&frames), Some("BATCH_EMPTY"));
}

#[tokio::test]
async fn applied_batch_rests_both_orders() {
    let state = ExchangeState::for_test_with(
        MdRateConfig {
            honor_dev_open: false,
            ..MdRateConfig::default()
        },
        Default::default(),
    );
    let req = BatchRequest {
        batch_id: "BAT-OK".into(),
        account: "TEST".into(),
        actions: vec![
            BatchAction {
                kind: BatchActionKind::NewOrder,
                new_order: Some(limit("A", "TEST", 10.0)),
                cancel: None,
                replace: None,
            },
            BatchAction {
                kind: BatchActionKind::NewOrder,
                new_order: Some(limit("B", "TEST", 9.0)),
                cancel: None,
                replace: None,
            },
        ],
    };
    let frames = handle_frame(batch_frame(&req), &state, uuid::Uuid::nil(), None).await;
    let report: BatchReport = decode_cbor(
        &frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response)
            .expect("BatchReport")
            .payload,
    )
    .unwrap();
    assert_eq!(report.outcome, BatchReportOutcome::Applied);
    let engine = state.engine.lock().await;
    assert!(engine.has_working("A"));
    assert!(engine.has_working("B"));
}

#[tokio::test]
async fn rejected_batch_rolls_back() {
    let state = ExchangeState::for_test_with(
        MdRateConfig {
            honor_dev_open: false,
            ..MdRateConfig::default()
        },
        Default::default(),
    );
    let req = BatchRequest {
        batch_id: "BAT-NO".into(),
        account: "TEST".into(),
        actions: vec![
            BatchAction {
                kind: BatchActionKind::NewOrder,
                new_order: Some(limit("KEEP", "TEST", 10.0)),
                cancel: None,
                replace: None,
            },
            BatchAction {
                kind: BatchActionKind::Cancel,
                new_order: None,
                cancel: Some(CancelRequest {
                    cl_ord_id: "CXL".into(),
                    orig_cl_ord_id: "MISSING".into(),
                    symbol: "AAPL".into(),
                    side: Side::Buy,
                    order_qty: None,
                }),
                replace: None,
            },
        ],
    };
    let frames = handle_frame(batch_frame(&req), &state, uuid::Uuid::nil(), None).await;
    let report: BatchReport = decode_cbor(
        &frames
            .iter()
            .find(|f| f.frame_type == FrameType::Response)
            .expect("BatchReport")
            .payload,
    )
    .unwrap();
    assert_eq!(report.outcome, BatchReportOutcome::Rejected);
    assert_eq!(
        report.results[0].reject_reason.as_deref(),
        Some("BATCH_ROLLED_BACK")
    );
    let engine = state.engine.lock().await;
    assert!(!engine.has_working("KEEP"));
}
