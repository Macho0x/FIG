//! JWT authentication support for FIG bearer tokens.
//!
//! Spec §6: bearer tokens may be JWTs. This module provides HS256
//! encode/decode for development and testing; production deployments
//! should use RS256 with a proper key management system.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::{AuthMethod, AuthResult};
use uuid::Uuid;

/// JWT validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum JwtError {
    #[error("invalid JWT format")]
    InvalidFormat,
    #[error("invalid JWT header")]
    InvalidHeader,
    #[error("invalid JWT payload")]
    InvalidPayload,
    #[error("JWT signature mismatch")]
    SignatureMismatch,
    #[error("JWT expired")]
    Expired,
    #[error("missing subject claim")]
    MissingSubject,
}

/// FIG JWT claims (Spec §6 bearer token payload).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FigJwtClaims {
    /// Subject (user/service identifier).
    pub sub: String,
    /// Expiration time (Unix seconds).
    pub exp: u64,
    /// Optional permission scopes.
    #[serde(default)]
    pub permissions: Vec<String>,
    /// Optional session binding.
    #[serde(default)]
    pub session_id: Option<String>,
}

impl FigJwtClaims {
    pub fn new(sub: impl Into<String>, exp: u64, permissions: Vec<String>) -> Self {
        Self {
            sub: sub.into(),
            exp,
            permissions,
            session_id: None,
        }
    }

    pub fn is_expired(&self, now: u64) -> bool {
        now >= self.exp
    }
}

/// Encode claims as an HS256 JWT string.
pub fn encode_jwt(claims: &FigJwtClaims, secret: &str) -> Result<String, JwtError> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let header = base64_url_encode(br#"{"alg":"HS256","typ":"JWT"}"#);
    let payload_bytes = serde_json::to_vec(claims).map_err(|_| JwtError::InvalidPayload)?;
    let payload = base64_url_encode(&payload_bytes);

    let signing_input = format!("{}.{}", header, payload);
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| JwtError::InvalidFormat)?;
    mac.update(signing_input.as_bytes());
    let signature = base64_url_encode(&mac.finalize().into_bytes());

    Ok(format!("{}.{}", signing_input, signature))
}

/// Decode and verify an HS256 JWT string.
pub fn decode_jwt(token: &str, secret: &str) -> Result<FigJwtClaims, JwtError> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(JwtError::InvalidFormat);
    }

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    type HmacSha256 = Hmac<Sha256>;
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).map_err(|_| JwtError::InvalidFormat)?;
    mac.update(signing_input.as_bytes());
    let expected = base64_url_encode(&mac.finalize().into_bytes());

    if parts[2] != expected {
        return Err(JwtError::SignatureMismatch);
    }

    let payload_bytes = base64_url_decode(parts[1]).map_err(|_| JwtError::InvalidPayload)?;
    let claims: FigJwtClaims =
        serde_json::from_slice(&payload_bytes).map_err(|_| JwtError::InvalidPayload)?;

    if claims.sub.is_empty() {
        return Err(JwtError::MissingSubject);
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    if claims.is_expired(now) {
        return Err(JwtError::Expired);
    }

    Ok(claims)
}

/// Verify a bearer token as JWT and produce an AuthResult.
pub fn verify_jwt_bearer(token: &str, secret: &str) -> Result<AuthResult, JwtError> {
    let claims = decode_jwt(token, secret)?;
    let session_id = claims
        .session_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or_else(Uuid::new_v4);

    Ok(AuthResult::new(
        AuthMethod::Token(token.to_string()),
        session_id,
        claims.permissions,
    ))
}

fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}

fn base64_url_decode(s: &str) -> Result<Vec<u8>, JwtError> {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| JwtError::InvalidPayload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_round_trip() {
        let claims = FigJwtClaims::new(
            "trader-1",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600,
            vec!["orders:read".into(), "orders:write".into()],
        );
        let secret = "fig-jwt-secret-key";
        let token = encode_jwt(&claims, secret).unwrap();
        let decoded = decode_jwt(&token, secret).unwrap();
        assert_eq!(decoded.sub, "trader-1");
        assert_eq!(decoded.permissions, claims.permissions);
    }

    #[test]
    fn test_jwt_expired() {
        let claims = FigJwtClaims::new("user", 1, vec![]);
        let token = encode_jwt(&claims, "secret").unwrap();
        assert_eq!(decode_jwt(&token, "secret"), Err(JwtError::Expired));
    }

    #[test]
    fn test_jwt_wrong_secret() {
        let claims = FigJwtClaims::new(
            "user",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600,
            vec![],
        );
        let token = encode_jwt(&claims, "secret-a").unwrap();
        assert_eq!(
            decode_jwt(&token, "secret-b"),
            Err(JwtError::SignatureMismatch)
        );
    }

    #[test]
    fn test_verify_jwt_bearer() {
        let claims = FigJwtClaims::new(
            "service",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600,
            vec!["marketdata:read".into()],
        );
        let token = encode_jwt(&claims, "key").unwrap();
        let result = verify_jwt_bearer(&token, "key").unwrap();
        assert!(result.has_permission("marketdata:read"));
    }
}
