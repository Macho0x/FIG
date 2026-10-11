//! FIX session state machine.
//!
//! Manages the lifecycle of a FIX 4.4 session: logon handshake, message
//! sequence numbering, heartbeat keep-alive, resend requests, and logout.
//!
//! # State Machine
//!
//! ```text
//! LoggedOut ──(send Logon)──► LogonSent ──(recv Logon)──► LoggedIn
//!      ▲                          │                          │
//!      │                          │ (recv Logout)            │ (send Logout)
//!      │                          ▼                          ▼
//!      └────────────────── LogoutSent ◄──────────────────────┘
//!               (recv Logout, timeout)
//! ```
//!
//! Any connection error or timeout also transitions any state to `LoggedOut`.

use std::sync::Arc;
use std::time::{Duration, Instant};

use thiserror::Error;

use crate::fix::{FixMessage, FixOutboundContext};
use crate::fix_seq_store::{FixSeqState, FixSeqStore};

// ─── State ───────────────────────────────────────────────────────

/// FIX session connection states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixSessionState {
    /// Initial state — no Logon received or sent.
    LoggedOut,
    /// Client has sent a Logon, waiting for server response.
    LogonSent,
    /// Session is active — messages can be exchanged.
    LoggedIn,
    /// Logout sent, waiting for confirmation.
    LogoutSent,
}

// ─── Actions ─────────────────────────────────────────────────────

/// Actions the caller must perform in response to session events.
#[derive(Debug, Clone, PartialEq)]
pub enum FixAction {
    /// Send a FIX message to the counterparty.
    SendMessage(FixMessage),
    /// Close the underlying transport connection.
    CloseConnection,
    /// The counterparty missed a message range; send a ResendRequest (35=2).
    SendResendRequest {
        /// First sequence number to resend.
        begin_seq_no: u32,
        /// Last sequence number to resend (0 = all messages to current).
        end_seq_no: u32,
    },
    /// No action required.
    NoAction,
}

// ─── Error ───────────────────────────────────────────────────────

/// Errors that can occur during FIX session processing.
#[derive(Error, Debug)]
pub enum FixSessionError {
    #[error("unexpected message type '{msg_type}' in state {state:?}")]
    UnexpectedMessage {
        state: FixSessionState,
        msg_type: String,
    },
    #[error("MsgSeqNum too low: expected {expected}, received {received}")]
    SeqNumTooLow { expected: u32, received: u32 },
    #[error("MsgSeqNum too high (gap): expected {expected}, received {received}")]
    SeqNumTooHigh { expected: u32, received: u32 },
    #[error("session not logged in")]
    NotLoggedIn,
}

// ─── Session ─────────────────────────────────────────────────────

/// A FIX 4.4 session state machine.
///
/// Tracks connection state, message sequence numbers, and heartbeat
/// timers for a single FIX session.
pub struct FixSession {
    state: FixSessionState,
    expected_recv_seq: u32,
    next_send_seq: u32,
    heartbeat_interval: Duration,
    last_received: Instant,
    sender_comp_id: String,
    target_comp_id: String,
    seq_store: Option<Arc<dyn FixSeqStore>>,
    outbound_log: Vec<FixMessage>,
}

impl FixSession {
    /// Create a new FIX session with the given sender and target CompIDs.
    ///
    /// The session starts in [`FixSessionState::LoggedOut`] with sequence
    /// numbers at 1 and a 30-second heartbeat interval.
    pub fn new(sender_comp_id: String, target_comp_id: String) -> Self {
        Self {
            state: FixSessionState::LoggedOut,
            expected_recv_seq: 1,
            next_send_seq: 1,
            heartbeat_interval: Duration::from_secs(30),
            last_received: Instant::now(),
            sender_comp_id,
            target_comp_id,
            seq_store: None,
            outbound_log: Vec::new(),
        }
    }

