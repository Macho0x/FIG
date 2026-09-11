# Contributing to FIG

Thank you for your interest in contributing to FIG — Fast Interchange Gateway!

## Development Setup

```bash
# Clone
git clone https://github.com/Macho0x/fig.git
cd fig

# Build
cargo build --workspace

# Test
cargo test --workspace

# Run the exchange simulator
cargo run -p fig-exchange-sim

# Run the trading client (in another terminal)
cargo run -p fig-cli

# Compile an FSL schema
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang rust --out src/generated/
```

## Architecture

```
fig-core          — Wire format, channels, sessions, TREE transport, SBE/CBOR codec
fig-fsl           — FSL schema language parser, multi-target codegen, ftlc CLI
fig-gateways      — FIX, REST, WebSocket, and SSE gateway adapters
fig-exchange-sim  — Order book, matching engine, FIG server
fig-cli           — Native FIG demo client (seven validated flows)
fig-bench         — Criterion microbenches + fig-latency tail harness
```

See [docs/API.md](docs/API.md) for the full module index.

## Protocol Design Principles

1. **Schema-native** — Every frame carries a Schema ID. Messages are validated at the protocol layer.
2. **Multiplexed** — Channels are TREE streams. Up to 65535 per connection.
3. **Backwards compatible** — Gateway adapters translate to/from FIX, REST, and WebSocket.
4. **Zero-RTT** — Sessions survive disconnects. Reconnect with 0-RTT and resume channels.
5. **Observable** — Trace ID, Correlation ID, and timestamps in the fixed header.

## Wire Format

16-byte fixed header + TLV extensions + payload:

```
| Length (32) | Type (8) | Flags (8) | Channel ID (16) |
| Stream Seq (32) | HeaderCount (8) | SchemaID (8) | reserved (16) |
| Extension TLV (variable) | Payload (variable) |
```

## Schema Changes

**FSL (`.fsl`) is the single source of truth** for message types, enums, field
numbers, and gateway mappings. See
[ADR 0004 — FSL as Single Source of Truth](docs/adr/0004-fsl-single-source-of-truth.md).

Do **not** hand-edit `fig-core/src/messages.rs`. FSL is the source of truth;
regenerate with `cargo xtask codegen` (or `cargo xtask codegen --check` in CI).

### Adding or changing a message type

1. Edit the FSL schema (e.g. [`schemas/orders.fsl`](schemas/orders.fsl)):

```
message MyMessage {
    channel_type: request_response
    correlation_field: request_id

    request_id: string(max_len: 32) @1
    field2: decimal64(precision: 2) @2

    priority: medium
}
```

2. Bump the schema version if the wire format changes (see ADR 0004):

```
schema trading.orders v1.1.0 {   // minor = additive; major = breaking
```

3. Validate and regenerate **all** targets you ship:

```bash
cargo xtask codegen
```

Or compile one language at a time:

```bash
cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.fsl

for lang in rust sbe python cpp csharp go typescript ocaml zig proto sbe-xml json-schema fix-yaml; do
  cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang "$lang" --out "generated/$lang"
done
```

4. Add or update gateway mappings in the same `.fsl` file:

```
gateway fix {
    message MyMessage -> MsgType: "X" {
        request_id -> tag: 100
        field2 -> tag: 101
    }
}

gateway rest {
    message MyMessage -> method: POST path: "/my/messages"
}
```

5. Update tests and conformance vectors if payloads or enums change.

`fig-core/src/messages.rs` is generated — never hand-edit it. Run
`cargo xtask codegen` after FSL changes.

### Enum and field rules

| Change | Version bump | Notes |
|---|---|---|
| Append enum variant | Minor | SBE discriminants are sequential — **never reorder** |
| Rename/remove enum variant | Major | CBOR uses string names (`"Buy"`) — breaking |
| Add optional field | Minor | New `@N` field number |
| Add required field | Major | Old clients cannot decode |
| Change `@N` field number | Major | Protobuf/SBE wire break |

## Adding a New Extension Tag

1. Add to `fig-core/src/ext.rs` in the `ExtensionTag` enum
2. Add the tag code in the `from_code` function
3. Add encoding/decoding support in the `Extension` struct
4. Add a test in the ext.rs test module

## Coding Standards

- All code must compile with zero warnings (`cargo build --workspace` must be clean)
- All tests must pass (`cargo test --workspace`)
- Use `#[derive(Debug, Clone)]` on all public types
- Use `thiserror` for error types
- Use `serde` for serialization
- Document all public items with `///` doc comments
- Write tests for all new functionality

## CI

GitHub Actions runs on every push to `main`:

| Job | What it runs |
|---|---|
| `build` | `cargo build/test/clippy/fmt --workspace --all-features` (Linux), Redis session round-trip, Java/TS/Zig/OCaml smoke |
| `coverage` | `cargo llvm-cov` coverage report |
| `benchmarks` | Smoke-run frame, transport, gateway, alloc, and tail-latency benches |
| `cross-platform` | `cargo test --workspace --all-features` on Linux, macOS, Windows |
| `fuzz` | nightly `cargo fuzz run frame_decode` (30s) |

Tagged releases trigger `.github/workflows/release.yml` (binaries + changelog).

## Fuzzing

CI runs `cargo fuzz run frame_decode -- -max_total_time=30` on the nightly
toolchain. Locally:

```bash
cargo install cargo-fuzz
cd crates/fig-core
cargo fuzz run frame_decode -- -max_total_time=60
```

Targets `Frame::decode` and `FrameDecoder` against arbitrary bytes. See [TODO.md §18](TODO.md#182-security).

## License

Dual-licensed under MIT or Apache-2.0. All contributions must be dual-licensed.

