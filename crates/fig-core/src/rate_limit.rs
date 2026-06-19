//! Token-bucket rate limiting for FIG connections and channels.
//!
//! Provides per-key rate limiters to protect against message floods on
//! individual connections or channels.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// A single token-bucket rate limiter.
#[derive(Debug)]
pub struct RateLimiter {
    max_tokens: u64,
    refill_interval: Duration,
    tokens: u64,
    last_refill: Instant,
}

impl RateLimiter {
    /// Create a limiter allowing `max_per_second` events per second.
    pub fn new(max_per_second: u64) -> Self {
        Self {
            max_tokens: max_per_second.max(1),
            refill_interval: Duration::from_secs(1),
            tokens: max_per_second.max(1),
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed();
        if elapsed >= self.refill_interval {
            let periods = elapsed.as_secs_f64() / self.refill_interval.as_secs_f64();
            let added = (periods.floor() as u64).saturating_mul(self.max_tokens);
            self.tokens = (self.tokens + added).min(self.max_tokens);
            self.last_refill = Instant::now();
        }
    }

    /// Attempt to consume one token. Returns true if allowed.
    pub fn try_acquire(&mut self) -> bool {
        self.refill();
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Remaining tokens (after refill).
    pub fn available(&mut self) -> u64 {
        self.refill();
        self.tokens
    }
}

/// Rate limiter keyed by u16 (typically channel ID).
#[derive(Debug, Default)]
pub struct ChannelRateLimiter {
    limiters: Mutex<HashMap<u16, RateLimiter>>,
    max_per_second: u64,
}

impl ChannelRateLimiter {
    pub fn new(max_per_second: u64) -> Self {
        Self {
            limiters: Mutex::new(HashMap::new()),
            max_per_second,
        }
    }

    /// Check whether a message on `channel_id` is allowed.
    pub fn allow(&self, channel_id: u16) -> bool {
        let mut guard = self.limiters.lock().unwrap();
        let limiter = guard
            .entry(channel_id)
            .or_insert_with(|| RateLimiter::new(self.max_per_second));
        limiter.try_acquire()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_burst() {
        let mut limiter = RateLimiter::new(5);
        for _ in 0..5 {
            assert!(limiter.try_acquire());
        }
        assert!(!limiter.try_acquire());
    }

    #[test]
    fn test_channel_rate_limiter_per_channel() {
        let limiter = ChannelRateLimiter::new(2);
        assert!(limiter.allow(1));
        assert!(limiter.allow(1));
        assert!(!limiter.allow(1));
        // Different channel has its own bucket
        assert!(limiter.allow(2));
    }
}
