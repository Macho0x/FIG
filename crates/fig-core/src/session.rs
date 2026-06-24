//! Session model and storage for FIG.
//!
//! A FIG session is a durable, migratable logical entity identified by
//! a SESSION_ID (UUID). Sessions survive disconnects and can be resumed
//! across TREE 0-RTT reconnections.
//!
//! The [`SessionStore`] trait provides a pluggable storage backend.
//! [`MemorySessionStore`] is an in-memory implementation suitable for
//! development and testing. Production deployments should use Redis,
//! etcd, or a durable database.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::SessionError;

/// Default idle TTL for sessions (1 hour). A TTL of 0 disables expiry checks.
pub const DEFAULT_SESSION_TTL_SECS: u64 = 3600;

/// A subscription persisted on a session for resume after reconnect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SessionSubscription {
    pub channel_path: String,
    pub routing_key: String,
    pub kind: String,
    pub account: Option<String>,
}

/// Internal struct for resumption token serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResumptionTokenData {
    session_id: Uuid,
    auth_token: Option<Vec<u8>>,
    channel_ids: Vec<u16>,
    subscriptions: Vec<SessionSubscription>,
    last_seq_sent: u32,
    last_seq_recv: u32,
    created_at: u64,
}

/// A durable, migratable session identified by SESSION_ID.
///
/// Sessions track active channels, sequence numbers, and
/// authentication state across connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier (UUID v4).
    pub session_id: Uuid,
    /// Opaque authentication token (JWT, bearer, etc.).
    pub auth_token: Option<Vec<u8>>,
    /// IDs of channels currently associated with this session.
    pub channels: Vec<u16>,
    /// Active pub/sub subscriptions to restore after reconnect.
    #[serde(default)]
    pub subscriptions: Vec<SessionSubscription>,
    /// Last sequence number sent on any channel in this session.
    pub last_seq_sent: u32,
    /// Last sequence number received on any channel in this session.
    pub last_seq_recv: u32,
    /// Unix timestamp (seconds) when the session was created.
    pub created_at: u64,
    /// Unix timestamp (seconds) of the last activity.
    pub last_active_at: u64,
}

