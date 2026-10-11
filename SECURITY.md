# Security Policy

FIG is a wire protocol for trading systems: this document covers the FIG
repository (protocol stack, libraries, gateways, and bindings). A venue running
FIG owns its own credentials, matching, and compliance — see
[docs/SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md) for the review checklist and
[SPEC.md §10](SPEC.md) for the normative security model.

## Reporting a vulnerability

**Please do not open a public issue for security problems.**

Use GitHub's private reporting flow — *Security* → *Report a vulnerability* on
this repository — or email the maintainers if you cannot use it. Include:

- affected component (`fig-core`, a gateway, a binding, the spec) and version;
- a reproducer (server config, crafted frames, and the observed behavior);
- impact: what an attacker gains (e.g. session forgery, panic/DoS, auth bypass);
- whether the issue is reachable in a default deployment.

We aim to acknowledge reports within **3 business days** and to ship a fix or a
documented mitigation for confirmed issues before public disclosure. Please give
us a reasonable window to release a fix before publishing details.

## Supported versions

Protocol version remains [SPEC.md](SPEC.md) 1.0.0 (*Stable*) and changes only
on a wire break. The schema catalog is additive. Rust library APIs may still
change between minor releases. Security fixes land on the latest minor release
line; older minor versions are not maintained. There are no published crates.io
artifacts yet — fixes are delivered as repository commits.

| Version | Supported |
|---|---|
| Latest minor (1.1.x and newer) | ✅ |
| Older minors | ❌ |

## What is in scope

| In scope | Notes |
|---|---|
| Frame/TLV decoding, fragmentation, SBE/CBOR codecs | Panics, OOB reads, unbounded allocation on hostile input |
| TREE transport configuration | TLS verification, 0-RTT handling, replay |
| Session resumption tokens | Forgery, replay, cross-node acceptance |
| Wire auth (`AUTH_TOKEN`, JWT, path scoping, mTLS) | Bypass, timing, algorithm confusion |
| Gateways (FIX/REST/WebSocket/OUCH/ITCH) | Parsing of untrusted legacy input, auth translation |
| C ABI (`fig-ffi`) and bindings | Memory safety, ownership across the boundary |

## What is out of scope

- **Credential issuance** — API keys, JWT signing services, KMS policy, and
  admin tooling are venue responsibilities (FIG only verifies).
- **Venue business logic** — matching, risk, funding, liquidations, compliance.
- Findings requiring a modified or local build with security controls disabled
  (`FIG_TLS_INSECURE=1`, `FIG_DEV_OPEN=1`, or the `*_insecure()` APIs). These are
  documented development-only switches; a report that they disable a control is
  not a vulnerability, but *reaching* them in the default configuration is.
- Denial of service via resource exhaustion on the reference simulator
  (`fig-exchange-sim`), which is not production infrastructure.

## Hardening already in place

Documented per item in [docs/SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md):

- TLS 1.3 with server-certificate verification **on by default**; private-PKI
  pinning via `FIG_TLS_CA`; CA-verified mTLS (`FIG_MTLS_CA`).
- Server-signed (HMAC-SHA256) session resumption tokens, verified before any
  state is restored; replays rejected inside the window, cluster-wide with Redis.
- Bounds-checked frame, SBE, FIX, and TLV decoding, with `frame_decode` and
  `sbe_decode` fuzz targets in CI.
- Connection resource limits (`DoSGuard`, `ChannelRateLimiter`) enforced on the
  receive path.
- Constant-time token comparison; JWT `alg` pinned to the expected algorithm.
- Token signing-key rotation without downtime:
  [docs/JWT_KEY_ROTATION.md](docs/JWT_KEY_ROTATION.md).
