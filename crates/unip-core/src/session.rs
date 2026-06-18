//! Session model and storage for UNIP.
//!
//! A UNIP session is a durable, migratable logical entity identified by
//! a SESSION_ID (UUID). Sessions survive disconnects and can be resumed
//! across QUIC 0-RTT reconnections.
//!
//! The [`SessionStore`] trait provides a pluggable storage backend.
//! [`MemorySessionStore`] is an in-memory implementation suitable for
//! development and testing. Production deployments should use Redis,
//! etcd, or a durable database.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use crate::error::SessionError;

/// A durable, migratable session identified by SESSION_ID.
///
/// Sessions track active channels, sequence numbers, and
/// authentication state across connections.
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique session identifier (UUID v4).
    pub session_id: Uuid,
    /// Opaque authentication token (JWT, bearer, etc.).
    pub auth_token: Option<Vec<u8>>,
    /// IDs of channels currently associated with this session.
    pub channels: Vec<u16>,
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
pub struct MemorySessionStore {
    sessions: Mutex<HashMap<Uuid, Session>>,
}

impl MemorySessionStore {
    /// Create a new empty in-memory store.
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Returns the number of sessions currently stored.
    pub fn len(&self) -> usize {
        self.sessions.lock().unwrap_or_else(|e| e.into_inner()).len()
    }

    /// Returns true if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.sessions.lock().unwrap_or_else(|e| e.into_inner()).is_empty()
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore for MemorySessionStore {
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError> {
        let guard = self
            .sessions
            .lock()
            .map_err(|_| SessionError::SessionNotFound(*id))?;
        Ok(guard.get(id).cloned())
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
        if guard.contains_key(&session.session_id) {
            guard.insert(session.session_id, session.clone());
            Ok(())
        } else {
            Err(SessionError::SessionNotFound(session.session_id))
        }
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
}
