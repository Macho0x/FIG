//! Authentication support for FIG connections (wire semantics).
//!
//! FIG defines how credentials are carried and verified on frames — not how
//! they are issued. Bearer tokens and JWTs use the `AUTH_TOKEN` extension;
//! mTLS uses client certificate CN at the TLS layer.
//!
//! Reference helpers here validate tokens for tests and conformance. Production
//! venues plug their own identity systems (key stores, JWT issuers, IdP) behind
//! the same on-wire contract (SPEC §9.3).

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Authentication method for FIG connections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    /// No authentication (development only).
    None,
    /// Token-based authentication (bearer or JWT on the wire).
    Token(String),
    /// mTLS authentication (client certificate CN).
    Mtls {
        /// Common Name from the client certificate.
        cn: String,
        /// SHA-256 fingerprint of the client certificate (hex-encoded).
        fingerprint: String,
    },
}

impl std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthMethod::None => write!(f, "none"),
            AuthMethod::Token(_) => write!(f, "token"),
            AuthMethod::Mtls { cn, .. } => write!(f, "mtls:{}", cn),
        }
    }
}

/// Result of a successful authentication.
#[derive(Debug, Clone)]
pub struct AuthResult {
    /// The authentication method that was used.
    pub method: AuthMethod,
    /// The session ID associated with this authentication.
    pub session_id: Uuid,
    /// Permissions granted to this authenticated principal.
    pub permissions: Vec<String>,
}

impl AuthResult {
    /// Create a new authentication result.
    pub fn new(method: AuthMethod, session_id: Uuid, permissions: Vec<String>) -> Self {
        Self {
            method,
            session_id,
            permissions,
        }
    }

    /// Check if the authenticated principal has a specific permission.
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
}

/// Verify a token against the expected value using constant-time comparison.
///
/// Both sides are hashed to a fixed-width SHA-256 digest and compared with a
/// constant-time equality check, so neither the token length nor the position
/// of the first differing byte leaks through timing.
///
/// For production deployments, compare against a hash lookup or JWT signature
/// from your venue's identity system — FIG does not define credential issuance.
pub fn verify_token(token: &str, expected: &str) -> bool {
    use sha2::{Digest, Sha256};
    use subtle::ConstantTimeEq;

    let token_digest = Sha256::digest(token.as_bytes());
    let expected_digest = Sha256::digest(expected.as_bytes());
    token_digest.ct_eq(&expected_digest).into()
}

/// Generate a simple bearer token (for development/testing only).
///
/// Not a production key-issuance API — venues own credential lifecycle.
pub fn generate_dev_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("fig-dev-{:x}-{}", ts, Uuid::new_v4())
}

// ─── Auth Token ──────────────────────────────────────────────────

/// An authentication token used for connection and session refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthToken {
    /// A development token (for testing only).
    DevToken(String),
    /// A bearer token (JWT or opaque string on the wire).
    BearerToken(String),
}

impl AuthToken {
    /// Generate a new development token for testing.
    pub fn refresh_dev_token() -> Self {
        AuthToken::DevToken(generate_dev_token())
    }

    /// Get the raw token string.
    pub fn as_str(&self) -> &str {
        match self {
            AuthToken::DevToken(s) | AuthToken::BearerToken(s) => s.as_str(),
        }
    }
}

// ─── Auth Error ─────────────────────────────────────────────────

/// Errors that can occur during authentication or token refresh.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid token format")]
    InvalidToken,
    #[error("token verification failed")]
    TokenMismatch,
    #[error("session is not valid for token refresh")]
    InvalidSession,
}

// ─── AuthMethod token verification ───────────────────────────────

impl AuthMethod {
    /// Verify a refreshed token.
    ///
    /// Same verification as initial auth, but also checks that the
    /// session is still valid. This is a single-direction refresh
    /// (client → server only).
    ///
    /// TODO: bidirectional refresh race handling
    pub fn verify_refresh_token(
        &self,
        token: &[u8],
        session_id: &Uuid,
    ) -> Result<AuthToken, AuthError> {
        // Invalid UTF-8 is rejected before the session check so a corrupt
        // token is not reported as an unknown session.
        let token_str = std::str::from_utf8(token).map_err(|_| AuthError::InvalidToken)?;
        if session_id.is_nil() {
            return Err(AuthError::InvalidSession);
        }
        match self {
            AuthMethod::None => {
                if token_str.is_empty() {
                    return Err(AuthError::InvalidToken);
                }
                Ok(AuthToken::DevToken(token_str.to_string()))
            }
            AuthMethod::Token(expected) => {
                if verify_token(token_str, expected) {
                    Ok(AuthToken::BearerToken(token_str.to_string()))
                } else {
                    Err(AuthError::TokenMismatch)
                }
            }
            AuthMethod::Mtls { fingerprint, .. } => {
                if token_str.is_empty() {
                    return Err(AuthError::InvalidToken);
                }
                // The refresh token must be the certificate fingerprint, not
                // an attacker-chosen string. The returned bearer is that
                // fingerprint, compared in constant time via `verify_token`.
                if verify_token(token_str, fingerprint) {
                    Ok(AuthToken::BearerToken(fingerprint.clone()))
                } else {
                    Err(AuthError::TokenMismatch)
                }
            }
        }
    }
}

