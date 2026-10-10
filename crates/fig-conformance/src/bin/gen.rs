use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use fig_conformance::{
    sample_account_summary, sample_aggregate_trade_batch, sample_aggregate_trade_event,
    sample_aggregate_trade_request, sample_all_mids_batch, sample_all_mids_request,
    sample_balance_update, sample_best_bid_offer, sample_cancel_reject,
    sample_cancel_replace_request, sample_cancel_request, sample_candle_bar_event,
    sample_candle_bar_request, sample_capabilities_request, sample_execution_report,
    sample_fill_history_batch, sample_fill_history_request, sample_funding_history_batch,
    sample_funding_history_request, sample_funding_payment, sample_instrument_catalog_request,
    sample_instrument_catalog_response, sample_ledger_history_batch, sample_ledger_history_request,
    sample_ledger_update, sample_liquidation_trade_event, sample_margin_summary,
    sample_margin_update, sample_mark_price_request, sample_mark_price_update,
    sample_market_data_incremental_refresh, sample_market_data_snapshot, sample_mini_ticker,
    sample_open_orders_request, sample_open_orders_snapshot, sample_order_book_delta,
    sample_order_book_request, sample_order_history_batch, sample_order_history_request,
    sample_order_list_status, sample_position_request, sample_position_snapshot,
    sample_position_update, sample_public_trade_batch, sample_public_trade_event,
    sample_ticker_request, sample_trade_history_request, sample_user_liquidation,
};
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
    InstrumentCatalogResponse, InstrumentMetadata, MarkPriceUpdate, MarketDataAction,
    MarketDataSnapshot, MarketDataUpdate, MiniTicker, NewOrderSingle, OpenOrdersRequest,
    OpenOrdersSnapshot, OrdStatus, OrderBookDelta, OrderBookSnapshot, OrderHistoryBatch,
    OrderHistoryRequest, OrderListStatus, OrderListStatusStatus, OrderType, PositionUpdate, Price,
    PriceLevel, Quantity, Side, SymbolTicker, TimeInForce,
};
use fig_core::sbe::{
    encode_account_summary, encode_aggregate_trade_batch, encode_aggregate_trade_event,
    encode_aggregate_trade_request, encode_all_mids_batch, encode_all_mids_request,
    encode_balance_snapshot, encode_balance_update, encode_best_bid_offer, encode_cancel_reject,
    encode_cancel_replace_request, encode_cancel_request, encode_candle_bar,
    encode_candle_bar_batch, encode_candle_bar_event, encode_candle_bar_request,
    encode_capabilities_request, encode_capabilities_response, encode_execution_report,
    encode_fill_history_batch, encode_fill_history_request, encode_funding_history_batch,
    encode_funding_history_request, encode_funding_payment, encode_instrument_catalog_request,
    encode_instrument_catalog_response, encode_ledger_history_batch, encode_ledger_history_request,
    encode_ledger_update, encode_liquidation_trade_event, encode_margin_summary,
    encode_margin_update, encode_mark_price_request, encode_mark_price_update,
    encode_market_data_incremental_refresh, encode_market_data_snapshot, encode_mini_ticker,
    encode_new_order_single, encode_open_orders_request, encode_open_orders_snapshot,
    encode_order_book_delta, encode_order_book_request, encode_order_book_snapshot,
    encode_order_history_batch, encode_order_history_request, encode_order_list_status,
    encode_position_request, encode_position_snapshot, encode_position_update,
    encode_public_trade_batch, encode_public_trade_event, encode_symbol_ticker,
    encode_ticker_request, encode_trade_history_request, encode_user_liquidation,
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
                id: "sbe.mark_price_update.demo".into(),
                category: "sbe".into(),
                description: "MarkPriceUpdate SBE".into(),
                message_type: "MarkPriceUpdate".into(),
                expected_hex: hex::encode(encode_mark_price_update(&sample_mark_price_update())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.market_data_snapshot.demo".into(),
                category: "sbe".into(),
                description: "MarketDataSnapshot SBE".into(),
                message_type: "MarketDataSnapshot".into(),
                expected_hex: hex::encode(encode_market_data_snapshot(&sample_market_data_snapshot())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.mini_ticker.demo".into(),
                category: "sbe".into(),
                description: "MiniTicker SBE".into(),
                message_type: "MiniTicker".into(),
                expected_hex: hex::encode(encode_mini_ticker(&sample_mini_ticker())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.open_orders_snapshot.demo".into(),
                category: "sbe".into(),
                description: "OpenOrdersSnapshot SBE".into(),
                message_type: "OpenOrdersSnapshot".into(),
                expected_hex: hex::encode(encode_open_orders_snapshot(&sample_open_orders_snapshot())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_book_delta.demo".into(),
                category: "sbe".into(),
                description: "OrderBookDelta SBE".into(),
                message_type: "OrderBookDelta".into(),
                expected_hex: hex::encode(encode_order_book_delta(&sample_order_book_delta())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_history_batch.demo".into(),
                category: "sbe".into(),
                description: "OrderHistoryBatch SBE".into(),
                message_type: "OrderHistoryBatch".into(),
                expected_hex: hex::encode(encode_order_history_batch(&sample_order_history_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_history_request.demo".into(),
                category: "sbe".into(),
                description: "OrderHistoryRequest SBE".into(),
                message_type: "OrderHistoryRequest".into(),
                expected_hex: hex::encode(encode_order_history_request(&sample_order_history_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_list_status.demo".into(),
                category: "sbe".into(),
                description: "OrderListStatus SBE".into(),
                message_type: "OrderListStatus".into(),
                expected_hex: hex::encode(encode_order_list_status(&sample_order_list_status())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.position_update.demo".into(),
                category: "sbe".into(),
                description: "PositionUpdate SBE".into(),
                message_type: "PositionUpdate".into(),
                expected_hex: hex::encode(encode_position_update(&sample_position_update())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.account_summary.demo".into(),
                category: "sbe".into(),
                description: "AccountSummary SBE".into(),
                message_type: "AccountSummary".into(),
                expected_hex: hex::encode(encode_account_summary(&sample_account_summary())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.aggregate_trade_batch.demo".into(),
                category: "sbe".into(),
                description: "AggregateTradeBatch SBE".into(),
                message_type: "AggregateTradeBatch".into(),
                expected_hex: hex::encode(encode_aggregate_trade_batch(&sample_aggregate_trade_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.aggregate_trade_event.demo".into(),
                category: "sbe".into(),
                description: "AggregateTradeEvent SBE".into(),
                message_type: "AggregateTradeEvent".into(),
                expected_hex: hex::encode(encode_aggregate_trade_event(&sample_aggregate_trade_event())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.aggregate_trade_request.demo".into(),
                category: "sbe".into(),
                description: "AggregateTradeRequest SBE".into(),
                message_type: "AggregateTradeRequest".into(),
                expected_hex: hex::encode(encode_aggregate_trade_request(&sample_aggregate_trade_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.all_mids_batch.demo".into(),
                category: "sbe".into(),
                description: "AllMidsBatch SBE".into(),
                message_type: "AllMidsBatch".into(),
                expected_hex: hex::encode(encode_all_mids_batch(&sample_all_mids_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.all_mids_request.demo".into(),
                category: "sbe".into(),
                description: "AllMidsRequest SBE".into(),
                message_type: "AllMidsRequest".into(),
                expected_hex: hex::encode(encode_all_mids_request(&sample_all_mids_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.balance_update.demo".into(),
                category: "sbe".into(),
                description: "BalanceUpdate SBE".into(),
                message_type: "BalanceUpdate".into(),
                expected_hex: hex::encode(encode_balance_update(&sample_balance_update())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.best_bid_offer.demo".into(),
                category: "sbe".into(),
                description: "BestBidOffer SBE".into(),
                message_type: "BestBidOffer".into(),
                expected_hex: hex::encode(encode_best_bid_offer(&sample_best_bid_offer())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.cancel_reject.demo".into(),
                category: "sbe".into(),
                description: "CancelReject SBE".into(),
                message_type: "CancelReject".into(),
                expected_hex: hex::encode(encode_cancel_reject(&sample_cancel_reject())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.cancel_replace_request.demo".into(),
                category: "sbe".into(),
                description: "CancelReplaceRequest SBE".into(),
                message_type: "CancelReplaceRequest".into(),
                expected_hex: hex::encode(encode_cancel_replace_request(&sample_cancel_replace_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.cancel_request.demo".into(),
                category: "sbe".into(),
                description: "CancelRequest SBE".into(),
                message_type: "CancelRequest".into(),
                expected_hex: hex::encode(encode_cancel_request(&sample_cancel_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.candle_bar_event.demo".into(),
                category: "sbe".into(),
                description: "CandleBarEvent SBE".into(),
                message_type: "CandleBarEvent".into(),
                expected_hex: hex::encode(encode_candle_bar_event(&sample_candle_bar_event())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.candle_bar_request.demo".into(),
                category: "sbe".into(),
                description: "CandleBarRequest SBE".into(),
                message_type: "CandleBarRequest".into(),
                expected_hex: hex::encode(encode_candle_bar_request(&sample_candle_bar_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.capabilities_request.demo".into(),
                category: "sbe".into(),
                description: "CapabilitiesRequest SBE".into(),
                message_type: "CapabilitiesRequest".into(),
                expected_hex: hex::encode(encode_capabilities_request(&sample_capabilities_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.execution_report.demo".into(),
                category: "sbe".into(),
                description: "ExecutionReport SBE".into(),
                message_type: "ExecutionReport".into(),
                expected_hex: hex::encode(encode_execution_report(&sample_execution_report())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.fill_history_batch.demo".into(),
                category: "sbe".into(),
                description: "FillHistoryBatch SBE".into(),
                message_type: "FillHistoryBatch".into(),
                expected_hex: hex::encode(encode_fill_history_batch(&sample_fill_history_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.fill_history_request.demo".into(),
                category: "sbe".into(),
                description: "FillHistoryRequest SBE".into(),
                message_type: "FillHistoryRequest".into(),
                expected_hex: hex::encode(encode_fill_history_request(&sample_fill_history_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.funding_history_batch.demo".into(),
                category: "sbe".into(),
                description: "FundingHistoryBatch SBE".into(),
                message_type: "FundingHistoryBatch".into(),
                expected_hex: hex::encode(encode_funding_history_batch(&sample_funding_history_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.funding_history_request.demo".into(),
                category: "sbe".into(),
                description: "FundingHistoryRequest SBE".into(),
                message_type: "FundingHistoryRequest".into(),
                expected_hex: hex::encode(encode_funding_history_request(&sample_funding_history_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.funding_payment.demo".into(),
                category: "sbe".into(),
                description: "FundingPayment SBE".into(),
                message_type: "FundingPayment".into(),
                expected_hex: hex::encode(encode_funding_payment(&sample_funding_payment())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.instrument_catalog_request.demo".into(),
                category: "sbe".into(),
                description: "InstrumentCatalogRequest SBE".into(),
                message_type: "InstrumentCatalogRequest".into(),
                expected_hex: hex::encode(encode_instrument_catalog_request(&sample_instrument_catalog_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.instrument_catalog_response.demo".into(),
                category: "sbe".into(),
                description: "InstrumentCatalogResponse SBE".into(),
                message_type: "InstrumentCatalogResponse".into(),
                expected_hex: hex::encode(encode_instrument_catalog_response(&sample_instrument_catalog_response())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.ledger_history_batch.demo".into(),
                category: "sbe".into(),
                description: "LedgerHistoryBatch SBE".into(),
                message_type: "LedgerHistoryBatch".into(),
                expected_hex: hex::encode(encode_ledger_history_batch(&sample_ledger_history_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.ledger_history_request.demo".into(),
                category: "sbe".into(),
                description: "LedgerHistoryRequest SBE".into(),
                message_type: "LedgerHistoryRequest".into(),
                expected_hex: hex::encode(encode_ledger_history_request(&sample_ledger_history_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.ledger_update.demo".into(),
                category: "sbe".into(),
                description: "LedgerUpdate SBE".into(),
                message_type: "LedgerUpdate".into(),
                expected_hex: hex::encode(encode_ledger_update(&sample_ledger_update())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.liquidation_trade_event.demo".into(),
                category: "sbe".into(),
                description: "LiquidationTradeEvent SBE".into(),
                message_type: "LiquidationTradeEvent".into(),
                expected_hex: hex::encode(encode_liquidation_trade_event(&sample_liquidation_trade_event())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.margin_summary.demo".into(),
                category: "sbe".into(),
                description: "MarginSummary SBE".into(),
                message_type: "MarginSummary".into(),
                expected_hex: hex::encode(encode_margin_summary(&sample_margin_summary())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.margin_update.demo".into(),
                category: "sbe".into(),
                description: "MarginUpdate SBE".into(),
                message_type: "MarginUpdate".into(),
                expected_hex: hex::encode(encode_margin_update(&sample_margin_update())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.mark_price_request.demo".into(),
                category: "sbe".into(),
                description: "MarkPriceRequest SBE".into(),
                message_type: "MarkPriceRequest".into(),
                expected_hex: hex::encode(encode_mark_price_request(&sample_mark_price_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.market_data_incremental_refresh.demo".into(),
                category: "sbe".into(),
                description: "MarketDataIncrementalRefresh SBE".into(),
                message_type: "MarketDataIncrementalRefresh".into(),
                expected_hex: hex::encode(encode_market_data_incremental_refresh(&sample_market_data_incremental_refresh())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.open_orders_request.demo".into(),
                category: "sbe".into(),
                description: "OpenOrdersRequest SBE".into(),
                message_type: "OpenOrdersRequest".into(),
                expected_hex: hex::encode(encode_open_orders_request(&sample_open_orders_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.order_book_request.demo".into(),
                category: "sbe".into(),
                description: "OrderBookRequest SBE".into(),
                message_type: "OrderBookRequest".into(),
                expected_hex: hex::encode(encode_order_book_request(&sample_order_book_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.position_request.demo".into(),
                category: "sbe".into(),
                description: "PositionRequest SBE".into(),
                message_type: "PositionRequest".into(),
                expected_hex: hex::encode(encode_position_request(&sample_position_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.position_snapshot.demo".into(),
                category: "sbe".into(),
                description: "PositionSnapshot SBE".into(),
                message_type: "PositionSnapshot".into(),
                expected_hex: hex::encode(encode_position_snapshot(&sample_position_snapshot())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.public_trade_batch.demo".into(),
                category: "sbe".into(),
                description: "PublicTradeBatch SBE".into(),
                message_type: "PublicTradeBatch".into(),
                expected_hex: hex::encode(encode_public_trade_batch(&sample_public_trade_batch())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.public_trade_event.demo".into(),
                category: "sbe".into(),
                description: "PublicTradeEvent SBE".into(),
                message_type: "PublicTradeEvent".into(),
                expected_hex: hex::encode(encode_public_trade_event(&sample_public_trade_event())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.ticker_request.demo".into(),
                category: "sbe".into(),
                description: "TickerRequest SBE".into(),
                message_type: "TickerRequest".into(),
                expected_hex: hex::encode(encode_ticker_request(&sample_ticker_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.trade_history_request.demo".into(),
                category: "sbe".into(),
                description: "TradeHistoryRequest SBE".into(),
                message_type: "TradeHistoryRequest".into(),
                expected_hex: hex::encode(encode_trade_history_request(&sample_trade_history_request())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.user_liquidation.demo".into(),
                category: "sbe".into(),
                description: "UserLiquidation SBE".into(),
                message_type: "UserLiquidation".into(),
                expected_hex: hex::encode(encode_user_liquidation(&sample_user_liquidation())),
                frame: None,
                payload: None,
                channel: None,
            },
            ConformanceVector {
                id: "sbe.capabilities_response.demo".into(),
                category: "sbe".into(),
                description: "CapabilitiesResponse SBE (template 51) — list<uint8>, list<Symbol>, list<CandleInterval>".into(),
                message_type: "CapabilitiesResponse".into(),
                expected_hex: hex::encode(encode_capabilities_response(
                    &CapabilitiesResponse {
                        schema_ids: vec![1, 2, 3],
                        paths: vec![CapabilityPath {
                            path: "marketdata/AAPL/candles/5m".into(),
                            pattern: CapabilityPathPattern::PubSub,
                            auth_required: false,
                        }],
                        symbols: vec!["AAPL".into()],
                        intervals: vec!["5m".into()],
                    },
                )),
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
            {
                let catalog = InstrumentCatalogResponse {
                    instruments: vec![InstrumentMetadata {
                        instrument_id: "BTC-PERP".to_string(),
                        symbol: "BTC".to_string(),
                        product_kind: "perp".to_string(),
                        margin_asset: Some("USDC".to_string()),
                        display_name: Some("Bitcoin Perpetual".to_string()),
                        tick_size: Some(0.1),
                        lot_size: Some(0.001),
                    }],
                };
                ConformanceVector {
                    id: "cbor.instrument_catalog.btc_perp".into(),
                    category: "cbor".into(),
                    description: "InstrumentCatalogResponse with BTC perp".into(),
                    message_type: "InstrumentCatalogResponse".into(),
                    expected_hex: hex::encode(encode_cbor(&catalog)?),
                    frame: None,
                    payload: Some(serde_json::json!({
                        "instruments": [{
                            "instrument_id": "BTC-PERP",
                            "symbol": "BTC",
                            "product_kind": "perp",
                            "margin_asset": "USDC",
                            "display_name": "Bitcoin Perpetual",
                            "tick_size": 0.1,
                            "lot_size": 0.001
                        }]
                    })),
                    channel: None,
                }
            },
            {
                let post_only_order = NewOrderSingle {
                    post_only: Some(true),
                    reduce_only: None,
                    ..sample_order()
                };
                ConformanceVector {
                    id: "cbor.new_order_single.post_only".into(),
                    category: "cbor".into(),
                    description: "Limit buy with post_only=true".into(),
                    message_type: "NewOrderSingle".into(),
                    expected_hex: hex::encode(encode_cbor(&post_only_order)?),
                    frame: None,
                    payload: Some(serde_json::json!({
                        "cl_ord_id": "CONF-001",
                        "side": "Buy",
                        "order_qty": 100.0,
                        "price": 50.25,
                        "symbol": "AAPL",
                        "order_type": "Limit",
                        "time_in_force": "Day",
                        "post_only": true
                    })),
                    channel: None,
                }
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
        post_only: None,
        reduce_only: None,
    }
}
