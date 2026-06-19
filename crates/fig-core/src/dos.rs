//! DoS protection for FIG connections (Spec §15).
//!
//! Limits connection-level resource consumption: max channels, max frame
//! size, and per-connection message rate.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::error::{ChannelError, FrameError};
use crate::frame::MIN_FRAME_SIZE;
use crate::rate_limit::RateLimiter;

/// Default maximum channels per connection.
pub const DEFAULT_MAX_CHANNELS: u32 = 1024;

/// Default maximum frame payload size (1 MiB).
pub const DEFAULT_MAX_FRAME_BYTES: usize = 1024 * 1024;

/// Connection-level DoS guard.
#[derive(Debug)]
pub struct DoSGuard {
    max_channels: u32,
    max_frame_bytes: usize,
    open_channels: AtomicU32,
    message_limiter: Mutex<RateLimiter>,
}

impl DoSGuard {
    pub fn new(max_channels: u32, max_frame_bytes: usize, max_messages_per_sec: u64) -> Self {
        Self {
            max_channels,
            max_frame_bytes,
            open_channels: AtomicU32::new(0),
            message_limiter: Mutex::new(RateLimiter::new(max_messages_per_sec)),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(DEFAULT_MAX_CHANNELS, DEFAULT_MAX_FRAME_BYTES, 10_000)
    }

    /// Check whether a new channel may be opened.
    pub fn allow_open_channel(&self) -> Result<(), ChannelError> {
        let current = self.open_channels.load(Ordering::SeqCst);
        if current >= self.max_channels {
            return Err(ChannelError::ChannelIdExhausted);
        }
        self.open_channels.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    /// Record channel close.
    pub fn channel_closed(&self) {
        self.open_channels.fetch_sub(1, Ordering::SeqCst);
    }

    /// Validate incoming/outgoing frame size.
    pub fn check_frame_size(&self, size: usize) -> Result<(), FrameError> {
        if size < MIN_FRAME_SIZE {
            return Err(FrameError::BufferTooShort {
                expected: MIN_FRAME_SIZE,
                actual: size,
            });
        }
        if size > self.max_frame_bytes {
            return Err(FrameError::InvalidHeader(size));
        }
        Ok(())
    }

    /// Rate-limit message processing.
    pub fn allow_message(&self) -> bool {
        self.message_limiter.lock().unwrap().try_acquire()
    }

    /// Current open channel count.
    pub fn open_channel_count(&self) -> u32 {
        self.open_channels.load(Ordering::SeqCst)
    }
}

/// Sliding-window flood detector for burst attacks.
#[derive(Debug)]
pub struct FloodDetector {
    window: Duration,
    max_events: u32,
    events: Mutex<Vec<Instant>>,
}

impl FloodDetector {
    pub fn new(window: Duration, max_events: u32) -> Self {
        Self {
            window,
            max_events,
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn record_event(&self) -> bool {
        let now = Instant::now();
        let mut events = self.events.lock().unwrap();
        events.retain(|t| now.duration_since(*t) < self.window);
        if events.len() as u32 >= self.max_events {
            return false;
        }
        events.push(now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dos_guard_channel_limit() {
        let guard = DoSGuard::new(2, 4096, 1000);
        guard.allow_open_channel().unwrap();
        guard.allow_open_channel().unwrap();
        assert!(guard.allow_open_channel().is_err());
        guard.channel_closed();
        guard.allow_open_channel().unwrap();
    }

    #[test]
    fn test_dos_guard_frame_size() {
        let guard = DoSGuard::new(10, 100, 1000);
        assert!(guard.check_frame_size(50).is_ok());
        assert!(guard.check_frame_size(200).is_err());
    }

    #[test]
    fn test_flood_detector() {
        let detector = FloodDetector::new(Duration::from_secs(1), 3);
        assert!(detector.record_event());
        assert!(detector.record_event());
        assert!(detector.record_event());
        assert!(!detector.record_event());
    }
}
