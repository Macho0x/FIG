//! Per-channel authentication policy for FIG connections.
//!
//! Spec §6: auth can be scoped to individual channels. Each channel may
//! require specific permissions granted during initial authentication.

use std::collections::HashMap;

use crate::auth::AuthResult;

/// Policy mapping channel IDs to required permissions.
#[derive(Debug, Clone, Default)]
pub struct ChannelAuthPolicy {
    /// Permissions required when no channel-specific rule exists (empty = allow).
    default_required: Vec<String>,
    /// Per-channel permission requirements.
    channel_requirements: HashMap<u16, Vec<String>>,
}

impl ChannelAuthPolicy {
    /// Create a policy with no default requirements (all channels allowed).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set default permissions required for all channels without explicit rules.
    pub fn with_default_required(mut self, permissions: Vec<String>) -> Self {
        self.default_required = permissions;
        self
    }

    /// Require specific permissions for a channel.
    pub fn require_for_channel(mut self, channel_id: u16, permissions: Vec<String>) -> Self {
        self.channel_requirements.insert(channel_id, permissions);
        self
    }

    /// Authorize access to a channel for an authenticated principal.
    ///
    /// Returns true if the principal holds all required permissions for the
    /// channel (or if no permissions are required).
    pub fn authorize(&self, channel_id: u16, auth: &AuthResult) -> bool {
        let required = self
            .channel_requirements
            .get(&channel_id)
            .unwrap_or(&self.default_required);

        if required.is_empty() {
            return true;
        }

        required.iter().all(|p| auth.has_permission(p))
    }

    /// Returns the permissions required for a channel, if any.
    pub fn requirements_for(&self, channel_id: u16) -> &[String] {
        self.channel_requirements
            .get(&channel_id)
            .map(|v| v.as_slice())
            .unwrap_or(self.default_required.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{AuthMethod, AuthResult};
    use uuid::Uuid;

    fn auth_with(perms: Vec<&str>) -> AuthResult {
        AuthResult::new(
            AuthMethod::Token("key".into()),
            Uuid::new_v4(),
            perms.into_iter().map(String::from).collect(),
        )
    }

    #[test]
    fn test_default_allow_all() {
        let policy = ChannelAuthPolicy::new();
        let auth = auth_with(vec![]);
        assert!(policy.authorize(1, &auth));
    }

    #[test]
    fn test_channel_specific_requirements() {
        let policy = ChannelAuthPolicy::new().require_for_channel(
            5,
            vec!["marketdata:read".into()],
        );

        let allowed = auth_with(vec!["marketdata:read"]);
        let denied = auth_with(vec!["orders:write"]);

        assert!(policy.authorize(5, &allowed));
        assert!(!policy.authorize(5, &denied));
        assert!(policy.authorize(99, &denied)); // no rule for ch 99
    }

    #[test]
    fn test_default_required_permissions() {
        let policy = ChannelAuthPolicy::new()
            .with_default_required(vec!["session:active".into()])
            .require_for_channel(2, vec!["admin".into()]);

        let session_auth = auth_with(vec!["session:active"]);
        let admin_auth = auth_with(vec!["session:active", "admin"]);

        assert!(policy.authorize(1, &session_auth));
        assert!(!policy.authorize(2, &session_auth));
        assert!(policy.authorize(2, &admin_auth));
    }
}
