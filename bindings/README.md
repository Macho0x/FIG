# FIG language bindings

Thin wrappers over the stable C ABI in `crates/fig-ffi/include/fig.h`.
Build `fig-ffi` first (`cargo build -p fig-ffi`) and link `libfig_ffi`.

**TLS:** the FFI client verifies the server certificate by default
(`client_config_from_env()`). Connecting to the self-signed development simulator
therefore requires `FIG_TLS_CA=<pem>` — export it with
`fig-exchange-sim --print-ca .fig/dev-ca.pem` — or the development-only bypass
`FIG_TLS_INSECURE=1`. Production uses the WebPKI roots or `FIG_TLS_CA` with your
own CA ([DEPLOYMENT.md](../docs/DEPLOYMENT.md)).

## Layout

| Path | Language | Notes |
|------|----------|-------|
| `go/fig` | Go | cgo — `Client.Subscribe` / `Subscription.Next` + JWT/SBE |
| `cpp/include/fig` | C++ | RAII `fig::Client::subscribe` + `Subscription::next` + JWT/SBE |
| `cpp/pure/fig_protocol.hpp` | C++ | Pure protocol library (frame/ext/channel, no Rust runtime) |
| `csharp/Fig` | C# | P/Invoke `FigClient.Subscribe` / `FigSubscription.Next` + JWT/SBE |
| `typescript/fig.ts` | TypeScript | Bun `FigClient.subscribe` / `FigSubscription.next` + JWT/SBE; browsers/Node use the gateway |
| `typescript/sbe_generated.ts` | TypeScript | FSL-generated SBE serializers |
| `ocaml/fig.ml` | OCaml | C stubs — `subscribe` / `sub_next` + JWT/SBE |
| `zig/fig.zig` | Zig | `@cImport` — `subscribe` / `Subscription.next` + JWT/SBE |
| `zig/pure/protocol.zig` | Zig | Pure protocol library (frame/ext/channel) |
| `zig/sbe_generated.zig` | Zig | FSL-generated SBE serializers |
| `java/` | Java | JNI `figClientSubscribe` / `figClientSubNext` + JWT/SBE |
| `python/` (`crates/fig-python`) | Python | PyO3 module `fig`: `FigPyClient` / `FigPySubscription` + states |

## FSL codegen

```bash
cargo run -p xtask -- codegen   # refreshes Rust + per-language SBE + pure protocol
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang sbe-go --out /tmp/out
# Also: sbe-cpp, sbe-csharp, sbe-typescript, sbe-zig, protocol-cpp, protocol-zig
# Types: go, typescript, ocaml, zig, java, python, cpp, csharp
```

All type targets emit typed enums, nested structs, type aliases, message metadata constants, and CBOR field manifests. SBE targets emit wire-compatible encoders/decoders (`schema_id=0x01`).

## JWT (dev / conformance)

All **FFI** bindings (Go, C++, C#, Java, OCaml, TypeScript, Zig) expose HS256 JWT
helpers via the C ABI (`fig_jwt_encode`, `fig_jwt_verify_bearer`) for the dev
`AUTH_TOKEN`. The Python binding does not wrap them yet — use the FFI library
directly or the gateway.

Production venues verify RS256 tokens with `fig_core::jwt_keys`
(`FIG_JWT_JWKS`); rotation is described in [JWT_KEY_ROTATION.md](../docs/JWT_KEY_ROTATION.md).

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
`crates/fig-ffi/include/fig.h`. Python (module `fig`): `PyOrderBookState`,
`PyMidsState`, `PyBboState`, `PyTradeTape`, `PyMarkPriceState`, `PyOrdersState`,
`PyAggTradesState`, `PyFundingState`, `PyLedgerState`, `PyLiquidationState`.

**Compile smoke** (CI also runs Java / TypeScript / Zig / OCaml):

```bash
cargo build -p fig-ffi
export LD_LIBRARY_PATH="$PWD/target/debug:${LD_LIBRARY_PATH:-}"
cd bindings/go/fig && go test -tags=conformance
cargo build -p fig-ffi --release
bash bindings/cpp/smoke/run.sh
export FIG_REPO_ROOT="$PWD"
bash bindings/csharp/smoke/run.sh
bash bindings/java/smoke/run.sh
bash bindings/typescript/smoke/run.sh
bash bindings/zig/smoke/run.sh
bash bindings/ocaml/smoke/run.sh
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

**Live subscribe:** `fig_client_subscribe` / `fig_client_sub_next` (C), `FigPyClient.subscribe_live` (Python), and the Go/C++/C#/Java/TS/OCaml/Zig wrappers. `fig_client_request_and_recv` is REQUEST only. Test: `cargo test -p fig-ffi --test client_integration`.

**Scope:** FFI wire codecs (encode/decode, client connect) are conformance-tested in CI. The
generated-code hex parity layers on top of that: the Go/C++/C# smoke scripts start with the FFI
encoders (`fig_sbe_encode_*`) checked against the §16.1 vectors and then encode the same messages
with the **per-language generated** code and compare the bytes.

**Per-language generated SBE (`*_generated.{go,hpp,cs,ts,zig}`): compiling and parity-checked.**

`cargo run -p xtask -- codegen` emits native SBE encoders/decoders per language from FSL. After
the 2026-10 rework of the emission layer (`crates/fig-fsl/src/sbe_target_codegen.rs`) every target
compiles in CI and encodes byte-for-byte identically to the shared conformance vectors:

| Target | Compiles | Parity vs `tests/conformance/vectors/v1.json` |
|---|---|---|
| Go (`go/fig/sbe/`) | ✅ CI | ✅ `go test ./sbe` — `NewOrderSingle`, `OrderBookSnapshot`, `CapabilitiesResponse` + round-trips |
| C++ (`cpp/include/fig/sbe_generated.hpp`) | ✅ CI | ✅ `bash bindings/cpp/smoke/sbe_parity.sh` |
| C# (`csharp/Fig/SbeGenerated.cs`) | ✅ CI | ✅ `bindings/csharp/smoke` (generated parity + FFI hex) |
| TypeScript (`typescript/sbe_generated.ts`) | ✅ CI | ✅ `bindings/typescript/smoke/run.sh` |
| Zig (`zig/sbe_generated.zig`) | ✅ CI | ✅ `bindings/zig/sbe_parity.zig` (CI builds and runs it) |

The wire layout mirrors `crates/fig-core/src/generated/sbe_generated.rs` (the normative Rust
encoder), including list-of-scalar payloads (verified by the `CapabilitiesResponse` vector, which
covers `list<uint8>`, `list<Symbol>` and `list<CandleInterval>`). The **FFI** wire codecs remain
the recommended path for production; the generated codecs are a drop-in alternative that avoids
shipping the native library.

Go smoke: `cargo build -p fig-ffi && cd bindings/go/fig && go test -c -tags=conformance`.
