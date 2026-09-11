# Building and Installing FIG SDKs

FIG is developed as a Rust workspace. Published crates.io/npm packages are not
automated in this repo — build from source or vendor artifacts in your CI.

## Rust

```bash
git clone https://github.com/Macho0x/fig.git && cd fig
cargo build --release -p fig-core -p fig-cli -p fig-client
cargo test --workspace
```

Key crates: `fig-core`, `fig-client`, `fig-gateways`, `fig-exchange-sim`, `fig-ffi`.

## C ABI (`fig-ffi`)

```bash
cargo build --release -p fig-ffi
# Linux:   target/release/libfig_ffi.so
# macOS:   target/release/libfig_ffi.dylib
# Windows: target/release/fig_ffi.dll
```

Headers: [crates/fig-ffi/include/fig.h](../crates/fig-ffi/include/fig.h).

Set `FIG_FFI_LIB` to the shared library path for language bindings.

## Python

```bash
cargo build -p fig-python
# Install via maturin when packaging:
# pip install maturin && maturin develop -m crates/fig-python/Cargo.toml
```

Import: `from fig import FigPyClient` (see [fig-python](../crates/fig-python/)).

```python
from fig import FigPyClient

c = FigPyClient()
c.connect("127.0.0.1:8443")
frames = c.request(".well-known/capabilities", "GET")  # REQUEST, waits for EOF
sub = c.subscribe_live("marketdata/AAPL/candles/5m")     # snapshot + live handle
nxt = sub.next(timeout_ms=5000)
```

## Go / TypeScript / C# / Java / OCaml / Zig / C++

1. Build `fig-ffi` (above).
2. Follow per-language [bindings/README.md](../bindings/README.md).
3. Regenerate FSL types: `cargo xtask codegen`.

**Conformance:** Rust vectors + `fig-ffi` `binding_conformance` are CI-tested.
Per-language generated SBE is compile-smoke only until hex parity lands — prefer
FFI wire codecs for production.

## Manual crates.io publish (maintainers)

```bash
cargo publish -p fig-core   # verify dependency order
cargo publish -p fig-client
# … publish dependents in dependency order
```

Requires crates.io credentials and version bumps in workspace `Cargo.toml`
(`[workspace.package] version`). That crate version is independent of
[SPEC.md](../SPEC.md) protocol `1.0.0` (bump SPEC only on a wire break).

`fig_version()` (C ABI) returns `CARGO_PKG_VERSION`.

## Binding smokes (CI)

```bash
cargo build -p fig-ffi --release
export FIG_FFI_LIB=$PWD/target/release/libfig_ffi.so   # .dylib on macOS
bash bindings/java/smoke/run.sh          # needs JAVA_HOME
bash bindings/typescript/smoke/run.sh    # needs bun
bash bindings/zig/smoke/run.sh           # needs zig
bash bindings/ocaml/smoke/run.sh         # needs ocamlopt
```

TypeScript is **Bun** (`bun:ffi`) compile smoke. Python `FigPyClient.request()` is
REQUEST (EOF); `subscribe_live()` holds the stream. Browsers/Node use the gateway.
