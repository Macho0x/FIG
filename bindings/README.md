# FIG language bindings

Thin wrappers over the stable C ABI in `crates/fig-ffi/include/fig.h`.
Build `fig-ffi` first (`cargo build -p fig-ffi`) and link `libfig_ffi`.

## Layout

| Path | Language | Notes |
|------|----------|-------|
| `go/fig` | Go | cgo — Tier 1–4 client + JWT/SBE helpers + generated `sbe_generated.go` |
| `cpp/include/fig` | C++ | RAII `fig::Client` + JWT/SBE + generated `sbe_generated.hpp` |
| `cpp/pure/fig_protocol.hpp` | C++ | Pure protocol library (frame/ext/channel, no Rust runtime) |
| `csharp/Fig` | C# | P/Invoke `FigClient` + JWT/SBE + `SbeGenerated.cs` |
| `typescript/fig.ts` | TypeScript | `node:ffi` — connect, request, subscribe, JWT, SBE |
| `typescript/sbe_generated.ts` | TypeScript | FSL-generated SBE serializers |
| `ocaml/fig.ml` | OCaml | ctypes over `fig.h` + JWT/SBE FFI |
| `zig/fig.zig` | Zig | `@cImport` + JWT/SBE helpers |
| `zig/pure/protocol.zig` | Zig | Pure protocol library (frame/ext/channel) |
| `zig/sbe_generated.zig` | Zig | FSL-generated SBE serializers |
| `java/` | Java | JNI (`native/fig_jni.c`) + JWT/SBE on `FigNative.java` |

## FSL codegen

```bash
cargo run -p xtask -- codegen   # refreshes Rust + per-language SBE + pure protocol
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang sbe-go --out /tmp/out
# Also: sbe-cpp, sbe-csharp, sbe-typescript, sbe-zig, protocol-cpp, protocol-zig
# Types: go, typescript, ocaml, zig, java, python, cpp, csharp
```

All type targets emit typed enums, nested structs, type aliases, message metadata constants, and CBOR field manifests. SBE targets emit wire-compatible encoders/decoders (`schema_id=0x01`).

## JWT (dev / conformance)

All bindings expose HS256 JWT helpers via FFI (`fig_jwt_encode`, `fig_jwt_verify_bearer`) for dev `AuthToken` on wire. Production venues verify RS256 JWTs from their identity service.

## Conformance

```bash
cargo build -p fig-ffi
cargo test -p fig-ffi binding_conformance
cargo test -p fig-ffi client_integration
cargo test -p fig-ffi --test advanced
cargo test -p fig-ffi --test jwt
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo test -p fig-python binding_conformance
cargo run -p xtask -- codegen --check
```

**Stream merge (CBOR payload → local state):** `fig_order_book_*`, `fig_mids_*`,
`fig_bbo_*`, `fig_trade_tape_*`, `fig_mark_price_*`, `fig_orders_*` — see
`crates/fig-ffi/include/fig.h`. Python: `OrderBookState`, `MidsState`, `BboState`,
`TradeTape`, `MarkPriceState`, `OrdersState` in `fig-python`.

Both `fig-ffi` and `fig-python` run shared §16.1 vectors from `tests/conformance/vectors/v1.json` (including CandleBar CBOR/SBE).

**Scope:** FFI wire codecs (encode/decode, client connect) are conformance-tested in CI. Per-language generated SBE (`sbe_generated.*`) is compile-smoke only until hex parity lands — prefer FFI for production wire paths.

Go smoke: `cargo build -p fig-ffi && cd bindings/go/fig && go test -c -tags=conformance`.
