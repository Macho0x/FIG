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
/// - `btcusdt@depth` → `marketdata/btcusdt/book`
/// - `orderlists@DEMO` → `trading/accounts/DEMO/orderlists`
pub fn binance_topic_to_subscribe(topic: &str) -> Option<LegacySubscribe> {
    if let Some((user, _)) = topic.split_once("@orderlists") {
        let account = user.to_uppercase();
        return Some(LegacySubscribe {
            routing_key: topic.to_string(),
            channel_path: format!("trading/accounts/{account}/orderlists"),
        });
    }
    if let Some((user, _)) = topic.split_once("@executionReport") {
        let account = user.to_uppercase();
        return Some(LegacySubscribe {
            routing_key: topic.to_string(),
            channel_path: format!("trading/accounts/{account}/executions"),
        });
    }
    if let Some((user, _)) = topic.split_once("@balance") {
        let account = user.to_uppercase();
        return Some(LegacySubscribe {
            routing_key: topic.to_string(),
            channel_path: format!("accounts/{account}/balances"),
        });
    }
    if let Some((user, _)) = topic.split_once("@positions") {
        let account = user.to_uppercase();
        return Some(LegacySubscribe {
            routing_key: topic.to_string(),
            channel_path: format!("accounts/{account}/positions"),
        });
    }

    let (symbol, stream) = topic.split_once('@')?;
    let symbol = symbol.to_lowercase();
    let channel_path = if let Some(interval) = stream.strip_prefix("kline_") {
        format!("marketdata/{symbol}/candles/{interval}")
    } else if stream == "trade" {
        format!("marketdata/{symbol}/trades")
    } else if stream == "aggTrade" {
        format!("marketdata/{symbol}/aggtrades")
    } else if stream == "bookTicker" {
        format!("marketdata/{symbol}/bbo")
    } else if stream == "ticker" {
        format!("marketdata/{symbol}/ticker")
    } else if stream == "miniTicker" {
        "marketdata/ticker/all".to_string()
    } else if stream == "markPrice" {
        format!("marketdata/{symbol}/mark")
    } else if stream == "forceOrder" {
        "marketdata/liquidations".to_string()
    } else if stream == "depth" || stream.starts_with("depth@") {
        format!("marketdata/{symbol}/book")
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
            let interval = sub.get("interval").and_then(|i| i.as_str()).unwrap_or("5m");
            (
                format!("marketdata/{symbol}/candles/{interval}"),
                format!("hl.candle.{symbol}.{interval}"),
            )
        }
        "l2Book" | "bbo" => (
            format!("marketdata/{symbol}/book"),
            format!("hl.l2Book.{symbol}"),
        ),
        "orderUpdates" | "userFills" | "userEvents" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("trading/accounts/{user}/executions"),
                format!("hl.orderUpdates.{user}"),
            )
        }
        "openOrders" | "orderState" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("trading/accounts/{user}/orders/open"),
                format!("hl.openOrders.{user}"),
            )
        }
        "orderLists" | "listStatus" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("trading/accounts/{user}/orderlists"),
                format!("hl.orderLists.{user}"),
            )
        }
        "balanceUpdate" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/balances"),
                format!("hl.balanceUpdate.{user}"),
            )
        }
        "subscribeBalance" | "spotState" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/balances"),
                format!("hl.balances.{user}"),
            )
        }
        "subscribePosition" | "clearinghouseState" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/positions"),
                format!("hl.positions.{user}"),
            )
        }
        "userFunding" | "fundingHistory" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/funding"),
                format!("hl.funding.{user}"),
            )
        }
        "userNonFundingLedgerUpdates" | "ledgerUpdates" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/ledger"),
                format!("hl.ledger.{user}"),
            )
        }
        "liquidation" | "userLiquidation" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/liquidations"),
                format!("hl.liquidation.{user}"),
            )
        }
        "activeAssetCtx" | "markPrice" => (
            format!("marketdata/{symbol}/mark"),
            format!("hl.mark.{symbol}"),
        ),
        "allMids" | "miniTicker" => (
            "marketdata/ticker/all".to_string(),
            "hl.allMids".to_string(),
        ),
        "aggTrades" => (
            format!("marketdata/{symbol}/aggtrades"),
            format!("hl.aggTrades.{symbol}"),
        ),
        "liquidations" | "forceOrder" => (
            "marketdata/liquidations".to_string(),
            "hl.liquidations".to_string(),
        ),
        "margin" | "clearinghouseMargin" => {
            let user = sub
                .get("user")
                .and_then(|u| u.as_str())
                .unwrap_or("default");
            (
                format!("accounts/{user}/margin"),
                format!("hl.margin.{user}"),
            )
        }
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
        .with_extension(Extension::text(ExtensionTag::RoutingKey, &sub.routing_key))
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            &sub.channel_path,
        ))
        .with_extension(Extension::text(
            ExtensionTag::CorrelationId,
            channel_id.to_string(),
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
    let correlation_id = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::CorrelationId)
        .and_then(|e| e.value.as_text())
        .unwrap_or("");

    let payload_json = if frame.payload.is_empty() {
        Value::Null
    } else {
        let json_str = fig_core::codec::cbor_to_json(&frame.payload)
            .map_err(|e| WsError::UnmappableFrameType(e.to_string()))?;
        serde_json::from_str(&json_str).map_err(|e| WsError::UnmappableFrameType(e.to_string()))?
    };

    let is_snapshot = payload_json
        .get("is_snapshot")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let event = serde_json::json!({
        "stream": routing_key,
        "channel_path": channel_path,
        "subscription_id": if correlation_id.is_empty() { frame.channel_id.to_string() } else { correlation_id.to_string() },
        "is_snapshot": is_snapshot,
        "data": payload_json,
    });
    serde_json::to_string(&event).map_err(|e| WsError::UnmappableFrameType(e.to_string()))
}

