//! REST GET → native FIG `REQUEST` mapping (§17.5).
//!
//! HTTP paths and query parameters are translated to `ChannelPath` extensions
//! and CBOR request payloads (`CandleBarRequest`, `TradeHistoryRequest`, …).
//! Binance `/klines` is a gateway alias only; native FIG uses `/candles/{interval}`.

use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    schema_id, AggregateTradeRequest, AllMidsRequest, CandleBarRequest, FillHistoryRequest,
    FundingHistoryRequest, LedgerHistoryRequest, MarkPriceRequest, OpenOrdersRequest,
    OrderBookRequest, OrderHistoryRequest, TickerRequest, TradeHistoryRequest,
};

use crate::rest::{HttpRequest, RestError, RestResult};

/// Strip leading slash and normalize REST path segments for FIG `ChannelPath`.
pub fn rest_path_to_channel_path(http_path: &str) -> String {
    http_path.trim_start_matches('/').to_string()
}

/// Map an HTTP GET (or query-style GET) to a FIG Request frame.
pub fn http_get_to_fig_request(request: &HttpRequest) -> RestResult<Frame> {
    let query = parse_query_string(&request.path);
    let path_only = request.path.split('?').next().unwrap_or(&request.path);
    let channel_path = map_http_path_to_channel_path(path_only, &query)?;

    let mut frame = Frame::new(FrameType::Request, 1)
        .with_schema_id(schema_id::TRADING_ORDERS)
        .with_extension(Extension::text(ExtensionTag::Method, "GET"))
        .with_extension(Extension::text(ExtensionTag::ChannelPath, &channel_path));

    if let Some(payload) = build_query_payload(&channel_path, &query)? {
        frame = frame
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/cbor",
            ))
            .with_payload(payload);
    }

    Ok(frame)
}

fn map_http_path_to_channel_path(path: &str, query: &[(String, String)]) -> RestResult<String> {
    let trimmed = path.trim_start_matches('/');

    if trimmed == "api/v3/klines" || trimmed == "klines" {
        let symbol = query_param(query, &["symbol", "Symbol"])
            .ok_or_else(|| RestError::InvalidRequestLine("klines requires symbol".into()))?;
        let interval = query_param(query, &["interval", "Interval"])
            .ok_or_else(|| RestError::InvalidRequestLine("klines requires interval".into()))?;
        return Ok(format!("marketdata/{symbol}/candles/{interval}"));
    }

    if trimmed == "api/v3/aggTrades" || trimmed == "aggTrades" {
        let symbol = query_param(query, &["symbol", "Symbol"])
            .ok_or_else(|| RestError::InvalidRequestLine("aggTrades requires symbol".into()))?;
        return Ok(format!("marketdata/{symbol}/aggtrades"));
    }

    if trimmed == "api/v3/depth" || trimmed == "depth" {
        let symbol = query_param(query, &["symbol", "Symbol"])
            .ok_or_else(|| RestError::InvalidRequestLine("depth requires symbol".into()))?;
        return Ok(format!("marketdata/{symbol}/book"));
    }

    if trimmed == "api/v3/ticker/24hr" || trimmed == "ticker/24hr" {
        let symbol = query_param(query, &["symbol", "Symbol"])
            .ok_or_else(|| RestError::InvalidRequestLine("ticker requires symbol".into()))?;
        return Ok(format!("marketdata/{symbol}/ticker"));
    }

    if trimmed == "api/v3/exchangeInfo" || trimmed == "exchangeInfo" {
        return Ok(".well-known/capabilities".to_string());
    }

    if trimmed == "api/v3/account" || trimmed == "account" {
        let account = account_from_query_or_default(query);
        return Ok(format!("accounts/{account}"));
    }

    if trimmed == "api/v3/openOrders" || trimmed == "openOrders" {
        let account = account_from_query_or_default(query);
        return Ok(format!("trading/accounts/{account}/orders/open"));
    }

    if trimmed == "api/v3/allOrders" || trimmed == "allOrders" {
        let account = account_from_query_or_default(query);
        return Ok(format!("trading/accounts/{account}/orders"));
    }

    if trimmed == "api/v3/myTrades" || trimmed == "myTrades" {
        let account = account_from_query_or_default(query);
        return Ok(format!("accounts/{account}/fills"));
    }

    if trimmed.starts_with("marketdata/")
        || trimmed.starts_with("accounts/")
        || trimmed.starts_with("trading/")
    {
        return Ok(trimmed.to_string());
    }

    if trimmed == ".well-known/capabilities"
        || trimmed == "capabilities"
        || trimmed == ".well-known/instruments"
        || trimmed == "instruments"
    {
        return Ok(trimmed.to_string());
    }

    Err(RestError::InvalidRequestLine(format!(
        "unsupported query path: {path}"
    )))
}

