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
`fig_bbo_*`, `fig_trade_tape_*`, `fig_mark_price_*`, `fig_orders_*`,
`fig_agg_trades_*`, `fig_funding_*`, `fig_ledger_*`, `fig_liquidation_*` — see
`crates/fig-ffi/include/fig.h`. Python: `OrderBookState`, `MidsState`, `BboState`,
`TradeTape`, `MarkPriceState`, `OrdersState`, `AggTradesState`, `FundingState`,
`LedgerState`, `LiquidationState` in `fig-python`.

## Compile smoke

```bash
cargo build -p fig-ffi
export LD_LIBRARY_PATH="$PWD/target/debug:${LD_LIBRARY_PATH:-}"
cd bindings/go/fig && go test -tags=conformance
cargo build -p fig-ffi --release
bash bindings/cpp/smoke/run.sh
export FIG_REPO_ROOT="$PWD"
bash bindings/csharp/smoke/run.sh
```

**SBE hex parity (FFI encode vs `v1.json`):**

```bash
export LD_LIBRARY_PATH="$PWD/target/release:${LD_LIBRARY_PATH:-}"
export FIG_REPO_ROOT="$PWD"
cd bindings/go/fig && go test -tags=conformance -run TestSbeHexViaFfi
bash bindings/cpp/smoke/sbe_hex.sh
bash bindings/csharp/smoke/run.sh
```

Both `fig-ffi` and `fig-python` run shared §16.1 vectors from `tests/conformance/vectors/v1.json` (including CandleBar CBOR/SBE).

**Scope:** FFI wire codecs (encode/decode, client connect) are conformance-tested in CI. Go/C++/C# SBE hex parity uses FFI `fig_sbe_encode_*` against §16.1 vectors.

Go smoke: `cargo build -p fig-ffi && cd bindings/go/fig && go test -c -tags=conformance`.
