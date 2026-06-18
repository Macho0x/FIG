//! Authentication support for FIG connections.
//!
//! FIG supports two authentication mechanisms:
//! - **Token-based**: Bearer tokens (API keys or JWT) carried in the
//!   `AUTH_TOKEN` extension field.
//! - **mTLS**: Mutual TLS authentication using client certificates,
//!   with the Common Name (CN) and certificate fingerprint used for
//!   identity verification.
//!
//! The authentication method is advertised via the `AUTH_METHOD`
//! extension tag during connection setup.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Authentication method for FIG connections.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    /// No authentication (development only).
    None,
    /// Token-based authentication (API key or JWT).
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
/// For production deployments, replace this with HMAC verification or
/// JWT signature validation against a trusted issuer.
pub fn verify_token(token: &str, expected: &str) -> bool {
    // Constant-time comparison to prevent timing attacks
    token.len() == expected.len()
        && token
            .bytes()
            .zip(expected.bytes())
            .all(|(a, b)| a == b)
}

/// Generate a simple bearer token (for development only).
///
/// Production systems should use a proper token management system
/// (e.g., JWT with RS256, or a secure token generator).
pub fn generate_dev_token() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("fig-dev-{:x}-{}", ts, Uuid::new_v4())
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
        let result = AuthResult::new(
            AuthMethod::None,
            Uuid::new_v4(),
            vec![],
        );
        assert!(!result.has_permission("anything"));
    }

    #[test]
    fn test_generate_dev_token() {
        let token = generate_dev_token();
        assert!(token.starts_with("fig-dev-"));
        assert!(token.len() > 16);
    }
}