fn account_from_query_or_default(query: &[(String, String)]) -> String {
    query_param(query, &["account", "Account", "accountId", "account_id"])
        .unwrap_or_else(|| "DEMO".to_string())
}

fn build_query_payload(
    channel_path: &str,
    query: &[(String, String)],
) -> RestResult<Option<Vec<u8>>> {
    if let Some((symbol, interval)) = parse_candle_path(channel_path) {
        let req = CandleBarRequest {
            symbol,
            interval,
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(symbol) = parse_trade_path(channel_path) {
        let req = TradeHistoryRequest {
            symbol,
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(symbol) = parse_agg_trade_path(channel_path) {
        let req = AggregateTradeRequest {
            symbol,
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if channel_path == "marketdata/ticker/all" {
        return Ok(Some(
            encode_cbor(&AllMidsRequest {})
                .map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(symbol) = parse_mark_path(channel_path) {
        return Ok(Some(
            encode_cbor(&MarkPriceRequest { symbol })
                .map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(symbol) = parse_ticker_path(channel_path) {
        let req = TickerRequest { symbol };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/positions") {
        return Ok(None);
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/fills") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = FillHistoryRequest {
            account: account.to_string(),
            symbol: query_param(query, &["symbol", "Symbol"]),
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/funding") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = FundingHistoryRequest {
            account: account.to_string(),
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/ledger") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = LedgerHistoryRequest {
            account: account.to_string(),
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(account) = parse_open_orders_path(channel_path) {
        let req = OpenOrdersRequest {
            account,
            symbol: query_param(query, &["symbol", "Symbol"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(account) = parse_order_history_path(channel_path) {
        let req = OrderHistoryRequest {
            account,
            symbol: query_param(query, &["symbol", "Symbol"]),
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
            cursor: query_param(query, &["cursor", "next_cursor"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if let Some(symbol) = parse_order_book_path(channel_path) {
        let req = OrderBookRequest {
            symbol,
            depth: parse_limit_param(query),
            at_time: parse_time_param(query, &["at_time", "atTime", "timestamp"]),
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    Ok(None)
}

fn parse_open_orders_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 5
        && parts[0] == "trading"
        && parts[1] == "accounts"
        && parts[3] == "orders"
        && parts[4] == "open"
    {
        return Some(parts[2].to_string());
    }
    None
}

fn parse_order_history_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 4 && parts[0] == "trading" && parts[1] == "accounts" && parts[3] == "orders" {
        return Some(parts[2].to_string());
    }
    None
}

fn parse_order_book_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" {
        match parts.get(2).copied() {
            Some("book") | Some("quotes") => Some(parts[1].to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn parse_candle_path(path: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 4 && parts[0] == "marketdata" && parts[2] == "candles" {
        Some((parts[1].to_string(), parts[3].to_string()))
    } else {
        None
    }
}

fn parse_trade_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "trades" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn parse_agg_trade_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "aggtrades" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn parse_mark_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "mark" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn parse_ticker_path(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() >= 3 && parts[0] == "marketdata" && parts[2] == "ticker" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn parse_query_string(path: &str) -> Vec<(String, String)> {
    let Some(qs) = path.split('?').nth(1) else {
        return Vec::new();
    };
    qs.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((
                parts.next()?.to_string(),
                parts.next().unwrap_or("").to_string(),
            ))
        })
        .collect()
}

fn query_param(query: &[(String, String)], keys: &[&str]) -> Option<String> {
    for (k, v) in query {
        if keys.iter().any(|key| k.eq_ignore_ascii_case(key)) && !v.is_empty() {
            return Some(v.clone());
        }
    }
    None
}

fn parse_time_param(query: &[(String, String)], keys: &[&str]) -> Option<i64> {
    query_param(query, keys).and_then(|s| s.parse().ok())
}

fn parse_limit_param(query: &[(String, String)]) -> Option<u32> {
    query_param(query, &["limit", "Limit"]).and_then(|s| s.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::parse_http_request;

    fn get_frame(path: &str) -> Frame {
        let raw = format!("GET {path} HTTP/1.1\r\n\r\n");
        let req = parse_http_request(raw.as_bytes()).unwrap();
        http_get_to_fig_request(&req).unwrap()
    }

    fn channel_path(frame: &Frame) -> &str {
        frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ChannelPath)
            .and_then(|e| e.value.as_text())
            .unwrap()
    }

    #[test]
    fn native_candle_path_maps_to_fig_request() {
        let frame = get_frame("/marketdata/BTC/candles/5m?start=1000&limit=10");
        assert_eq!(channel_path(&frame), "marketdata/BTC/candles/5m");
        assert!(!frame.payload.is_empty());
    }

    #[test]
    fn binance_klines_alias_maps_to_native_path() {
        let frame = get_frame("/api/v3/klines?symbol=BTCUSDT&interval=5m");
        assert_eq!(channel_path(&frame), "marketdata/BTCUSDT/candles/5m");
    }

    #[test]
    fn capabilities_path_maps() {
        let frame = get_frame("/.well-known/capabilities");
        assert_eq!(channel_path(&frame), ".well-known/capabilities");
    }

    #[test]
    fn instruments_path_maps() {
        let frame = get_frame("/.well-known/instruments");
        assert_eq!(channel_path(&frame), ".well-known/instruments");
    }

    #[test]
    fn open_orders_path_maps_with_payload() {
        let frame = get_frame("/trading/accounts/DEMO/orders/open");
        assert_eq!(channel_path(&frame), "trading/accounts/DEMO/orders/open");
        assert!(!frame.payload.is_empty());
    }

    #[test]
    fn order_history_path_maps_with_payload() {
        let frame = get_frame("/trading/accounts/DEMO/orders?limit=50");
        assert_eq!(channel_path(&frame), "trading/accounts/DEMO/orders");
        assert!(!frame.payload.is_empty());
    }

    #[test]
    fn order_book_path_maps_with_payload() {
        let frame = get_frame("/marketdata/BTC/book?limit=20");
        assert_eq!(channel_path(&frame), "marketdata/BTC/book");
        assert!(!frame.payload.is_empty());
    }

    #[test]
    fn binance_rest_aliases_map() {
        let cases = [
            (
                "/api/v3/aggTrades?symbol=BTCUSDT",
                "marketdata/BTCUSDT/aggtrades",
            ),
            ("/api/v3/depth?symbol=BTCUSDT", "marketdata/BTCUSDT/book"),
            (
                "/api/v3/ticker/24hr?symbol=BTCUSDT",
                "marketdata/BTCUSDT/ticker",
            ),
            ("/api/v3/exchangeInfo", ".well-known/capabilities"),
            ("/api/v3/account?account=DEMO", "accounts/DEMO"),
            (
                "/api/v3/openOrders?account=DEMO",
                "trading/accounts/DEMO/orders/open",
            ),
            (
                "/api/v3/allOrders?account=DEMO",
                "trading/accounts/DEMO/orders",
            ),
            ("/api/v3/myTrades?account=DEMO", "accounts/DEMO/fills"),
        ];
        for (path, expected) in cases {
            let frame = get_frame(path);
            assert_eq!(channel_path(&frame), expected, "path {path}");
        }
    }

    #[test]
    fn gateway_rest_catalog_paths_are_mappable() {
        let paths = [
            "/marketdata/BTC/ticker",
            "/marketdata/BTC/trades",
            "/marketdata/BTC/aggtrades",
            "/marketdata/BTC/mark",
            "/marketdata/ticker/all",
            "/accounts/DEMO",
            "/accounts/DEMO/fills",
            "/accounts/DEMO/funding",
            "/accounts/DEMO/ledger",
            "/accounts/DEMO/positions",
            "/accounts/DEMO/margin",
            "/trading/accounts/DEMO/orders/open",
            "/trading/accounts/DEMO/orders",
            "/marketdata/BTC/book",
            "/.well-known/capabilities",
            "/.well-known/instruments",
        ];
        for path in paths {
            get_frame(path);
        }
    }
}