impl Session {
    /// Create a new session with a fresh UUID v4.
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            session_id: Uuid::new_v4(),
            auth_token: None,
            channels: Vec::new(),
            subscriptions: Vec::new(),
            last_seq_sent: 0,
            last_seq_recv: 0,
            created_at: now,
            last_active_at: now,
        }
    }

    /// Attach an authentication token.
    pub fn with_auth_token(mut self, token: Vec<u8>) -> Self {
        self.auth_token = Some(token);
        self
    }

    /// Associate a channel with this session.
    pub fn add_channel(&mut self, channel_id: u16) {
        if !self.channels.contains(&channel_id) {
            self.channels.push(channel_id);
        }
        self.touch();
    }

    /// Remove a channel from this session.
    pub fn remove_channel(&mut self, channel_id: u16) {
        self.channels.retain(|&id| id != channel_id);
        self.touch();
    }

    /// Persist a subscription for session resume.
    pub fn add_subscription(&mut self, sub: SessionSubscription) {
        if !self.subscriptions.iter().any(|s| {
            s.channel_path == sub.channel_path
                && s.routing_key == sub.routing_key
                && s.kind == sub.kind
        }) {
            self.subscriptions.push(sub);
            self.touch();
        }
    }

    /// Drop a persisted subscription (UNSUBSCRIBE).
    pub fn remove_subscription(&mut self, channel_path: &str, routing_key: &str) {
        self.subscriptions.retain(|s| {
            !(s.channel_path == channel_path
                && (routing_key.is_empty() || s.routing_key == routing_key))
        });
        self.touch();
    }

    /// Update the last-active timestamp to now.
    pub fn touch(&mut self) {
        self.last_active_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Record a sent sequence number (per-session aggregate).
    pub fn record_sent_seq(&mut self, seq: u32) {
        self.last_seq_sent = self.last_seq_sent.max(seq);
        self.touch();
    }

    /// Record a received sequence number (per-session aggregate).
    pub fn record_recv_seq(&mut self, seq: u32) {
        self.last_seq_recv = self.last_seq_recv.max(seq);
        self.touch();
    }

    /// Create a resumption token for 0-RTT reconnection.
    ///
    /// The token contains the session ID, auth token, and channel state,
    /// encoded as CBOR. This allows re-establishing a session without
    /// a full handshake.
    pub fn resumption_token(&self) -> Result<Vec<u8>, SessionError> {
        let data = ResumptionTokenData {
            session_id: self.session_id,
            auth_token: self.auth_token.clone(),
            channel_ids: self.channels.clone(),
            subscriptions: self.subscriptions.clone(),
            last_seq_sent: self.last_seq_sent,
            last_seq_recv: self.last_seq_recv,
            created_at: self.created_at,
        };
        crate::codec::encode_cbor(&data)
            .map_err(|e| SessionError::SerializationError(e.to_string()))
    }

    /// Restore a session from a resumption token.
    ///
    /// The token must have been created by [`Session::resumption_token`].
    pub fn from_resumption_token(token: &[u8]) -> Result<Session, SessionError> {
        let data: ResumptionTokenData =
            crate::codec::decode_cbor(token).map_err(|_| SessionError::InvalidResumptionToken)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(Session {
            session_id: data.session_id,
            auth_token: data.auth_token,
            channels: data.channel_ids,
            subscriptions: data.subscriptions,
            last_seq_sent: data.last_seq_sent,
            last_seq_recv: data.last_seq_recv,
            created_at: data.created_at,
            last_active_at: now,
        })
    }

    /// Check if this session can be resumed (has active channels).
    pub fn can_resume(&self) -> bool {
        !self.channels.is_empty()
    }

    /// Seconds since last activity on this session.
    pub fn idle_secs(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_active_at)
    }

    /// Returns true if the session has been idle longer than `ttl_secs`.
    ///
    /// A `ttl_secs` of 0 disables expiry (always returns false).
    pub fn is_expired(&self, ttl_secs: u64) -> bool {
        if ttl_secs == 0 {
            return false;
        }
        self.idle_secs() > ttl_secs
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for pluggable session storage backends.
///
/// Implementations can use in-memory `HashMap` (for testing),
/// Redis, etcd, PostgreSQL, or any other durable store.
pub trait SessionStore: Send + Sync {
    /// Retrieve a session by its ID.
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError>;

    /// Store a new session.
    fn put(&self, session: &Session) -> Result<(), SessionError>;

    /// Delete a session.
    fn delete(&self, id: &Uuid) -> Result<(), SessionError>;

    /// Update an existing session (must already exist).
    fn update(&self, session: &Session) -> Result<(), SessionError>;
}

/// In-memory session store for development and testing.
///
/// **Production use:** replace with a durable store (Redis, etcd, …).
#[derive(Debug)]
pub struct MemorySessionStore {
    sessions: Mutex<HashMap<Uuid, Session>>,
    /// Idle TTL in seconds. None or 0 disables automatic expiry.
    ttl_secs: Option<u64>,
}

impl MemorySessionStore {
    /// Create a new empty in-memory store (no TTL expiry).
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            ttl_secs: None,
        }
    }

    /// Create a store that expires idle sessions after `ttl_secs`.
    pub fn with_ttl(ttl_secs: u64) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            ttl_secs: if ttl_secs == 0 { None } else { Some(ttl_secs) },
        }
    }

    /// Remove all expired sessions. Returns the number purged.
    pub fn purge_expired(&self) -> Result<usize, SessionError> {
        let Some(ttl) = self.ttl_secs else {
            return Ok(0);
        };
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SerializationError("lock poisoned".into()))?;
        let before = guard.len();
        guard.retain(|_, session| !session.is_expired(ttl));
        Ok(before - guard.len())
    }

    /// Returns the number of sessions currently stored.
    pub fn len(&self) -> usize {
        self.sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .len()
    }

    /// Returns true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.sessions
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .is_empty()
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore for MemorySessionStore {
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SessionNotFound(*id))?;
        if let Some(session) = guard.get(id) {
            if let Some(ttl) = self.ttl_secs {
                if session.is_expired(ttl) {
                    guard.remove(id);
                    return Err(SessionError::SessionExpired(*id));
                }
            }
            return Ok(Some(session.clone()));
        }
        Ok(None)
    }

    fn put(&self, session: &Session) -> Result<(), SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SessionNotFound(session.session_id))?;
        guard.insert(session.session_id, session.clone());
        Ok(())
    }

    fn delete(&self, id: &Uuid) -> Result<(), SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SessionNotFound(*id))?;
        guard.remove(id);
        Ok(())
    }

    fn update(&self, session: &Session) -> Result<(), SessionError> {
        let mut guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SessionNotFound(session.session_id))?;
        if let std::collections::hash_map::Entry::Occupied(mut entry) =
            guard.entry(session.session_id)
        {
            entry.insert(session.clone());
            Ok(())
        } else {
            Err(SessionError::SessionNotFound(session.session_id))
        }
    }
}

