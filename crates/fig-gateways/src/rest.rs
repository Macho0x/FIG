//! REST/HTTP adapter.
//!
//! Parses HTTP/1.1 requests and maps between HTTP semantics and FIG
//! Request/Response frames. Converts JSON bodies to/from CBOR payloads
//! for efficient binary transport over the FIG protocol.
//!
//! # HTTP Request Format
//!
//! ```text
//! METHOD PATH HTTP/1.1\r\n
//! Header-Name: value\r\n
//! \r\n
//! Body
//! ```

use thiserror::Error;

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};

// ─── HTTP Types ───────────────────────────────────────────────────

// ─── HTTP Types ───────────────────────────────────────────────────

/// A parsed HTTP/1.1 request.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest {
    /// HTTP method (GET, POST, PUT, DELETE, PATCH)
    pub method: String,
    /// Request path (e.g. `/orders` or `/accounts/123`)
    pub path: String,
    /// Headers as (name, value) pairs
    pub headers: Vec<(String, String)>,
    /// Raw request body bytes
    pub body: Vec<u8>,
}

/// An HTTP response to be serialized.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpResponse {
    /// HTTP status code (e.g. 200)
    pub status_code: u16,
    /// Reason phrase (e.g. "OK")
    pub reason: String,
    /// Response headers as (name, value) pairs
    pub headers: Vec<(String, String)>,
    /// Raw response body bytes
    pub body: Vec<u8>,
}

// ─── Error ───────────────────────────────────────────────────────

/// Errors that can occur when parsing or converting HTTP/REST messages.
#[derive(Error, Debug)]
pub enum RestError {
    #[error("empty HTTP request")]
    EmptyRequest,

    #[error("invalid HTTP request line: {0}")]
    InvalidRequestLine(String),

    #[error("invalid HTTP method: {0}")]
    InvalidMethod(String),

    #[error("invalid HTTP version")]
    InvalidVersion,

    #[error("invalid HTTP header: {0}")]
    InvalidHeader(String),

    #[error("invalid HTTP response: missing status code")]
    MissingStatusCode,

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("CBOR decode error: {0}")]
    CborDecodeError(String),

    #[error("CBOR encode error: {0}")]
    CborEncodeError(String),

    #[error("missing required extension: {0}")]
    MissingExtension(String),

    #[error("invalid UTF-8: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// Convenience type alias for REST operations.
pub type RestResult<T> = Result<T, RestError>;

// ─── HTTP Parsing ─────────────────────────────────────────────────

/// Parse a raw HTTP/1.1 request from bytes.
///
/// The request must have the format:
/// ```text
/// METHOD PATH HTTP/1.1\r\n
/// Headers\r\n
/// \r\n
/// Body
/// ```
pub fn parse_http_request(input: &[u8]) -> RestResult<HttpRequest> {
    if input.is_empty() {
        return Err(RestError::EmptyRequest);
    }

    let text = std::str::from_utf8(input)?;

    // Split header section from body
    let (header_section, body) = match text.find("\r\n\r\n") {
        Some(pos) => (&text[..pos], &text[pos + 4..]),
        None => {
            // Try with just \n\n
            match text.find("\n\n") {
                Some(pos) => (&text[..pos], &text[pos + 2..]),
                None => {
                    return Err(RestError::InvalidRequestLine(
                        "missing header/body separator".to_string(),
                    ))
                }
            }
        }
    };

    let mut lines: Vec<&str> = header_section.split("\r\n").collect();
    if lines.len() == 1 && header_section.contains('\n') {
        // Try LF-only line endings
        lines = header_section.split('\n').collect();
    }

    if lines.is_empty() {
        return Err(RestError::EmptyRequest);
    }

    // Parse request line: METHOD PATH HTTP/1.1
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(RestError::InvalidRequestLine(request_line.to_string()));
    }

    let method = parts[0].to_uppercase();
    let path = parts[1].to_string();

    // Validate version if present
    if parts.len() >= 3 {
        let version = parts[2];
        if !version.starts_with("HTTP/") {
            return Err(RestError::InvalidVersion);
        }
    }

    // Parse headers
    let mut headers = Vec::new();
    for line in &lines[1..] {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.push((name, value));
        } else {
            return Err(RestError::InvalidHeader(line.to_string()));
        }
    }

    Ok(HttpRequest {
        method,
        path,
        headers,
        body: body.as_bytes().to_vec(),
    })
}

// ─── HTTP Serialization ──────────────────────────────────────────

/// Serialize an `HttpResponse` to HTTP/1.1 wire format bytes.
pub fn serialize_http_response(response: &HttpResponse) -> Vec<u8> {
    let mut buf = Vec::new();

    // Status line
    buf.extend_from_slice(
        format!("HTTP/1.1 {} {}\r\n", response.status_code, response.reason).as_bytes(),
    );

    // Headers
    for (name, value) in &response.headers {
        buf.extend_from_slice(format!("{}: {}\r\n", name, value).as_bytes());
    }

    // Empty line separator
    buf.extend_from_slice(b"\r\n");

    // Body
    buf.extend_from_slice(&response.body);

    buf
}

