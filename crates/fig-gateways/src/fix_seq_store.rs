//! Persistent FIX session sequence state for multi-node gateway deployments.
//!
//! Stores `(expected_recv_seq, next_send_seq)` keyed by `SenderCompID:TargetCompID`
//! so gateway replicas can resume FIX sessions after reconnect or failover.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::fix_session::FixSessionState;

/// Persisted FIX session sequence state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixSeqState {
    pub expected_recv_seq: u32,
    pub next_send_seq: u32,
    pub logged_in: bool,
}

impl Default for FixSeqState {
    fn default() -> Self {
        Self {
            expected_recv_seq: 1,
            next_send_seq: 1,
            logged_in: false,
        }
    }
}

impl FixSeqState {
    pub fn from_session(
        expected_recv_seq: u32,
        next_send_seq: u32,
        state: FixSessionState,
    ) -> Self {
        Self {
            expected_recv_seq,
            next_send_seq,
            logged_in: state == FixSessionState::LoggedIn,
        }
    }
}

/// Errors from FIX sequence store operations.
#[derive(Error, Debug)]
pub enum FixSeqStoreError {
    #[error("sequence state not found for session {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(String),
}

/// Pluggable backend for FIX session sequence persistence.
pub trait FixSeqStore: Send + Sync {
    fn get(&self, session_key: &str) -> Result<Option<FixSeqState>, FixSeqStoreError>;
    fn put(&self, session_key: &str, state: &FixSeqState) -> Result<(), FixSeqStoreError>;
    fn delete(&self, session_key: &str) -> Result<(), FixSeqStoreError>;
}

/// In-memory sequence store for tests and single-process gateways.
#[derive(Debug, Default)]
pub struct MemoryFixSeqStore {
    states: Mutex<HashMap<String, FixSeqState>>,
}

impl MemoryFixSeqStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FixSeqStore for MemoryFixSeqStore {
    fn get(&self, session_key: &str) -> Result<Option<FixSeqState>, FixSeqStoreError> {
        Ok(self
            .states
            .lock()
            .map_err(|_| FixSeqStoreError::Serialization("lock poisoned".into()))?
            .get(session_key)
            .copied())
    }

    fn put(&self, session_key: &str, state: &FixSeqState) -> Result<(), FixSeqStoreError> {
        self.states
            .lock()
            .map_err(|_| FixSeqStoreError::Serialization("lock poisoned".into()))?
            .insert(session_key.to_string(), *state);
        Ok(())
    }

    fn delete(&self, session_key: &str) -> Result<(), FixSeqStoreError> {
        self.states
            .lock()
            .map_err(|_| FixSeqStoreError::Serialization("lock poisoned".into()))?
            .remove(session_key);
        Ok(())
    }
}

/// File-backed sequence store for single-node deployments.
pub struct FileFixSeqStore {
    dir: PathBuf,
}

impl FileFixSeqStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            tracing::warn!("Failed to create FIX seq directory {:?}: {}", dir, e);
        }
        Self { dir }
    }

    fn file_path(&self, session_key: &str) -> PathBuf {
        let safe = session_key.replace([':', '/', '\\'], "_");
        self.dir.join(format!("{safe}.json"))
    }
}

impl FixSeqStore for FileFixSeqStore {
    fn get(&self, session_key: &str) -> Result<Option<FixSeqState>, FixSeqStoreError> {
        let path = self.file_path(session_key);
        if !path.exists() {
            return Ok(None);
        }
        let data = std::fs::read(&path)?;
        let state: FixSeqState = serde_json::from_slice(&data)
            .map_err(|e| FixSeqStoreError::Serialization(e.to_string()))?;
        Ok(Some(state))
    }

    fn put(&self, session_key: &str, state: &FixSeqState) -> Result<(), FixSeqStoreError> {
        let path = self.file_path(session_key);
        let data = serde_json::to_vec_pretty(state)
            .map_err(|e| FixSeqStoreError::Serialization(e.to_string()))?;
        std::fs::write(path, data)?;
        Ok(())
    }

    fn delete(&self, session_key: &str) -> Result<(), FixSeqStoreError> {
        let path = self.file_path(session_key);
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        Ok(())
    }
}

/// Backend selector for the gateway CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixSeqStoreBackend {
    Memory,
    File,
}

impl FixSeqStoreBackend {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "memory" => Some(Self::Memory),
            "file" => Some(Self::File),
            _ => None,
        }
    }
}

/// Build a shared sequence store from CLI/backend configuration.
pub fn build_seq_store(backend: FixSeqStoreBackend, path: &Path) -> Arc<dyn FixSeqStore> {
    match backend {
        FixSeqStoreBackend::Memory => Arc::new(MemoryFixSeqStore::new()),
        FixSeqStoreBackend::File => Arc::new(FileFixSeqStore::new(path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_store_tests<S: FixSeqStore>(store: S) {
        let key = "FIG-GW:CLIENT";
        assert!(store.get(key).unwrap().is_none());

        let state = FixSeqState {
            expected_recv_seq: 42,
            next_send_seq: 17,
            logged_in: true,
        };
        store.put(key, &state).unwrap();
        assert_eq!(store.get(key).unwrap(), Some(state));

        store.delete(key).unwrap();
        assert!(store.get(key).unwrap().is_none());
    }

    #[test]
    fn test_memory_fix_seq_store() {
        run_store_tests(MemoryFixSeqStore::new());
    }

    #[test]
    fn test_file_fix_seq_store() {
        let dir = std::env::temp_dir().join(format!("fig-fix-seq-{}", uuid::Uuid::new_v4()));
        run_store_tests(FileFixSeqStore::new(&dir));
    }

    #[test]
    fn test_multi_node_resume() {
        let dir = std::env::temp_dir().join(format!("fig-fix-seq-shared-{}", uuid::Uuid::new_v4()));
        let node_a = FileFixSeqStore::new(&dir);
        let node_b = FileFixSeqStore::new(&dir);
        let key = "GW:CLIENT";

        node_a
            .put(
                key,
                &FixSeqState {
                    expected_recv_seq: 100,
                    next_send_seq: 55,
                    logged_in: true,
                },
            )
            .unwrap();

        let resumed = node_b.get(key).unwrap().unwrap();
        assert_eq!(resumed.expected_recv_seq, 100);
        assert_eq!(resumed.next_send_seq, 55);
        assert!(resumed.logged_in);
    }
}