/// Persistent file-based session store.
///
/// Each session is stored as a JSON file at `<dir>/<session-id>.json`.
/// Suitable for single-node deployments. For multi-node deployments,
/// use a shared Redis or database-backed store instead.
pub struct FileSessionStore {
    dir: PathBuf,
    /// Idle TTL in seconds. None or 0 disables automatic expiry.
    ttl_secs: Option<u64>,
}

impl FileSessionStore {
    /// Create a new file-based session store (no TTL expiry).
    ///
    /// Creates the directory if it doesn't exist.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            tracing::warn!("Failed to create session directory {:?}: {}", dir, e);
        }
        FileSessionStore {
            dir,
            ttl_secs: None,
        }
    }

    /// Create a store that expires idle sessions after `ttl_secs`.
    pub fn with_ttl(dir: impl Into<PathBuf>, ttl_secs: u64) -> Self {
        let mut store = Self::new(dir);
        store.ttl_secs = if ttl_secs == 0 { None } else { Some(ttl_secs) };
        store
    }

    /// Remove all expired session files. Returns the number purged.
    pub fn purge_expired(&self) -> Result<usize, SessionError> {
        let Some(ttl) = self.ttl_secs else {
            return Ok(0);
        };
        let mut purged = 0;
        for entry in std::fs::read_dir(&self.dir).map_err(SessionError::IoError)? {
            let entry = entry.map_err(SessionError::IoError)?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let data = std::fs::read(&path).map_err(SessionError::IoError)?;
            let session: Session = serde_json::from_slice(&data)
                .map_err(|e| SessionError::SerializationError(e.to_string()))?;
            if session.is_expired(ttl) {
                std::fs::remove_file(&path).map_err(SessionError::IoError)?;
                purged += 1;
            }
        }
        Ok(purged)
    }

    /// Return the file path for a given session ID.
    fn file_path(&self, id: &Uuid) -> PathBuf {
        self.dir.join(format!("{}.json", id))
    }
}