// ─── Unit Tests ──────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_method_none() {
        let m = AuthMethod::None;
        assert_eq!(m.to_string(), "none");
    }

    #[test]
    fn test_auth_method_token_display() {
        let m = AuthMethod::Token("secret-123".into());
        assert_eq!(m.to_string(), "token");
    }

    #[test]
    fn test_auth_method_mtls_display() {
        let m = AuthMethod::Mtls {
            cn: "client.example.com".into(),
            fingerprint: "abc123def456".into(),
        };
        assert_eq!(m.to_string(), "mtls:client.example.com");
    }

    #[test]
    fn test_auth_method_serialization_round_trip() {
        // CBOR round-trip
        let m = AuthMethod::Token("my-api-key".into());
        let encoded = crate::codec::encode_cbor(&m).unwrap();
        let decoded: AuthMethod = crate::codec::decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, m);

        // JSON round-trip
        let json = serde_json::to_string(&m).unwrap();
        let decoded: AuthMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, m);
    }

    #[test]
    fn test_auth_method_mtls_serialization() {
        let m = AuthMethod::Mtls {
            cn: "trading-client".into(),
            fingerprint: "abcdef0123456789abcdef0123456789abcdef01".into(),
        };
        let encoded = crate::codec::encode_cbor(&m).unwrap();
        let decoded: AuthMethod = crate::codec::decode_cbor(&encoded).unwrap();
        assert_eq!(decoded, m);
    }

    #[test]
    fn test_verify_token_match() {
        assert!(verify_token("secret-token-123", "secret-token-123"));
    }

    #[test]
    fn test_verify_token_mismatch() {
        assert!(!verify_token("secret-token-123", "wrong-token"));
    }

    #[test]
    fn test_verify_token_different_length() {
        assert!(!verify_token("short", "longer-token-value"));
    }

    #[test]
    fn test_verify_token_empty() {
        assert!(verify_token("", ""));
        assert!(!verify_token("", "non-empty"));
        assert!(!verify_token("non-empty", ""));
    }

    #[test]
    fn test_auth_result_permissions() {
        let result = AuthResult::new(
            AuthMethod::Token("key".into()),
            Uuid::new_v4(),
            vec!["orders:read".into(), "orders:write".into()],
        );
        assert!(result.has_permission("orders:read"));
        assert!(result.has_permission("orders:write"));
        assert!(!result.has_permission("admin"));
    }

    #[test]
    fn test_auth_result_empty_permissions() {
        let result = AuthResult::new(AuthMethod::None, Uuid::new_v4(), vec![]);
        assert!(!result.has_permission("anything"));
    }

    #[test]
    fn test_generate_dev_token() {
        let token = generate_dev_token();
        assert!(token.starts_with("fig-dev-"));
        assert!(token.len() > 16);
    }

    #[test]
    fn test_auth_token_refresh_dev_token() {
        let token = AuthToken::refresh_dev_token();
        match token {
            AuthToken::DevToken(ref s) => {
                assert!(s.starts_with("fig-dev-"));
                assert!(s.len() > 16);
            }
            _ => panic!("expected DevToken"),
        }
    }

    #[test]
    fn test_auth_token_as_str() {
        let token = AuthToken::BearerToken("my-key".into());
        assert_eq!(token.as_str(), "my-key");
        let token = AuthToken::DevToken("dev-key".into());
        assert_eq!(token.as_str(), "dev-key");
    }

    #[test]
    fn test_verify_refresh_token_none_accepts_any() {
        let auth = AuthMethod::None;
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"anything", &session);
        assert!(result.is_ok());
        let token = result.unwrap();
        match token {
            AuthToken::DevToken(ref s) => assert_eq!(s, "anything"),
            _ => panic!("expected DevToken"),
        }
    }

    #[test]
    fn test_verify_refresh_token_nil_session() {
        let auth = AuthMethod::None;
        let result = auth.verify_refresh_token(b"anything", &Uuid::nil());
        assert!(matches!(result, Err(AuthError::InvalidSession)));
    }

    #[test]
    fn test_verify_refresh_token_valid() {
        let auth = AuthMethod::Token("secret-key".into());
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"secret-key", &session);
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.as_str(), "secret-key");
    }

    #[test]
    fn test_verify_refresh_token_mismatch() {
        let auth = AuthMethod::Token("secret-key".into());
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"wrong-key", &session);
        assert!(result.is_err());
        match result {
            Err(AuthError::TokenMismatch) => {}
            _ => panic!("expected TokenMismatch"),
        }
    }

    #[test]
    fn test_verify_refresh_token_invalid_utf8() {
        let auth = AuthMethod::Token("secret-key".into());
        // Invalid UTF-8 bytes are reported before the session check.
        let invalid = vec![0xFF, 0xFE, 0xFD];
        let result = auth.verify_refresh_token(&invalid, &Uuid::nil());
        assert!(result.is_err());
        match result {
            Err(AuthError::InvalidToken) => {}
            _ => panic!("expected InvalidToken"),
        }
    }

    #[test]
    fn test_verify_refresh_token_mtls_fingerprint() {
        let auth = AuthMethod::Mtls {
            cn: "client".into(),
            fingerprint: "abc".into(),
        };
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"abc", &session).unwrap();
        assert_eq!(result.as_str(), "abc");
    }

    #[test]
    fn test_verify_refresh_token_mtls_unbound() {
        let auth = AuthMethod::Mtls {
            cn: "client".into(),
            fingerprint: "abc".into(),
        };
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"mtls-token", &session);
        assert!(matches!(result, Err(AuthError::TokenMismatch)));
    }

    #[test]
    fn test_verify_refresh_token_mtls_empty() {
        let auth = AuthMethod::Mtls {
            cn: "client".into(),
            fingerprint: "abc".into(),
        };
        let session = Uuid::new_v4();
        let result = auth.verify_refresh_token(b"", &session);
        assert!(result.is_err());
        match result {
            Err(AuthError::InvalidToken) => {}
            _ => panic!("expected InvalidToken"),
        }
    }
}