    /// Attach a shared sequence store for multi-node session continuity.
    pub fn with_seq_store(mut self, store: Arc<dyn FixSeqStore>) -> Self {
        self.seq_store = Some(store);
        self
    }

    /// Session key used for sequence persistence (`SenderCompID:TargetCompID`).
    pub fn session_key(&self) -> String {
        format!("{}:{}", self.sender_comp_id, self.target_comp_id)
    }

    /// Load persisted sequence numbers if available.
    pub fn restore_from_store(&mut self) {
        let Some(store) = &self.seq_store else {
            return;
        };
        if let Ok(Some(state)) = store.get(&self.session_key()) {
            self.expected_recv_seq = state.expected_recv_seq;
            self.next_send_seq = state.next_send_seq;
            if state.logged_in {
                self.state = FixSessionState::LoggedIn;
            }
        }
    }

    fn persist_to_store(&self) {
        let Some(store) = &self.seq_store else {
            return;
        };
        let state =
            FixSeqState::from_session(self.expected_recv_seq, self.next_send_seq, self.state);
        let _ = store.put(&self.session_key(), &state);
    }

    // ── State Queries ─────────────────────────────────────────

    /// Returns `true` if the session is currently logged in.
    pub fn is_logged_in(&self) -> bool {
        self.state == FixSessionState::LoggedIn
    }

    /// Returns the current session state.
    pub fn state(&self) -> FixSessionState {
        self.state
    }

    pub fn sender_comp_id(&self) -> &str {
        &self.sender_comp_id
    }

    pub fn target_comp_id(&self) -> &str {
        &self.target_comp_id
    }

    /// Current outbound sequence number without incrementing.
    pub fn next_send_seq(&self) -> u32 {
        self.next_send_seq
    }

    /// Allocate the next outbound sequence number.
    pub fn allocate_send_seq(&mut self) -> u32 {
        let seq = self.next_send_seq;
        self.next_send_seq += 1;
        self.persist_to_store();
        seq
    }

    pub fn outbound_context(&mut self) -> FixOutboundContext {
        FixOutboundContext {
            sender_comp_id: self.sender_comp_id.clone(),
            target_comp_id: self.target_comp_id.clone(),
            msg_seq_num: self.allocate_send_seq(),
        }
    }

    pub fn record_outbound(&mut self, msg: FixMessage) {
        self.outbound_log.push(msg);
    }

    pub fn set_target_comp_id(&mut self, id: impl Into<String>) {
        self.target_comp_id = id.into();
    }

    /// Expected inbound sequence number.
    pub fn expected_recv_seq(&self) -> u32 {
        self.expected_recv_seq
    }

    /// Set the heartbeat interval.
    pub fn set_heartbeat_interval(&mut self, interval: Duration) {
        self.heartbeat_interval = interval;
    }

    /// Returns the heartbeat interval.
    pub fn heartbeat_interval(&self) -> Duration {
        self.heartbeat_interval
    }

    // ── Outgoing Messages ─────────────────────────────────────

