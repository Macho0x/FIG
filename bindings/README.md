# FIG language bindings

Thin wrappers over the stable C ABI in `crates/fig-ffi/include/fig.h`.
Build `fig-ffi` first (`cargo build -p fig-ffi`) and link `libfig_ffi`.

## Layout

| Path | Language | Notes |
|------|----------|-------|
| `go/fig` | Go | cgo — encode + `Client` connect/request/ping |
| `cpp/include/fig` | C++ | header-only RAII — encode + `fig::Client` |
| `csharp/Fig` | C# | P/Invoke — encode + `FigClient` |
| `typescript/fig.ts` | TypeScript | `node:ffi` over `fig.h` (Node, Bun, Deno; no WASM) |

## Conformance

```bash
cargo build -p fig-ffi
cargo test -p fig-ffi binding_conformance
cargo test -p fig-ffi client_integration
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo test -p fig-python binding_conformance
```

Both `fig-ffi` and `fig-python` run shared §16.1 vectors from `tests/conformance/vectors/v1.json`.
