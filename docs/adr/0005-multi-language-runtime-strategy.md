# ADR 0005: Multi-Language Runtime Strategy

## Status

Accepted

## Context

Section 16 of [TODO.md](../../TODO.md) requires non-Rust SDKs to reach FIG SPEC parity.
Three runtime strategies were considered (Track B in §16):

1. **Rust core + FFI bindings** (PyO3, P/Invoke, cgo, ctypes, `@cImport`)
2. **Pure generated protocol libraries** per language
3. **Gateway proxy** for operational parity without native FIG clients

Implementing eight independent copies of `fig-core` (~20 modules) guarantees wire drift
without [§16.1 conformance vectors](../../tests/conformance/README.md).

## Decision

### Default path: FFI-first (Track B option 1)

- Add `fig-ffi` with a stable C ABI (`fig.h`) over `fig-core`.
- Ship **`fig-python`** first as the reference binding (PyO3 + maturin).
- Add C# / Go / C++ bindings as thin wrappers over the same ABI.
- Tier rollout: Tier 1–2 (frames, CBOR, STREAM_OPEN, auth) before Tier 4 advanced features.

### Exceptions: pure-generated protocol (Track B option 2)

Use generated frame/channel/control libraries **only** where FFI is unacceptable:

| Language | Pure-generated exception | Rationale |
|---|---|---|
| **C++** | Optional HFT path | Latency-sensitive desks may forbid Rust runtime in-process |
| **Zig** | Optional comptime path | `@cImport` preferred; pure Zig codec as fallback |

Python, C#, Go, OCaml, and TypeScript **do not** get hand-ported `fig-core` clones in v1.

### TypeScript (Node, Bun, Deno)

Trading bots and desk tools target **server-side** runtimes only:

- **Native FIG client:** N-API addon (Node + Bun) or Deno FFI over `libfig_ffi` / `fig.h`.
- **No WASM** — we do not compile `fig-core` to `wasm32` for TS clients.
- **Browser:** use `fig-gateway` (REST/WebSocket) for operational parity; not Tier 4 native FIG.

This matches the FFI-first default: one Rust runtime, thin TS bindings, conformance vectors
for parity.

### Gateway proxy (Track B option 3)

- `fig-gateway --fig-backend` forwards translated FIX/REST to a native FIG server.
- Browser and migration scenarios use the gateway; it is **not** counted as Tier 4 native parity.
- TypeScript in the browser is gateway-only; server-side TS uses native addon/FFI (see above).

## Consequences

### Positive

- One behavioral reference (`fig-core`) behind all bindings.
- Conformance vectors prove parity once; bindings re-run the same suite.
- Faster time-to-MVP for Python and desk integrations.

### Negative

- FFI surface must be versioned carefully (`fig.h` semver).
- Pure C++/Zig clients that reject FFI carry duplicate maintenance for transport.
- PyO3/native addons complicate cross-platform CI (Linux first; macOS/Windows follow).

## Implementation order

1. §16.1 conformance vectors (done in `fig-conformance`)
2. ADR codegen transition (`cargo xtask codegen`)
3. §16.2 FSL enum/nested codegen parity
4. `fig-ffi` + `fig-python` Tier 1–2
5. §16.5 multi-codec exchange-sim + gateway backend proxy

## References

- [ADR 0004 — FSL single source of truth](0004-fsl-single-source-of-truth.md)
- [TODO.md §16](../../TODO.md)
- [tests/conformance/README.md](../../tests/conformance/README.md)