    /// Build and send a Logon (35=A) message.
    ///
    /// Transitions to [`FixSessionState::LogonSent`].
    pub fn send_logon(&mut self) -> FixMessage {
        let seq = self.next_send_seq;
        self.next_send_seq += 1;

        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "A".to_string()),
            (49, self.sender_comp_id.clone()),
            (56, self.target_comp_id.clone()),
            (34, seq.to_string()),
            (98, "0".to_string()), // EncryptMethod = None
            (108, self.heartbeat_interval.as_secs().to_string()), // HeartBtInt
        ];

        if self.state == FixSessionState::LoggedOut {
            self.state = FixSessionState::LogonSent;
        }

        let msg = FixMessage::new(tags);
        self.record_outbound(msg.clone());
        msg
    }

    /// Build and send a Logout (35=5) message.
    ///
    /// Transitions to [`FixSessionState::LogoutSent`].
    pub fn send_logout(&mut self) -> FixMessage {
        let seq = self.next_send_seq;
        self.next_send_seq += 1;

        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "5".to_string()),
            (49, self.sender_comp_id.clone()),
            (56, self.target_comp_id.clone()),
            (34, seq.to_string()),
        ];

        if self.state == FixSessionState::LoggedIn || self.state == FixSessionState::LogonSent {
            self.state = FixSessionState::LogoutSent;
        }

        let msg = FixMessage::new(tags);
        self.record_outbound(msg.clone());
        msg
    }

    /// Build and return a Heartbeat (35=0) message.
    ///
    /// Increments the send sequence number.
    pub fn send_heartbeat(&mut self) -> FixMessage {
        let msg = FixMessage::new(self.build_heartbeat_tags());
        self.record_outbound(msg.clone());
        msg
    }

    /// Check if the heartbeat timer has expired. Returns a Heartbeat
    /// message if so, or `None` otherwise.
    ///
    /// Only sends heartbeats when the session is [`FixSessionState::LoggedIn`].
    pub fn check_heartbeat(&mut self) -> Option<FixMessage> {
        if self.state == FixSessionState::LoggedIn
            && self.last_received.elapsed() >= self.heartbeat_interval
        {
            Some(self.send_heartbeat())
        } else {
            None
        }
    }

    // ── Incoming Message Processing ───────────────────────────

    /// Process an incoming FIX message and return the actions to take.
    ///
    /// This method:
    /// 1. Validates the `MsgSeqNum` (tag 34) against `expected_recv_seq`
    /// 2. Detects gaps and returns `SendResendRequest` if needed
    /// 3. Handles duplicate messages with `PossDupFlag` (tag 43)
    /// 4. Updates the session state based on message type
    /// 5. Generates response actions (TestRequest → Heartbeat, Logout → Logout, etc.)
    ///
    /// # Errors
    ///
    /// Returns `FixSessionError` for protocol violations (unexpected
    /// message types in certain states, sequence number underflows
    /// without PossDupFlag).
    pub fn process_incoming(
        &mut self,
        msg: &FixMessage,
    ) -> Result<Vec<FixAction>, FixSessionError> {
        let msg_type = msg.msg_type().unwrap_or("?");
        let seq_num: u32 = msg.get_tag(34).and_then(|s| s.parse().ok()).unwrap_or(0);

        let poss_dup = msg.get_tag(43) == Some("Y");

        // ── Sequence Number Validation ──
        if seq_num < self.expected_recv_seq {
            if poss_dup {
                // Possible duplicate with PossDupFlag — silently ignore
                return Ok(vec![FixAction::NoAction]);
            }
            return Err(FixSessionError::SeqNumTooLow {
                expected: self.expected_recv_seq,
                received: seq_num,
            });
        }

        if seq_num > self.expected_recv_seq {
            // Gap detected — request resend, but still process heartbeat/TestRequest
            // to keep the session alive during gap recovery.
            let mut actions = vec![FixAction::SendResendRequest {
                begin_seq_no: self.expected_recv_seq,
                end_seq_no: 0, // 0 = all messages from begin to current
            }];

            // Heartbeat and TestRequest must be processed even during gap recovery
            // to prevent timeout disconnects.
            self.last_received = Instant::now();
            if msg_type == "0" {
                // Heartbeat — no additional action needed
            } else if msg_type == "1" {
                // TestRequest — must echo back a Heartbeat with the same TestReqID
                let test_req_id = msg.get_tag(112).unwrap_or_default();
                let heartbeat = FixMessage::new(vec![
                    (35, "0".to_string()),
                    (112, test_req_id.to_string()),
                    (49, self.sender_comp_id.clone()),
                    (56, self.target_comp_id.clone()),
                    (34, self.next_send_seq.to_string()),
                ]);
                self.next_send_seq += 1;
                actions.push(FixAction::SendMessage(heartbeat));
            }
            // Note: we do NOT advance expected_recv_seq for the gapped message.
            // The gap will be filled by the ResendRequest response.
            return Ok(actions);
        }

        // SeqNum is correct — advance expected and update last_received
        self.expected_recv_seq = seq_num + 1;
        self.last_received = Instant::now();

        let mut actions = Vec::new();

        match msg_type {
            "A" => {
                // ── Logon ──
                let reset_seq = msg.get_tag(141) != Some("N");
                if !reset_seq {
                    self.restore_from_store();
                }
                match self.state {
                    FixSessionState::LoggedOut | FixSessionState::LogonSent => {
                        self.state = FixSessionState::LoggedIn;
                    }
                    FixSessionState::LoggedIn => {
                        // Re-logon — reset state, typically for seq num reset
                        // Already logged in; stay logged in
                    }
                    FixSessionState::LogoutSent => {
                        return Err(FixSessionError::UnexpectedMessage {
                            state: self.state,
                            msg_type: msg_type.to_string(),
                        });
                    }
                }
            }
            "5" => {
                // ── Logout ──
                match self.state {
                    FixSessionState::LoggedIn | FixSessionState::LogonSent => {
                        // Peer initiated logout — confirm and close
                        self.state = FixSessionState::LoggedOut;
                        actions.push(FixAction::SendMessage(self.build_logout_message()));
                    }
                    FixSessionState::LogoutSent => {
                        // Our logout was confirmed
                        self.state = FixSessionState::LoggedOut;
                        actions.push(FixAction::CloseConnection);
                    }
                    FixSessionState::LoggedOut => {
                        return Err(FixSessionError::UnexpectedMessage {
                            state: self.state,
                            msg_type: msg_type.to_string(),
                        });
                    }
                }
            }
            "0" => {
                // ── Heartbeat ──
                // No action needed — last_received already updated
            }
            "1" => {
                // ── TestRequest ──
                // Respond with Heartbeat containing the same TestReqID (tag 112)
                let test_req_id = msg.get_tag(112).unwrap_or("");
                let mut tags = self.build_heartbeat_tags();
                if !test_req_id.is_empty() {
                    tags.push((112, test_req_id.to_string()));
                }
                actions.push(FixAction::SendMessage(FixMessage::new(tags)));
            }
            "2" => {
                let begin = msg
                    .get_tag(7)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(1);
                let end = msg
                    .get_tag(16)
                    .and_then(|s| s.parse::<u32>().ok())
                    .unwrap_or(0);
                actions.extend(self.replay_outbound(begin, end));
            }
            "3" => {
                // ── Reject ──
                // Session-level reject — log and continue
            }
            _ => {
                // ── Application-level message ──
                if !self.is_logged_in() {
                    return Err(FixSessionError::NotLoggedIn);
                }
                // Application messages are passed through — no state change
            }
        }

        self.persist_to_store();
        Ok(actions)
    }

    // ── Force State Changes ───────────────────────────────────

    /// Force the session to [`FixSessionState::LoggedOut`], e.g. on
    /// connection error or timeout.
    pub fn force_logout(&mut self) {
        self.state = FixSessionState::LoggedOut;
    }

    fn is_admin_msg_type(msg_type: &str) -> bool {
        matches!(msg_type, "0" | "1" | "2" | "3" | "4" | "5" | "A")
    }

    fn seq_of(msg: &FixMessage) -> Option<u32> {
        msg.get_tag(34).and_then(|s| s.parse().ok())
    }

    fn gap_fill_message(&self, start: u32, new_seq: u32) -> FixMessage {
        FixMessage::new(vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "4".to_string()),
            (49, self.sender_comp_id.clone()),
            (56, self.target_comp_id.clone()),
            (34, start.to_string()),
            (43, "Y".to_string()),
            (36, new_seq.to_string()),
            (123, "Y".to_string()),
        ])
    }

    fn with_poss_dup(msg: &FixMessage) -> FixMessage {
        let mut tags = msg.tags.clone();
        if !tags.iter().any(|(t, _)| *t == 43) {
            tags.push((43, "Y".to_string()));
        }
        FixMessage::new(tags)
    }

    /// Replay stored application messages in `[begin, end]`; gap-fill admin/missing seqs.
    fn replay_outbound(&self, begin: u32, end: u32) -> Vec<FixAction> {
        let begin = begin.max(1);
        let end = if end == 0 {
            self.next_send_seq.saturating_sub(1).max(begin)
        } else {
            end
        };
        let mut actions = Vec::new();
        let mut gap_start: Option<u32> = None;
        for seq in begin..=end {
            let stored = self
                .outbound_log
                .iter()
                .find(|m| Self::seq_of(m) == Some(seq));
            let is_app = stored
                .and_then(|m| m.msg_type())
                .is_some_and(|t| !Self::is_admin_msg_type(t));
            if is_app {
                if let Some(gs) = gap_start.take() {
                    actions.push(FixAction::SendMessage(self.gap_fill_message(gs, seq)));
                }
                if let Some(msg) = stored {
                    actions.push(FixAction::SendMessage(Self::with_poss_dup(msg)));
                }
            } else if gap_start.is_none() {
                gap_start = Some(seq);
            }
        }
        if let Some(gs) = gap_start {
            actions.push(FixAction::SendMessage(self.gap_fill_message(gs, end + 1)));
        }
        if actions.is_empty() {
            actions.push(FixAction::SendMessage(
                self.gap_fill_message(begin, end + 1),
            ));
        }
        actions
    }

    // ── Private Helpers ───────────────────────────────────────

    /// Build the tag list for a Heartbeat (35=0) message, incrementing
    /// the send sequence number.
    fn build_heartbeat_tags(&mut self) -> Vec<(u32, String)> {
        let seq = self.next_send_seq;
        self.next_send_seq += 1;

        vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "0".to_string()),
            (49, self.sender_comp_id.clone()),
            (56, self.target_comp_id.clone()),
            (34, seq.to_string()),
        ]
    }

    /// Build a Logout (35=5) message without changing state.
    /// Used when responding to peer-initiated logout.
    fn build_logout_message(&mut self) -> FixMessage {
        let seq = self.next_send_seq;
        self.next_send_seq += 1;

        let tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, "5".to_string()),
            (49, self.sender_comp_id.clone()),
            (56, self.target_comp_id.clone()),
            (34, seq.to_string()),
        ];

        FixMessage::new(tags)
    }
}

