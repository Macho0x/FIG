//! Connection and session migration support.
//!
//! Spec §1.1: sessions survive IP changes via TREE connection migration.
//! This module captures channel state for handoff and reapplies it on the
//! new connection.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::channel::ChannelManager;
use crate::error::{FigError, SessionError};
use crate::session::Session;

/// Token capturing migratable connection state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationToken {
    pub session_id: Uuid,
    pub channels: Vec<MigratedChannel>,
}

/// Per-channel state preserved across migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratedChannel {
    pub channel_id: u16,
    pub last_sent_seq: u32,
    pub last_recv_seq: u32,
}

/// Prepare a migration token from an active session and channel manager.
pub fn prepare_migration(session: &Session, channels: &ChannelManager) -> MigrationToken {
    let mut migrated = Vec::new();
    for &channel_id in &session.channels {
        if let Some(ch) = channels.get_channel(channel_id) {
            migrated.push(MigratedChannel {
                channel_id,
                last_sent_seq: ch.last_sent_seq,
                last_recv_seq: ch.last_recv_seq,
            });
        }
    }
    MigrationToken {
        session_id: session.session_id,
        channels: migrated,
    }
}

/// Apply a migration token to rebuild session + channel sequence state.
pub fn apply_migration(token: &MigrationToken, session: &mut Session) -> Result<(), SessionError> {
    if session.session_id != token.session_id {
        return Err(SessionError::InvalidSessionId);
    }
    session.channels = token.channels.iter().map(|c| c.channel_id).collect();
    for ch in &token.channels {
        session.record_sent_seq(ch.last_sent_seq);
        session.record_recv_seq(ch.last_recv_seq);
    }
    session.touch();
    Ok(())
}

/// Reconstruct a channel manager from a migration token (client side).
pub fn reconstruct_channels(token: &MigrationToken, is_server: bool) -> ChannelManager {
    let mut session = Session::new();
    session.session_id = token.session_id;
    for ch in &token.channels {
        session.channels.push(ch.channel_id);
        session.record_sent_seq(ch.last_sent_seq);
        session.record_recv_seq(ch.last_recv_seq);
    }
    ChannelManager::reconstruct(&session, is_server)
}

/// Validate that a migration token matches an expected session.
pub fn validate_migration(token: &MigrationToken, session_id: &Uuid) -> Result<(), FigError> {
    if &token.session_id != session_id {
        return Err(FigError::HandshakeFailed(
            "migration session_id mismatch".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ChannelManager, ChannelMode};

    #[test]
    fn test_prepare_and_apply_migration() {
        let mut session = Session::new();
        let mut mgr = ChannelManager::new(false);
        let ch = mgr.open_channel(ChannelMode::Session, None).unwrap();
        session.add_channel(ch);
        mgr.get_channel_mut(ch).unwrap().last_sent_seq = 10;
        mgr.get_channel_mut(ch).unwrap().last_recv_seq = 20;

        let token = prepare_migration(&session, &mgr);
        assert_eq!(token.channels.len(), 1);
        assert_eq!(token.channels[0].last_sent_seq, 10);

        let mut restored = Session::new();
        restored.session_id = session.session_id;
        apply_migration(&token, &mut restored).unwrap();
        assert_eq!(restored.last_seq_sent, 10);
        assert_eq!(restored.last_seq_recv, 20);
    }

    #[test]
    fn test_reconstruct_channels_from_token() {
        let mut session = Session::new();
        session.add_channel(3);
        session.record_sent_seq(5);
        session.record_recv_seq(7);
        let token = prepare_migration(&session, &ChannelManager::reconstruct(&session, false));
        let mgr = reconstruct_channels(&token, false);
        assert!(mgr.get_channel(3).is_some());
    }

    #[test]
    fn test_validate_migration_rejects_mismatch() {
        let token = MigrationToken {
            session_id: Uuid::new_v4(),
            channels: vec![],
        };
        assert!(validate_migration(&token, &Uuid::new_v4()).is_err());
    }
}
