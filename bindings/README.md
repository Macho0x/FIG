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

**Scope:** FFI wire codecs (encode/decode, client connect) are conformance-tested in CI. The SBE hex parity checks for Go/C++/C# call the **FFI** encoders (`fig_sbe_encode_*`) against the §16.1 vectors — they do not exercise the per-language generated SBE files.

**Per-language generated SBE (`*_generated.{go,hpp,cs,ts,zig}`): experimental — does not
compile yet.** `cargo run -p xtask -- codegen` emits them from FSL, but a 2026-10 audit
found that **no target currently produces compilable code** (TypeScript is closest; Go,
C++, Zig have known residual errors; C# is unverified in CI). They are not compiled or
parity-verified by CI. Use the **FFI** wire codecs for production. The full defect list,
the design decisions to follow, and the remaining work are recorded in
[TODO.md §16.2](../TODO.md).

Go smoke: `cargo build -p fig-ffi && cd bindings/go/fig && go test -c -tags=conformance`.
