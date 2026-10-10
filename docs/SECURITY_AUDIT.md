# FIG Security Audit Checklist

Internal security review checklist. **Not a third-party audit report** — tracked
readiness for the FIG protocol stack. Integration-1.0 sign-off was recorded
2026-07-29; a hardening review followed and is recorded below as 1.1.
See [API.md](API.md) for module references and [SPEC.md §10](../SPEC.md) for the
normative security model.

## Transport

- [x] TLS 1.3 mandatory on TREE
- [x] Server certificate verification **on by default** (`client_config`; WebPKI roots)
- [x] Private-PKI pinning (`client_config_from_ca_pem`, `FIG_TLS_CA`)
- [x] mTLS with real client-CA verification (`server_config_mtls_with_ca`, `FIG_MTLS_CA`)
- [x] Certificate rotation (`RotatingServerCerts`)
- [x] TREE 0-RTT early data enabled (`enable_early_data` / `max_early_data_size`)
- [x] Insecure verifiers isolated to explicitly named `*_insecure` functions (dev/test only)
- [ ] External penetration test of the TREE listener

## Authentication

FIG specifies **wire auth** (`AUTH_TOKEN`, path scoping, mTLS). Credential
issuance (API keys, JWT signing services, admin UI) is venue infrastructure —
outside this checklist.

- [x] Constant-time token comparison (`verify_token` hashes to fixed width and compares with `subtle::ConstantTimeEq`)
- [x] JWT HS256 validation: constant-time MAC compare, `alg` header enforced to `HS256` (reference; production venues use RS256 + KMS)
- [x] OAuth2/OIDC dev validator stub (production: wire real introspection)
- [x] Per-channel auth policy
- [x] Dev bypass (`FIG_DEV_OPEN`) refused when `FIG_PRODUCTION=1`

## Session resumption

- [x] Resumption tokens are **server-issued and HMAC-SHA256 signed** (`fig_core::session_token::SessionSigner`)
- [x] Servers verify the signature before restoring any state; unsigned/forged/tampered tokens are rejected
- [x] Restored state prefers the server's authoritative session store over token payload
- [x] Client-supplied `SESSION_ID` extensions are never used to select a session
- [x] Replay window enforced (`fig_core::replay`, SPEC §10.1)
- [x] Shared Redis replay cache for multi-node HA (`FIG_REPLAY_REDIS_URL`)

## Input handling

- [x] Frame decoder bounds declared length against `MAX_FRAME_SIZE` before buffering
- [x] SBE decoders bounds-check every read; list allocations capped by remaining buffer
- [x] FIX gateway timestamp parsing is ASCII-validated (no panics on hostile fields)
- [x] Extension TLV encoding rejects values > `u16::MAX` (no silent truncation)
- [x] Variable-length binary extensions (`REDIRECT_TOKEN`, `IDEMPOTENCY_KEY`) decode without truncation
- [x] Fuzz frame decoder (`cargo fuzz run frame_decode`) and SBE decoder (`cargo fuzz run sbe_decode`) in CI

## DoS / Abuse

- [x] Per-channel rate limiting (`ChannelRateLimiter`) **wired into the receive path**
- [x] Connection-level guard (`DoSGuard`: max channels, frame size, message rate) **wired into the receive path**
- [x] Load test with sustained frame flood (`fig-load` smoke)
- [x] Fragmentation/reassembly bounds

## Data

- [x] Session TTL and expiry
- [x] Reserved header field validation
- [x] Secret rotation guidance for the resumption key ([DEPLOYMENT.md](DEPLOYMENT.md))
- [x] JWT signing-key rotation runbook ([JWT_KEY_ROTATION.md](JWT_KEY_ROTATION.md))
- [x] RS256 verification from a JWKS/PEM key set with `kid`-based rotation (`fig_core::jwt_keys`)
- [x] KMS/HSM integration point (`JwtVerifier` trait + `FnVerifier`)

## Sign-off

| Review | Scope | Date | Status |
|---|---|---|---|
| Security hardening review | Resumption auth, TLS verification, untrusted-input bounds, DoS wiring | 2026-10-10 | Approved |
| 1.0 integration sign-off | Protocol stack | 2026-07-29 | Approved |

Remaining unchecked items are external (third-party penetration test) or
venue-owned (credential-issuance runbooks); neither blocks FIG stack deployment
behind a venue's own PKI and credential service.
