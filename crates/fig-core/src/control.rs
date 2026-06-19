//! Control frame dispatcher for FIG connections.
//!
//! Control frames (channel 0) carry connection-level commands such as
//! PING, PONG, GOAWAY, SETTINGS, AUTH_REFRESH, and SEQ_RESET.
//! The dispatcher inspects each control frame and returns a
//! [`ControlAction`] for the caller to execute.

use crate::auth::{AuthError, AuthMethod};
use crate::channel::ChannelManager;
use crate::error::ChannelError;
use crate::frame::{ControlSubtype, Frame};
use uuid::Uuid;

// ─── Control Action ──────────────────────────────────────────────

/// Actions that the caller must take after processing a control frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlAction {
    /// Send a PONG back to the peer.
    SendPong,
    /// Acknowledge settings (echo back).
    SendSettings,
    /// Close the connection with an optional reason.
    CloseConnection { reason: String },
    /// Update the session with a refreshed auth token.
    RefreshAuth { new_token: Vec<u8> },
    /// Reset sequence numbers for a channel.
    ResetSeq {
        channel_id: u16,
        new_send_seq: u32,
        new_recv_seq: u32,
    },
    /// Request retransmission of a sequence range on a channel.
    ResendRange {
        channel_id: u16,
        begin_seq: u32,
        end_seq: u32,
    },
    /// No action required (acknowledgement frames, etc.).
    NoAction,
}

// ─── Control Error ──────────────────────────────────────────────

/// Errors that can occur while handling a control frame.
#[derive(Debug, thiserror::Error)]
pub enum ControlError {
    #[error("missing auth token in AUTH_REFRESH frame")]
    MissingToken,
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    #[error("channel error: {0}")]
    Channel(#[from] ChannelError),
    #[error("invalid control frame payload")]
    InvalidPayload,
    #[error("unknown control subtype")]
    UnknownSubtype,
}

// ─── Dispatcher ─────────────────────────────────────────────────

/// Handle a single control frame and return the action to take.
///
/// The caller is responsible for providing the correct `session_id`
/// for AUTH_REFRESH verification.
pub fn handle_control_frame(
    frame: &Frame,
    auth: &AuthMethod,
    channel_manager: &mut ChannelManager,
    session_id: &Uuid,
) -> Result<ControlAction, ControlError> {
    match frame.control_subtype() {
        Some(ControlSubtype::Ping) => Ok(ControlAction::SendPong),
        Some(ControlSubtype::Pong) => Ok(ControlAction::NoAction),
        Some(ControlSubtype::Goaway) => Ok(ControlAction::CloseConnection {
            reason: String::from_utf8_lossy(&frame.payload[1..]).to_string(),
        }),
        Some(ControlSubtype::Settings) => Ok(ControlAction::SendSettings),
        Some(ControlSubtype::AuthRefresh) => {
            let token = frame
                .auth_refresh_token()
                .ok_or(ControlError::MissingToken)?;
            // Verify the refreshed token
            auth.verify_refresh_token(token, session_id)
                .map(|_| ControlAction::RefreshAuth {
                    new_token: token.to_vec(),
                })
                .map_err(ControlError::Auth)
        }
        Some(ControlSubtype::SeqReset) => {
            // Skip the subtype byte (first byte of payload)
            let (channel_id, new_send_seq, new_recv_seq) =
                parse_seq_reset_payload(&frame.payload[1..])?;
            channel_manager
                .reset_seq(channel_id, new_send_seq, new_recv_seq)
                .map_err(ControlError::Channel)?;
            Ok(ControlAction::ResetSeq {
                channel_id,
                new_send_seq,
                new_recv_seq,
            })
        }
        Some(ControlSubtype::Resend) => {
            let (channel_id, begin_seq, end_seq) =
                parse_resend_payload(&frame.payload[1..])?;
            Ok(ControlAction::ResendRange {
                channel_id,
                begin_seq,
                end_seq,
            })
        }
        None => Err(ControlError::UnknownSubtype),
    }
}

// ─── Payload Parsing ────────────────────────────────────────────

/// Parse the data portion of a SEQ_RESET payload.
///
/// Payload format (10 bytes, after the subtype byte):
/// - channel_id: 2 bytes BE
/// - new_send_seq: 4 bytes BE
/// - new_recv_seq: 4 bytes BE
fn parse_seq_reset_payload(payload: &[u8]) -> Result<(u16, u32, u32), ControlError> {
    if payload.len() < 10 {
        return Err(ControlError::InvalidPayload);
    }
    let channel_id = u16::from_be_bytes([payload[0], payload[1]]);
    let new_send_seq = u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]);
    let new_recv_seq = u32::from_be_bytes([payload[6], payload[7], payload[8], payload[9]]);
    Ok((channel_id, new_send_seq, new_recv_seq))
}

