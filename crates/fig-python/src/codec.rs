//! Shared encode/decode helpers for Python bindings and conformance tests.

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    CapabilitiesResponse, CapabilityPath, CapabilityPathPattern, InstrumentCatalogResponse,
    InstrumentMetadata, MarketDataSnapshot, NewOrderSingle, OpenOrdersRequest, OpenOrdersSnapshot,
    OrderHistoryRequest, OrderType, Price, PriceLevel, Quantity, Side, TimeInForce,
};

pub fn encode_new_order_single(
    cl_ord_id: &str,
    symbol: &str,
    side: Side,
    order_qty: f64,
    price: Option<f64>,
    post_only: Option<bool>,
    reduce_only: Option<bool>,
) -> Result<Vec<u8>, String> {
    let order = NewOrderSingle {
        cl_ord_id: cl_ord_id.to_string(),
        side,
        order_qty: Quantity(order_qty),
        price: price.map(Price),
        stop_price: None,
        symbol: symbol.to_string(),
        order_type: if price.is_some() {
            OrderType::Limit
        } else {
            OrderType::Market
        },
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only,
        reduce_only,
    };
    encode_cbor(&order).map_err(|e| e.to_string())
}

pub fn decode_new_order_cl_ord_id(data: &[u8]) -> Result<String, String> {
    let order: NewOrderSingle = decode_cbor(data).map_err(|e| e.to_string())?;
    Ok(order.cl_ord_id)
}

#[allow(clippy::too_many_arguments)]
pub fn encode_request_frame(
    channel_id: u16,
    stream_seq: u32,
    schema_id: u8,
    channel_path: Option<&str>,
    method: Option<&str>,
    content_type: Option<&str>,
    payload: Option<&[u8]>,
    auth_token: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut frame = Frame::new(FrameType::Request, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(schema_id);
    if let Some(path) = channel_path {
        frame = frame.with_extension(Extension::text(ExtensionTag::ChannelPath, path));
    }
    if let Some(method) = method {
        frame = frame.with_extension(Extension::text(ExtensionTag::Method, method));
    }
    if let Some(ct) = content_type {
        frame = frame.with_extension(Extension::text(ExtensionTag::ContentType, ct));
    }
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    if let Some(payload) = payload {
        frame = frame.with_payload(payload.to_vec());
    }
    frame.encode().map_err(|e| e.to_string())
}

pub fn encode_subscribe_frame(
    channel_id: u16,
    stream_seq: u32,
    routing_key: &str,
    channel_path: &str,
    auth_token: Option<&str>,
) -> Result<Vec<u8>, String> {
    let mut frame = Frame::new(FrameType::Subscribe, channel_id)
        .with_seq(stream_seq)
        .with_schema_id(1)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, channel_path))
        .with_extension(Extension::text(ExtensionTag::RoutingKey, routing_key))
        .with_extension(Extension::text(
            ExtensionTag::CorrelationId,
            stream_seq.to_string(),
        ));
    if let Some(token) = auth_token {
        frame = frame.with_extension(Extension::text(ExtensionTag::AuthToken, token));
    }
    frame.encode().map_err(|e| e.to_string())
}

pub fn encode_ping_frame() -> Result<Vec<u8>, String> {
    Frame::ping().encode().map_err(|e| e.to_string())
}

pub fn frame_decode_header(data: &[u8]) -> Result<(u16, u32, u8), String> {
    let (frame, _) = Frame::decode(data).map_err(|e| e.to_string())?;
    Ok((frame.channel_id, frame.stream_seq, frame.schema_id))
}

pub fn channel_stream_id(channel_id: u16, is_server: bool) -> u64 {
    if is_server {
        channel_id as u64 * 4 + 1
    } else {
        channel_id as u64 * 4
    }
}

pub fn sample_capabilities_response() -> CapabilitiesResponse {
    CapabilitiesResponse {
        schema_ids: vec![1, 2, 3],
        paths: vec![CapabilityPath {
            path: "marketdata/AAPL/candles/5m".to_string(),
            pattern: CapabilityPathPattern::PubSub,
            auth_required: false,
        }],
        symbols: vec!["AAPL".to_string()],
        intervals: vec!["5m".to_string()],
    }
}

pub fn sample_open_orders_snapshot() -> OpenOrdersSnapshot {
    OpenOrdersSnapshot {
        account: "DEMO".to_string(),
        orders: vec![],
        is_snapshot: Some(true),
    }
}

pub fn sample_market_data_snapshot() -> MarketDataSnapshot {
    MarketDataSnapshot {
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
    }
}

pub fn sample_order_history_request() -> OrderHistoryRequest {
    OrderHistoryRequest {
        account: "DEMO".to_string(),
        symbol: None,
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    }
}

pub fn sample_open_orders_request() -> OpenOrdersRequest {
    OpenOrdersRequest {
        account: "DEMO".to_string(),
        symbol: None,
    }
}

pub fn sample_instrument_catalog_response() -> InstrumentCatalogResponse {
    InstrumentCatalogResponse {
        instruments: vec![InstrumentMetadata {
            instrument_id: "BTC-PERP".to_string(),
            symbol: "BTC".to_string(),
            product_kind: "perp".to_string(),
            margin_asset: Some("USDC".to_string()),
            display_name: Some("Bitcoin Perpetual".to_string()),
            tick_size: Some(0.1),
            lot_size: Some(0.001),
        }],
    }
}

pub fn parse_side(s: &str) -> Result<Side, String> {
    match s {
        "Buy" | "buy" => Ok(Side::Buy),
        "Sell" | "sell" => Ok(Side::Sell),
        "SellShort" | "sell_short" => Ok(Side::SellShort),
        "SellShortExempt" | "sell_short_exempt" => Ok(Side::SellShortExempt),
        _ => Err(format!("unknown side: {s}")),
    }
}
