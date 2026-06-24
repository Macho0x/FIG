//! §17 path interaction enforcement (SUBSCRIBE vs GET vs POST).

use fig_core::ext::ExtensionTag;
use fig_core::frame::{Frame, FrameType};

use crate::broker_session::{
    parse_capabilities_path, parse_open_orders_path, parse_order_book_path,
    parse_order_history_path,
};
use crate::market_data::{
    parse_agg_trade_query_path, parse_all_mids_path, parse_candle_query_path,
    parse_mark_query_path, parse_ticker_query_path, parse_trade_query_path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
    Subscribe,
    RequestGet,
    RequestPost,
}

fn extension_text(frame: &Frame, tag: ExtensionTag) -> String {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == tag)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
        .to_string()
}

/// Paths that reject SUBSCRIBE (historical-only or capability discovery).
fn is_subscribe_forbidden_path(path: &str) -> bool {
    parse_capabilities_path(path)
        || parse_order_history_path(path).is_some()
        || path.ends_with("/fills")
        || (path.starts_with("accounts/") && path.matches('/').count() == 1)
}

/// Paths that support GET via Request (may also support SUBSCRIBE for live streams).
fn is_get_query_path(path: &str) -> bool {
    parse_capabilities_path(path)
        || parse_order_history_path(path).is_some()
        || parse_open_orders_path(path).is_some()
        || parse_candle_query_path(path).is_some()
        || parse_trade_query_path(path).is_some()
        || parse_agg_trade_query_path(path).is_some()
        || parse_ticker_query_path(path).is_some()
        || parse_mark_query_path(path).is_some()
        || parse_all_mids_path(path)
        || parse_order_book_path(path).is_some()
        || path.ends_with("/fills")
        || path.ends_with("/funding")
        || path.ends_with("/ledger")
        || (path.starts_with("accounts/") && path.matches('/').count() == 1)
}

/// Expected interaction for a normalized channel path (request routing).
pub fn expected_interaction(path: &str) -> Interaction {
    if parse_order_history_path(path).is_some() {
        return Interaction::RequestGet;
    }
    if path.contains("/orders")
        && path.contains("/accounts/")
        && !path.contains("/open")
        && !path.contains("/cancel")
        && !path.contains("/replace")
    {
        return Interaction::RequestPost;
    }
    if path.contains("/cancel") || path.contains("/replace") {
        return Interaction::RequestPost;
    }
    if is_get_query_path(path) {
        return Interaction::RequestGet;
    }
    Interaction::Subscribe
}

/// Returns an error code when the frame type/method does not match the path contract.
pub fn validate_interaction(frame: &Frame) -> Option<&'static str> {
    if matches!(
        frame.frame_type,
        FrameType::Unsubscribe | FrameType::Control
    ) {
        return None;
    }
    let path = extension_text(frame, ExtensionTag::ChannelPath);
    if path.is_empty() {
        return None;
    }
    let method = extension_text(frame, ExtensionTag::Method);

    match frame.frame_type {
        FrameType::Subscribe => {
            if is_subscribe_forbidden_path(&path) {
                Some("INVALID_SUBSCRIBE_PATH")
            } else {
                None
            }
        }
        FrameType::Request => {
            if parse_order_history_path(&path).is_some() {
                if method.eq_ignore_ascii_case("GET") || method.is_empty() {
                    return None;
                }
                if method.eq_ignore_ascii_case("POST") {
                    return None;
                }
                return Some("INVALID_METHOD");
            }
            match expected_interaction(&path) {
                Interaction::RequestGet
                    if method.eq_ignore_ascii_case("GET") || method.is_empty() =>
                {
                    None
                }
                Interaction::RequestPost if method.eq_ignore_ascii_case("POST") => None,
                Interaction::RequestPost => Some("METHOD_POST_REQUIRED"),
                Interaction::RequestGet => Some("METHOD_GET_REQUIRED"),
                Interaction::Subscribe => Some("INVALID_REQUEST_PATH"),
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::ext::{Extension, ExtensionTag};
    use fig_core::frame::{Frame, FrameType};

    #[test]
    fn subscribe_allows_book_path() {
        let frame = Frame::new(FrameType::Subscribe, 1).with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/book",
        ));
        assert!(validate_interaction(&frame).is_none());
    }

    #[test]
    fn subscribe_allows_candles_path() {
        let frame = Frame::new(FrameType::Subscribe, 1).with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/candles/5m",
        ));
        assert!(validate_interaction(&frame).is_none());
    }

    #[test]
    fn subscribe_allows_aggtrades_path() {
        let frame = Frame::new(FrameType::Subscribe, 1).with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/aggtrades",
        ));
        assert!(validate_interaction(&frame).is_none());
    }

    #[test]
    fn get_still_allowed_on_candles_path() {
        let frame = Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                "marketdata/AAPL/candles/5m",
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "GET"));
        assert!(validate_interaction(&frame).is_none());
    }

    #[test]
    fn subscribe_rejects_capabilities_path() {
        let frame = Frame::new(FrameType::Subscribe, 1).with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            ".well-known/capabilities",
        ));
        assert_eq!(validate_interaction(&frame), Some("INVALID_SUBSCRIBE_PATH"));
    }

    #[test]
    fn post_allowed_for_new_order_on_orders_path() {
        let frame = Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(
                ExtensionTag::ChannelPath,
                "trading/accounts/DEMO/orders",
            ))
            .with_extension(Extension::text(ExtensionTag::Method, "POST"));
        assert!(validate_interaction(&frame).is_none());
    }
}
