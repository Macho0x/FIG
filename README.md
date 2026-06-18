# FIG — Unified Network Interchange Protocol

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
- **Single mental model** — one auth, one error model, one codegen pipeline.

## Crates

| Crate | Description |
|---|---|
| `fig-core` | Frame parser, channel manager, session model, QUIC transport |
| `fig-usl` | USL (FIG Schema Language) parser and multi-target codegen |
| `fig-exchange-sim` | Native FIG exchange simulator (killer demo server) |
| `fig-cli` | Native FIG trading client (killer demo client) |

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

See [SPEC.md](SPEC.md) for the full protocol specification.

## License

Dual-licensed under MIT or Apache-2.0.
