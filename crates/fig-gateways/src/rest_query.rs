//! REST GET → native FIG `REQUEST` mapping (§17.5).
//!
//! HTTP paths and query parameters are translated to `ChannelPath` extensions
//! and CBOR request payloads (`CandleBarRequest`, `TradeHistoryRequest`, …).
//! Binance `/klines` is a gateway alias only; native FIG uses `/candles/{interval}`.

use fig_core::codec::encode_cbor;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    CandleBarRequest, FillHistoryRequest, FundingHistoryRequest, LedgerHistoryRequest,
    TradeHistoryRequest, schema_id,
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

fn map_http_path_to_channel_path(
    path: &str,
    query: &[(String, String)],
) -> RestResult<String> {
    let trimmed = path.trim_start_matches('/');

    if trimmed == "api/v3/klines" || trimmed == "klines" {
        let symbol = query_param(query, &["symbol", "Symbol"])
            .ok_or_else(|| RestError::InvalidRequestLine("klines requires symbol".into()))?;
        let interval = query_param(query, &["interval", "Interval"])
            .ok_or_else(|| RestError::InvalidRequestLine("klines requires interval".into()))?;
        return Ok(format!("marketdata/{symbol}/candles/{interval}"));
    }

    if trimmed.starts_with("marketdata/") || trimmed.starts_with("accounts/") {
        return Ok(trimmed.to_string());
    }

    Err(RestError::InvalidRequestLine(format!(
        "unsupported query path: {path}"
    )))
}

fn build_query_payload(channel_path: &str, query: &[(String, String)]) -> RestResult<Option<Vec<u8>>> {
    if let Some((symbol, interval)) = parse_candle_path(channel_path) {
        let req = CandleBarRequest {
            symbol,
            interval,
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
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
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    if channel_path.starts_with("accounts/") && channel_path.ends_with("/fills") {
        let account = channel_path.split('/').nth(1).unwrap_or("default");
        let req = FillHistoryRequest {
            account: account.to_string(),
            symbol: query_param(query, &["symbol", "Symbol"]),
            start_time: parse_time_param(query, &["start", "startTime", "start_time"]),
            end_time: parse_time_param(query, &["end", "endTime", "end_time"]),
            limit: parse_limit_param(query),
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
        };
        return Ok(Some(
            encode_cbor(&req).map_err(|e| RestError::CborEncodeError(e.to_string()))?,
        ));
    }

    Ok(None)
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

fn parse_query_string(path: &str) -> Vec<(String, String)> {
    let Some(qs) = path.split('?').nth(1) else {
        return Vec::new();
    };
    qs.split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            Some((parts.next()?.to_string(), parts.next().unwrap_or("").to_string()))
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

    #[test]
    fn native_candle_path_maps_to_fig_request() {
        let raw = b"GET /marketdata/BTC/candles/5m?start=1000&limit=10 HTTP/1.1\r\n\r\n";
        let req = parse_http_request(raw).unwrap();
        let frame = http_get_to_fig_request(&req).unwrap();
        let path = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ChannelPath)
            .and_then(|e| e.value.as_text())
            .unwrap();
        assert_eq!(path, "marketdata/BTC/candles/5m");
        assert!(!frame.payload.is_empty());
    }

    #[test]
    fn binance_klines_alias_maps_to_native_path() {
        let raw = b"GET /api/v3/klines?symbol=BTCUSDT&interval=5m HTTP/1.1\r\n\r\n";
        let req = parse_http_request(raw).unwrap();
        let frame = http_get_to_fig_request(&req).unwrap();
        let path = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ChannelPath)
            .and_then(|e| e.value.as_text())
            .unwrap();
        assert_eq!(path, "marketdata/BTCUSDT/candles/5m");
    }
}
