# TypeScript / JavaScript bindings

Server-side trading bots on **Node**, **Bun**, and **Deno** — native FIG over TREE,
not WASM.

## Usage

Build the shared library, then import the thin FFI wrapper:

```bash
cargo build -p fig-ffi
export FIG_FFI_LIB=$PWD/target/debug/libfig_ffi.so   # or .dylib / .dll
node --experimental-default-type=module -e "import { version } from './bindings/typescript/fig.ts'; console.log(version())"
```

See [`fig.ts`](fig.ts) for `FigClient` (connect, ping) over `node:ffi`.

## Approach

- Link [`fig-ffi`](../../crates/fig-ffi/) (`libfig_ffi` + `fig.h`) from a thin package.
- **Node / Bun:** `node:ffi` (or N-API addon for production packaging).
- **Deno:** FFI to `fig.h` where supported.
- **FSL:** `ftlc --lang typescript` for message types; wire runtime stays in Rust.

## Out of scope

- **WASM** — not part of the TS SDK plan.
- **In-browser native FIG** — use [`fig-gateway`](../../crates/fig-gateways/) REST/WebSocket instead.

## Status

🔶 `fig.ts` wrapper landed — connect (incl. 0-RTT token), request, subscribe, stream payload decode, compression. FSL types via `ftlc --lang typescript`. Tracks [TODO.md §16](../../TODO.md).
