//! WebSocket topic catalog: legacy broker topics → native FIG `SUBSCRIBE` (§17.5 / §17.8).
//!
//! Binance combined-stream topics (`symbol@kline_5m`, `@trade`, `@bookTicker`) and
//! Hyperliquid subscription JSON map to the same native FIG channel paths and FSL
//! payloads used by TREE clients.

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::schema_id;
use serde_json::Value;

use crate::ws::{WsError, WsResult};

/// Parsed legacy subscribe intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacySubscribe {
    pub channel_path: String,
    pub routing_key: String,
}

/// Map a Binance-style stream name to native FIG subscribe paths.
///
/// Examples:
/// - `btcusdt@kline_5m` → `marketdata/btcusdt/candles/5m`
/// - `ethusdt@trade` → `marketdata/ethusdt/trades`
/// - `btcusdt@bookTicker` → `marketdata/btcusdt/bbo`
/// - `btcusdt@depth` → `marketdata/btcusdt/quotes`
pub fn binance_topic_to_subscribe(topic: &str) -> Option<LegacySubscribe> {
    let (symbol, stream) = topic.split_once('@')?;
    let symbol = symbol.to_lowercase();
    let channel_path = if let Some(interval) = stream.strip_prefix("kline_") {
        format!("marketdata/{symbol}/candles/{interval}")
    } else if stream == "trade" || stream == "aggTrade" {
        format!("marketdata/{symbol}/trades")
    } else if stream == "bookTicker" {
        format!("marketdata/{symbol}/bbo")
    } else if stream == "ticker" {
        format!("marketdata/{symbol}/ticker")
    } else if stream == "depth" || stream.starts_with("depth@") {
        format!("marketdata/{symbol}/quotes")
    } else {
        return None;
    };
    Some(LegacySubscribe {
        routing_key: topic.to_string(),
        channel_path,
    })
}

/// Map Hyperliquid-style subscription JSON to native FIG subscribe paths.
pub fn hyperliquid_subscribe_to_fig(sub: &Value) -> Option<LegacySubscribe> {
    let sub_type = sub.get("type")?.as_str()?;
    let coin = sub.get("coin").and_then(|c| c.as_str()).unwrap_or("BTC");
    let symbol = coin.to_uppercase();
    let (channel_path, routing_key) = match sub_type {
        "trades" => (
            format!("marketdata/{symbol}/trades"),
            format!("hl.trades.{symbol}"),
        ),
        "candle" => {
            let interval = sub
                .get("interval")
                .and_then(|i| i.as_str())
                .unwrap_or("5m");
            (
                format!("marketdata/{symbol}/candles/{interval}"),
                format!("hl.candle.{symbol}.{interval}"),
            )
        }
        "l2Book" | "bbo" => (
            format!("marketdata/{symbol}/quotes"),
            format!("hl.l2Book.{symbol}"),
        ),
        "orderUpdates" | "userFills" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("trading/accounts/{user}/executions"),
                format!("hl.orderUpdates.{user}"),
            )
        },
        "subscribeBalance" | "spotState" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/balances"),
                format!("hl.balances.{user}"),
            )
        },
        "subscribePosition" | "clearinghouseState" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/positions"),
                format!("hl.positions.{user}"),
            )
        },
        _ => return None,
    };
    Some(LegacySubscribe {
        channel_path,
        routing_key,
    })
}

/// Parse legacy WebSocket subscribe JSON into a FIG `SUBSCRIBE` frame.
pub fn legacy_ws_json_to_fig_subscribe(json: &str, channel_id: u16) -> WsResult<Frame> {
    let value: Value =
        serde_json::from_str(json).map_err(|e| WsError::UnmappableFrameType(e.to_string()))?;

    let sub = if value.get("params").is_some() {
        // Binance: {"method":"SUBSCRIBE","params":["btcusdt@kline_5m"]}
        let params = value
            .get("params")
            .and_then(|p| p.as_array())
            .ok_or_else(|| WsError::UnmappableFrameType("missing params".into()))?;
        let topic = params
            .first()
            .and_then(|t| t.as_str())
            .ok_or_else(|| WsError::UnmappableFrameType("empty params".into()))?;
        binance_topic_to_subscribe(topic)
            .ok_or_else(|| WsError::UnmappableFrameType(format!("unknown topic: {topic}")))?
    } else if value.get("subscription").is_some() {
        // Hyperliquid: {"method":"subscribe","subscription":{"type":"trades","coin":"BTC"}}
        let sub_obj = value.get("subscription").unwrap();
        hyperliquid_subscribe_to_fig(sub_obj)
            .ok_or_else(|| WsError::UnmappableFrameType("unknown subscription".into()))?
    } else if let Some(path) = value.get("channel_path").and_then(|p| p.as_str()) {
        // Native FIG-over-WS: {"channel_path":"marketdata/BTC/candles/5m"}
        LegacySubscribe {
            channel_path: path.to_string(),
            routing_key: value
                .get("routing_key")
                .and_then(|r| r.as_str())
                .unwrap_or(path)
                .to_string(),
        }
    } else {
        return Err(WsError::UnmappableFrameType(
            "unsupported subscribe JSON".into(),
        ));
    };

    Ok(fig_subscribe_frame(channel_id, &sub))
}

/// Build a FIG `SUBSCRIBE` frame from native paths.
pub fn fig_subscribe_frame(channel_id: u16, sub: &LegacySubscribe) -> Frame {
    Frame::new(FrameType::Subscribe, channel_id)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(
            ExtensionTag::RoutingKey,
            &sub.routing_key,
        ))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            &sub.channel_path,
        ))
}

/// Wrap a FIG `STREAM_ITEM` CBOR payload as legacy WS JSON for egress adapters.
pub fn fig_stream_item_to_legacy_json(frame: &Frame) -> WsResult<String> {
    let routing_key = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::RoutingKey)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");
    let channel_path = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");

    let payload_json = if frame.payload.is_empty() {
        Value::Null
    } else {
        let json_str = fig_core::codec::cbor_to_json(&frame.payload)
            .map_err(|e| WsError::UnmappableFrameType(e.to_string()))?;
        serde_json::from_str(&json_str)
            .map_err(|e| WsError::UnmappableFrameType(e.to_string()))?
    };

    let event = serde_json::json!({
        "stream": routing_key,
        "channel_path": channel_path,
        "data": payload_json,
    });
    serde_json::to_string(&event).map_err(|e| WsError::UnmappableFrameType(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binance_kline_topic_maps_to_candles_path() {
        let sub = binance_topic_to_subscribe("btcusdt@kline_5m").unwrap();
        assert_eq!(sub.channel_path, "marketdata/btcusdt/candles/5m");
        assert_eq!(sub.routing_key, "btcusdt@kline_5m");
    }

    #[test]
    fn hyperliquid_trades_subscription() {
        let sub = hyperliquid_subscribe_to_fig(&serde_json::json!({"type":"trades","coin":"BTC"}))
            .unwrap();
        assert_eq!(sub.channel_path, "marketdata/BTC/trades");
    }

    #[test]
    fn legacy_json_to_fig_subscribe() {
        let json = r#"{"method":"SUBSCRIBE","params":["ethusdt@trade"]}"#;
        let frame = legacy_ws_json_to_fig_subscribe(json, 2).unwrap();
        assert_eq!(frame.frame_type, FrameType::Subscribe);
        assert_eq!(frame.channel_id, 2);
    }
}
