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

# Compile a USL schema
cargo run -p fig-usl --bin uslc -- compile schemas/orders.usl --lang rust --out src/generated/
```

## Architecture

```
fig-core          — Wire format, channels, sessions, TREE transport, SBE codec
fig-usl           — USL schema language parser, codegen, and uslc CLI
fig-gateways      — FIX, REST, and WebSocket gateway adapters
fig-exchange-sim  — Order book, matching engine, FIG server
fig-cli           — Trading client demo
```

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

## Adding a New Message Type

1. Define the message in a USL schema (`.usl` file):
```
message MyMessage {
    @number 1  field1: string(max_len: 32)
    @number 2  field2: decimal64
    channel_type: session
    priority: medium
}
```

2. Compile to Rust:
```bash
cargo run -p fig-usl --bin uslc -- compile schemas/my_schema.usl --lang rust --out src/generated/
```

3. Add gateway mappings if needed:
```
gateway fix {
    message MyMessage -> MsgType: "X" {
        field1 -> tag: 100
        field2 -> tag: 101
    }
}
```

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

GitHub Actions runs on every push:
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace`
- `cargo fmt --check`

## License

Dual-licensed under MIT or Apache-2.0. All contributions must be dual-licensed.

