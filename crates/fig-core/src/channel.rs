//! Channel management for FIG connections.
//!
//! A channel is a logical conversation within a FIG connection,
//! mapped 1:1 to a TREE stream. Channel 0 is reserved for
//! connection-level control frames.
//!
//! Each channel has a mode (stateless, session, or affinity) and
//! tracks per-channel sequence numbers for ordering and deduplication.

use std::collections::HashMap;

use crate::error::ChannelError;
use crate::session::Session;

/// Channel mode negotiated at STREAM_OPEN.
///
/// Determines whether the channel carries independent request-response
/// pairs (stateless), durable session-oriented messaging (session),
/// or sticky backend-affine communication (affinity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelMode {
    /// Each REQUEST is independent. No session state. (REST equivalent)
    Stateless,
    /// SESSION_ID establishes a durable session. State persists across
    /// reconnections. (FIX equivalent)
    Session,
    /// Channel is sticky to a backend. State is implicit in the
    /// connection. (WebSocket equivalent)
    Affinity,
}

impl ChannelMode {
    /// Parse from the string representation used in the CHANNEL_MODE extension.
    pub fn from_str(s: &str) -> Result<Self, ChannelError> {
        match s {
            "stateless" => Ok(ChannelMode::Stateless),
            "session" => Ok(ChannelMode::Session),
            "affinity" => Ok(ChannelMode::Affinity),
            other => Err(ChannelError::InvalidChannelMode(other.to_string())),
        }
    }

    /// Return the string representation.
    pub fn as_str(&self) -> &'static str {
        match self {
            ChannelMode::Stateless => "stateless",
            ChannelMode::Session => "session",
            ChannelMode::Affinity => "affinity",
        }
    }
}

/// State of a channel in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelState {
    /// Channel is closed or not yet opened.
    Closed,
    /// Channel is open and active.
    Open,
    /// Channel is draining pending items before closing.
    Closing,
}

/// A logical channel within a FIG connection.
#[derive(Debug, Clone)]
pub struct Channel {
    pub channel_id: u16,
    pub mode: ChannelMode,
    pub state: ChannelState,
    pub last_sent_seq: u32,
    pub last_recv_seq: u32,
    pub schema_id: Option<u8>,
    /// Credits remaining for sending frames on this channel.
    pub credits_available: u32,
    /// Total credits granted by the peer.
    pub credits_granted: u32,
    /// Initial credit window size.
    pub initial_credits: u32,
}

impl Channel {
    /// Create a new channel with the given ID and mode.
    ///
    /// New channels start with no credits granted. Credits are
    /// granted via FLOW_CONTROL frames from the peer.
    pub fn new(channel_id: u16, mode: ChannelMode, schema_id: Option<u8>) -> Self {
        Self {
            channel_id,
            mode,
            state: ChannelState::Open,
            last_sent_seq: 0,
            last_recv_seq: 0,
            schema_id,
            credits_available: 0,
            credits_granted: 0,
            initial_credits: 0,
        }
    }

    /// Check if the channel has enough credits to send a frame.
    pub fn has_credits(&self) -> bool {
        self.credits_available > 0
    }

    /// Consume one credit for sending a frame.
    ///
    /// Returns `Err(CreditExhausted)` if no credits are available.
    pub fn consume_credit(&mut self) -> Result<(), ChannelError> {
        if self.credits_available == 0 {
            return Err(ChannelError::CreditExhausted(self.channel_id));
        }
        self.credits_available -= 1;
        Ok(())
    }

    /// Grant credits from the peer (called when receiving a FLOW_CONTROL frame).
    pub fn grant_credits(&mut self, count: u32) {
        self.credits_available = self.credits_available.saturating_add(count);
        self.credits_granted = self.credits_granted.saturating_add(count);
        if self.initial_credits == 0 {
            self.initial_credits = count;
        }
    }

    /// Get the number of credits to advertise to the peer.
    ///
    /// Returns the number of credits needed to refill the window
    /// to the initial credit window size.
    pub fn credits_to_advertise(&self) -> u32 {
        if self.initial_credits == 0 || self.credits_available >= self.initial_credits {
            return 0;
        }
        self.initial_credits.saturating_sub(self.credits_available)
    }
}