/// Map FIG subscribe `RESPONSE` ack to legacy WS subscription ack JSON.
pub fn fig_subscribe_ack_to_legacy_json(frame: &Frame) -> WsResult<String> {
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
    let status = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::StatusCode)
        .and_then(|e| e.value.as_u16())
        .unwrap_or(200);

    let ack = serde_json::json!({
        "result": {
            "subscription_id": frame.channel_id,
            "stream": routing_key,
            "channel_path": channel_path,
            "status": status,
            "is_snapshot": true,
        },
        "id": frame.channel_id,
    });
    serde_json::to_string(&ack).map_err(|e| WsError::UnmappableFrameType(e.to_string()))
}

/// Map FIG `STREAM_CLOSE` / unsubscribe to Binance `eventStreamTerminated` style JSON.
pub fn fig_stream_close_to_legacy_json(frame: &Frame) -> WsResult<String> {
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

    let event = serde_json::json!({
        "e": "eventStreamTerminated",
        "stream": routing_key,
        "channel_path": channel_path,
        "subscription_id": frame.channel_id,
    });
    serde_json::to_string(&event).map_err(|e| WsError::UnmappableFrameType(e.to_string()))
}

/// Convert any FIG egress frame to legacy WS JSON text.
pub fn fig_frame_to_legacy_ws_json(frame: &Frame) -> WsResult<String> {
    match frame.frame_type {
        FrameType::StreamItem => fig_stream_item_to_legacy_json(frame),
        FrameType::Response => fig_subscribe_ack_to_legacy_json(frame),
        FrameType::StreamClose => fig_stream_close_to_legacy_json(frame),
        other => Err(WsError::UnmappableFrameType(format!(
            "unsupported egress frame: {other:?}"
        ))),
    }
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
    fn binance_ticker_topic_maps() {
        let sub = binance_topic_to_subscribe("btcusdt@ticker").unwrap();
        assert_eq!(sub.channel_path, "marketdata/btcusdt/ticker");
    }

    #[test]
    fn hyperliquid_funding_subscription() {
        let sub = hyperliquid_subscribe_to_fig(&serde_json::json!({
            "type": "userFunding",
            "user": "alice"
        }))
        .unwrap();
        assert_eq!(sub.channel_path, "accounts/alice/funding");
    }

    #[test]
    fn hyperliquid_ledger_subscription() {
        let sub = hyperliquid_subscribe_to_fig(&serde_json::json!({
            "type": "ledgerUpdates",
            "user": "alice"
        }))
        .unwrap();
        assert_eq!(sub.channel_path, "accounts/alice/ledger");
    }

    #[test]
    fn ws_catalog_high_priority_topics_map() {
        let topics = [
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
        for topic in topics {
            assert!(
                binance_topic_to_subscribe(topic).is_some(),
                "missing mapping for {topic}"
            );
        }
    }

    #[test]
    fn hyperliquid_order_lists_subscription() {
        let sub = hyperliquid_subscribe_to_fig(&serde_json::json!({
            "type": "listStatus",
            "user": "alice"
        }))
        .unwrap();
        assert_eq!(sub.channel_path, "trading/accounts/alice/orderlists");
    }

    #[test]
    fn binance_orderlists_topic_maps() {
        let sub = binance_topic_to_subscribe("DEMO@orderlists").unwrap();
        assert_eq!(sub.channel_path, "trading/accounts/DEMO/orderlists");
    }

    #[test]
    fn stream_close_legacy_json() {
        use fig_core::ext::{Extension, ExtensionTag};
        use fig_core::frame::{Frame, FrameType};
        let frame = Frame::new(FrameType::StreamClose, 7)
            .with_extension(Extension::text(ExtensionTag::RoutingKey, "btcusdt@ticker"))
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                "marketdata/btcusdt/ticker",
            ));
        let json = fig_stream_close_to_legacy_json(&frame).unwrap();
        assert!(json.contains("eventStreamTerminated"));
    }

    #[test]
    fn gateway_ws_catalog_private_topics_map() {
        let hl_topics = [
            ("orderUpdates", "executions"),
            ("spotState", "balances"),
            ("clearinghouseState", "positions"),
            ("listStatus", "orderlists"),
            ("balanceUpdate", "balances"),
        ];
        for (ty, fragment) in hl_topics {
            let sub = hyperliquid_subscribe_to_fig(&serde_json::json!({
                "type": ty,
                "user": "alice"
            }))
            .unwrap();
            assert!(
                sub.channel_path.contains(fragment),
                "{ty} -> {}",
                sub.channel_path
            );
        }
    }

    #[test]
    fn legacy_json_to_fig_subscribe() {
        let json = r#"{"method":"SUBSCRIBE","params":["ethusdt@trade"]}"#;
        let frame = legacy_ws_json_to_fig_subscribe(json, 2).unwrap();
        assert_eq!(frame.frame_type, FrameType::Subscribe);
        assert_eq!(frame.channel_id, 2);
    }

    /// Every `subscription.type` handled by [`hyperliquid_subscribe_to_fig`] must map.
    #[test]
    fn hyperliquid_ws_catalog_completeness() {
        let types = [
            (
                "trades",
                serde_json::json!({"type": "trades", "coin": "BTC"}),
            ),
            (
                "candle",
                serde_json::json!({"type": "candle", "coin": "BTC"}),
            ),
            (
                "l2Book",
                serde_json::json!({"type": "l2Book", "coin": "BTC"}),
            ),
            ("bbo", serde_json::json!({"type": "bbo", "coin": "BTC"})),
            (
                "orderUpdates",
                serde_json::json!({"type": "orderUpdates", "user": "alice"}),
            ),
            (
                "userFills",
                serde_json::json!({"type": "userFills", "user": "alice"}),
            ),
            (
                "userEvents",
                serde_json::json!({"type": "userEvents", "user": "alice"}),
            ),
            (
                "openOrders",
                serde_json::json!({"type": "openOrders", "user": "alice"}),
            ),
            (
                "orderState",
                serde_json::json!({"type": "orderState", "user": "alice"}),
            ),
            (
                "orderLists",
                serde_json::json!({"type": "orderLists", "user": "alice"}),
            ),
            (
                "listStatus",
                serde_json::json!({"type": "listStatus", "user": "alice"}),
            ),
            (
                "balanceUpdate",
                serde_json::json!({"type": "balanceUpdate", "user": "alice"}),
            ),
            (
                "spotState",
                serde_json::json!({"type": "spotState", "user": "alice"}),
            ),
            (
                "subscribeBalance",
                serde_json::json!({"type": "subscribeBalance", "user": "alice"}),
            ),
            (
                "clearinghouseState",
                serde_json::json!({"type": "clearinghouseState", "user": "alice"}),
            ),
            (
                "subscribePosition",
                serde_json::json!({"type": "subscribePosition", "user": "alice"}),
            ),
            (
                "userFunding",
                serde_json::json!({"type": "userFunding", "user": "alice"}),
            ),
            (
                "fundingHistory",
                serde_json::json!({"type": "fundingHistory", "user": "alice"}),
            ),
            (
                "ledgerUpdates",
                serde_json::json!({"type": "ledgerUpdates", "user": "alice"}),
            ),
            (
                "userNonFundingLedgerUpdates",
                serde_json::json!({"type": "userNonFundingLedgerUpdates", "user": "alice"}),
            ),
            (
                "liquidation",
                serde_json::json!({"type": "liquidation", "user": "alice"}),
            ),
            (
                "userLiquidation",
                serde_json::json!({"type": "userLiquidation", "user": "alice"}),
            ),
            (
                "activeAssetCtx",
                serde_json::json!({"type": "activeAssetCtx", "coin": "BTC"}),
            ),
            (
                "markPrice",
                serde_json::json!({"type": "markPrice", "coin": "BTC"}),
            ),
            ("allMids", serde_json::json!({"type": "allMids"})),
            ("miniTicker", serde_json::json!({"type": "miniTicker"})),
            (
                "aggTrades",
                serde_json::json!({"type": "aggTrades", "coin": "BTC"}),
            ),
            ("liquidations", serde_json::json!({"type": "liquidations"})),
            ("forceOrder", serde_json::json!({"type": "forceOrder"})),
            (
                "margin",
                serde_json::json!({"type": "margin", "user": "alice"}),
            ),
            (
                "clearinghouseMargin",
                serde_json::json!({"type": "clearinghouseMargin", "user": "alice"}),
            ),
        ];
        for (name, sub) in types {
            assert!(
                hyperliquid_subscribe_to_fig(&sub).is_some(),
                "missing HL mapping for type {name}"
            );
        }
    }
}
