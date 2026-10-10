//! Authenticated session resumption tokens (SPEC §10.1).
//!
//! A resumption token lets a client prove continuity of an existing session
//! after a reconnect (including a TREE 0-RTT reconnect) without replaying a
//! full logon. Because the token carries session identity
//! (`SESSION_ID`, channels, subscriptions), it **must not** be trusted unless
//! it is authenticated: an unauthenticated token would let any peer forge an
//! arbitrary session identity.
//!
//! Tokens are therefore **issued and signed by the server** with HMAC-SHA256
//! over a process/deployment key. The client stores the opaque token and
//! echoes it back on reconnect; only the server that holds the key can mint a
//! token that [`SessionSigner::verify`] accepts.
//!
//! ```
//! use fig_core::session::Session;
//! use fig_core::session_token::SessionSigner;
//!
//! let signer = SessionSigner::from_key_bytes(b"0123456789abcdef0123456789abcdef").unwrap();
//! let mut session = Session::new();
//! session.add_channel(1);
//!
//! let token = signer.issue(&session).unwrap();
//! let restored = signer.verify(&token).unwrap();
//! assert_eq!(restored.session_id, session.session_id);
//! ```

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::SessionError;
use crate::session::Session;

type HmacSha256 = Hmac<Sha256>;

/// Magic prefix identifying a signed FIG resumption token.
pub const TOKEN_MAGIC: &[u8; 6] = b"FIGRT1";

/// Length of the trailing HMAC-SHA256 tag.
pub const TOKEN_MAC_LEN: usize = 32;

/// Environment variable holding the resumption-token signing key.
///
/// Accepts a hex string (64 hex chars = 32 bytes) or a raw passphrase. Set the
/// same value on every node that must accept the same tokens (HA deployments).
pub const RESUMPTION_KEY_ENV: &str = "FIG_RESUMPTION_KEY";

/// Signs and verifies session resumption tokens with HMAC-SHA256.
///
/// The key is held only by the server (and any node that must accept the
/// tokens). Clients treat issued tokens as opaque bytes.
#[derive(Clone)]
pub struct SessionSigner {
    key: Vec<u8>,
}

impl std::fmt::Debug for SessionSigner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Never print key material.
        f.debug_struct("SessionSigner")
            .field("key", &"<redacted>")
            .finish()
    }
}

impl SessionSigner {
    /// Create a signer from raw key bytes.
    pub fn from_key_bytes(key: impl Into<Vec<u8>>) -> Result<Self, SessionError> {
        let key = key.into();
        if key.is_empty() {
            return Err(SessionError::SerializationError(
                "resumption signing key must not be empty".into(),
            ));
        }
        Ok(Self { key })
    }