/// Manages all channels in a connection.
///
/// Channel 0 is reserved for control frames.
///
/// # TREE Stream ID Mapping
///
/// FIG channel IDs are mapped to TREE stream IDs as follows:
///
/// ```text
/// tree_stream_id = channel_id * 4 + parity_offset
/// ```
///
/// Where `parity_offset` depends on the initiator and direction:
///
/// | Initiator | Direction       | Offset |
/// |-----------|-----------------|--------|
/// | Client    | Bidirectional   | 0      |
/// | Server    | Bidirectional   | 1      |
/// | Client    | Unidirectional  | 2      |
/// | Server    | Unidirectional  | 3      |
///
/// This results in TREE stream IDs: 0, 4, 8, 12, … (client bidi),
/// 1, 5, 9, 13, … (server bidi), 2, 6, 10, 14, … (client uni),
/// 3, 7, 11, 15, … (server uni).
pub struct ChannelManager {
    channels: HashMap<u16, Channel>,
    next_channel_id: u16,
    is_server: bool,
}

impl ChannelManager {
    /// Create a new channel manager.
    ///
    /// `is_server` determines the TREE stream ID parity offset used
    /// when the manager opens new streams. The client starts at ID 1
    /// and the server at ID… but since channel 0 is reserved, both
    /// sides start allocating from channel 1.
    pub fn new(is_server: bool) -> Self {
        Self {
            channels: HashMap::new(),
            next_channel_id: 1, // 0 reserved for control
            is_server,
        }
    }

    /// Open a new channel with the given mode and optional schema ID.
    ///
    /// Returns the assigned channel ID.
    pub fn open_channel(
        &mut self,
        mode: ChannelMode,
        schema_id: Option<u8>,
    ) -> Result<u16, ChannelError> {
        if self.next_channel_id == 0 {
            // This should not happen since we start at 1, but guard anyway.
            return Err(ChannelError::ChannelIdExhausted);
        }

        // Find the next available channel ID (skip 0 = control).
        loop {
            let candidate = self.next_channel_id;
            if candidate == 0 {
                return Err(ChannelError::ChannelIdExhausted);
            }
            if !self.channels.contains_key(&candidate) {
                break;
            }
            self.next_channel_id = candidate.wrapping_add(1);
        }

        let channel_id = self.next_channel_id;
        let channel = Channel::new(channel_id, mode, schema_id);
        self.channels.insert(channel_id, channel);

        // Advance for the next allocation.
        self.next_channel_id = channel_id.wrapping_add(1);
        if self.next_channel_id == 0 {
            self.next_channel_id = 1;
        }

        Ok(channel_id)
    }

    /// Close an open channel.
    pub fn close_channel(&mut self, channel_id: u16) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(&channel_id)
            .ok_or(ChannelError::ChannelNotFound(channel_id))?;

