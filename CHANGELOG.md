# Changelog

All notable changes to the FIG repository. The wire format is versioned in
[SPEC.md](SPEC.md); this file tracks the libraries and tooling.

## 1.2.1

### Fixed
- CI: `fig-python`'s live-subscribe test configured TLS for itself instead of
  depending on an environment variable set by another CI job, so it passes under
  `cargo llvm-cov --workspace` (and for anyone running the crate's tests locally).
- `DoSGuard::channel_closed` no longer uses `Atomic::fetch_update`, which is
  deprecated on current stable toolchains; the saturating decrement is now a
  compare-exchange loop that keeps the declared MSRV working.

### Changed
- CI no longer exports `FIG_TLS_INSECURE` globally, so a test that depends on
  ambient TLS configuration fails loudly instead of passing by accident.

## 1.2.0

### Added
- `fig-exchange-sim --print-ca <path>`: exports the development certificate so
  clients can pin it with `FIG_TLS_CA` instead of disabling TLS verification.
- `fig_core::transport::certificate_to_pem` for exporting certificates as PEM.
- `SECURITY.md` (vulnerability reporting, scope), `CHANGELOG.md`, and the
  `LICENSE-MIT` / `LICENSE-APACHE` texts backing the advertised dual license.
- README rewritten as a landing page (what/why/run within one screen).

## 1.1.0

### Added
- `fig_core::session_token::SessionSigner`: server-issued, HMAC-SHA256 signed
  session resumption tokens; servers verify before restoring state and prefer
  their own session store over the token payload.
- `fig_core::jwt_keys`: RS256 verification from a JWKS document or PEM key,
  `kid`-based rotation, and `JwtVerifier`/`FnVerifier` adapters for KMS/HSM.
  Rotation runbook: [docs/JWT_KEY_ROTATION.md](docs/JWT_KEY_ROTATION.md).
- `fig_core::replay::RedisReplayCache` and `FIG_REPLAY_REDIS_URL`: shared,
  cluster-wide 0-RTT replay rejection.
- TLS: `client_config` verifies the server by default (WebPKI); private-PKI
  pinning (`FIG_TLS_CA`, `client_config_from_ca_pem`);
  `server_config_mtls_with_ca` performs real client-certificate verification;
  TREE 0-RTT early data enabled.
- `FrameDecoder` maximum frame size, and `DoSGuard`/`ChannelRateLimiter` are now
  enforced on the connection receive path.
- CI: SBE fuzz target and a real-Redis replay-cache test.

### Fixed
- **Session forgery:** resumption tokens were unauthenticated CBOR, letting a
  peer restore an attacker-chosen session identity.
- **MITM:** every shipped client skipped server-certificate verification.
- **Client auth bypass:** mTLS accepted any client certificate.
- **Remote panic:** SBE decoders indexed attacker-controlled payloads (also
  capped list allocations that could drive unbounded memory use).
- **Pre-auth buffering:** a 4-byte length prefix could make a server buffer
  gigabytes per stream.
- **Timing:** `verify_token` and the JWT MAC comparison were not constant-time;
  the JWT `alg` header is now enforced to `HS256`.
- **FIX gateway panic** on non-ASCII timestamp fields.
- Extension TLV encoding silently truncated values larger than 64 KiB;
  `REDIRECT_TOKEN` / `IDEMPOTENCY_KEY` were decoded as 16-byte values and
  truncated.
- Seed/extraction race in a benchmark environment test.

### Changed
- Dev-only switches are explicit and refuse to engage in production:
  `FIG_PRODUCTION=1` disables `FIG_DEV_OPEN`, and insecure TLS verifiers are
  named `*_insecure()`.
- README, SPEC §10.1, DEPLOYMENT, SECURITY_AUDIT, and TODO updated.

## 1.0.0

- First stable release. SPEC `1.0.0` (`Status: Stable`): frames, channels,
  sessions, TREE transport, FSL schemas, gateways, and the conformance suite.
- Rust core (§1–15), `fig-client`, `fig-ffi` + Python and language bindings,
  `fig-gateways` (FIX/REST/WebSocket), and the `fig-exchange-sim` reference venue.