// ─── HTTP → FIG Conversion ───────────────────────────────────────

/// Convert an HTTP request to a FIG Request frame.
///
/// Mappings:
/// - HTTP method → `ExtensionTag::Method` extension
/// - HTTP path → `ExtensionTag::ChannelPath` extension
/// - HTTP headers → extension tags (ContentType, UserAgent, Accept, etc.)
/// - JSON body → CBOR payload
pub fn http_to_fig_frame(request: &HttpRequest) -> RestResult<Frame> {
    let mut frame = Frame::new(FrameType::Request, 1);

    // Method → METHOD extension
    frame = frame.with_extension(Extension::text(ExtensionTag::Method, &request.method));

    // Path → CHANNEL_PATH extension
    frame = frame.with_extension(Extension::text(ExtensionTag::ChannelPath, &request.path));

    // Map known headers to extensions
    for (name, value) in &request.headers {
        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            "content-type" => {
                frame = frame.with_extension(Extension::text(ExtensionTag::ContentType, value));
            }
            "user-agent" => {
                frame = frame.with_extension(Extension::text(ExtensionTag::UserAgent, value));
            }
            "accept" => {
                frame = frame.with_extension(Extension::text(ExtensionTag::Accept, value));
            }
            "authorization" => {
                frame = frame.with_extension(Extension::binary(
                    ExtensionTag::AuthToken,
                    value.as_bytes().to_vec(),
                ));
            }
            "cache-control" => {
                frame = frame.with_extension(Extension::text(ExtensionTag::CacheControl, value));
            }
            _ => {
                // Unknown headers: store as custom extension with tag in private range (0x8000+)
                // For simplicity, skip unknown headers
            }
        }
    }

    // Convert JSON body to CBOR payload (if body is non-empty)
    if !request.body.is_empty() {
        let cbor_payload = json_to_cbor(std::str::from_utf8(&request.body)?)?;
        frame = frame.with_payload(cbor_payload);
    }

    Ok(frame)
}

// ─── FIG → HTTP Conversion ───────────────────────────────────────

/// Convert a FIG Response frame to an HTTP response.
///
/// Mappings:
/// - STATUS_CODE extension → HTTP status code
/// - CBOR payload → JSON body
/// - extensions → HTTP headers
pub fn fig_to_http_response(frame: &Frame) -> RestResult<HttpResponse> {
    let status_code = frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::StatusCode)
        .and_then(|e| e.value.as_u16())
        .ok_or_else(|| RestError::MissingExtension("StatusCode".to_string()))?;

    let reason = match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        408 => "Request Timeout",
        409 => "Conflict",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
    .to_string();

    // Convert FIG extensions to HTTP headers
    let mut headers = Vec::new();
    for ext in &frame.extensions {
        match ext.tag {
            ExtensionTag::ContentType => {
                if let Some(ct) = ext.value.as_text() {
                    headers.push(("Content-Type".to_string(), ct.to_string()));
                }
            }
            ExtensionTag::UserAgent => {
                if let Some(ua) = ext.value.as_text() {
                    headers.push(("User-Agent".to_string(), ua.to_string()));
                }
            }
            ExtensionTag::CacheControl => {
                if let Some(cc) = ext.value.as_text() {
                    headers.push(("Cache-Control".to_string(), cc.to_string()));
                }
            }
            _ => {}
        }
    }

    // Add Content-Length if not already present
    let has_content_length = headers
        .iter()
        .any(|(n, _)| n.to_lowercase() == "content-length");
    if !has_content_length && !frame.payload.is_empty() {
        let json_body = cbor_to_json(&frame.payload)?;
        headers.push(("Content-Length".to_string(), json_body.len().to_string()));
        headers.push(("Content-Type".to_string(), "application/json".to_string()));
    }

    // Convert CBOR payload to JSON body
    let body = if frame.payload.is_empty() {
        Vec::new()
    } else {
        cbor_to_json(&frame.payload)?.into_bytes()
    };

    Ok(HttpResponse {
        status_code,
        reason,
        headers,
        body,
    })
}

// ─── JSON ↔ CBOR Conversion ──────────────────────────────────────

/// Convert a JSON string to CBOR bytes.
///
/// Delegates to [`fig_core::codec::json_to_cbor`].
pub fn json_to_cbor(json: &str) -> RestResult<Vec<u8>> {
    fig_core::codec::json_to_cbor(json).map_err(|e| RestError::CborEncodeError(e.to_string()))
}

/// Convert CBOR bytes to a JSON string.
///
/// Delegates to [`fig_core::codec::cbor_to_json`].
pub fn cbor_to_json(cbor: &[u8]) -> RestResult<String> {
    fig_core::codec::cbor_to_json(cbor).map_err(|e| RestError::CborDecodeError(e.to_string()))
}