// ─── Tests ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid FIX message with given MsgType and MsgSeqNum.
    fn fix_msg(msg_type: &str, seq: u32) -> FixMessage {
        FixMessage::new(vec![
            (8u32, "FIX.4.4".to_string()),
            (35, msg_type.to_string()),
            (49, "CLIENT".to_string()),
            (56, "SERVER".to_string()),
            (34, seq.to_string()),
        ])
    }

    fn fix_msg_with_tags(msg_type: &str, seq: u32, extra_tags: Vec<(u32, String)>) -> FixMessage {
        let mut tags = vec![
            (8u32, "FIX.4.4".to_string()),
            (35, msg_type.to_string()),
            (49, "CLIENT".to_string()),
            (56, "SERVER".to_string()),
            (34, seq.to_string()),
        ];
        tags.extend(extra_tags);
        FixMessage::new(tags)
    }

    // ── State Transitions ──────────────────────────────────────

    #[test]
    fn test_state_transitions_logged_out_to_logged_in() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        assert_eq!(session.state(), FixSessionState::LoggedOut);

        // Send Logon
        let logon = session.send_logon();
        assert_eq!(logon.msg_type(), Some("A"));
        assert_eq!(session.state(), FixSessionState::LogonSent);

        // Receive Logon response
        let response = fix_msg("A", 1);
        let actions = session.process_incoming(&response).unwrap();
        assert_eq!(session.state(), FixSessionState::LoggedIn);
        assert!(session.is_logged_in());
        assert!(actions.is_empty());
    }

    #[test]
    fn test_state_transitions_logged_in_to_logout_sent_to_logged_out() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        // Get to LoggedIn
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        assert!(session.is_logged_in());

        // Send Logout
        let logout = session.send_logout();
        assert_eq!(logout.msg_type(), Some("5"));
        assert_eq!(session.state(), FixSessionState::LogoutSent);

        // Receive Logout confirmation
        let actions = session.process_incoming(&fix_msg("5", 2)).unwrap();
        assert_eq!(session.state(), FixSessionState::LoggedOut);
        assert!(!session.is_logged_in());
        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], FixAction::CloseConnection));
    }

    #[test]
    fn test_state_peer_initiated_logout() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        assert!(session.is_logged_in());

        // Peer sends Logout
        let actions = session.process_incoming(&fix_msg("5", 2)).unwrap();
        assert_eq!(session.state(), FixSessionState::LoggedOut);
        // Should respond with our own Logout
        assert_eq!(actions.len(), 1);
        match &actions[0] {
            FixAction::SendMessage(msg) => {
                assert_eq!(msg.msg_type(), Some("5"));
            }
            _ => panic!("expected SendMessage action"),
        }
    }

    #[test]
    fn test_error_logout_in_logged_out_state() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        let result = session.process_incoming(&fix_msg("5", 1));
        assert!(matches!(
            result,
            Err(FixSessionError::UnexpectedMessage { .. })
        ));
    }

    // ── Heartbeat Timer ───────────────────────────────────────

    #[test]
    fn test_heartbeat_sent_after_interval() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.set_heartbeat_interval(Duration::from_millis(50));

        // Must be logged in for heartbeat
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        assert!(session.is_logged_in());

        // Immediately check — no heartbeat yet
        assert!(session.check_heartbeat().is_none());

        // Wait for the interval to expire
        std::thread::sleep(Duration::from_millis(60));

        let hb = session.check_heartbeat().unwrap();
        assert_eq!(hb.msg_type(), Some("0"));
    }

    #[test]
    fn test_no_heartbeat_when_not_logged_in() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.set_heartbeat_interval(Duration::from_millis(10));

        std::thread::sleep(Duration::from_millis(20));
        assert!(session.check_heartbeat().is_none());
    }

    // ── TestRequest → Heartbeat ───────────────────────────────

    #[test]
    fn test_test_request_triggers_heartbeat() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        let test_req = fix_msg_with_tags("1", 2, vec![(112, "TEST-001".to_string())]);
        let actions = session.process_incoming(&test_req).unwrap();

        assert_eq!(actions.len(), 1);
        match &actions[0] {
            FixAction::SendMessage(msg) => {
                assert_eq!(msg.msg_type(), Some("0"));
                assert_eq!(msg.get_tag(112), Some("TEST-001"));
            }
            _ => panic!("expected SendMessage action"),
        }
    }

    #[test]
    fn test_test_request_without_test_req_id() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        let test_req = fix_msg("1", 2);
        let actions = session.process_incoming(&test_req).unwrap();

        assert_eq!(actions.len(), 1);
        match &actions[0] {
            FixAction::SendMessage(msg) => {
                assert_eq!(msg.msg_type(), Some("0"));
                // No TestReqID should be present
                assert_eq!(msg.get_tag(112), None);
            }
            _ => panic!("expected SendMessage action"),
        }
    }

    // ── ResendRequest / Gap Fill ──────────────────────────────

    #[test]
    fn test_resend_request_on_seq_gap() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        // Send a message with seq=5 when we expect seq=2
        let msg = fix_msg("D", 5);
        let actions = session.process_incoming(&msg).unwrap();

        assert_eq!(actions.len(), 1);
        match &actions[0] {
            FixAction::SendResendRequest {
                begin_seq_no,
                end_seq_no,
            } => {
                assert_eq!(*begin_seq_no, 2); // We expected 2
                assert_eq!(*end_seq_no, 0); // All messages from 2 to current
            }
            _ => panic!("expected SendResendRequest action"),
        }
    }

    #[test]
    fn test_received_resend_request_generates_gap_fill() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        let resend_req = fix_msg_with_tags(
            "2",
            2,
            vec![
                (7, "1".to_string()),  // BeginSeqNo
                (16, "3".to_string()), // EndSeqNo
            ],
        );
        let actions = session.process_incoming(&resend_req).unwrap();

        assert_eq!(actions.len(), 1);
        match &actions[0] {
            FixAction::SendMessage(msg) => {
                assert_eq!(msg.msg_type(), Some("4")); // SequenceReset
                assert_eq!(msg.get_tag(36), Some("4")); // NewSeqNo = EndSeqNo + 1
                assert_eq!(msg.get_tag(123), Some("Y")); // GapFillFlag
            }
            _ => panic!("expected SendMessage action"),
        }
    }

    #[test]
    fn test_resend_request_end_zero_uses_current_seq() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        let resend_req = fix_msg_with_tags(
            "2",
            2,
            vec![
                (7, "1".to_string()),  // BeginSeqNo
                (16, "0".to_string()), // EndSeqNo = 0
            ],
        );
        let actions = session.process_incoming(&resend_req).unwrap();

        match &actions[0] {
            FixAction::SendMessage(msg) => {
                // EndSeqNo=0 means "through last outbound" (logon seq 1).
                assert_eq!(msg.get_tag(36), Some("2"));
            }
            _ => panic!("expected SendMessage action"),
        }
    }

    #[test]
    fn test_resend_replays_application_messages_with_poss_dup() {
        let mut session = FixSession::new("FIG-GW".into(), "CLIENT".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        let ctx = session.outbound_context();
        let exec = crate::fix::fig_to_fix_execution_report(
            &fig_core::messages::ExecutionReport {
                cl_ord_id: "ORD-1".to_string(),
                order_id: "OX-1".to_string(),
                exec_id: "EX-1".to_string(),
                exec_type: fig_core::messages::ExecType::New,
                ord_status: fig_core::messages::OrdStatus::New,
                side: fig_core::messages::Side::Buy,
                last_qty: None,
                last_price: None,
                leaves_qty: fig_core::messages::Quantity(1.0),
                cum_qty: fig_core::messages::Quantity(0.0),
                avg_price: fig_core::messages::Price(10.0),
                symbol: "AAPL".to_string(),
                transact_time: 1,
            },
            &ctx,
        );
        let exec_msg = crate::fix::FixMessage::from_bytes(&exec).unwrap();
        session.record_outbound(exec_msg);

        let resend_req =
            fix_msg_with_tags("2", 2, vec![(7, "1".to_string()), (16, "2".to_string())]);
        let actions = session.process_incoming(&resend_req).unwrap();
        assert!(actions.iter().any(|a| matches!(
            a,
            FixAction::SendMessage(m) if m.msg_type() == Some("8") && m.get_tag(43) == Some("Y")
        )));
        assert!(actions.iter().any(|a| matches!(
            a,
            FixAction::SendMessage(m) if m.msg_type() == Some("4") && m.get_tag(123) == Some("Y")
        )));
    }

    // ── SeqNum Tracking ───────────────────────────────────────

    #[test]
    fn test_seq_num_increment_on_send() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());

        // First message
        let msg1 = session.send_logon();
        assert_eq!(msg1.get_tag(34), Some("1"));

        // Second message
        let msg2 = session.send_heartbeat();
        assert_eq!(msg2.get_tag(34), Some("2"));

        // Third message
        let msg3 = session.send_logout();
        assert_eq!(msg3.get_tag(34), Some("3"));
    }

    #[test]
    fn test_expected_recv_seq_advances() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();

        // Receive seq=1
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        // forward to LoggedIn; expected_recv_seq should now be 2

        // Receive seq=2
        session.process_incoming(&fix_msg("0", 2)).unwrap();
        // expected_recv_seq should now be 3

        // Receive seq=4 (gap)
        let actions = session.process_incoming(&fix_msg("D", 4)).unwrap();
        match &actions[0] {
            FixAction::SendResendRequest { begin_seq_no, .. } => {
                assert_eq!(*begin_seq_no, 3); // We expected 3 but got 4
            }
            _ => panic!("expected SendResendRequest"),
        }
    }

    #[test]
    fn test_poss_dup_ignored_when_seq_too_low() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        // Send a message we've already seen (seq=1) with PossDupFlag=Y
        let dup = fix_msg_with_tags("A", 1, vec![(43, "Y".to_string())]);
        let actions = session.process_incoming(&dup).unwrap();
        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], FixAction::NoAction));
    }

    #[test]
    fn test_seq_too_low_without_poss_dup_is_error() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        // seq=1 again without PossDupFlag
        let result = session.process_incoming(&fix_msg("A", 1));
        assert!(matches!(result, Err(FixSessionError::SeqNumTooLow { .. })));
    }

    // ── Error Cases ───────────────────────────────────────────

    #[test]
    fn test_not_logged_in_rejects_app_message() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        // Still in LoggedOut state
        let result = session.process_incoming(&fix_msg("D", 1));
        assert!(matches!(result, Err(FixSessionError::NotLoggedIn)));
    }

    #[test]
    fn test_unexpected_logon_in_logout_sent_state() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        session.send_logout(); // Now in LogoutSent
        let result = session.process_incoming(&fix_msg("A", 2));
        assert!(matches!(
            result,
            Err(FixSessionError::UnexpectedMessage { .. })
        ));
    }

    #[test]
    fn test_force_logout_from_any_state() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        assert!(session.is_logged_in());

        session.force_logout();
        assert!(!session.is_logged_in());
        assert_eq!(session.state(), FixSessionState::LoggedOut);
    }

    #[test]
    fn test_reject_message_ignored() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();

        // Reject (35=3) should not change state
        let actions = session.process_incoming(&fix_msg("3", 2)).unwrap();
        assert!(actions.is_empty());
        assert!(session.is_logged_in());
    }

    #[test]
    fn test_default_heartbeat_interval() {
        let session = FixSession::new("CLIENT".into(), "SERVER".into());
        assert_eq!(session.heartbeat_interval(), Duration::from_secs(30));
    }

    #[test]
    fn test_custom_heartbeat_interval() {
        let mut session = FixSession::new("CLIENT".into(), "SERVER".into());
        session.set_heartbeat_interval(Duration::from_secs(60));
        assert_eq!(session.heartbeat_interval(), Duration::from_secs(60));
    }

    #[test]
    fn test_seq_store_persists_across_sessions() {
        use std::sync::Arc;

        use crate::fix_seq_store::MemoryFixSeqStore;

        let store: Arc<dyn crate::fix_seq_store::FixSeqStore> = Arc::new(MemoryFixSeqStore::new());

        let mut session =
            FixSession::new("GW".into(), "CLIENT".into()).with_seq_store(store.clone());
        session.send_logon();
        session.process_incoming(&fix_msg("A", 1)).unwrap();
        session.process_incoming(&fix_msg("0", 2)).unwrap();

        let mut resumed = FixSession::new("GW".into(), "CLIENT".into()).with_seq_store(store);
        resumed.restore_from_store();
        assert_eq!(resumed.next_send_seq(), session.next_send_seq());
        assert_eq!(resumed.expected_recv_seq(), session.expected_recv_seq());
        assert!(resumed.is_logged_in());
    }
}
