# FIG Security Audit Checklist (1.0)

Internal security review checklist. Not a third-party audit report — tracked
readiness for FIG 1.0; sign-off recorded 2026-07-29. See [API.md](API.md) for
module references.

## Transport

- [x] TLS 1.3 mandatory on TREE
- [x] mTLS support for production
- [x] Certificate rotation (`RotatingServerCerts`)
- [ ] External penetration test of TREE listener
- [x] 0-RTT replay protection (`fig_core::replay`, SPEC §10.1)

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
- [x] Load test with sustained frame flood (`fig-load` smoke)
- [x] Fuzz frame decoder (`cargo fuzz run frame_decode` in CI — see CONTRIBUTING)

## Data

- [x] Session TTL and expiry
- [x] Reserved header field validation
- [ ] Secret rotation runbook for JWT signing keys

## Sign-off

| Role | Name | Date | Status |
|---|---|---|---|
| Engineering | Macho0x | 2026-07-29 | Approved |
| Security | Macho0x | 2026-07-29 | Approved |
