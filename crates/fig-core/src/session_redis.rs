//! Redis-backed session store (`session-redis` feature).

use std::sync::Mutex;

use redis::Commands;
use uuid::Uuid;

use crate::error::SessionError;
use crate::session::{Session, SessionStore, DEFAULT_SESSION_TTL_SECS};

fn key(id: &Uuid) -> String {
    format!("fig:session:{id}")
}

/// Redis session store for multi-node FIG backends.
pub struct RedisSessionStore {
    client: Mutex<redis::Connection>,
    ttl_secs: u64,
}

impl RedisSessionStore {
    pub fn open(redis_url: &str) -> Result<Self, SessionError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        let conn = client
            .get_connection()
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        Ok(Self {
            client: Mutex::new(conn),
            ttl_secs: DEFAULT_SESSION_TTL_SECS,
        })
    }

    pub fn with_ttl(mut self, ttl_secs: u64) -> Self {
        self.ttl_secs = ttl_secs.max(1);
        self
    }
}

impl SessionStore for RedisSessionStore {
    fn get(&self, id: &Uuid) -> Result<Option<Session>, SessionError> {
        let mut conn = self
            .client
            .lock()
            .map_err(|_| SessionError::SerializationError("lock poisoned".into()))?;
        let data: Option<Vec<u8>> = conn
            .get(key(id))
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        match data {
            None => Ok(None),
            Some(bytes) => {
                let session: Session = serde_json::from_slice(&bytes)
                    .map_err(|e| SessionError::SerializationError(e.to_string()))?;
                Ok(Some(session))
            }
        }
    }

    fn put(&self, session: &Session) -> Result<(), SessionError> {
        let bytes = serde_json::to_vec(session)
            .map_err(|e| SessionError::SerializationError(e.to_string()))?;
        let mut conn = self
            .client
            .lock()
            .map_err(|_| SessionError::SerializationError("lock poisoned".into()))?;
        redis::cmd("SET")
            .arg(key(&session.session_id))
            .arg(bytes)
            .arg("EX")
            .arg(self.ttl_secs)
            .query::<()>(&mut *conn)
            .map_err(|e| SessionError::SerializationError(e.to_string()))
    }

    fn delete(&self, id: &Uuid) -> Result<(), SessionError> {
        let mut conn = self
            .client
            .lock()
            .map_err(|_| SessionError::SerializationError("lock poisoned".into()))?;
        conn.del(key(id))
            .map_err(|e| SessionError::SerializationError(e.to_string()))
    }

    fn update(&self, session: &Session) -> Result<(), SessionError> {
        self.put(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_session() {
        let Ok(url) = std::env::var("REDIS_URL") else {
            eprintln!("skipping Redis session round-trip (REDIS_URL not set)");
            return;
        };
        let store = RedisSessionStore::open(&url).unwrap();
        let session = Session::new();
        store.put(&session).unwrap();
        let got = store.get(&session.session_id).unwrap().expect("session");
        assert_eq!(got.session_id, session.session_id);
        store.delete(&session.session_id).unwrap();
        assert!(store.get(&session.session_id).unwrap().is_none());
    }
}