/// Parse the data portion of a RESEND payload.
///
/// Payload format (10 bytes, after the subtype byte):
/// - channel_id: 2 bytes BE
/// - begin_seq: 4 bytes BE
/// - end_seq: 4 bytes BE (0 = all messages to current)
fn parse_resend_payload(payload: &[u8]) -> Result<(u16, u32, u32), ControlError> {
    if payload.len() < 10 {
        return Err(ControlError::InvalidPayload);
    }
    let channel_id = u16::from_be_bytes([payload[0], payload[1]]);
    let begin_seq = u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]);
    let end_seq = u32::from_be_bytes([payload[6], payload[7], payload[8], payload[9]]);
    Ok((channel_id, begin_seq, end_seq))
}

// ─── Observability ──────────────────────────────────────────────

/// Create a tracing span for handling a control frame.
pub fn span_control_frame(subtype: &str, channel_id: u16) -> tracing::Span {
    tracing::info_span!("control_frame", subtype = subtype, channel_id = channel_id)
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::{ChannelManager, ChannelMode};

    // ── AUTH_REFRESH constructor ────────────────────────────────

    #[test]
    fn test_auth_refresh_constructor() {
        let token = b"my-new-token".to_vec();
        let frame = Frame::auth_refresh(token.clone());

        assert_eq!(frame.control_subtype(), Some(ControlSubtype::AuthRefresh));
        assert_eq!(frame.auth_refresh_token(), Some(&token[..]));
    }

    // ── AUTH_REFRESH token extraction ───────────────────────────

    #[test]
    fn test_auth_refresh_token_extraction() {
        let token = b"refresh-token-abc123".to_vec();
        let frame = Frame::auth_refresh(token.clone());
        let extracted = frame.auth_refresh_token();
        assert_eq!(extracted, Some(&token[..]));
    }

    #[test]
    fn test_auth_refresh_token_non_auth_frame() {
        let frame = Frame::ping();
        assert_eq!(frame.auth_refresh_token(), None);
    }

    #[test]
    fn test_auth_refresh_token_empty() {
        // AUTH_REFRESH with only the subtype byte (no token) — should return None
        let frame = Frame::control(ControlSubtype::AuthRefresh);
        assert_eq!(frame.auth_refresh_token(), None);
    }

    // ── SEQ_RESET constructor ────────────────────────────────────

    #[test]
    fn test_seq_reset_constructor() {
        let frame = Frame::seq_reset(42, 1000, 2000);

        assert_eq!(frame.control_subtype(), Some(ControlSubtype::SeqReset));

        // Payload: subtype(1) + channel_id(2) + send_seq(4) + recv_seq(4) = 11 bytes
        assert_eq!(frame.payload.len(), 11);
        assert_eq!(frame.payload[0], ControlSubtype::SeqReset.code());

        // Verify the payload encoding
        let (ch_id, send_seq, recv_seq) = parse_seq_reset_payload(&frame.payload[1..]).unwrap();
        assert_eq!(ch_id, 42);
        assert_eq!(send_seq, 1000);
        assert_eq!(recv_seq, 2000);
    }

    // ── SEQ_RESET payload parsing ────────────────────────────────

    #[test]
    fn test_parse_seq_reset_payload_valid() {
        // Manually construct a 10-byte payload (channel_id + send_seq + recv_seq)
        let mut payload = Vec::new();
        payload.extend_from_slice(&42u16.to_be_bytes());
        payload.extend_from_slice(&1000u32.to_be_bytes());
        payload.extend_from_slice(&9999u32.to_be_bytes());

        let (ch_id, send_seq, recv_seq) = parse_seq_reset_payload(&payload).unwrap();
        assert_eq!(ch_id, 42);
        assert_eq!(send_seq, 1000);
        assert_eq!(recv_seq, 9999);
    }

    #[test]
    fn test_parse_seq_reset_payload_too_short() {
        let payload = vec![0u8; 5]; // Only 5 bytes, need 10
        let result = parse_seq_reset_payload(&payload);
        assert!(result.is_err());
        match result {
            Err(ControlError::InvalidPayload) => {}
            _ => panic!("expected InvalidPayload"),
        }
    }

    // ── Control dispatcher ───────────────────────────────────────

    #[test]
    fn test_handle_ping_returns_send_pong() {
        let frame = Frame::ping();
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(action, ControlAction::SendPong);
    }

    #[test]
    fn test_handle_pong_returns_no_action() {
        let frame = Frame::pong();
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(action, ControlAction::NoAction);
    }

    #[test]
    fn test_handle_goaway_returns_close_connection() {
        let frame = Frame::goaway("server shutting down");
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(
            action,
            ControlAction::CloseConnection {
                reason: "server shutting down".to_string()
            }
        );
    }

    #[test]
    fn test_handle_settings_returns_send_settings() {
        let frame = Frame::settings();
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(action, ControlAction::SendSettings);
    }

    #[test]
    fn test_resend_constructor() {
        let frame = Frame::resend(42, 100, 200);

        assert_eq!(frame.control_subtype(), Some(ControlSubtype::Resend));
        assert_eq!(frame.payload.len(), 11);
        assert_eq!(frame.payload[0], ControlSubtype::Resend.code());

        let (ch_id, begin, end) = parse_resend_payload(&frame.payload[1..]).unwrap();
        assert_eq!(ch_id, 42);
        assert_eq!(begin, 100);
        assert_eq!(end, 200);
        assert_eq!(frame.resend_range(), Some((42, 100, 200)));
    }

    #[test]
    fn test_parse_resend_payload_too_short() {
        let payload = vec![0u8; 5];
        let result = parse_resend_payload(&payload);
        assert!(matches!(result, Err(ControlError::InvalidPayload)));
    }

    #[test]
    fn test_handle_resend_returns_resend_range() {
        let frame = Frame::resend(7, 10, 50);
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(
            action,
            ControlAction::ResendRange {
                channel_id: 7,
                begin_seq: 10,
                end_seq: 50,
            }
        );
    }

    #[test]
    fn test_handle_resend_invalid_payload() {
        let frame = Frame::control(ControlSubtype::Resend);
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let result = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil());
        assert!(matches!(result, Err(ControlError::InvalidPayload)));
    }

    #[test]
    fn test_handle_non_control_frame() {
        let frame = Frame::new(crate::frame::FrameType::Request, 1);
        let auth = AuthMethod::None;
        let mut mgr = ChannelManager::new(false);
        let result = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil());
        assert!(result.is_err());
        match result {
            Err(ControlError::UnknownSubtype) => {}
            _ => panic!("expected UnknownSubtype"),
        }
    }

    // ── AUTH_REFRESH verification ────────────────────────────────

    #[test]
    fn test_handle_auth_refresh_valid_token() {
        let auth = AuthMethod::Token("valid-token".into());
        let frame = Frame::auth_refresh(b"valid-token".to_vec());
        let mut mgr = ChannelManager::new(false);

        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(
            action,
            ControlAction::RefreshAuth {
                new_token: b"valid-token".to_vec()
            }
        );
    }

    #[test]
    fn test_handle_auth_refresh_missing_token() {
        let auth = AuthMethod::Token("valid-token".into());
        // Create an AUTH_REFRESH frame with only the subtype byte (no token)
        let frame = Frame::control(ControlSubtype::AuthRefresh);
        let mut mgr = ChannelManager::new(false);

        let result = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil());
        assert!(result.is_err());
        match result {
            Err(ControlError::MissingToken) => {}
            _ => panic!("expected MissingToken"),
        }
    }

    #[test]
    fn test_handle_auth_refresh_invalid_token() {
        let auth = AuthMethod::Token("valid-token".into());
        let frame = Frame::auth_refresh(b"wrong-token".to_vec());
        let mut mgr = ChannelManager::new(false);

        let result = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil());
        assert!(result.is_err());
        match result {
            Err(ControlError::Auth(AuthError::TokenMismatch)) => {}
            _ => panic!("expected Auth(TokenMismatch), got {:?}", result),
        }
    }

    #[test]
    fn test_handle_auth_refresh_dev_mode() {
        let auth = AuthMethod::None;
        let frame = Frame::auth_refresh(b"any-token-works".to_vec());
        let mut mgr = ChannelManager::new(false);

        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(
            action,
            ControlAction::RefreshAuth {
                new_token: b"any-token-works".to_vec()
            }
        );
    }

    // ── SEQ_RESET with ChannelManager ────────────────────────────

    #[test]
    fn test_handle_seq_reset_updates_channel_manager() {
        let mut mgr = ChannelManager::new(false);
        let ch_id = mgr.open_channel(ChannelMode::Stateless, None).unwrap();

        // Set initial seq values
        mgr.get_channel_mut(ch_id).unwrap().last_sent_seq = 100;
        mgr.get_channel_mut(ch_id).unwrap().last_recv_seq = 200;

        let frame = Frame::seq_reset(ch_id, 5000, 6000);
        let auth = AuthMethod::None;

        let action = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil()).unwrap();
        assert_eq!(
            action,
            ControlAction::ResetSeq {
                channel_id: ch_id,
                new_send_seq: 5000,
                new_recv_seq: 6000,
            }
        );

        // Verify channel sequences were reset
        let ch = mgr.get_channel(ch_id).unwrap();
        assert_eq!(ch.last_sent_seq, 5000);
        assert_eq!(ch.last_recv_seq, 6000);
    }

    #[test]
    fn test_handle_seq_reset_channel_not_found() {
        let mut mgr = ChannelManager::new(false);
        let frame = Frame::seq_reset(999, 100, 200); // channel 999 doesn't exist
        let auth = AuthMethod::None;

        let result = handle_control_frame(&frame, &auth, &mut mgr, &Uuid::nil());
        assert!(result.is_err());
        match result {
            Err(ControlError::Channel(ChannelError::ChannelNotFound(999))) => {}
            _ => panic!("expected Channel(ChannelNotFound)"),
        }
    }

    // ── Observability ────────────────────────────────────────────

    #[test]
    fn test_span_control_frame() {
        let span = span_control_frame("AUTH_REFRESH", 0);
        // Verify the span was created with the correct name.
        // metadata() returns None when no tracing subscriber is active,
        // so we only assert the name when a subscriber is present.
        if let Some(meta) = span.metadata() {
            assert_eq!(meta.name(), "control_frame");
        }
    }
}
