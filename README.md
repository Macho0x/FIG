# FIG — Fast Interchange Gateway

> A schema-native, multiplexed, zero-RTT protocol for trading systems.
> Unifies and supersedes FIX, REST, and WebSocket.

FIG is a single protocol that combines the strengths of FIX (financial
session semantics, sequence numbers, market data), REST (resource-oriented
request-response), and WebSocket (bidirectional streaming) into one
wire format over QUIC.

## Why FIG?

- **One connection, all patterns** — orders, market data, and account queries
  flow over a single QUIC connection with per-stream flow control.
- **Schema-native** — every frame carries a Schema ID; messages are validated
  at the protocol layer, not in application code.
- **Free observability** — Trace ID, Correlation ID, and nanosecond timestamps
  live in the fixed header, readable without parsing the payload.
- **Zero-RTT session continuity** — sessions survive disconnects; reconnect
  with 0-RTT and resume all open channels.
- **Backwards compatible** — gateway adapters translate to/from FIX, REST,
  and WebSocket for incremental migration.
- **Single mental model** — one auth, one error model, one codegen pipeline.

## Crates

| Crate | Description |
|---|---|
| `fig-core` | Frame parser, channel manager, session model, QUIC transport, SBE codec |
| `fig-usl` | USL (FIG Schema Language) parser, Rust codegen, and `uslc` CLI |
| `fig-gateways` | Gateway adapters: FIX 4.4, REST/HTTP, WebSocket ↔ FIG translation |
| `fig-exchange-sim` | Native FIG exchange simulator with order book and matching engine |
| `fig-cli` | Native FIG trading client demo |

## Quick Start

```bash
# Build everything
cargo build --workspace

# Run the exchange simulator
cargo run -p fig-exchange-sim

# In another terminal, run the trading client
cargo run -p fig-cli

# Compile a USL schema to Rust
cargo run -p fig-usl --bin uslc -- compile schemas/orders.usl --lang rust --out src/generated/

# Validate a USL schema
cargo run -p fig-usl --bin uslc -- validate schemas/orders.usl
```

## Protocol Overview

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            Length (32)                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Type (8)   |  Flags (8)   |        Channel ID (16)         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Stream Seq (32)                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|  HeaderCount |   SchemaID   |          reserved (16)         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Extension TLV (variable)  |   Payload (variable)           |
```

### Frame Types

| Type | Code | Description |
|---|---|---|
| CONTROL | 0x00 | Ping/pong, settings, sequence reset |
| REQUEST | 0x01 | Request-response (REST parity) |
| RESPONSE | 0x02 | Response to REQUEST |
| SUBSCRIBE | 0x03 | Subscribe to a routing key (pub/sub) |
| UNSUBSCRIBE | 0x04 | Unsubscribe |
| STREAM_OPEN | 0x05 | Open a bidirectional stream (FIX/WS parity) |
| STREAM_ITEM | 0x06 | Data frame within a stream |
| STREAM_CLOSE | 0x07 | Close a stream |
| STREAM_ERROR | 0x08 | Error on a stream |

### Channel Modes

| Mode | Description | Legacy equivalent |
|---|---|---|
| `stateless` | Each request is independent, no session state | REST |
| `session` | Durable session with sequence numbers, survives reconnects | FIX |
| `affinity` | Sticky to a backend, state implicit in connection | WebSocket |

### Payload Encodings

| Encoding | Content-Type | Use Case |
|---|---|---|
| CBOR | `application/cbor` | Self-describing, serde-compatible (default) |
| SBE | `application/fig+sbe` | Zero-alloc, fixed-offset (production) |

## Gateway Adapters

FIG includes gateway adapters that translate between legacy protocols and FIG frames:

- **FIX 4.4 Adapter** — Parse/serialize FIX tag=value messages, map to FIG message types
- **REST Adapter** — Parse HTTP/1.1 requests, map to FIG frames, convert JSON↔CBOR
- **WebSocket Adapter** — Parse WS frames (RFC 6455), map to FIG stream items

## USL — FIG Schema Language

USL is an IDL that defines messages, channels, and gateway mappings in one schema file:

```
schema trading.orders v1 {
    type Symbol = string(max_len: 16)
    
    message NewOrderSingle {
        @number 1  cl_ord_id: string(max_len: 32)
        @number 2  side: Side
        @number 3  order_qty: decimal64
        ...
        
        channel_type: session
        correlation_field: cl_ord_id
        priority: high
        idempotent: true
        
        gateway fix {
            message NewOrderSingle -> MsgType: "D" {
                cl_ord_id -> tag: 11
                side -> tag: 54 values: { buy: "1", sell: "2" }
                ...
            }
        }
        
        gateway rest {
            message NewOrderSingle -> method: POST path: "/accounts/{account}/orders"
        }
    }
}
```

See [SPEC.md](SPEC.md) for the full protocol specification and [schemas/orders.usl](schemas/orders.usl) for a complete example.

## Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p fig-core
cargo test -p fig-gateways
cargo test -p fig-exchange-sim

# Run integration tests
cargo test -p fig-exchange-sim --test integration
```

## License

Dual-licensed under MIT or Apache-2.0.