impl SessionStore for FileSessionStore {
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError> {
        let path = self.file_path(id);
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&path).map_err(SessionError::IoError)?;
        let session: Session = serde_json::from_slice(&data)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        if let Some(ttl) = self.ttl_secs {
            if session.is_expired(ttl) {
                std::fs::remove_file(&path).map_err(SessionError::IoError)?;
                return Err(SessionError::SessionExpired(*id));
            }
        }
        Ok(Some(session))
    }

    fn put(&self, session: &Session) -> Result<(), SessionError> {
        let path = self.file_path(&session.session_id);
        let data = serde_json::to_vec(session)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        std::fs::write(&path, &data).map_err(SessionError::IoError)?;
        Ok(())
    }

    fn delete(&self, id: &Uuid) -> Result<(), SessionError> {
        let path = self.file_path(id);
        if path.exists() {
            std::fs::remove_file(&path).map_err(SessionError::IoError)?;
        }
        Ok(())
    }

    fn update(&self, session: &Session) -> Result<(), SessionError> {
        let path = self.file_path(&session.session_id);
        if !path.exists() {
            return Err(SessionError::SessionNotFound(session.session_id));
        }
        let data = serde_json::to_vec(session)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        std::fs::write(&path, &data).map_err(SessionError::IoError)?;
        Ok(())
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Session creation ──────────────────────────────────────

    #[test]
    fn test_session_new() {
        let s = Session::new();
        // Verify UUID v4 byte layout: variant bits = 10xx xxxx, version nibble = 0100
        let bytes = s.session_id.as_bytes();
        assert_eq!(bytes[6] >> 4, 4, "UUID version should be 4");
        assert!(bytes[8] >> 6 == 2, "UUID variant should be RFC 4122");

        assert!(s.auth_token.is_none());
        assert!(s.channels.is_empty());
        assert_eq!(s.last_seq_sent, 0);
        assert_eq!(s.last_seq_recv, 0);
        assert!(s.created_at > 0);
        assert_eq!(s.last_active_at, s.created_at);
    }

    #[test]
    fn test_session_with_auth_token() {
        let token = b"bearer-token-123".to_vec();
        let s = Session::new().with_auth_token(token.clone());
        assert_eq!(s.auth_token.as_deref(), Some(&token[..]));
    }

    #[test]
    fn test_session_channels() {
        let mut s = Session::new();
        s.add_channel(1);
        s.add_channel(2);
        s.add_channel(1); // duplicate — ignored
        assert_eq!(s.channels, vec![1, 2]);

        s.remove_channel(1);
        assert_eq!(s.channels, vec![2]);
    }

    #[test]
    fn test_session_seq_tracking() {
        let mut s = Session::new();
        s.record_sent_seq(5);
        s.record_sent_seq(3); // should not decrease
        assert_eq!(s.last_seq_sent, 5);

        s.record_recv_seq(10);
        assert_eq!(s.last_seq_recv, 10);
    }

    #[test]
    fn test_session_touch() {
        let mut s = Session::new();
        let ts1 = s.last_active_at;

        // Simulate time passing — touch should update the timestamp.
        // In unit tests the clock may not advance, but the method should
        // not panic and should produce a value >= the original.
        std::thread::sleep(std::time::Duration::from_millis(1));
        s.touch();
        assert!(s.last_active_at >= ts1);
    }

    // ── MemorySessionStore ────────────────────────────────────

    #[test]
    fn test_store_create_and_retrieve() {
        let store = MemorySessionStore::new();
        let session = Session::new();

        store.put(&session).unwrap();
        assert_eq!(store.len(), 1);

        let retrieved = store.get(&session.session_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().session_id, session.session_id);
    }

    #[test]
    fn test_store_update() {
        let store = MemorySessionStore::new();
        let mut session = Session::new();
        store.put(&session).unwrap();

        session.add_channel(42);
        store.update(&session).unwrap();

        let retrieved = store.get(&session.session_id).unwrap().unwrap();
        assert_eq!(retrieved.channels, vec![42]);
    }

    #[test]
    fn test_store_update_nonexistent() {
        let store = MemorySessionStore::new();
        let session = Session::new();
        let result = store.update(&session);
        assert!(result.is_err());
        match result {
            Err(SessionError::SessionNotFound(_)) => {} // expected
            _ => panic!("expected SessionNotFound"),
        }
    }

    #[test]
    fn test_store_delete() {
        let store = MemorySessionStore::new();
        let session = Session::new();
        let id = session.session_id;

        store.put(&session).unwrap();
        assert_eq!(store.len(), 1);

        store.delete(&id).unwrap();
        assert_eq!(store.len(), 0);
        assert!(store.get(&id).unwrap().is_none());
    }

    #[test]
    fn test_store_get_nonexistent() {
        let store = MemorySessionStore::new();
        let fake_id = Uuid::new_v4();
        let result = store.get(&fake_id).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_store_multiple_sessions() {
        let store = MemorySessionStore::new();
        let s1 = Session::new();
        let s2 = Session::new();
        let s3 = Session::new();

        store.put(&s1).unwrap();
        store.put(&s2).unwrap();
        store.put(&s3).unwrap();
        assert_eq!(store.len(), 3);

        store.delete(&s2.session_id).unwrap();
        assert_eq!(store.len(), 2);

        assert!(store.get(&s1.session_id).unwrap().is_some());
        assert!(store.get(&s2.session_id).unwrap().is_none());
        assert!(store.get(&s3.session_id).unwrap().is_some());
    }

    #[test]
    fn test_session_default() {
        let s = Session::default();
        assert!(s.auth_token.is_none());
        assert!(s.channels.is_empty());
    }

    // ── Resumption tokens ─────────────────────────────────────

    #[test]
    fn test_resumption_token_round_trip() {
        let mut s = Session::new();
        s.add_channel(1);
        s.add_channel(2);
        s.record_sent_seq(42);
        s.record_recv_seq(99);
        s.auth_token = Some(b"test-token".to_vec());

        let token = s.resumption_token().unwrap();
        assert!(!token.is_empty());

        let restored = Session::from_resumption_token(&token).unwrap();
        assert_eq!(restored.session_id, s.session_id);
        assert_eq!(restored.auth_token, s.auth_token);
        assert_eq!(restored.channels, s.channels);
        assert_eq!(restored.last_seq_sent, s.last_seq_sent);
        assert_eq!(restored.last_seq_recv, s.last_seq_recv);
        assert_eq!(restored.created_at, s.created_at);
    }

    #[test]
    fn test_resumption_token_no_channels() {
        let s = Session::new();
        let token = s.resumption_token().unwrap();
        let restored = Session::from_resumption_token(&token).unwrap();
        assert_eq!(restored.channels, Vec::<u16>::new());
    }

    #[test]
    fn test_can_resume() {
        let mut s = Session::new();
        assert!(!s.can_resume());

        s.add_channel(5);
        assert!(s.can_resume());

        s.remove_channel(5);
        assert!(!s.can_resume());
    }

    #[test]
    fn test_from_resumption_token_invalid() {
        let garbage = b"this is not a valid resumption token";
        let result = Session::from_resumption_token(garbage);
        assert!(result.is_err());
    }

    // ── FileSessionStore CRUD ────────────────────────────────

    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("fig-test-{}", Uuid::new_v4()));
        dir
    }

    #[test]
    fn test_file_store_create_and_retrieve() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let session = Session::new();

        store.put(&session).unwrap();
        assert!(dir.join(format!("{}.json", session.session_id)).exists());

        let retrieved = store.get(&session.session_id).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().session_id, session.session_id);

        // Clean up
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_update() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let mut session = Session::new();
        store.put(&session).unwrap();

        session.add_channel(42);
        store.update(&session).unwrap();

        let retrieved = store.get(&session.session_id).unwrap().unwrap();
        assert_eq!(retrieved.channels, vec![42]);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_update_nonexistent() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let session = Session::new();
        let result = store.update(&session);
        assert!(result.is_err());
        match result {
            Err(SessionError::SessionNotFound(_)) => {} // expected
            _ => panic!("expected SessionNotFound"),
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_delete() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let session = Session::new();
        let id = session.session_id;

        store.put(&session).unwrap();
        assert!(dir.join(format!("{}.json", id)).exists());

        store.delete(&id).unwrap();
        assert!(!dir.join(format!("{}.json", id)).exists());
        assert!(store.get(&id).unwrap().is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_get_nonexistent() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let fake_id = Uuid::new_v4();
        let result = store.get(&fake_id).unwrap();
        assert!(result.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_multiple_sessions() {
        let dir = temp_dir();
        let store = FileSessionStore::new(&dir);
        let s1 = Session::new();
        let s2 = Session::new();
        let s3 = Session::new();

        store.put(&s1).unwrap();
        store.put(&s2).unwrap();
        store.put(&s3).unwrap();

        // All files should exist
        assert!(dir.join(format!("{}.json", s1.session_id)).exists());
        assert!(dir.join(format!("{}.json", s2.session_id)).exists());
        assert!(dir.join(format!("{}.json", s3.session_id)).exists());

        store.delete(&s2.session_id).unwrap();

        assert!(store.get(&s1.session_id).unwrap().is_some());
        assert!(store.get(&s2.session_id).unwrap().is_none());
        assert!(store.get(&s3.session_id).unwrap().is_some());

        // File for s2 should be gone
        assert!(!dir.join(format!("{}.json", s2.session_id)).exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_persistence() {
        let dir = temp_dir();
        let session = Session::new();
        let id = session.session_id;

        {
            let store = FileSessionStore::new(&dir);
            store.put(&session).unwrap();
        }

        // Create a new store pointing to the same directory
        {
            let store = FileSessionStore::new(&dir);
            let retrieved = store.get(&id).unwrap();
            assert!(retrieved.is_some());
            assert_eq!(retrieved.unwrap().session_id, id);
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_creates_dir() {
        let dir = temp_dir();
        // Ensure the directory does not exist yet
        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }
        assert!(!dir.exists());

        let _store = FileSessionStore::new(&dir);
        assert!(dir.exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    // ── Session TTL / expiry ────────────────────────────────────

    #[test]
    fn test_session_is_expired() {
        let mut session = Session::new();
        assert!(!session.is_expired(60));
        session.last_active_at = session.last_active_at.saturating_sub(120);
        assert!(session.is_expired(60));
        assert!(!session.is_expired(0)); // TTL 0 disables expiry
    }

    #[test]
    fn test_memory_store_expires_on_get() {
        let store = MemorySessionStore::with_ttl(30);
        let mut session = Session::new();
        let id = session.session_id;
        session.last_active_at = session.last_active_at.saturating_sub(60);
        store.put(&session).unwrap();

        let result = store.get(&id);
        assert!(matches!(result, Err(SessionError::SessionExpired(_))));
        assert!(store.get(&id).unwrap().is_none());
    }

    #[test]
    fn test_memory_store_purge_expired() {
        let store = MemorySessionStore::with_ttl(10);
        let mut expired = Session::new();
        expired.last_active_at = expired.last_active_at.saturating_sub(100);
        let active = Session::new();

        store.put(&expired).unwrap();
        store.put(&active).unwrap();
        assert_eq!(store.len(), 2);

        let purged = store.purge_expired().unwrap();
        assert_eq!(purged, 1);
        assert_eq!(store.len(), 1);
        assert!(store.get(&active.session_id).unwrap().is_some());
    }

    #[test]
    fn test_file_store_expires_on_get() {
        let dir = temp_dir();
        let store = FileSessionStore::with_ttl(&dir, 30);
        let mut session = Session::new();
        let id = session.session_id;
        session.last_active_at = session.last_active_at.saturating_sub(60);
        store.put(&session).unwrap();

        let result = store.get(&id);
        assert!(matches!(result, Err(SessionError::SessionExpired(_))));
        assert!(!dir.join(format!("{}.json", id)).exists());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_store_purge_expired() {
        let dir = temp_dir();
        let store = FileSessionStore::with_ttl(&dir, 10);
        let mut expired = Session::new();
        expired.last_active_at = expired.last_active_at.saturating_sub(100);
        let active = Session::new();

        store.put(&expired).unwrap();
        store.put(&active).unwrap();

        let purged = store.purge_expired().unwrap();
        assert_eq!(purged, 1);
        assert!(store.get(&active.session_id).unwrap().is_some());
        assert!(store.get(&expired.session_id).unwrap().is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
