//! Private path auth for the exchange simulator (test harness only).
//!
//! Checks `AUTH_TOKEN` = `fig-dev-{account}` so integration tests can exercise
//! SPEC §9.3 path scoping without a credential store. Not a production key
//! issuance or validation service.

use fig_core::auth::verify_token;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::Frame;

use crate::server::make_error_frame;

/// Dev token format: `fig-dev-{account}` (Bearer prefix optional).
pub fn expected_dev_token(account: &str) -> String {
    format!("fig-dev-{account}")
}

pub fn auth_token_from_frame(frame: &Frame) -> Option<String> {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::AuthToken)
        .and_then(|e| {
            e.value
                .as_text()
                .map(str::to_string)
                .or_else(|| String::from_utf8(e.value.as_bytes().to_vec()).ok())
        })
        .map(|t| t.trim_start_matches("Bearer ").trim().to_string())
}

pub fn account_from_private_path(path: &str) -> Option<String> {
    if path.starts_with("accounts/") {
        return path.split('/').nth(1).map(str::to_string);
    }
    if path.starts_with("trading/accounts/") {
        return path.split('/').nth(2).map(str::to_string);
    }
    None
}

pub fn is_private_path(path: &str) -> bool {
    path.starts_with("accounts/") || path.starts_with("trading/accounts/")
}

/// Returns an error frame if auth fails; `None` if access is allowed.
pub fn authorize_private(frame: &Frame, account: &str) -> Option<Frame> {
    if std::env::var("FIG_DEV_OPEN").ok().as_deref() == Some("1") {
        return None;
    }
    let Some(token) = auth_token_from_frame(frame) else {
        return Some(make_error_frame(
            frame.channel_id,
            frame.stream_seq,
            "AUTH_REQUIRED",
        ));
    };
    let expected = expected_dev_token(account);
    if verify_token(&token, &expected) {
        return None;
    }
    Some(make_error_frame(
        frame.channel_id,
        frame.stream_seq,
        "UNAUTHORIZED",
    ))
}

pub fn with_auth_token(frame: Frame, account: &str) -> Frame {
    frame.with_extension(Extension::text(
        ExtensionTag::AuthToken,
        expected_dev_token(account),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::frame::{Frame, FrameType};

    #[test]
    fn dev_token_matches_account() {
        let frame = with_auth_token(Frame::new(FrameType::Subscribe, 1), "DEMO-ACCT");
        assert!(authorize_private(&frame, "DEMO-ACCT").is_none());
        assert!(authorize_private(&frame, "OTHER").is_some());
    }
}
