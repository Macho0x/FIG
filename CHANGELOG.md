# Changelog

All notable changes to the FIG repository.
## 1.3.0

### Added
- **Native per-language SBE codecs (Go, C++, C#, TypeScript, Zig).** `cargo xtask codegen`
  now emits working encoders/decoders for all five targets; each compiles in CI and is
  checked byte-for-byte against `tests/conformance/vectors/v1.json` by its own smoke
  (`bindings/{go,cpp,csharp,typescript,zig}`). Previously the generated code did not compile
  (or, for Go, matched only the simplest message).

### Changed
- **SBE list-of-scalar payloads are now on the wire.** `CapabilitiesResponse`'s
  `list<uint8>`, `list<Symbol>` and `list<CandleInterval>` previously wrote a count and dropped
  the items. Lists now write `u32 count`, then per item `u32 len + body` for variable-length
  items and raw big-endian bytes for fixed-width scalars — in the Rust reference codec and in
  every generated target. A `sbe.capabilities_response.demo` conformance vector pins the layout.
- Zig generated encoders take list parameters as `[]const T` (previously `[]T`, which rejected
  const array literals at call sites).

### Fixed
- Generated C# now compiles: parameters are declared type-first, `Schema.SCHEMA_ID` and the
  `{Enum}Helpers` classes are no longer emitted at namespace level, nullable-reference optionals
  test `!= null` instead of `.HasValue`, and `Wire.TakeU8` is qualified.
- Generated Zig enums carry their 1-based wire values instead of defaulting to zero.
 The wire format is versioned in
[SPEC.md](SPEC.md); this file tracks the libraries and tooling.

## 1.2.2

### Fixed
- **TypeScript binding:** `FigClient.ping()` always threw
  `fig_client_ping failed: undefined` because the `bun:ffi` call was declared with
  a `void` return while the C ABI returns `int32_t`. Verified against the simulator.
- **SPEC §6.1/§15** now list `AUTH_METHOD` (`0x001E`), matching
  `fig_core::ext`. §11.1's example (symbol length, `ExecutionReport` field numbers)
  was aligned with `schemas/orders.fsl` and marked abridged.
- **Conformance vectors:** the CandleBar and BalanceSnapshot SBE descriptions named
  the wrong template ids (20 and 33 are what the encoders emit).
- `fig_core::sbe::encode_balance_snapshot` doc comment said template 31;
  `ext.rs` and `durable_session.rs` doc comments stated tag ranges / session
  backends that do not match the code.
- **Docs:** Python binding class names and usage (module `fig`, `Py*` state
  classes, frame-list returns, `FigPySubscription.snapshot` property,
  `next()` raising on timeout), Python install via `maturin`, tutorial MSRV
  (1.75 → 1.97), `FIG_SESSION_STORE`/`FIG_SESSION_PATH`/`FIG_SERVER` documented,
  the non-existent `memory` session backend and `redis` `fix-seq-store` removed
  from DEPLOYMENT, ftlc `--lang` lists completed, `sbe_decode` fuzz target added
  to CONTRIBUTING.
- **TODO.md** counts and markers reconciled with the code (test counts, SPEC line
  count, 30 extension tags, template ids, historical §18.x rows).

### Changed
- The generated Go SBE target is excluded from Go builds (`//go:build ignore`):
  it does not compile yet (incomplete field coverage), so a plain
  `go build ./...` in `bindings/go/fig` previously failed. It is now documented as
  experimental in `bindings/README.md`, `docs/PUBLISHING.md` and TODO §16.2.

### Added
- CI: Python module import smoke (`import fig; fig.version()`), and
  `go build ./... && go vet ./... && go test -tags=conformance` for the Go wrapper.

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
