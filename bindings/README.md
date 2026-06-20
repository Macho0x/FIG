# FIG language bindings

Thin wrappers over the stable C ABI in `crates/fig-ffi/include/fig.h`.
Build `fig-ffi` first (`cargo build -p fig-ffi`) and link `libfig_ffi`.

## Layout

| Path | Language | Notes |
|------|----------|-------|
| `go/fig` | Go | cgo — Tier 1–4 encode + `Client` connect/request/ping |
| `cpp/include/fig` | C++ | RAII `fig::Client` + auth encode |
| `csharp/Fig` | C# | P/Invoke `FigClient` |
| `typescript/fig.ts` | TypeScript | `node:ffi` — connect, request, subscribe, stream decode |
| `ocaml/fig.ml` | OCaml | ctypes over `fig.h` |
| `zig/fig.zig` | Zig | `@cImport` + helpers |
| `java/` | Java | JNI (`native/fig_jni.c`) + `FigNative.java` |

## FSL codegen

```bash
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang go --out /tmp/out
# Also: typescript, ocaml, zig, java, python, cpp, csharp
```

All targets emit typed enums, nested structs, type aliases, message metadata constants, and CBOR field manifests.

## Conformance

```bash
cargo build -p fig-ffi
cargo test -p fig-ffi binding_conformance
cargo test -p fig-ffi client_integration
cargo test -p fig-ffi --test advanced
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo test -p fig-python binding_conformance
```

Both `fig-ffi` and `fig-python` run shared §16.1 vectors from `tests/conformance/vectors/v1.json`.
