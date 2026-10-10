//! 0-RTT resumption token replay protection (SPEC §10.1).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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

/// Process-wide replay cache (see [`default_replay_cache`]).
static DEFAULT_CACHE: std::sync::OnceLock<Arc<dyn ReplayCache>> = std::sync::OnceLock::new();

/// Shared reference replay cache used by default transport paths.
///
/// Resolves to a [`RedisReplayCache`] when `FIG_REPLAY_REDIS_URL` is set (and
/// the `session-redis` feature is enabled), otherwise a per-process
/// [`MemoryReplayCache`]. Use [`set_default_replay_cache`] to install a cache
/// explicitly.
pub fn default_replay_cache() -> &'static Arc<dyn ReplayCache> {
    DEFAULT_CACHE.get_or_init(init_default_replay_cache)
}

fn init_default_replay_cache() -> Arc<dyn ReplayCache> {
    #[cfg(feature = "session-redis")]
    {
        if let Ok(url) = std::env::var(REPLAY_REDIS_URL_ENV) {
            if !url.trim().is_empty() {
                match RedisReplayCache::open(url.trim()) {
                    Ok(cache) => {
                        tracing::info!(
                            "replay cache: using shared Redis cache from {REPLAY_REDIS_URL_ENV}"
                        );
                        return Arc::new(cache);
                    }
                    Err(e) => {
                        tracing::error!(
                            "failed to open Redis replay cache from {REPLAY_REDIS_URL_ENV} \
                             ({e}); falling back to a per-process cache — replays are only \
                             rejected on the node that first saw the token"
                        );
                    }
                }
            }
        }
    }
    #[cfg(not(feature = "session-redis"))]
    {
        if std::env::var(REPLAY_REDIS_URL_ENV).is_ok_and(|v| !v.trim().is_empty()) {
            tracing::error!(
                "{REPLAY_REDIS_URL_ENV} is set but the `session-redis` feature is disabled; \
                 using a per-process replay cache — replays are only rejected on the node that \
                 first saw the token. Rebuild with `--features session-redis`."
            );
        }
    }
    Arc::new(MemoryReplayCache::new(10_000))
}

/// Install the process-wide replay cache (only possible before first use).
///
/// Embedders (and HA deployments using a shared store) call this once at
/// startup. Returns the cache back if one was already installed.
pub fn set_default_replay_cache(cache: Arc<dyn ReplayCache>) -> Result<(), Arc<dyn ReplayCache>> {
    DEFAULT_CACHE.set(cache)
}

/// Validate a resumption token against the default replay cache.
pub fn validate_resumption_token(token: &[u8]) -> Result<(), FigError> {
    default_replay_cache().check_and_record(token, DEFAULT_REPLAY_WINDOW)
}

/// Environment variable selecting a shared Redis replay cache.
pub const REPLAY_REDIS_URL_ENV: &str = "FIG_REPLAY_REDIS_URL";

/// Redis-backed replay cache for multi-node deployments (`session-redis` feature).
///
/// Uses `SET key 1 NX PX <window>` so a token replayed to a *different* node is
/// still rejected inside the replay window. Redis performs the check-and-set
/// atomically, so concurrent connections cannot both win.
#[cfg(feature = "session-redis")]
pub struct RedisReplayCache {
    client: Mutex<redis::Connection>,
    prefix: String,
}

#[cfg(feature = "session-redis")]
impl RedisReplayCache {
    /// Open a cache using keys prefixed `fig:replay:`.
    pub fn open(redis_url: &str) -> Result<Self, FigError> {
        Self::with_prefix(redis_url, "fig:replay:")
    }

    /// Open a cache with an explicit key prefix (for tenant isolation).
    pub fn with_prefix(redis_url: &str, prefix: &str) -> Result<Self, FigError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| FigError::ConnectionFailed(format!("redis: {e}")))?;
        let conn = client
            .get_connection()
            .map_err(|e| FigError::ConnectionFailed(format!("redis: {e}")))?;
        Ok(Self {
            client: Mutex::new(conn),
            prefix: prefix.to_string(),
        })
    }

    fn redis_key(&self, key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key);
        format!("{}{}", self.prefix, hex::encode(hasher.finalize()))
    }
}

#[cfg(feature = "session-redis")]
impl ReplayCache for RedisReplayCache {
    fn check_and_record(&self, key: &[u8], ttl: Duration) -> Result<(), FigError> {
        let ms = ttl.as_millis().clamp(1, u64::from(u32::MAX) as u128) as u64;
        let mut conn = self
            .client
            .lock()
            .map_err(|_| FigError::ConnectionFailed("replay cache lock poisoned".into()))?;

        // SET <key> 1 NX PX <ttl_ms> — `Some` means we created it (first sight).
        let set: Option<String> = redis::cmd("SET")
            .arg(self.redis_key(key))
            .arg(1)
            .arg("NX")
            .arg("PX")
            .arg(ms)
            .query(&mut *conn)
            .map_err(|e| FigError::ConnectionFailed(format!("redis: {e}")))?;

        if set.is_some() {
            Ok(())
        } else {
            Err(FigError::ReplayRejected)
        }
    }
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

    #[test]
    fn default_cache_is_usable() {
        // Whatever backend is installed, the trait object must work.
        assert!(default_replay_cache()
            .check_and_record(b"unit-test-token", Duration::from_secs(1))
            .is_ok());
        assert!(matches!(
            default_replay_cache().check_and_record(b"unit-test-token", Duration::from_secs(1)),
            Err(FigError::ReplayRejected)
        ));
    }