    /// Create a signer from a hex string, or from a raw passphrase if the
    /// value is not valid hex.
    pub fn from_hex_or_passphrase(value: &str) -> Result<Self, SessionError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SessionError::SerializationError(
                "resumption signing key must not be empty".into(),
            ));
        }
        // Try hex first (even length, all hex digits).
        if trimmed.len().is_multiple_of(2) && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            if let Ok(bytes) = hex::decode(trimmed) {
                return Self::from_key_bytes(bytes);
            }
        }
        Self::from_key_bytes(trimmed.as_bytes().to_vec())
    }

    /// Build a signer from [`RESUMPTION_KEY_ENV`], or generate a random
    /// per-process key when the variable is unset.
    ///
    /// A random key is safe (tokens cannot be forged) but not shareable: tokens
    /// issued by one node will not verify on another. Multi-node deployments
    /// MUST set `FIG_RESUMPTION_KEY` to a shared secret.
    pub fn from_env() -> Self {
        match std::env::var(RESUMPTION_KEY_ENV) {
            Ok(value) if !value.trim().is_empty() => match Self::from_hex_or_passphrase(&value) {
                Ok(signer) => signer,
                Err(_) => Self::random(),
            },
            _ => {
                tracing::warn!(
                    "{RESUMPTION_KEY_ENV} is not set; generating a random per-process resumption \
                     key. Resumption tokens will not verify on other nodes — set the variable for \
                     multi-node deployments."
                );
                Self::random()
            }
        }
    }

    /// Generate a signer with a fresh random 32-byte key.
    pub fn random() -> Self {
        // Two UUIDv4 values = 256 bits of OS entropy.
        let mut key = Vec::with_capacity(32);
        key.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
        key.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
        Self { key }
    }

    fn mac(&self, payload: &[u8]) -> [u8; TOKEN_MAC_LEN] {
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC accepts any key length");
        mac.update(TOKEN_MAGIC);
        mac.update(payload);
        let bytes = mac.finalize().into_bytes();
        let mut out = [0u8; TOKEN_MAC_LEN];
        out.copy_from_slice(&bytes);
        out
    }

    /// Issue a signed, opaque resumption token for `session`.
    ///
    /// Layout: `MAGIC (6) || u32 BE payload len || payload || HMAC-SHA256 (32)`.
    pub fn issue(&self, session: &Session) -> Result<Vec<u8>, SessionError> {
        let payload = session.to_token_payload()?;
        let mut token = Vec::with_capacity(TOKEN_MAGIC.len() + 4 + payload.len() + TOKEN_MAC_LEN);
        token.extend_from_slice(TOKEN_MAGIC);
        token.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        token.extend_from_slice(&payload);
        token.extend_from_slice(&self.mac(&payload));
        Ok(token)
    }

    /// Verify a signed resumption token and return the authenticated session.
    ///
    /// Returns [`SessionError::InvalidResumptionToken`] for any token that is
    /// malformed, tampered with, or signed with a different key. The HMAC
    /// check is constant-time.
    pub fn verify(&self, token: &[u8]) -> Result<Session, SessionError> {
        let header = TOKEN_MAGIC.len() + 4;
        if token.len() < header + TOKEN_MAC_LEN {
            return Err(SessionError::InvalidResumptionToken);
        }
        if &token[..TOKEN_MAGIC.len()] != TOKEN_MAGIC {
            return Err(SessionError::InvalidResumptionToken);
        }

        let len_bytes: [u8; 4] = token[TOKEN_MAGIC.len()..header]
            .try_into()
            .map_err(|_| SessionError::InvalidResumptionToken)?;
        let payload_len = u32::from_be_bytes(len_bytes) as usize;

        let payload_end = header
            .checked_add(payload_len)
            .ok_or(SessionError::InvalidResumptionToken)?;
        if token.len() != payload_end + TOKEN_MAC_LEN {
            return Err(SessionError::InvalidResumptionToken);
        }

        let payload = &token[header..payload_end];
        let tag = &token[payload_end..];

        // Constant-time MAC verification.
        let mut mac = HmacSha256::new_from_slice(&self.key).expect("HMAC accepts any key length");
        mac.update(TOKEN_MAGIC);
        mac.update(payload);
        mac.verify_slice(tag)
            .map_err(|_| SessionError::InvalidResumptionToken)?;

        Session::from_token_payload(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signer() -> SessionSigner {
        SessionSigner::from_key_bytes(b"0123456789abcdef0123456789abcdef".to_vec()).unwrap()
    }

    fn sample_session() -> Session {
        let mut s = Session::new();
        s.add_channel(1);
        s.add_channel(7);
        s.record_sent_seq(10);
        s.record_recv_seq(20);
        s
    }

    #[test]
    fn round_trip() {
        let signer = signer();
        let session = sample_session();
        let token = signer.issue(&session).unwrap();
        let restored = signer.verify(&token).unwrap();
        assert_eq!(restored.session_id, session.session_id);
        assert_eq!(restored.channels, vec![1, 7]);
        assert_eq!(restored.last_seq_sent, 10);
        assert_eq!(restored.last_seq_recv, 20);
    }

    #[test]
    fn rejects_tampered_payload() {
        let signer = signer();
        let mut token = signer.issue(&sample_session()).unwrap();
        let last_payload = token.len() - TOKEN_MAC_LEN - 1;
        token[last_payload] ^= 0x01;
        assert!(matches!(
            signer.verify(&token),
            Err(SessionError::InvalidResumptionToken)
        ));
    }

    #[test]
    fn rejects_forged_unsigned_cbor() {
        // The original vulnerability: a client-minted, unsigned CBOR token.
        let forged = sample_session().to_token_payload().unwrap();
        assert!(matches!(
            signer().verify(&forged),
            Err(SessionError::InvalidResumptionToken)
        ));
    }

    #[test]
    fn rejects_token_signed_with_other_key() {
        let attacker = SessionSigner::from_key_bytes(b"attacker-key-attacker-key-123456").unwrap();
        let token = attacker.issue(&sample_session()).unwrap();
        assert!(matches!(
            signer().verify(&token),
            Err(SessionError::InvalidResumptionToken)
        ));
    }

    #[test]
    fn rejects_truncated_and_garbage() {
        let signer = signer();
        let token = signer.issue(&sample_session()).unwrap();
        assert!(signer.verify(&[]).is_err());
        assert!(signer.verify(b"garbage").is_err());
        assert!(signer.verify(&token[..token.len() - 1]).is_err());
        assert!(signer
            .verify(&[TOKEN_MAGIC.as_slice(), &[0xff; 40]].concat())
            .is_err());
    }

    #[test]
    fn key_from_hex_matches_key_from_bytes() {
        let hex = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        let from_hex = SessionSigner::from_hex_or_passphrase(hex).unwrap();
        let raw = hex::decode(hex).unwrap();
        let from_bytes = SessionSigner::from_key_bytes(raw).unwrap();
        let session = sample_session();
        let token = from_hex.issue(&session).unwrap();
        assert!(from_bytes.verify(&token).is_ok());
    }
}
