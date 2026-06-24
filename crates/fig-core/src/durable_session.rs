//! Pluggable durable session backend for multi-node deployments.

use uuid::Uuid;

use crate::error::SessionError;
use crate::session::{FileSessionStore, Session, SessionStore};

/// Durable session store selected at runtime.
pub enum DurableSessionStore {
    File(FileSessionStore),
    #[cfg(feature = "session-redis")]
    Redis(super::session_redis::RedisSessionStore),
}

impl DurableSessionStore {
    /// `FIG_SESSION_STORE=memory|file|redis` (default `file`). `REDIS_URL` required for redis.
    pub fn from_env() -> Result<Self, SessionError> {
        let backend = std::env::var("FIG_SESSION_STORE").unwrap_or_else(|_| "file".into());
        match backend.as_str() {
            "file" => {
                let path = std::env::var("FIG_SESSION_PATH")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|_| std::env::temp_dir().join("fig-exchange-sessions"));
                Ok(Self::File(FileSessionStore::new(path)))
            }
            #[cfg(feature = "session-redis")]
            "redis" => {
                let url = std::env::var("REDIS_URL")
                    .map_err(|_| SessionError::SerializationError("REDIS_URL unset".into()))?;
                Ok(Self::Redis(super::session_redis::RedisSessionStore::open(
                    &url,
                )?))
            }
            #[cfg(not(feature = "session-redis"))]
            "redis" => Err(SessionError::SerializationError(
                "fig-core built without session-redis feature".into(),
            )),
            other => Err(SessionError::SerializationError(format!(
                "unknown FIG_SESSION_STORE: {other}"
            ))),
        }
    }
}

impl SessionStore for DurableSessionStore {
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError> {
        match self {
            Self::File(s) => s.get(id),
            #[cfg(feature = "session-redis")]
            Self::Redis(s) => s.get(id),
        }
    }

    fn put(&self, session: &Session) -> Result<(), SessionError> {
        match self {
            Self::File(s) => s.put(session),
            #[cfg(feature = "session-redis")]
            Self::Redis(s) => s.put(session),
        }
    }

    fn delete(&self, id: &Uuid) -> Result<(), SessionError> {
        match self {
            Self::File(s) => s.delete(id),
            #[cfg(feature = "session-redis")]
            Self::Redis(s) => s.delete(id),
        }
    }

    fn update(&self, session: &Session) -> Result<(), SessionError> {
        match self {
            Self::File(s) => s.update(session),
            #[cfg(feature = "session-redis")]
            Self::Redis(s) => s.update(session),
        }
    }
}
