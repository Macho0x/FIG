//! CBOR encoding/decoding helpers for extension metadata and payloads.
//!
//! FIG uses CBOR (RFC 7049) as its self-describing payload format when
//! no well-known schema is available (Schema ID = 0x00 with CONTENT_TYPE
//! "application/cbor"). This module provides thin wrappers around ciborium
//! for convenience.

use crate::error::FrameError;

/// Encode a serializable value to CBOR bytes.
///
/// # Errors
///
/// Returns `FrameError::CborEncodeError` if serialization fails.
pub fn encode_cbor<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, FrameError> {
    let mut buf = Vec::new();
    ciborium::ser::into_writer(value, &mut buf)
        .map_err(|e| FrameError::CborEncodeError(e.to_string()))?;
    Ok(buf)
}

/// Decode CBOR bytes into a value of type `T`.
///
/// # Errors
///
/// Returns `FrameError::CborDecodeError` if deserialization fails.
pub fn decode_cbor<T: serde::de::DeserializeOwned>(data: &[u8]) -> Result<T, FrameError> {
    ciborium::de::from_reader(data).map_err(|e| FrameError::CborDecodeError(e.to_string()))
}

// ─── JSON Codec ──────────────────────────────────────────────────

/// Parse JSON bytes into a `serde_json::Value`.
pub fn decode_json(data: &[u8]) -> Result<serde_json::Value, FrameError> {
    serde_json::from_slice(data).map_err(|e| FrameError::CborDecodeError(e.to_string()))
}

/// Serialize a `serde_json::Value` to JSON bytes.
pub fn encode_json(value: &serde_json::Value) -> Result<Vec<u8>, FrameError> {
    serde_json::to_vec(value).map_err(|e| FrameError::CborEncodeError(e.to_string()))
}

/// Convert a JSON string to CBOR bytes (REST gateway hot path).
///
/// Parses JSON into a generic value, then encodes as CBOR. Invalid JSON
/// is rejected before CBOR encoding begins.
pub fn json_to_cbor(json: &str) -> Result<Vec<u8>, FrameError> {
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| FrameError::CborDecodeError(e.to_string()))?;
    encode_cbor(&value)
}

/// Convert CBOR bytes to a JSON string (REST gateway hot path).
pub fn cbor_to_json(cbor: &[u8]) -> Result<String, FrameError> {
    let value: serde_json::Value = decode_cbor(cbor)?;
    serde_json::to_string(&value).map_err(|e| FrameError::CborEncodeError(e.to_string()))
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// Example order message (mirrors fig-exchange-sim OrderRequest).
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct OrderRequest {
        pub cl_ord_id: String,
        pub symbol: String,
        pub side: String,
        pub order_qty: u64,
        pub price: Option<f64>,
    }

    /// Example execution report.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ExecutionReport {
        pub cl_ord_id: String,
        pub exec_id: String,
        pub exec_type: String,
        pub last_qty: u64,
        pub last_price: f64,
    }

    /// Example market data quote.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Quote {
        pub symbol: String,
        pub bid: f64,
        pub ask: f64,
        pub bid_size: u64,
        pub ask_size: u64,
        pub timestamp: u64,
    }

    #[test]
    fn test_round_trip_order() {
        let order = OrderRequest {
            cl_ord_id: "ORD-001".into(),
            symbol: "AAPL".into(),
            side: "buy".into(),
            order_qty: 100,
            price: Some(150.25),
        };

        let encoded = encode_cbor(&order).unwrap();
        assert!(!encoded.is_empty());

        let decoded: OrderRequest = decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, order);
    }

    #[test]
    fn test_round_trip_execution_report() {
        let report = ExecutionReport {
            cl_ord_id: "ORD-001".into(),
            exec_id: "EXEC-100".into(),
            exec_type: "fill".into(),
            last_qty: 50,
            last_price: 150.25,
        };

        let encoded = encode_cbor(&report).unwrap();
        let decoded: ExecutionReport = decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, report);
    }

    #[test]
    fn test_round_trip_quote() {
        let quote = Quote {
            symbol: "AAPL".into(),
            bid: 150.10,
            ask: 150.20,
            bid_size: 500,
            ask_size: 300,
            timestamp: 1718400000000000000,
        };

        let encoded = encode_cbor(&quote).unwrap();
        let decoded: Quote = decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, quote);
    }

    #[test]
    fn test_encode_empty_order() {
        let order = OrderRequest {
            cl_ord_id: "".into(),
            symbol: "".into(),
            side: "sell".into(),
            order_qty: 0,
            price: None,
        };
        let encoded = encode_cbor(&order).unwrap();
        let decoded: OrderRequest = decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, order);
    }

    #[test]
    fn test_round_trip_nested() {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct OrderBook {
            pub symbol: String,
            pub bids: Vec<Level>,
            pub asks: Vec<Level>,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Level {
            pub price: f64,
            pub size: u64,
        }

        let book = OrderBook {
            symbol: "AAPL".into(),
            bids: vec![
                Level {
                    price: 150.10,
                    size: 500,
                },
                Level {
                    price: 150.05,
                    size: 300,
                },
            ],
            asks: vec![
                Level {
                    price: 150.20,
                    size: 200,
                },
                Level {
                    price: 150.25,
                    size: 400,
                },
            ],
        };

        let encoded = encode_cbor(&book).unwrap();
        let decoded: OrderBook = decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, book);
    }

    #[test]
    fn test_decode_invalid_cbor() {
        let garbage = b"this is not valid cbor";
        let result: Result<OrderRequest, _> = decode_cbor(garbage);
        assert!(result.is_err());
    }

    // ── JSON codec ────────────────────────────────────────────

    #[test]
    fn test_json_cbor_round_trip() {
        let json = r#"{"symbol":"AAPL","side":"buy","qty":100,"price":150.25}"#;
        let cbor = json_to_cbor(json).unwrap();
        assert!(!cbor.is_empty());

        let json_back = cbor_to_json(&cbor).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();
        assert_eq!(parsed["symbol"], "AAPL");
        assert_eq!(parsed["qty"], 100);
    }

    #[test]
    fn test_json_cbor_nested() {
        let json = r#"{"order":{"id":"123","items":[{"symbol":"AAPL","qty":10}]}}"#;
        let cbor = json_to_cbor(json).unwrap();
        let json_back = cbor_to_json(&cbor).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();
        assert_eq!(parsed["order"]["id"], "123");
        assert_eq!(parsed["order"]["items"][0]["symbol"], "AAPL");
    }

    #[test]
    fn test_decode_json_invalid() {
        let result = decode_json(b"{not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_json_value() {
        let value = serde_json::json!({"status": "ok", "count": 42});
        let bytes = encode_json(&value).unwrap();
        let decoded = decode_json(&bytes).unwrap();
        assert_eq!(decoded, value);
    }

    #[test]
    fn test_json_to_cbor_rejects_invalid_json() {
        let result = json_to_cbor("{broken");
        assert!(result.is_err());
    }
}
