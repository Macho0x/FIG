//! 0-RTT resumption token replay protection (SPEC §10.1).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use crate::error::FigError;

/// Default replay rejection window for resumption tokens.
pub const DEFAULT_REPLAY_WINDOW: Duration = Duration::from_secs(30);

/// Reject duplicate keys seen within `ttl`.
pub trait ReplayCache: Send + Sync {
    fn check_and_record(&self, key: &[u8], ttl: Duration) -> Result<(), FigError>;
}

/// In-memory bounded replay cache for single-node deployments.
pub struct MemoryReplayCache {
    max_entries: usize,
    inner: Mutex<HashMap<[u8; 32], Instant>>,
}

impl MemoryReplayCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries,
            inner: Mutex::new(HashMap::new()),
        }
    }

    fn hash_key(key: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(key);
        hasher.finalize().into()
    }
}

impl ReplayCache for MemoryReplayCache {
    fn check_and_record(&self, key: &[u8], ttl: Duration) -> Result<(), FigError> {
        let digest = Self::hash_key(key);
        let now = Instant::now();
        let mut map = self.inner.lock().expect("replay cache lock");

        map.retain(|_, seen| now.duration_since(*seen) < ttl);

        if map.contains_key(&digest) {
            return Err(FigError::ReplayRejected);
        }

        if map.len() >= self.max_entries {
            if let Some(oldest) = map.iter().min_by_key(|(_, t)| *t).map(|(k, _)| *k) {
                map.remove(&oldest);
            }
        }

        map.insert(digest, now);
        Ok(())
    }
}

/// Shared reference replay cache used by default transport paths.
pub fn default_replay_cache() -> &'static MemoryReplayCache {
    static CACHE: std::sync::OnceLock<MemoryReplayCache> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| MemoryReplayCache::new(10_000))
}

/// Validate a resumption token against the default replay cache.
pub fn validate_resumption_token(token: &[u8]) -> Result<(), FigError> {
    default_replay_cache().check_and_record(token, DEFAULT_REPLAY_WINDOW)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_within_ttl() {
        let cache = MemoryReplayCache::new(100);
        let token = b"resumption-token-abc";
        cache
            .check_and_record(token, Duration::from_secs(30))
            .unwrap();
        assert!(matches!(
            cache.check_and_record(token, Duration::from_secs(30)),
            Err(FigError::ReplayRejected)
        ));
    }

    #[test]
    fn allows_different_tokens() {
        let cache = MemoryReplayCache::new(100);
        cache
            .check_and_record(b"token-a", Duration::from_secs(30))
            .unwrap();
        cache
            .check_and_record(b"token-b", Duration::from_secs(30))
            .unwrap();
    }
}