// ─── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── HTTP Parsing ──────────────────────────────────────────

    #[test]
    fn test_parse_http_request() {
        let raw = b"POST /orders HTTP/1.1\r\nContent-Type: application/json\r\nUser-Agent: fig-client/1.0\r\n\r\n{\"symbol\":\"AAPL\"}";
        let req = parse_http_request(raw).unwrap();

        assert_eq!(req.method, "POST");
        assert_eq!(req.path, "/orders");
        assert_eq!(req.headers.len(), 2);
        assert_eq!(req.headers[0].0, "Content-Type");
        assert_eq!(req.headers[0].1, "application/json");
        assert_eq!(req.headers[1].0, "User-Agent");
        assert_eq!(req.headers[1].1, "fig-client/1.0");
        assert_eq!(req.body, b"{\"symbol\":\"AAPL\"}");
    }

    #[test]
    fn test_parse_http_get() {
        let raw = b"GET /accounts/123 HTTP/1.1\r\nAccept: application/json\r\n\r\n";
        let req = parse_http_request(raw).unwrap();

        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/accounts/123");
        assert_eq!(req.headers.len(), 1);
        assert_eq!(req.body, b"");
    }

    // ── HTTP Serialization ────────────────────────────────────

    #[test]
    fn test_serialize_http_response() {
        let resp = HttpResponse {
            status_code: 200,
            reason: "OK".to_string(),
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: b"{\"status\":\"ok\"}".to_vec(),
        };

        let serialized = serialize_http_response(&resp);
        let text = String::from_utf8(serialized).unwrap();

        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.contains("Content-Type: application/json"));
        assert!(text.contains("{\"status\":\"ok\"}"));
    }

    // ── JSON ↔ CBOR ──────────────────────────────────────────

    #[test]
    fn test_json_cbor_round_trip() {
        let json = r#"{"symbol":"AAPL","side":"buy","qty":100,"price":150.25}"#;
        let cbor = json_to_cbor(json).unwrap();
        assert!(!cbor.is_empty());

        let json_back = cbor_to_json(&cbor).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();

        assert_eq!(parsed["symbol"], "AAPL");
        assert_eq!(parsed["side"], "buy");
        assert_eq!(parsed["qty"], 100);
        assert_eq!(parsed["price"], 150.25);
    }

    #[test]
    fn test_json_order_flags_round_trip() {
        let json = r#"{"cl_ord_id":"PO-1","symbol":"BTC","side":"buy","order_qty":1.0,"order_type":"Limit","price":50000.0,"time_in_force":"Day","post_only":true,"reduce_only":false}"#;
        let cbor = json_to_cbor(json).unwrap();
        let json_back = cbor_to_json(&cbor).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();

        assert_eq!(parsed["post_only"], true);
        assert_eq!(parsed["reduce_only"], false);
    }

    #[test]
    fn test_json_cbor_nested() {
        let json = r#"{"order":{"id":"123","items":[{"symbol":"AAPL","qty":10},{"symbol":"MSFT","qty":20}]}}"#;
        let cbor = json_to_cbor(json).unwrap();
        let json_back = cbor_to_json(&cbor).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_back).unwrap();

        assert_eq!(parsed["order"]["id"], "123");
        assert_eq!(parsed["order"]["items"][1]["symbol"], "MSFT");
    }

    // ── HTTP → FIG Frame ──────────────────────────────────────

    #[test]
    fn test_http_to_fig_frame() {
        let json_body = r#"{"cl_ord_id":"ORD-001","symbol":"AAPL","side":"Buy","order_qty":100.0,"price":150.25}"#;
        let raw = format!(
            "POST /trading/orders HTTP/1.1\r\nContent-Type: application/json\r\nAccept: application/json\r\n\r\n{}",
            json_body
        );

        let req = parse_http_request(raw.as_bytes()).unwrap();
        let frame = http_to_fig_frame(&req).unwrap();

        assert_eq!(frame.frame_type, FrameType::Request);

        // Check method extension
        let method_ext = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::Method)
            .unwrap();
        assert_eq!(method_ext.value.as_text(), Some("POST"));

        // Check path extension
        let path_ext = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ChannelPath)
            .unwrap();
        assert_eq!(path_ext.value.as_text(), Some("/trading/orders"));

        // Check content type extension
        let ct_ext = frame
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ContentType)
            .unwrap();
        assert_eq!(ct_ext.value.as_text(), Some("application/json"));

        // Payload should be CBOR
        assert!(!frame.payload.is_empty());

        // Round-trip the payload to verify
        let json_back = cbor_to_json(&frame.payload).unwrap();
        assert!(json_back.contains("ORD-001"));
    }

    // ── FIG Frame → HTTP Response ─────────────────────────────

    #[test]
    fn test_fig_to_http_response() {
        let json = r#"{"exec_id":"EX-001","status":"filled"}"#;
        let cbor = json_to_cbor(json).unwrap();

        let frame = Frame::new(FrameType::Response, 1)
            .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
            .with_extension(Extension::text(
                ExtensionTag::ContentType,
                "application/json",
            ))
            .with_payload(cbor);

        let resp = fig_to_http_response(&frame).unwrap();

        assert_eq!(resp.status_code, 200);
        assert_eq!(resp.reason, "OK");

        let body_str = String::from_utf8(resp.body).unwrap();
        assert!(body_str.contains("EX-001"));
        assert!(body_str.contains("filled"));
    }
}
