# TypeScript / JavaScript bindings (planned)

Server-side trading bots on **Node**, **Bun**, and **Deno** — native FIG over TREE,
not WASM.

## Approach

- Link [`fig-ffi`](../../crates/fig-ffi/) (`libfig_ffi` + `fig.h`) from a thin package.
- **Node / Bun:** N-API addon exposing connect, request, subscribe (mirror `fig-python`).
- **Deno:** FFI to `fig.h` or N-API where supported.
- **FSL:** `ftlc --lang typescript` for message types; wire runtime stays in Rust.

## Out of scope

- **WASM** — not part of the TS SDK plan.
- **In-browser native FIG** — use [`fig-gateway`](../../crates/fig-gateways/) REST/WebSocket instead.

## Status

Not implemented yet. Track in [TODO.md §16.3](../../TODO.md). Decision: [ADR 0005](../docs/adr/0005-multi-language-runtime-strategy.md).
