//! OAuth2 / OIDC token validation for enterprise authentication.
//!
//! Validates bearer tokens via configured issuer introspection or static
//! JWKS for development. Production deployments should use full OIDC
//! discovery and RS256 signature verification.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::auth::{AuthMethod, AuthResult};
use uuid::Uuid;

/// OAuth2/OIDC validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum OAuthError {
    #[error("token expired")]
    Expired,
    #[error("invalid token")]
    InvalidToken,
    #[error("issuer mismatch")]
    IssuerMismatch,
    #[error("insufficient scope")]
    InsufficientScope,
}

/// OAuth2 token introspection response (RFC 7662 subset).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OAuthTokenInfo {
    pub active: bool,
    pub sub: String,
    pub iss: String,
    pub exp: u64,
    #[serde(default)]
    pub scope: String,
}

/// OAuth2/OIDC validator configuration.
#[derive(Debug, Clone)]
pub struct OAuthValidator {
    issuer: String,
    /// Static token registry for dev/testing (token -> info).
    tokens: HashMap<String, OAuthTokenInfo>,
}

impl OAuthValidator {
    pub fn new(issuer: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
            tokens: HashMap::new(),
        }
    }

    /// Register a token for dev/testing introspection.
    pub fn register_token(mut self, token: impl Into<String>, info: OAuthTokenInfo) -> Self {
        self.tokens.insert(token.into(), info);
        self
    }

    /// Introspect a bearer token and return auth result.
    pub fn validate(&self, token: &str) -> Result<AuthResult, OAuthError> {
        let info = self
            .tokens
            .get(token)
            .ok_or(OAuthError::InvalidToken)?;

        if !info.active {
            return Err(OAuthError::InvalidToken);
        }
        if info.iss != self.issuer {
            return Err(OAuthError::IssuerMismatch);
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if now >= info.exp {
            return Err(OAuthError::Expired);
        }

        let permissions: Vec<String> = info
            .scope
            .split_whitespace()
            .map(String::from)
            .collect();

        Ok(AuthResult::new(
            AuthMethod::Token(token.to_string()),
            Uuid::new_v4(),
            permissions,
        ))
    }

    /// Convert validated OAuth result to AuthMethod for connection setup.
    pub fn auth_method_from_token(&self, token: &str) -> Result<AuthMethod, OAuthError> {
        self.validate(token)?;
        Ok(AuthMethod::Token(token.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_info() -> OAuthTokenInfo {
        OAuthTokenInfo {
            active: true,
            sub: "user-1".into(),
            iss: "https://auth.example.com".into(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600,
            scope: "orders:read orders:write".into(),
        }
    }

    #[test]
    fn test_oauth_validate_success() {
        let validator = OAuthValidator::new("https://auth.example.com")
            .register_token("access-token-xyz", sample_info());
        let result = validator.validate("access-token-xyz").unwrap();
        assert!(result.has_permission("orders:read"));
        assert!(result.has_permission("orders:write"));
    }

    #[test]
    fn test_oauth_rejects_unknown_token() {
        let validator = OAuthValidator::new("https://auth.example.com");
        assert!(matches!(
            validator.validate("unknown"),
            Err(OAuthError::InvalidToken)
        ));
    }

    #[test]
    fn test_oauth_rejects_expired() {
        let mut info = sample_info();
        info.exp = 1;
        let validator = OAuthValidator::new("https://auth.example.com")
            .register_token("expired", info);
        assert!(matches!(
            validator.validate("expired"),
            Err(OAuthError::Expired)
        ));
    }
}