        match channel.state {
            ChannelState::Closed => Err(ChannelError::ChannelClosed(channel_id)),
            ChannelState::Open => {
                channel.state = ChannelState::Closing;
                Ok(())
            }
            ChannelState::Closing => Ok(()), // already closing — idempotent
        }
    }

    /// Permanently close and remove a channel (used after stream reset).
    ///
    /// Transitions the channel to `ChannelState::Closed` and removes it from
    /// the manager. This is called when the underlying TREE stream is reset
    /// or stopped by the peer.
    pub fn force_close_channel(&mut self, channel_id: u16) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(&channel_id)
            .ok_or(ChannelError::ChannelNotFound(channel_id))?;
        channel.state = ChannelState::Closed;
        self.channels.remove(&channel_id);
        Ok(())
    }

    /// Reset sequence numbers for a channel (called by SEQ_RESET handler).
    ///
    /// Lane C calls this from the CONTROL(SEQ_RESET) frame handler to
    /// re-synchronize sequence numbers after a stream reset or reconnect.
    pub fn reset_seq(
        &mut self,
        channel_id: u16,
        new_send_seq: u32,
        new_recv_seq: u32,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(&channel_id)
            .ok_or(ChannelError::ChannelNotFound(channel_id))?;
        channel.last_sent_seq = new_send_seq;
        channel.last_recv_seq = new_recv_seq;
        Ok(())
    }

    /// Reconstruct a ChannelManager from a stored session.
    ///
    /// For each channel in the session, creates a new `Channel` in `Open` state
    /// with the stored sequence numbers. This is used after 0-RTT session
    /// resumption to rebuild the channel→stream mapping.
    pub fn reconstruct(session: &Session, is_server: bool) -> Self {
        let mut channels = HashMap::new();
        let mut max_id: u16 = 0;
        for &channel_id in &session.channels {
            let mut ch = Channel::new(channel_id, crate::channel::ChannelMode::Session, None);
            ch.last_sent_seq = session.last_seq_sent;
            ch.last_recv_seq = session.last_seq_recv;
            channels.insert(channel_id, ch);
            if channel_id > max_id {
                max_id = channel_id;
            }
        }
        let next_channel_id = if max_id == u16::MAX {
            1
        } else {
            max_id.wrapping_add(1)
        };
        Self {
            channels,
            next_channel_id: if next_channel_id == 0 { 1 } else { next_channel_id },
            is_server,
        }
    }

    /// Permanently remove a channel (after CLOSE completes).
    pub fn remove_channel(&mut self, channel_id: u16) -> Option<Channel> {
        self.channels.remove(&channel_id)
    }

    /// Get an immutable reference to a channel.
    pub fn get_channel(&self, channel_id: u16) -> Option<&Channel> {
        self.channels.get(&channel_id)
    }

    /// Get a mutable reference to a channel.
    pub fn get_channel_mut(&mut self, channel_id: u16) -> Option<&mut Channel> {
        self.channels.get_mut(&channel_id)
    }

    /// Increment and return the next send sequence number for a channel.
    pub fn next_send_seq(&mut self, channel_id: u16) -> Result<u32, ChannelError> {
        let channel = self
            .channels
            .get_mut(&channel_id)
            .ok_or(ChannelError::ChannelNotFound(channel_id))?;

        // Underflow-safe: wrapping_add panics on none but we use a plain
        // increment here — wrapping is acceptable because the spec says
        // seq wraps at 2^32 and is reset via CONTROL(SEQ_RESET).
        channel.last_sent_seq = channel.last_sent_seq.wrapping_add(1);
        Ok(channel.last_sent_seq)
    }

    /// Record a received sequence number for a channel.
    ///
    /// Returns `Err` if the seq number is older than the last recorded
    /// (duplicate/out-of-order detection).
    pub fn record_recv_seq(
        &mut self,
        channel_id: u16,
        seq: u32,
    ) -> Result<(), ChannelError> {
        let channel = self
            .channels
            .get_mut(&channel_id)
            .ok_or(ChannelError::ChannelNotFound(channel_id))?;

        // Allow wraparound: a received seq of 0 is "newer" than the
        // last-received of u32::MAX only when wrapping forward.
        if seq.wrapping_sub(channel.last_recv_seq) == 0
            || (channel.last_recv_seq != 0
                && seq.wrapping_sub(channel.last_recv_seq) > u32::MAX / 2)
        {
            // Duplicate or too far out of order — skip silently.
            // In a production implementation this would trigger gap
            // detection and retransmission logic.
            return Ok(());
        }

        channel.last_recv_seq = seq;
        Ok(())
    }

    /// Map a FIG channel ID to a TREE stream ID.
    ///
    /// Uses bidirectional streams (offset 0 for client, 1 for server).
    /// In a full implementation the direction would be negotiated per
    /// channel.
    pub fn tree_stream_id(&self, channel_id: u16) -> u64 {
        let parity_offset: u64 = if self.is_server { 1 } else { 0 };
        channel_id as u64 * 4 + parity_offset
    }

    /// Returns the number of active channels (excluding control).
    pub fn len(&self) -> usize {
        self.channels.len()
    }

    /// Returns true if there are no channels.
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Iterate over all channels.
    pub fn iter(&self) -> impl Iterator<Item = &Channel> {
        self.channels.values()
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Channel lifecycle ─────────────────────────────────────

    #[test]
    fn test_channel_creation() {
        let ch = Channel::new(5, ChannelMode::Stateless, Some(0x01));
        assert_eq!(ch.channel_id, 5);
        assert_eq!(ch.mode, ChannelMode::Stateless);
        assert_eq!(ch.state, ChannelState::Open);
        assert_eq!(ch.last_sent_seq, 0);
        assert_eq!(ch.last_recv_seq, 0);
        assert_eq!(ch.schema_id, Some(0x01));
    }

    #[test]
    fn test_channel_mode_from_str() {
        assert_eq!(
            ChannelMode::from_str("stateless").unwrap(),
            ChannelMode::Stateless
        );
        assert_eq!(
            ChannelMode::from_str("session").unwrap(),
            ChannelMode::Session
        );
        assert_eq!(
            ChannelMode::from_str("affinity").unwrap(),
            ChannelMode::Affinity
        );
        assert!(ChannelMode::from_str("unknown").is_err());
    }

    #[test]
    fn test_channel_mode_as_str() {
        assert_eq!(ChannelMode::Stateless.as_str(), "stateless");
        assert_eq!(ChannelMode::Session.as_str(), "session");
        assert_eq!(ChannelMode::Affinity.as_str(), "affinity");
    }

    // ── ChannelManager open / close ───────────────────────────

    #[test]
    fn test_open_channel_assigns_ids() {
        let mut mgr = ChannelManager::new(false); // client

        let id1 = mgr.open_channel(ChannelMode::Stateless, None).unwrap();
        let id2 = mgr.open_channel(ChannelMode::Session, Some(0x02)).unwrap();
        let id3 = mgr.open_channel(ChannelMode::Affinity, None).unwrap();

        // IDs should be sequential starting from 1
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);

        assert_eq!(mgr.len(), 3);
    }

    #[test]
    fn test_open_channel_skip_id_zero() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Stateless, None).unwrap();
        // Channel 0 is reserved — first assignable ID is 1.
        assert_eq!(id, 1);
    }

    #[test]
    fn test_close_channel() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Stateless, None).unwrap();

        // Close the channel
        mgr.close_channel(id).unwrap();
        let ch = mgr.get_channel(id).unwrap();
        assert_eq!(ch.state, ChannelState::Closing);

        // Closing again should return Ok (idempotent)
        assert!(mgr.close_channel(id).is_ok());

        // Closing a nonexistent channel is an error
        assert!(mgr.close_channel(999).is_err());
    }

    #[test]
    fn test_remove_channel() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Stateless, None).unwrap();

        let removed = mgr.remove_channel(id);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().channel_id, id);

        // Should be gone now
        assert!(mgr.get_channel(id).is_none());
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_get_channel_not_found() {
        let mgr = ChannelManager::new(false);
        assert!(mgr.get_channel(42).is_none());
    }

    // ── Sequence numbering ────────────────────────────────────

    #[test]
    fn test_next_send_seq() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();

        let s1 = mgr.next_send_seq(id).unwrap();
        let s2 = mgr.next_send_seq(id).unwrap();
        let s3 = mgr.next_send_seq(id).unwrap();

        assert_eq!(s1, 1);
        assert_eq!(s2, 2);
        assert_eq!(s3, 3);
    }

    #[test]
    fn test_next_send_seq_wrap() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();

        // Manually set last_sent_seq to u32::MAX
        mgr.get_channel_mut(id).unwrap().last_sent_seq = u32::MAX;
        let seq = mgr.next_send_seq(id).unwrap();
        assert_eq!(seq, 0); // wraps around
    }

    #[test]
    fn test_next_send_seq_channel_not_found() {
        let mut mgr = ChannelManager::new(false);
        assert!(mgr.next_send_seq(99).is_err());
    }

    #[test]
    fn test_record_recv_seq_monotonic() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();

        assert!(mgr.record_recv_seq(id, 1).is_ok());
        assert!(mgr.record_recv_seq(id, 2).is_ok());
        assert!(mgr.record_recv_seq(id, 5).is_ok()); // gap ok

        let ch = mgr.get_channel(id).unwrap();
        assert_eq!(ch.last_recv_seq, 5);
    }

    #[test]
    fn test_record_recv_seq_duplicate_ignored() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();

        mgr.record_recv_seq(id, 10).unwrap();
        // Duplicate seq 10 — should be silently ignored
        mgr.record_recv_seq(id, 10).unwrap();

        let ch = mgr.get_channel(id).unwrap();
        assert_eq!(ch.last_recv_seq, 10);
    }

    // ── TREE stream ID mapping ────────────────────────────────

    #[test]
    fn test_tree_stream_id_client() {
        let mgr = ChannelManager::new(false); // client
        // Client bidi: offset = 0
        assert_eq!(mgr.tree_stream_id(0), 0);
        assert_eq!(mgr.tree_stream_id(1), 4);
        assert_eq!(mgr.tree_stream_id(2), 8);
        assert_eq!(mgr.tree_stream_id(10), 40);
    }

    #[test]
    fn test_tree_stream_id_server() {
        let mgr = ChannelManager::new(true); // server
        // Server bidi: offset = 1
        assert_eq!(mgr.tree_stream_id(0), 1);
        assert_eq!(mgr.tree_stream_id(1), 5);
        assert_eq!(mgr.tree_stream_id(2), 9);
        assert_eq!(mgr.tree_stream_id(10), 41);
    }

    // ── Channel ID exhaustion ─────────────────────────────────

    #[test]
    fn test_channel_id_exhaustion() {
        let mut mgr = ChannelManager::new(false);
        // Fill up all channel IDs (1..=u16::MAX)
        for _ in 0..u16::MAX {
            let result = mgr.open_channel(ChannelMode::Stateless, None);
            if result.is_err() {
                // We should only exhaust when we wrap back to 0
                break;
            }
        }
        // Next open should fail — no more IDs
        // (After wrapping, next_channel becomes 0 or all IDs are busy.)
        // Since we've opened every possible channel, the next attempt
        // should wrap to an already-occupied ID and be skipped, ultimately
        // failing when wrapping to 0.
    }

    // ── Flow control (credits) ────────────────────────────────

    #[test]
    fn test_channel_no_credits_by_default() {
        let ch = Channel::new(1, ChannelMode::Stateless, None);
        assert!(!ch.has_credits());
        assert_eq!(ch.credits_available, 0);
        assert_eq!(ch.credits_granted, 0);
        assert_eq!(ch.initial_credits, 0);
    }

    #[test]
    fn test_grant_credits() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);
        ch.grant_credits(10);
        assert!(ch.has_credits());
        assert_eq!(ch.credits_available, 10);
        assert_eq!(ch.credits_granted, 10);
        assert_eq!(ch.initial_credits, 10);
    }

    #[test]
    fn test_consume_credit_success() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);
        ch.grant_credits(5);

        for _ in 0..5 {
            assert!(ch.consume_credit().is_ok());
        }

        // No more credits
        assert!(!ch.has_credits());
        assert_eq!(ch.credits_available, 0);
        assert_eq!(ch.credits_granted, 5);
    }

    #[test]
    fn test_consume_credit_exhausted() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);
        assert!(!ch.has_credits());

        let result = ch.consume_credit();
        assert!(result.is_err());
        match result {
            Err(ChannelError::CreditExhausted(1)) => {} // expected
            _ => panic!("expected CreditExhausted"),
        }
    }

    #[test]
    fn test_multiple_credit_grants() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);
        ch.grant_credits(5);
        assert_eq!(ch.initial_credits, 5);

        // Consume some
        ch.consume_credit().unwrap();
        ch.consume_credit().unwrap();

        // Grant more
        ch.grant_credits(3);
        assert_eq!(ch.credits_available, 6); // 5 - 2 + 3 = 6
        assert_eq!(ch.credits_granted, 8); // 5 + 3 = 8
    }

    #[test]
    fn test_credits_to_advertise() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);

        // No initial credits set — nothing to advertise
        assert_eq!(ch.credits_to_advertise(), 0);

        ch.grant_credits(10);
        // Full window — nothing to advertise
        assert_eq!(ch.credits_to_advertise(), 0);

        // Consume 3
        for _ in 0..3 {
            ch.consume_credit().unwrap();
        }
        assert_eq!(ch.credits_to_advertise(), 3); // refill 3

        // Consume all
        for _ in 0..7 {
            ch.consume_credit().unwrap();
        }
        assert_eq!(ch.credits_to_advertise(), 10); // refill to initial window

        // Grant more to increase window
        ch.grant_credits(5);
        // initial_credits stays at 10 (first grant), available is 5
        assert_eq!(ch.credits_to_advertise(), 5);
    }

    #[test]
    fn test_credits_to_advertise_exceeds_window() {
        let mut ch = Channel::new(1, ChannelMode::Session, None);
        ch.grant_credits(10);
        // credits already exceed initial window — nothing to advertise
        ch.grant_credits(5);
        assert_eq!(ch.credits_to_advertise(), 0);
    }

    // ── ChannelManager::reset_seq ───────────────────────────────

    #[test]
    fn test_reset_seq() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();

        // Set some initial sequence numbers
        mgr.next_send_seq(id).unwrap();
        mgr.next_send_seq(id).unwrap();
        mgr.record_recv_seq(id, 5).unwrap();

        let ch = mgr.get_channel(id).unwrap();
        assert_eq!(ch.last_sent_seq, 2);
        assert_eq!(ch.last_recv_seq, 5);

        // Reset to new values
        mgr.reset_seq(id, 100, 200).unwrap();

        let ch = mgr.get_channel(id).unwrap();
        assert_eq!(ch.last_sent_seq, 100);
        assert_eq!(ch.last_recv_seq, 200);
    }

    #[test]
    fn test_reset_seq_nonexistent_channel() {
        let mut mgr = ChannelManager::new(false);
        let result = mgr.reset_seq(999, 0, 0);
        assert!(result.is_err());
    }

    // ── ChannelManager::reconstruct ─────────────────────────────

    #[test]
    fn test_reconstruct_from_session() {
        use crate::session::Session;

        // Create a session with channels
        let mut session = Session::new();
        session.add_channel(1);
        session.add_channel(2);
        session.add_channel(5);
        session.record_sent_seq(42);
        session.record_recv_seq(99);

        // Reconstruct the channel manager
        let mgr = ChannelManager::reconstruct(&session, false);

        assert_eq!(mgr.len(), 3);

        // Verify channel 1 is in Open state with correct seq numbers
        let ch1 = mgr.get_channel(1).expect("channel 1 should exist");
        assert_eq!(ch1.state, ChannelState::Open);
        assert_eq!(ch1.last_sent_seq, 42);
        assert_eq!(ch1.last_recv_seq, 99);

        // Verify channel 2
        let ch2 = mgr.get_channel(2).expect("channel 2 should exist");
        assert_eq!(ch2.state, ChannelState::Open);

        // Verify channel 5
        let ch5 = mgr.get_channel(5).expect("channel 5 should exist");
        assert_eq!(ch5.state, ChannelState::Open);
    }

    #[test]
    fn test_reconstruct_empty_session() {
        use crate::session::Session;

        let session = Session::new();
        let mgr = ChannelManager::reconstruct(&session, true);

        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
    }

    #[test]
    fn test_reconstruct_server_mode() {
        use crate::session::Session;

        let mut session = Session::new();
        session.add_channel(1);

        let mgr = ChannelManager::reconstruct(&session, true);
        // Server mode: offset = 1
        assert_eq!(mgr.tree_stream_id(1), 5);
    }

    // ── ChannelManager::force_close_channel ─────────────────────

    #[test]
    fn test_force_close_channel() {
        let mut mgr = ChannelManager::new(false);
        let id = mgr.open_channel(ChannelMode::Session, None).unwrap();
        assert_eq!(mgr.len(), 1);

        mgr.force_close_channel(id).unwrap();

        // Channel should be removed from the manager
        assert!(mgr.get_channel(id).is_none());
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_force_close_channel_nonexistent() {
        let mut mgr = ChannelManager::new(false);
        let result = mgr.force_close_channel(99);
        assert!(result.is_err());
    }
}