    /// Shared-cache behaviour across *separate* cache instances (as two HA nodes
    /// would have): the second node must reject a token the first node recorded.
    #[cfg(feature = "session-redis")]
    #[test]
    fn redis_replay_is_shared_across_instances() {
        let Ok(url) = std::env::var(REPLAY_REDIS_URL_ENV).or_else(|_| std::env::var("REDIS_URL"))
        else {
            eprintln!("skipping Redis replay test (FIG_REPLAY_REDIS_URL / REDIS_URL not set)");
            return;
        };

        let prefix = format!("fig:replay-test:{}:", uuid::Uuid::new_v4());
        let node_a = RedisReplayCache::with_prefix(&url, &prefix).expect("node A");
        let node_b = RedisReplayCache::with_prefix(&url, &prefix).expect("node B");
        assert_shared_replay(&node_a, &node_b);
    }

    #[cfg(feature = "session-redis")]
    fn assert_shared_replay(node_a: &RedisReplayCache, node_b: &RedisReplayCache) {
        let token = b"node-a-resumption-token";
        node_a
            .check_and_record(token, Duration::from_secs(30))
            .expect("first sight is accepted");
        assert!(
            matches!(
                node_b.check_and_record(token, Duration::from_secs(30)),
                Err(FigError::ReplayRejected)
            ),
            "replay via a second node must be rejected"
        );
        // A different token still passes.
        node_b
            .check_and_record(b"other-token", Duration::from_secs(30))
            .expect("distinct token accepted");
    }

    /// Protocol-level check that two cache instances share state through the
    /// `SET key 1 NX PX <ttl>` command, verified against a minimal RESP server.
    ///
    /// Runs everywhere (no external Redis needed); the real-Redis variant above
    /// is exercised in CI, which provisions a `redis:7-alpine` service.
    #[cfg(feature = "session-redis")]
    #[test]
    fn redis_replay_uses_set_nx_px_against_stub_server() {
        let (url, handle) = spawn_resp_stub();
        let prefix = "fig:test:";
        let node_a = RedisReplayCache::with_prefix(&url, prefix).expect("node A");
        let node_b = RedisReplayCache::with_prefix(&url, prefix).expect("node B");
        assert_shared_replay(&node_a, &node_b);
        // Close the client connections so the stub's workers see EOF and exit.
        drop(node_a);
        drop(node_b);
        handle.join().unwrap();
    }

    /// Minimal RESP server implementing only what the replay cache uses:
    /// `SET <key> 1 NX PX <ms>` with real NX/TTL semantics, plus `PING`.
    #[cfg(feature = "session-redis")]
    fn spawn_resp_stub() -> (String, std::thread::JoinHandle<()>) {
        use std::collections::HashMap;
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("bind stub");
        let addr = listener.local_addr().unwrap();
        let handle = std::thread::spawn(move || {
            // Shared store + accept loop; each connection handled on its own thread.
            let store: Arc<Mutex<HashMap<String, Instant>>> = Arc::new(Mutex::new(HashMap::new()));
            let mut workers = Vec::new();
            for stream in listener.incoming().take(2) {
                let mut stream = stream.expect("stub accept");
                let store = store.clone();
                workers.push(std::thread::spawn(move || {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    loop {
                        let mut line = String::new();
                        if reader.read_line(&mut line).unwrap_or(0) == 0 {
                            break;
                        }
                        let line = line.trim_end().to_string();
                        let Some(count) = line.strip_prefix('*') else {
                            continue;
                        };
                        let count: usize = count.parse().unwrap_or(0);
                        let mut args: Vec<Vec<u8>> = Vec::new();
                        for _ in 0..count {
                            let mut len_line = String::new();
                            if reader.read_line(&mut len_line).unwrap_or(0) == 0 {
                                break;
                            }
                            let len: usize = len_line
                                .trim_end()
                                .trim_start_matches('$')
                                .parse()
                                .unwrap_or(0);
                            let mut buf = vec![0u8; len + 2];
                            std::io::Read::read_exact(&mut reader, &mut buf).unwrap();
                            buf.truncate(len);
                            args.push(buf);
                        }
                        let cmd = args
                            .first()
                            .map(|a| String::from_utf8_lossy(a).to_uppercase())
                            .unwrap_or_default();
                        let reply: Vec<u8> = match cmd.as_str() {
                            "SET" => {
                                let key = String::from_utf8_lossy(&args[1]).to_string();
                                let nx = args.iter().any(|a| a.eq_ignore_ascii_case(b"NX"));
                                let px: Option<u64> = args
                                    .iter()
                                    .position(|a| a.eq_ignore_ascii_case(b"PX"))
                                    .and_then(|i| args.get(i + 1))
                                    .and_then(|v| String::from_utf8_lossy(v).parse().ok());
                                let mut store = store.lock().unwrap();
                                let now = Instant::now();
                                let existing = store.get(&key).copied().filter(|t| {
                                    px.is_none_or(|ms| {
                                        now.duration_since(*t) < Duration::from_millis(ms)
                                    })
                                });
                                if nx && existing.is_some() {
                                    b"$-1\r\n".to_vec() // nil: not set (already present)
                                } else {
                                    store.insert(key, now);
                                    b"+OK\r\n".to_vec()
                                }
                            }
                            "PING" => b"+PONG\r\n".to_vec(),
                            _ => b"+OK\r\n".to_vec(),
                        };
                        stream.write_all(&reply).unwrap();
                    }
                }));
            }
            for w in workers {
                let _ = w.join();
            }
        });
        (format!("redis://{addr}/"), handle)
    }
}
