# TypeScript / JavaScript bindings

Server-side bots on **Bun** via [`bun:ffi`](https://bun.sh/docs/api/ffi) over
[`fig.h`](../../crates/fig-ffi/include/fig.h). Not WASM. Browsers should use
[`fig-gateway`](../../crates/fig-gateways/) REST/WebSocket.

## Quick check (same as CI)

```bash
cargo build -p fig-ffi --release
export FIG_FFI_LIB=$PWD/target/release/libfig_ffi.so   # or .dylib / .dll
bash bindings/typescript/smoke/run.sh
# prints: fig-ts smoke OK <crate-version>
```

Or:

```bash
bun -e "import { version } from './bindings/typescript/fig.ts'; console.log(version())"
```

See [`fig.ts`](fig.ts) for `version()`, `jwtEncode`, and `FigClient` (connect / request).
`dlopen` uses Bun's `args` / `returns` (not Node `node:ffi`).

## Approach

- Link [`fig-ffi`](../../crates/fig-ffi/) (`libfig_ffi` + `fig.h`).
- **Bun:** `bun:ffi`. A Node N-API addon is not in this repo.
- **FSL:** `ftlc --lang typescript` for message types; wire runtime stays in Rust.

## Out of scope

- **WASM**
- **In-browser native FIG** — use the gateway
- **Node `node:ffi`** — that module does not exist; do not import it
