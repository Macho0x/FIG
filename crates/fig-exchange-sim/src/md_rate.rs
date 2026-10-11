//! Public market-data rate limits by tier (SPEC §9.3).
//!
//! Connection-level [`fig_core::DoSGuard`] and per-channel
//! [`fig_core::ChannelRateLimiter`] still apply. This module is the
//! **path class × auth presence** policy for `marketdata/…`:
//!
//! | Tier | Qualifies when | Default cap |
//! |---|---|---|
//! | Public | `marketdata/…` and no `AUTH_TOKEN` | 32 REQUEST+SUBSCRIBE / session / s |
//! | Authenticated | `marketdata/…` with `AUTH_TOKEN` | 256 / session / s |
//! | Internal | `FIG_DEV_OPEN=1` (honored) | unlimited |
//!
//! Exceeding the cap returns `STREAM_ERROR` / `RATE_LIMITED` / 429.

use std::collections::HashMap;
use std::sync::Mutex;

use fig_core::ext::ExtensionTag;
use fig_core::frame::{Frame, FrameType};
use fig_core::rate_limit::RateLimiter;
use uuid::Uuid;

use crate::auth::{auth_token_from_frame, dev_open};

/// Caps for the public and authenticated MD buckets.
#[derive(Debug, Clone, Copy)]
pub struct MdRateConfig {
    pub public_per_sec: u64,
    pub authenticated_per_sec: u64,
    /// When true (default), `FIG_DEV_OPEN=1` is the Internal tier.
    pub honor_dev_open: bool,
}

impl Default for MdRateConfig {
    fn default() -> Self {
        Self {
            public_per_sec: 32,
            authenticated_per_sec: 256,
            honor_dev_open: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdTier {
    Public,
    Authenticated,
    Internal,
}

pub fn is_public_md_path(path: &str) -> bool {
    path.trim_start_matches('/').starts_with("marketdata/")
}

pub fn classify_tier(frame: &Frame, honor_dev_open: bool) -> MdTier {
    if honor_dev_open && dev_open() {
        return MdTier::Internal;
    }
    if auth_token_from_frame(frame).is_some() {
        MdTier::Authenticated
    } else {
        MdTier::Public
    }
}

fn channel_path(frame: &Frame) -> &str {
    frame
        .extensions
        .iter()
        .find(|e| e.tag == ExtensionTag::ChannelPath)
        .and_then(|e| e.value.as_text())
        .unwrap_or("")
}

/// Per-session buckets for public MD REQUEST and SUBSCRIBE.
#[derive(Debug)]
pub struct MdRateLimiter {
    config: MdRateConfig,
    public: Mutex<HashMap<Uuid, RateLimiter>>,
    authenticated: Mutex<HashMap<Uuid, RateLimiter>>,
}

impl MdRateLimiter {
    pub fn new(config: MdRateConfig) -> Self {
        Self {
            config,
            public: Mutex::new(HashMap::new()),
            authenticated: Mutex::new(HashMap::new()),
        }
    }

    pub fn config(&self) -> MdRateConfig {
        self.config
    }

    /// `None` = allowed. `Some(error frame)` = reject.
    pub fn admit(&self, session_id: Uuid, frame: &Frame) -> Option<Frame> {
        if !matches!(frame.frame_type, FrameType::Request | FrameType::Subscribe) {
            return None;
        }
        if !is_public_md_path(channel_path(frame)) {
            return None;
        }
        let allowed = match classify_tier(frame, self.config.honor_dev_open) {
            MdTier::Internal => true,
            MdTier::Public => allow(&self.public, session_id, self.config.public_per_sec),
            MdTier::Authenticated => allow(
                &self.authenticated,
                session_id,
                self.config.authenticated_per_sec,
            ),
        };
        if allowed {
            None
        } else {
            Some(crate::server::make_rate_limited_frame(
                frame.channel_id,
                frame.stream_seq,
            ))
        }
    }
}

fn allow(map: &Mutex<HashMap<Uuid, RateLimiter>>, session_id: Uuid, max_per_sec: u64) -> bool {
    let mut guard = map.lock().unwrap();
    let limiter = guard
        .entry(session_id)
        .or_insert_with(|| RateLimiter::new(max_per_sec));
    limiter.try_acquire()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_core::ext::Extension;

    fn md_get() -> Frame {
        Frame::new(FrameType::Request, 1).with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "marketdata/AAPL/book",
        ))
    }

    fn private_get() -> Frame {
        Frame::new(FrameType::Request, 1)
            .with_extension(Extension::text(ExtensionTag::ChannelPath, "accounts/DEMO"))
    }

    #[test]
    fn public_path_class() {
        assert!(is_public_md_path("marketdata/AAPL/book"));
        assert!(is_public_md_path("/marketdata/session"));
        assert!(!is_public_md_path(".well-known/capabilities"));
        assert!(!is_public_md_path("trading/accounts/DEMO/orders"));
    }

    #[test]
    fn public_bucket_exhausts() {
        let limiter = MdRateLimiter::new(MdRateConfig {
            public_per_sec: 2,
            authenticated_per_sec: 8,
            honor_dev_open: false,
        });
        let sid = Uuid::nil();
        let frame = md_get();
        assert!(limiter.admit(sid, &frame).is_none());
        assert!(limiter.admit(sid, &frame).is_none());
        let err = limiter.admit(sid, &frame).expect("third is limited");
        assert_eq!(err.frame_type, FrameType::StreamError);
        let code = err
            .extensions
            .iter()
            .find(|e| e.tag == ExtensionTag::ErrorCode)
            .and_then(|e| e.value.as_text());
        assert_eq!(code, Some("RATE_LIMITED"));
    }

    #[test]
    fn authenticated_bucket_is_separate() {
        let limiter = MdRateLimiter::new(MdRateConfig {
            public_per_sec: 1,
            authenticated_per_sec: 2,
            honor_dev_open: false,
        });
        let sid = Uuid::nil();
        let public = md_get();
        assert!(limiter.admit(sid, &public).is_none());
        assert!(limiter.admit(sid, &public).is_some());

        let authed =
            md_get().with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-DEMO"));
        assert!(limiter.admit(sid, &authed).is_none());
        assert!(limiter.admit(sid, &authed).is_none());
        assert!(limiter.admit(sid, &authed).is_some());
    }

    #[test]
    fn private_paths_are_not_this_limiter() {
        let limiter = MdRateLimiter::new(MdRateConfig {
            public_per_sec: 1,
            authenticated_per_sec: 1,
            honor_dev_open: false,
        });
        let sid = Uuid::nil();
        let frame = private_get();
        assert!(limiter.admit(sid, &frame).is_none());
        assert!(limiter.admit(sid, &frame).is_none());
    }
}
