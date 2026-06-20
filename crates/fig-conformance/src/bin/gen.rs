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
    AggregateTrade, BalanceEntry, BalanceSnapshot, CandleBar, CandleBarBatch, CandleBarRequest,
    CapabilitiesResponse, CapabilityPath, CapabilityPathPattern, ExecType, ExecutionReport,
    MarkPriceUpdate, MarketDataAction, MarketDataSnapshot, MarketDataUpdate, MiniTicker,
    NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot, OrdStatus, OrderBookDelta,
    OrderBookSnapshot, OrderHistoryBatch, OrderHistoryRequest, OrderListStatus,
    OrderListStatusStatus, OrderType, PositionUpdate, Price, PriceLevel, Quantity, Side,
    SymbolTicker, TimeInForce,
};
use fig_core::sbe::encode_new_order_single;
use fig_core::sbe_stream::{
    encode_balance_snapshot, encode_candle_bar, encode_candle_bar_batch,
    encode_order_book_snapshot, encode_symbol_ticker,
};

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
        cursor: None,
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
    let caps = CapabilitiesResponse {
        schema_ids: vec![1, 2, 3],
        paths: vec![CapabilityPath {
            path: "marketdata/AAPL/candles/5m".to_string(),
            pattern: CapabilityPathPattern::PubSub,
            auth_required: false,
        }],
        symbols: vec!["AAPL".to_string()],
        intervals: vec!["5m".to_string()],
    };
    let open_orders = OpenOrdersSnapshot {
        account: "DEMO".to_string(),
        orders: vec![],
        is_snapshot: Some(true),
    };
    let md_snap = MarketDataSnapshot {
        symbol: "AAPL".to_string(),
        exchange: "SIM".to_string(),
        bids: vec![PriceLevel {
            price: Price(100.0),
            qty: Quantity(5.0),
            order_count: Some(1),
        }],
        asks: vec![],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(42),
        is_snapshot: Some(true),
    };
    let order_hist_req = OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    };
    let order_hist_page = OrderHistoryBatch {
        account: "DEMO".to_string(),
        orders: vec![ExecutionReport {
            cl_ord_id: "HIST-1".to_string(),
            order_id: "1".to_string(),
            exec_id: "exec-1".to_string(),
            exec_type: ExecType::New,
            ord_status: OrdStatus::New,
            side: Side::Buy,
            last_qty: None,
            last_price: None,
            leaves_qty: Quantity(10.0),
            cum_qty: Quantity(0.0),
            avg_price: Price(0.0),
            symbol: "AAPL".to_string(),
            transact_time: 1_700_000_000_000_000_000,
        }],
        has_more: true,
        next_cursor: Some("exec-1".to_string()),
    };
    let order_list_status = OrderListStatus {
        account: "DEMO".to_string(),
        list_id: "OL-1".to_string(),
        status: OrderListStatusStatus::Executing,
        symbol: Some("AAPL".to_string()),
    };
    let order_hist_paged_req = OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(5),
        cursor: Some("exec-1".to_string()),
    };
    let book_delta = OrderBookDelta {
        symbol: "AAPL".to_string(),
        updates: vec![MarketDataUpdate {
            action: MarketDataAction::Change,
            side: Side::Buy,
            price: Price(100.0),
            qty: Quantity(5.0),
        }],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(43),
    };
    let position_update = PositionUpdate {
        account: "DEMO".to_string(),
        symbol: "AAPL".to_string(),
        qty: Quantity(10.0),
        entry_price: Price(100.0),
        unrealized_pnl: 0.0,
    };
    let order_hist_paged_frame = Frame::new(FrameType::Request, 5)
        .with_seq(1)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEMO/orders",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-DEMO"))
        .with_payload(encode_cbor(&order_hist_paged_req)?);
    let open_orders_req = OpenOrdersRequest {
        account: "DEMO".to_string(),
        symbol: None,
    };
    let caps_query_frame = Frame::new(FrameType::Request, 3)
        .with_seq(1)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            ".well-known/capabilities",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"));
    let open_orders_frame = Frame::new(FrameType::Request, 4)
        .with_seq(1)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/DEMO/orders/open",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_payload(encode_cbor(&open_orders_req)?);
    let subscribe_frame = Frame::new(FrameType::Subscribe, 1)
        .with_seq(1)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/quotes",
        ))
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            "marketdata.AAPL.quotes",
        ))
        .with_extension(Extension::text(ExtensionTag::CorrelationId, "1"));
    let candle_batch = CandleBarBatch {
        symbol: "AAPL".to_string(),
        interval: "5m".to_string(),
        bars: vec![candle.clone()],
        has_more: false,
        next_cursor: None,
    };
    let symbol_ticker = SymbolTicker {
        symbol: "AAPL".to_string(),
        last_price: Price(150.5),
        price_change: 0.5,
        price_change_pct: 0.33,
        volume: Quantity(1000.0),
        high: Price(151.0),
        low: Price(149.5),
        open: Price(150.0),
        timestamp: 1_700_000_000_000_000_000,
        is_snapshot: Some(true),
    };
    let book_snap = OrderBookSnapshot {
        symbol: "AAPL".to_string(),
        exchange: "SIM".to_string(),
        bids: vec![PriceLevel {
            price: Price(100.0),
            qty: Quantity(5.0),
            order_count: Some(1),
        }],
        asks: vec![],
        timestamp: 1_700_000_000_000_000_000,
        sequence: Some(42),
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
                payload: Some(candle_payload.clone()),
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
                id: "cbor.capabilities_response.demo".into(),
                category: "cbor".into(),
                description: "CapabilitiesResponse CBOR".into(),
                message_type: "CapabilitiesResponse".into(),
                expected_hex: hex::encode(encode_cbor(&caps)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.open_orders_snapshot.demo".into(),
                category: "cbor".into(),
                description: "OpenOrdersSnapshot CBOR".into(),
                message_type: "OpenOrdersSnapshot".into(),
                expected_hex: hex::encode(encode_cbor(&open_orders)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.market_data_snapshot.demo".into(),
                category: "cbor".into(),
                description: "MarketDataSnapshot CBOR with sequence".into(),
                message_type: "MarketDataSnapshot".into(),
                expected_hex: hex::encode(encode_cbor(&md_snap)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.order_history_request.demo".into(),
                category: "cbor".into(),
                description: "OrderHistoryRequest CBOR".into(),
                message_type: "OrderHistoryRequest".into(),
                expected_hex: hex::encode(encode_cbor(&order_hist_req)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.order_history_batch.demo".into(),
                category: "cbor".into(),
                description: "OrderHistoryBatch CBOR with pagination cursor".into(),
                message_type: "OrderHistoryBatch".into(),
                expected_hex: hex::encode(encode_cbor(&order_hist_page)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.order_list_status.demo".into(),
                category: "cbor".into(),
                description: "OrderListStatus CBOR for order list stream".into(),
                message_type: "OrderListStatus".into(),
                expected_hex: hex::encode(encode_cbor(&order_list_status)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "frame.request.capabilities".into(),
                category: "frame".into(),
                description: "GET capabilities discovery path".into(),
                message_type: "CapabilitiesRequest".into(),
                expected_hex: hex::encode(caps_query_frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Request".into(),
                    channel_id: 3,
                    stream_seq: 1,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text(".well-known/capabilities".into()),
                        },
                        ExtensionSpec {
                            tag: "Method".into(),
                            value: ExtensionValueSpec::Text("GET".into()),
                        },
                    ],
                    payload_hex: None,
                }),
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "frame.request.open_orders".into(),
                category: "frame".into(),
                description: "GET open orders query".into(),
                message_type: "OpenOrdersRequest".into(),
                expected_hex: hex::encode(open_orders_frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Request".into(),
                    channel_id: 4,
                    stream_seq: 1,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text(
                                "trading/accounts/DEMO/orders/open".into(),
                            ),
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
                    payload_hex: Some(hex::encode(encode_cbor(&open_orders_req)?)),
                }),
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "frame.subscribe.quotes".into(),
                category: "frame".into(),
                description: "SUBSCRIBE to order book quotes".into(),
                message_type: String::new(),
                expected_hex: hex::encode(subscribe_frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Subscribe".into(),
                    channel_id: 1,
                    stream_seq: 1,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text("marketdata/AAPL/quotes".into()),
                        },
                        ExtensionSpec {
                            tag: "RoutingKey".into(),
                            value: ExtensionValueSpec::Text("marketdata.AAPL.quotes".into()),
                        },
                        ExtensionSpec {
                            tag: "CorrelationId".into(),
                            value: ExtensionValueSpec::Text("1".into()),
                        },
                    ],
                    payload_hex: None,
                }),
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
                id: "cbor.order_book_snapshot.demo".into(),
                category: "cbor".into(),
                description: "OrderBookSnapshot CBOR (snapshot pair)".into(),
                message_type: "OrderBookSnapshot".into(),
                expected_hex: hex::encode(encode_cbor(&OrderBookSnapshot {
                    symbol: "AAPL".to_string(),
                    exchange: "SIM".to_string(),
                    bids: vec![PriceLevel {
                        price: Price(100.0),
                        qty: Quantity(5.0),
                        order_count: Some(1),
                    }],
                    asks: vec![],
                    timestamp: 1_700_000_000_000_000_000,
                    sequence: Some(42),
                    is_snapshot: Some(true),
                })?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.order_book_delta.demo".into(),
                category: "cbor".into(),
                description: "OrderBookDelta CBOR (delta pair)".into(),
                message_type: "OrderBookDelta".into(),
                expected_hex: hex::encode(encode_cbor(&book_delta)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.position_update.demo".into(),
                category: "cbor".into(),
                description: "PositionUpdate CBOR for private stream conformance".into(),
                message_type: "PositionUpdate".into(),
                expected_hex: hex::encode(encode_cbor(&position_update)?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "frame.request.order_history_paginated".into(),
                category: "frame".into(),
                description: "GET order history with cursor pagination".into(),
                message_type: "OrderHistoryRequest".into(),
                expected_hex: hex::encode(order_hist_paged_frame.encode()?),
                frame: Some(FrameSpec {
                    frame_type: "Request".into(),
                    channel_id: 5,
                    stream_seq: 1,
                    schema_id: 0x01,
                    extensions: vec![
                        ExtensionSpec {
                            tag: "ChannelPath".into(),
                            value: ExtensionValueSpec::Text("trading/accounts/DEMO/orders".into()),
                        },
                        ExtensionSpec {
                            tag: "Method".into(),
                            value: ExtensionValueSpec::Text("GET".into()),
                        },
                        ExtensionSpec {
                            tag: "ContentType".into(),
                            value: ExtensionValueSpec::Text("application/cbor".into()),
                        },
                        ExtensionSpec {
                            tag: "AuthToken".into(),
                            value: ExtensionValueSpec::Text("fig-dev-DEMO".into()),
                        },
                    ],
                    payload_hex: Some(hex::encode(encode_cbor(&order_hist_paged_req)?)),
                }),
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.aggregate_trade.demo".into(),
                category: "cbor".into(),
                description: "AggregateTrade CBOR".into(),
                message_type: "AggregateTrade".into(),
                expected_hex: hex::encode(encode_cbor(&AggregateTrade {
                    symbol: "AAPL".to_string(),
                    agg_trade_id: "A-1".to_string(),
                    price: Price(100.0),
                    qty: Quantity(5.0),
                    side: Side::Buy,
                    first_trade_id: "T-1".to_string(),
                    last_trade_id: "T-1".to_string(),
                    timestamp: 1_700_000_000_000_000_000,
                })?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.mini_ticker.demo".into(),
                category: "cbor".into(),
                description: "MiniTicker CBOR".into(),
                message_type: "MiniTicker".into(),
                expected_hex: hex::encode(encode_cbor(&MiniTicker {
                    symbol: "AAPL".to_string(),
                    last_price: Price(100.0),
                    volume: Quantity(50.0),
                    timestamp: 1_700_000_000_000_000_000,
                    is_snapshot: Some(true),
                })?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "cbor.mark_price_update.demo".into(),
                category: "cbor".into(),
                description: "MarkPriceUpdate CBOR".into(),
                message_type: "MarkPriceUpdate".into(),
                expected_hex: hex::encode(encode_cbor(&MarkPriceUpdate {
                    symbol: "AAPL".to_string(),
                    mark_price: Price(100.01),
                    index_price: Some(Price(100.0)),
                    funding_rate: Some(0.0001),
                    timestamp: 1_700_000_000_000_000_000,
                    is_snapshot: Some(true),
                })?),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.candle_bar.snapshot".into(),
                category: "sbe".into(),
                description: "CandleBar SBE (template 21)".into(),
                message_type: "CandleBar".into(),
                expected_hex: hex::encode(encode_candle_bar(&candle)),
                frame: None,
                payload: Some(candle_payload.clone()),
                channel: None,
            },
            ConformanceVector {
                id: "sbe.candle_bar_batch.demo".into(),
                category: "sbe".into(),
                description: "CandleBarBatch SBE (template 22)".into(),
                message_type: "CandleBarBatch".into(),
                expected_hex: hex::encode(encode_candle_bar_batch(&candle_batch)),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.symbol_ticker.demo".into(),
                category: "sbe".into(),
                description: "SymbolTicker SBE (template 27)".into(),
                message_type: "SymbolTicker".into(),
                expected_hex: hex::encode(encode_symbol_ticker(&symbol_ticker)),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_book_snapshot.demo".into(),
                category: "sbe".into(),
                description: "OrderBookSnapshot SBE (template 9)".into(),
                message_type: "OrderBookSnapshot".into(),
                expected_hex: hex::encode(encode_order_book_snapshot(&book_snap)),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.balance_snapshot.demo".into(),
                category: "sbe".into(),
                description: "BalanceSnapshot SBE (template 30)".into(),
                message_type: "BalanceSnapshot".into(),
                expected_hex: hex::encode(encode_balance_snapshot(&balance_snap)),
                frame: None,
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
