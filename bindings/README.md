# FIG language bindings

Thin wrappers over the stable C ABI in `crates/fig-ffi/include/fig.h`.
Build `fig-ffi` first (`cargo build -p fig-ffi`) and link `libfig_ffi`.

## Layout

| Path | Language | Notes |
|------|----------|-------|
| `go/fig` | Go | cgo wrapper |
| `cpp/include/fig` | C++ | header-only RAII helpers |
| `csharp/Fig` | C# | P/Invoke over `fig.h` |
| `typescript/` | TypeScript | planned — N-API / Deno FFI over `fig.h` (Node, Bun, Deno; no WASM) |

Conformance: run `cargo test -p fig-python binding_conformance` (reference binding).
