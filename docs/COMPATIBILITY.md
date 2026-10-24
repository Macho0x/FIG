# FIG compatibility policy

This is the contract integrators should rely on. [SPEC.md](../SPEC.md) is the
normative wire document. Crate versions, git tags, and FSL catalogs are
**separate series**. Do not treat them as one number.

There are **no published crates.io or npm artifacts**. Build from source or
vendor the binaries from a git tag ([PUBLISHING.md](PUBLISHING.md)). Publishing
those registries is deferred until a release is willing to freeze the Rust crate
API; that is not this policy.

---

## Three version series

| Series | Where | What it versions | When it moves |
|---|---|---|---|
| **Protocol** | [SPEC.md](../SPEC.md) header (`1.0.0`, *Stable*) | 16-byte frame, TREE, ALPN `fig/1`, extension tags, existing field numbers | **Only on a wire break** |
| **Crate** | `[workspace.package] version` in `Cargo.toml` (today `1.4.x`) | Rust library API (`fig-core`, `fig-client`, …) and the C ABI shipped with that crate | Every library release. **Minor versions may break Rust APIs.** |
| **Git tag** | `v1.0.x` from `.github/workflows/release.yml` | A GitHub Release of `fig-exchange-sim`, `fig-gateway`, `fig-cli` | Every push to `main` that does not contain `[skip release]`. **Not the protocol version.** |

`fig_version()` (C ABI) returns `CARGO_PKG_VERSION` — the crate series, not SPEC.

Examples that are easy to mix up, and are not the same thing:

- SPEC `1.0.0` + crate `1.4.11` + git tag `v1.0.25` can all describe one commit.
- A docs-only crate bump still cuts the next `v1.0.x` tag unless the commit
  message contains `[skip release]`.

---

## Protocol and FSL

- SPEC `1.0.0` (*Stable*) is the native FIG wire: framing, TREE, channels,
  sessions, and the field numbers already assigned. It changes only on a
  **wire break** (removed/renamed fields, reused `@N` numbers, reordered SBE
  enums, a different frame header).
- The **schema catalog is additive**. New FSL messages, optional fields, and
  new `CHANNEL_PATH` values are allowed under SPEC `1.0.0`. They do not bump
  the protocol version. Gateways and clients that do not understand a new
  message must ignore it (unknown schema / unknown path), not fail the
  connection.
- FSL lives in [`schemas/*.fsl`](../schemas/). [ADR 0004](adr/0004-fsl-single-source-of-truth.md)
  is the evolution rule: append enum variants; never reuse field numbers.
- OUCH and ITCH are **gateway listeners**, not the FIG wire
  ([ADR 0008](adr/0008-named-gateway-adapters.md),
  [ADR 0009](adr/0009-market-data-plane.md)).

---

## Multi-language contract: `fig.h`

Non-Rust clients bind [`crates/fig-ffi/include/fig.h`](../crates/fig-ffi/include/fig.h)
over `libfig_ffi`. That C ABI is the multi-language contract:

- Connect, request (EOF), live subscribe (`fig_client_subscribe` /
  `sub_next` / `sub_close`), JWT helpers, SBE helpers exposed on the ABI,
  and the market-state helpers (`request_session_status`,
  `request_security_status`, `mass_cancel`).
- Language wrappers (Go, C++, C#, TypeScript/Bun, Java, OCaml, Zig, Python)
  must not invent a second wire. If `fig.h` does not expose a call, neither
  does that language's production path.

Rust `fig-client` may grow typed helpers ahead of `fig.h`. That is a crate
API change, not a protocol change. Wrappers catch up by wrapping new `fig.h`
entry points, not by reimplementing TREE.

---

## Rust crate API

Crate **minor** versions (`1.4` → `1.5`) **may break** Rust APIs (`fig-core`,
`fig-client`, `fig-gateways`, …). Patch versions (`1.4.6` → `1.4.7`) are for
docs, additive helpers, and fixes that keep existing Rust call sites compiling.

Security fixes land on the **latest crate minor** only. Older minors are not
maintained ([SECURITY.md](../SECURITY.md)).

---

## What is not supported

| Not supported | What to use instead |
|---|---|
| Older crate minors | Latest `1.x` line on `main` |
| Treating git `v1.0.x` as SPEC | Read [SPEC.md](../SPEC.md); the tag is a release of binaries |
| crates.io / npm as the install path | Build from source / vendor CI artifacts ([PUBLISHING.md](PUBLISHING.md)) |
| Localhost TLS bypass in production (`FIG_TLS_INSECURE=1`, `*_insecure()`) | `FIG_TLS_CA` or WebPKI. `FIG_PRODUCTION=1` refuses `FIG_DEV_OPEN` |
| Generated per-language SBE as the production codec | `fig-ffi` / `fig-python` CBOR+SBE on the C ABI. Generated SBE is CI-parity-checked and useful for colo encode/decode; it is not the supported multi-language runtime |
| Native TREE (QUIC) reimplemented per language | FFI over `fig.h` ([ADR 0005](adr/0005-multi-language-runtime-strategy.md)) |
| Exchange-named protocol forks (`fig-binance`, NASDAQ-byte-identical OUCH) | One FSL catalog + named gateway adapters |

---

## Recommended pin for an integrator

1. Pin a **git commit or `v1.0.x` tag** for the binaries you run.
2. Read SPEC `1.0.0` for the wire. New catalog messages are additive.
3. For non-Rust: build `fig-ffi` from that tag and compile against that
   `fig.h`. Do not mix a newer header with an older `libfig_ffi`.
4. For Rust: pin the workspace crate version from that same tag, and expect
   API churn on the next crate minor.
