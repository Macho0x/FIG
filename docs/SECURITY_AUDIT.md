# FIG Security Audit Checklist (Pre-1.0)

Internal security review checklist before declaring FIG 1.0. Not a third-party
audit report — use this to track readiness. See [API.md](API.md) for module
references.

## Transport

- [x] TLS 1.3 mandatory on TREE
- [x] mTLS support for production
- [x] Certificate rotation (`RotatingServerCerts`)
- [ ] External penetration test of TREE listener
- [ ] 0-RTT replay protection (documented TODO in transport)

## Authentication

FIG specifies **wire auth** (`AUTH_TOKEN`, path scoping, mTLS). Credential
issuance (API keys, JWT signing services, admin UI) is venue infrastructure —
outside this checklist.

- [x] Constant-time token comparison
- [x] JWT HS256 validation (reference; production venues use RS256 + KMS)
- [x] OAuth2/OIDC dev validator stub (production: wire real introspection)
- [x] Per-channel auth policy

## DoS / Abuse

- [x] Per-channel rate limiting (`ChannelRateLimiter`)
- [x] Connection-level DoS guard (`DoSGuard`, `FloodDetector`)
- [ ] Load test with sustained frame flood
- [ ] Fuzz frame decoder (`cargo fuzz` — recommended)

## Data

- [x] Session TTL and expiry
- [x] Reserved header field validation
- [ ] Secret rotation runbook for JWT signing keys

## Sign-off

| Role | Name | Date | Status |
|---|---|---|---|
| Engineering | — | — | Pending |
| Security | — | — | Pending |
