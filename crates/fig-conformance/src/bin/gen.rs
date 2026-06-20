use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use fig_conformance::{
    write_suite, ChannelSpec, ConformanceSuite, ConformanceVector, ExtensionSpec,
    ExtensionValueSpec, FrameSpec,
};
use fig_core::channel::{ChannelManager, ChannelMode};
use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    BalanceEntry, BalanceSnapshot, CandleBar, CandleBarRequest, NewOrderSingle, OrderType, Price,
    Quantity, Side, TimeInForce,
};
use fig_core::sbe::encode_new_order_single;

#[derive(Parser)]
#[command(
    name = "fig-conformance-gen",
    about = "Regenerate FIG conformance vectors"
)]
struct Args {
    #[arg(long, default_value = "tests/conformance/vectors/v1.json")]
    write: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let suite = build_suite()?;
    write_suite(&args.write, &suite)?;
    println!(
        "Wrote {} ({} vectors)",
        args.write.display(),
        suite.vectors.len()
    );
    Ok(())
}

fn build_suite() -> Result<ConformanceSuite> {
    let order_payload = serde_json::json!({
        "cl_ord_id": "CONF-001",
        "side": "Buy",
        "order_qty": 100.0,
        "price": 50.25,
        "symbol": "AAPL",
        "order_type": "Limit",
        "time_in_force": "Day"
    });
    let order = sample_order();

    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(42)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEMO/orders",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(encode_cbor(&order)?);

    let mut client = ChannelManager::new(false);
    let ch = client.open_channel(ChannelMode::Stateless, None)?;
    let client_stream = client.tree_stream_id(ch);
    let server = ChannelManager::new(true);
    let server_stream = server.tree_stream_id(ch);

    let candle = CandleBar {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        open: Price(150.0),
        high: Price(151.0),
        low: Price(149.5),
        close: Price(150.5),
        volume: Quantity(1000.0),
        bar_start: 1_700_000_000_000_000_000,
        bar_end: 1_700_000_300_000_000_000,
        is_final: true,
        is_snapshot: Some(false),
    };
    let candle_payload = serde_json::json!({
        "symbol": "AAPL",
        "interval": "5m",
        "open": 150.0,
        "high": 151.0,
        "low": 149.5,
        "close": 150.5,
        "volume": 1000.0,
        "bar_start": 1_700_000_000_000_000_000_i64,
        "bar_end": 1_700_000_300_000_000_000_i64,
        "is_final": true
    });
    let candle_req = CandleBarRequest {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        start_time: None,
        end_time: None,
        limit: Some(100),
    };
    let candle_query_frame = Frame::new(FrameType::Request, 2)
        .with_seq(1)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/candles/5m",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(encode_cbor(&candle_req)?);
    let balance_snap = BalanceSnapshot {
        account: "DEMO".to_string(),
        balances: vec![BalanceEntry {
            asset: "USD".to_string(),
            total: 1_000_000.0,
            available: 2_000_000.0,
            hold: 0.0,
        }],
        is_snapshot: Some(true),
    };

    Ok(ConformanceSuite {
        version: 1,
        suite: "fig-conformance-v1".to_string(),
        vectors: vec![
            ConformanceVector {
                id: "frame.request.limit_order_cbor".into(),
                category: "frame".into(),
                description: "REQUEST with CBOR NewOrderSingle payload".into(),
                message_type: String::new(),
                expected_hex: hex::encode(frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Request".into(),
                    channel_id: 1,
                    stream_seq: 42,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text("trading/accounts/DEMO/orders".into()),
                        },
                        ExtensionSpec {
                            tag: "Method".into(),
                            value: ExtensionValueSpec::Text("POST".into()),
                        },
                        ExtensionSpec {
                            tag: "ContentType".into(),
                            value: ExtensionValueSpec::Text("application/cbor".into()),
                        },
                    ],
                    payload_hex: Some(hex::encode(encode_cbor(&order)?)),
                }),
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.new_order_single.limit_buy".into(),
                category: "cbor".into(),
                description: "Canonical limit buy NewOrderSingle CBOR".into(),
                message_type: "NewOrderSingle".into(),
                expected_hex: hex::encode(encode_cbor(&order)?),
                frame: None,
                payload: Some(order_payload.clone()),
                channel: None,
            },
            ConformanceVector {
                id: "sbe.new_order_single.limit_buy".into(),
                category: "sbe".into(),
                description: "Canonical limit buy NewOrderSingle SBE".into(),
                message_type: "NewOrderSingle".into(),
                expected_hex: hex::encode(encode_new_order_single(&order)),
                frame: None,
                payload: Some(order_payload.clone()),
                channel: None,
            },
            ConformanceVector {
                id: "cbor.candle_bar.snapshot".into(),
                category: "cbor".into(),
                description: "Canonical CandleBar CBOR".into(),
                message_type: "CandleBar".into(),
                expected_hex: hex::encode(encode_cbor(&candle)?),
                frame: None,
                payload: Some(candle_payload),
                channel: None,
            },
            ConformanceVector {
                id: "cbor.balance_snapshot.demo".into(),
                category: "cbor".into(),
                description: "BalanceSnapshot CBOR for private stream conformance".into(),
                message_type: "BalanceSnapshot".into(),
                expected_hex: hex::encode(encode_cbor(&balance_snap)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "frame.request.candle_bar_query".into(),
                category: "frame".into(),
                description: "GET CandleBarRequest on marketdata path".into(),
                message_type: "CandleBarRequest".into(),
                expected_hex: hex::encode(candle_query_frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Request".into(),
                    channel_id: 2,
                    stream_seq: 1,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text("marketdata/AAPL/candles/5m".into()),
                        },
                        ExtensionSpec {
                            tag: "Method".into(),
                            value: ExtensionValueSpec::Text("GET".into()),
                        },
                        ExtensionSpec {
                            tag: "ContentType".into(),
                            value: ExtensionValueSpec::Text("application/cbor".into()),
                        },
                    ],
                    payload_hex: Some(hex::encode(encode_cbor(&candle_req)?)),
                }),
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "channel.stream_id.client_server".into(),
                category: "channel".into(),
                description: "Client stream = channel*4+0, server = channel*4+1".into(),
                message_type: String::new(),
                expected_hex: String::new(),
                frame: None,
                payload: None,
                channel: Some(ChannelSpec {
                    channel_id: ch,
                    client_stream_id: client_stream,
                    server_stream_id: server_stream,
                }),
            },
        ],
    })
}

fn sample_order() -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: "CONF-001".to_string(),
        side: Side::Buy,
        order_qty: Quantity(100.0),
        price: Some(Price(50.25)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    }
}
