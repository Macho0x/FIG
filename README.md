# FIG — Fast Interchange Gateway

> A schema-native, multiplexed, zero-RTT binary protocol for trading systems.
> Unifies and supersedes FIX, REST, and WebSocket over TREE.

[![CI](https://github.com/Macho0x/fig/actions/workflows/ci.yml/badge.svg)](https://github.com/Macho0x/fig/actions/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

FIG is a single binary protocol that combines the strengths of FIX (financial
session semantics, sequence numbers, market data), REST (resource-oriented
request-response), and WebSocket (bidirectional streaming) into one wire format
over TREE. It is designed to compete with FIX/REST/WebSocket as a new standard
on its own merits — with gateway adapters providing backwards compatibility as
a migration path, not as the protocol's identity.

---

## Table of Contents

- [Why FIG?](#why-fig)
- [Architecture](#architecture)
- [Crates](#crates)
- [Quick Start](#quick-start)
- [Protocol Overview](#protocol-overview)
- [Gateway Adapters](#gateway-adapters)
- [USL — FIG Schema Language](#usl--fig-schema-language)
- [Benchmarks](#benchmarks)
- [Testing](#testing)
- [Project Stats](#project-stats)
- [License](#license)

---

## Why FIG?

| Problem with status quo | FIG solution |
|---|---|
| FIX needs 4 RTTs to connect (TCP + TLS + Logon) | TREE 0-RTT session resumption — 1 RTT new, 0 RTT resumed |
| FIX is ASCII, 200-500 bytes header overhead | Binary 16-byte fixed header + compact TLV extensions |
| REST is stateless — no session, no sequencing | Three channel modes: stateless, session, affinity |
| WebSocket has no built-in schema or semantics | Schema-native framing — every frame carries a Schema ID |
| Each protocol needs its own auth, error, observability | One auth model, one error model, one tracing pipeline |
| No multiplexing — one session per FIX/TCP connection | 65,535 concurrent channels per TREE connection |
| JSON parsing is 10-50μs; FIX ASCII parsing is 5-20μs | SBE zero-copy decode: ~205ns (25-250x faster) |

### Key Features

- **One connection, all patterns** — orders, market data, and account queries
  flow over a single TREE connection with per-stream flow control.
- **Schema-native** — every frame carries a Schema ID; messages are validated
  at the protocol layer, not in application code.
- **Free observability** — Trace ID, Correlation ID, and nanosecond timestamps
  live in the fixed header, readable without parsing the payload.
- **Zero-RTT session continuity** — sessions survive disconnects; reconnect
  with 0-RTT and resume all open channels.
- **Backwards compatible** — gateway adapters translate to/from FIX, REST,
  and WebSocket for incremental migration.
- **Dual encoding** — CBOR for self-describing development, SBE for
  zero-alloc production hot paths.
- **Per-channel flow control** — credit-based backpressure prevents one slow
  consumer from blocking others.
- **Single mental model** — one auth, one error model, one codegen pipeline.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        FIG Client                            │
│  (fig-cli or native app using fig-core)                      │
└────────────────────────┬────────────────────────────────────┘
                         │ TREE (ALPN: fig/1)
                         │ 65,535 channels, 0-RTT, TLS 1.3
                         │
┌────────────────────────▼────────────────────────────────────┐
│                     FIG Server                               │
│  (fig-exchange-sim or native venue)                          │
│  ┌──────────┐  ┌───────────┐  ┌────────────┐  ┌──────────┐ │
│  │ Frame    │  │ Channel   │  │ Session    │  │ Auth     │ │
│  │ Parser   │  │ Manager   │  │ Store      │  │ (mTLS/   │ │
│  │ (16B+TLV)│  │ (65535)   │  │ (0-RTT)    │  │  token)  │ │
│  └────┬─────┘  └─────┬─────┘  └─────┬──────┘  └────┬─────┘ │
│       │               │              │              │       │
│  ┌────▼─────┐  ┌──────▼──────┐  ┌────▼──────┐  ┌───▼─────┐ │
│  │ SBE/CBOR │  │ Flow Control│  │ Observability│ │ Codec   │ │
│  │ Codec    │  │ (credits)   │  │ (spans+metrics)│ │ (serde)│ │
│  └──────────┘  └─────────────┘  └─────────────┘  └─────────┘ │
└─────────────────────────────────────────────────────────────┘

         ┌─────────────────────────────────────┐
         │        Gateway Adapters              │
         │  ┌─────────┐ ┌─────┐ ┌────────────┐ │
         │  │ FIX 4.4 │ │REST │ │ WebSocket  │ │
         │  │ adapter │ │adptr│ │  adapter   │ │
         │  └────┬────┘ └──┬──┘ └─────┬──────┘ │
         │       │         │           │       │
         │  Legacy FIX   HTTP/JSON   WS clients │
         │  clients      clients      clients   │
         └─────────────────────────────────────┘
```

---

## Crates

| Crate | Description | Tests | Key Modules |
|---|---|---|---|
| [`fig-core`](crates/fig-core/) | Frame parser, channel manager, session model, TREE transport, SBE/CBOR codec, auth, flow control, observability | 118 | `frame`, `ext`, `channel`, `session`, `codec`, `transport`, `sbe`, `auth`, `observability` |
| [`fig-usl`](crates/fig-usl/) | USL (FIG Schema Language) parser, Rust codegen, and `uslc` CLI | 27 | `ast`, `parser`, `codegen`, `bin/uslc` |
| [`fig-gateways`](crates/fig-gateways/) | Gateway adapters: FIX 4.4, REST/HTTP, WebSocket ↔ FIG translation | 29 | `fix`, `rest`, `ws` |
| [`fig-exchange-sim`](crates/fig-exchange-sim/) | Native FIG exchange simulator with order book and matching engine | 16 | `orderbook`, `matching`, `server` |
| [`fig-cli`](crates/fig-cli/) | Native FIG trading client demo | — | `main` |
| [`fig-bench`](crates/fig-bench/) | Criterion benchmarks for all components | 21 benchmarks | `frame_bench`, `codec_bench`, `gateway_bench`, `matching_bench` |

---

## Quick Start

### Prerequisites

- Rust 1.75+ (stable)
- Linux, macOS, or Windows

### Build and Run

```bash
# Clone
git clone https://github.com/Macho0x/fig.git
cd fig

# Build everything
cargo build --workspace

# Run the exchange simulator (FIG server on 127.0.0.1:8443)
cargo run -p fig-exchange-sim

# In another terminal, run the trading client
cargo run -p fig-cli
```

The CLI demonstrates:
1. **Order entry** — sends a NewOrderSingle, receives an ExecutionReport
2. **Market data subscription** — subscribes to AAPL quotes, receives a snapshot
3. **Account query** — queries account balance and buying power
4. **PING/PONG** — control frame heartbeat

All over a single TREE connection with per-stream multiplexing.

### Schema Compilation

```bash
# Compile a USL schema to Rust
cargo run -p fig-usl --bin uslc -- compile schemas/orders.usl --lang rust --out src/generated/

# Validate a USL schema
cargo run -p fig-usl --bin uslc -- validate schemas/orders.usl
```

### Run Benchmarks

```bash
# Run all benchmarks
cargo bench -p fig-bench

# Run a specific benchmark
cargo bench -p fig-bench -- --bench codec_bench
```

---

## Protocol Overview

### Wire Format

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

**16-byte fixed header** + TLV extensions + schema-aware payload. The fixed
header is always the same size, so gateways can route frames without parsing
extensions or payloads. Extensions carry protocol-level metadata (URI, method,
status code, session ID, routing key, etc.) — the same frame serves all three
legacy protocols by varying which extensions are populated.

### Frame Types

| Type | Code | Description |
|---|---|---|
| CONTROL | 0x00 | Ping/pong, settings, sequence reset, goaway |
| REQUEST | 0x01 | Request-response (REST parity) |
| RESPONSE | 0x02 | Response to REQUEST |
| SUBSCRIBE | 0x03 | Subscribe to a routing key (pub/sub) |
| UNSUBSCRIBE | 0x04 | Unsubscribe |
| STREAM_OPEN | 0x05 | Open a bidirectional stream (FIX/WS parity) |
| STREAM_ITEM | 0x06 | Data frame within a stream |
| STREAM_CLOSE | 0x07 | Close a stream |
| STREAM_ERROR | 0x08 | Error on a stream |
| ONE_WAY | 0x09 | Fire-and-forget (no response expected) |
| ACK_RANGE | 0x0A | Acknowledge a range of sequence numbers |
| FLOW_CONTROL | 0x0B | Credit grant/update |
| REDIRECT | 0x0C | Redirect to another server |

### Channel Modes

| Mode | Description | Legacy equivalent |
|---|---|---|
| `stateless` | Each request is independent, no session state | REST |
| `session` | Durable session with sequence numbers, survives reconnects | FIX |
| `affinity` | Sticky to a backend, state implicit in connection | WebSocket |

### Payload Encodings

| Encoding | Content-Type | Encode | Decode | Use Case |
|---|---|---|---|---|
| CBOR | `application/cbor` | ~927ns | ~2.38μs | Self-describing, serde-compatible (default) |
| SBE | `application/fig+sbe` | ~361ns | ~205ns | Zero-alloc, fixed-offset (production hot path) |

### Compliance Tiers

| Tier | Required features | Covers |
|---|---|---|
| **1 — Core** | Fixed header, REQUEST/RESPONSE, CHANNEL_PATH, STATUS_CODE, CONTENT_TYPE | REST parity |
| **2 — Session** | + SESSION_ID, SEQUENCE_NUM, TIMESTAMP, STREAM_OPEN/CLOSE, CONTROL | FIX parity |
| **3 — Pub/Sub** | + ROUTING_KEY, SUBSCRIBE/UNSUBSCRIBE, STREAM_ITEM streaming | Market data |
| **4 — Advanced** | + ACK_RANGE, FLOW_CONTROL, REDIRECT, FRAGMENTED, COMPRESSION | Full protocol |

---

## Gateway Adapters

FIG includes gateway adapters that translate between legacy protocols and FIG
frames. These are **migration tools** — the protocol stands on its own without
them.

### FIX 4.4 Adapter

Translates between FIX tag=value ASCII messages and FIG frames:

| FIX concept | FIG equivalent |
|---|---|
| MsgSeqNum (34=) | Stream Seq + SEQUENCE_NUM extension |
| MsgType (35=) | Schema ID |
| SenderCompID/TargetCompID | SESSION_ID |
| Logon (35=A) | STREAM_OPEN + AUTH_TOKEN |
| Logout (35=5) | STREAM_CLOSE |
| Heartbeat (35=0) | CONTROL(PING/PONG) |
| NewOrderSingle (35=D) | STREAM_ITEM, Schema ID 0x01, SBE payload |
| ExecutionReport (35=8) | STREAM_ITEM, Schema ID 0x02 |

Features: FIX checksum computation (mod 256), tag=value parsing, message type
mapping, side/order type/time-in-force enum mapping.

### REST Adapter

Translates between HTTP/1.1 requests and FIG frames:

| HTTP concept | FIG equivalent |
|---|---|
| Method | METHOD extension |
| URI | CHANNEL_PATH / REQUEST_URI extension |
| Headers | Extension tags |
| Body | Payload (CBOR ↔ JSON) |
| Status code | STATUS_CODE extension |
| Chunked transfer | STREAM_ITEM frames |
| SSE | STREAM_ITEM with CONTENT_TYPE |

### WebSocket Adapter

Translates between WebSocket frames (RFC 6455) and FIG stream items:

| WebSocket concept | FIG equivalent |
|---|---|
| Upgrade handshake | TREE handshake + ALPN |
| Text frame | STREAM_ITEM, CONTENT_TYPE "text/plain" |
| Binary frame | STREAM_ITEM, CONTENT_TYPE "application/octet-stream" |
| Close frame | STREAM_CLOSE |
| Ping/Pong | CONTROL(PING/PONG) |

Features: RFC 6455 frame parsing (FIN, opcode, masking, payload length),
client/server masking, text/binary/close/ping/pong opcodes.

---

## USL — FIG Schema Language

USL is an IDL that defines messages, channels, and gateway mappings in one
schema file:

```
schema trading.orders v1 {
    type Symbol = string(max_len: 16)
    
    message NewOrderSingle {
        @number 1  cl_ord_id: string(max_len: 32)
        @number 2  side: Side
        @number 3  order_qty: decimal64
        @number 4  price: optional decimal64
        @number 5  symbol: Symbol
        @number 6  order_type: OrderType
        @number 7  time_in_force: TimeInForce
        @number 8  account: optional string(max_len: 32)
        
        channel_type: session
        correlation_field: cl_ord_id
        priority: high
        idempotent: true
        
        gateway fix {
            message NewOrderSingle -> MsgType: "D" {
                cl_ord_id -> tag: 11
                side -> tag: 54 values: { buy: "1", sell: "2" }
                order_qty -> tag: 38
                price -> tag: 44
                symbol -> tag: 55
            }
        }
        
        gateway rest {
            message NewOrderSingle -> method: POST path: "/accounts/{account}/orders"
        }
    }
}
```

### Codegen Targets

| Target | Status | Output |
|---|---|---|
| Rust | ✅ Implemented | Structs + serde + encode/decode |
| Go | ⬜ Planned | Structs + marshal/unmarshal |
| Python | ⬜ Planned | Dataclasses + serde |
| TypeScript | ⬜ Planned | Interfaces + encode/decode |
| Protobuf | ⬜ Planned | `.proto` file (for gRPC interop) |
| SBE | ⬜ Planned | `.xml` (for trading fast path) |
| JSON Schema | ⬜ Planned | `.schema.json` (for REST docs) |
| FIX mapping | ⬜ Planned | `.fix.yaml` (for gateway config) |

See [SPEC.md](SPEC.md) for the full protocol specification and
[schemas/orders.usl](schemas/orders.usl) for a complete example.

---

## Benchmarks

All benchmarks run with [Criterion](https://bheisler.github.io/criterion.rs/)
on the release profile. Results below are from the reference development
machine — your numbers will vary.

### Frame Encode/Decode

| Benchmark | Time | Description |
|---|---|---|
| `frame_encode` | **574 ns** | Encode a Request frame with extensions + payload |
| `frame_decode` | **649 ns** | Decode a Request frame from bytes |
| `frame_decoder_streaming` | **670 ns** | FrameDecoder with chunked input |
| `frame_encode_large_payload` | **529 ns** | Encode frame with 10KB payload |

### Codec: CBOR vs SBE

| Benchmark | Time | Description |
|---|---|---|
| `cbor_encode_order` | **927 ns** | CBOR encode NewOrderSingle |
| `cbor_decode_order` | **2.38 μs** | CBOR decode NewOrderSingle |
| `sbe_encode_order` | **361 ns** | SBE encode NewOrderSingle |
| `sbe_decode_order` | **205 ns** | SBE decode NewOrderSingle |
| `cbor_encode_execution_report` | **1.16 μs** | CBOR encode ExecutionReport |
| `sbe_encode_execution_report` | **437 ns** | SBE encode ExecutionReport |

**SBE is 2.6x faster to encode and 11.6x faster to decode than CBOR.**
SBE's zero-copy decode (205ns) is 25-250x faster than JSON parsing (10-50μs)
and 25-100x faster than FIX ASCII parsing (5-20μs).

### Gateway Adapters

| Benchmark | Time | Description |
|---|---|---|
| `fix_parse` | **2.37 μs** | Parse FIX NewOrderSingle (tag=value) |
| `fix_serialize` | **2.67 μs** | Serialize FIX ExecutionReport |
| `fix_to_fig_order` | **695 ns** | Convert FIX → FIG NewOrderSingle |
| `rest_parse_request` | **1.19 μs** | Parse HTTP/1.1 request |
| `rest_serialize_response` | **1.21 μs** | Serialize HTTP/1.1 response |
| `ws_parse_text_frame` | **52 ns** | Parse WebSocket text frame |
| `ws_serialize_text_frame` | **182 ns** | Serialize WebSocket text frame |

### Matching Engine

| Benchmark | Time | Description |
|---|---|---|
| `order_book_add` | **530 μs** | Add 1000 orders to the order book |
| `matching_engine_process_order` | **805 μs** | Process market order against full book |
| `matching_engine_cancel` | **1.21 μs** | Cancel an order |

### Performance Comparison

| Metric | FIG (native SBE) | FIX ASCII | HTTP/1.1+JSON | WebSocket |
|---|---|---|---|---|
| Min header overhead | 16 bytes | 200-500 bytes | 200-800 bytes | 2-10 bytes |
| Decode speed | ~205 ns | ~5-20 μs | ~10-50 μs | N/A |
| Handshake RTTs | 1 (TREE) / 0 (resumed) | 4 (TCP+TLS+Logon) | 3-5 (DNS+TCP+TLS+HTTP) | 2-3 (upgrade+TLS) |
| Multiplexing | 65,535 channels/conn | 1 session/conn | 6 (browser) / HTTP/2 | 1/conn |
| Session resumption | 0-RTT | Full reconnect | N/A (stateless) | Full reconnect |

---

## Testing

```bash
# Run all 190 tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p fig-core       # 118 tests
cargo test -p fig-usl         # 27 tests
cargo test -p fig-gateways    # 29 tests
cargo test -p fig-exchange-sim # 16 tests

# Run integration tests (end-to-end over TREE)
cargo test -p fig-exchange-sim --test integration

# Run benchmarks
cargo bench -p fig-bench
```

### Test Coverage by Area

| Area | Tests | What's covered |
|---|---|---|
| Frame encode/decode | 29 | All 13 frame types, all flags, all extension tags, streaming decoder |
| Channel management | 15 | Open/close, sequence numbers, TREE stream mapping, credit exhaustion |
| Session management | 14 | UUID, auth token, seq tracking, memory + file store CRUD |
| SBE codec | 79 | Encode/decode round-trips for all trading message types |
| CBOR codec | 6 | Round-trips, empty values, invalid data, nested structures |
| Auth | 12 | Token verification, mTLS, dev tokens, expiry |
| Observability | 8 | Metrics increment/add/reset/snapshot, concurrent, spans |
| FIX gateway | 8 | Parse/serialize, checksum, convert to/from FIG |
| REST gateway | 7 | Parse HTTP, serialize HTTP, JSON↔CBOR |
| WebSocket gateway | 14 | All opcodes, masking, fragmentation, close frames |
| USL parser | 12 | Tokenizer, all constructs, full orders.usl |
| USL codegen | 5 | Struct generation, enums, associated constants |
| USL CLI | 10 | Compile, validate, output formats |
| Order book | 8 | Add/cancel/reduce, price-time priority, depth |
| Matching engine | 8 | Market/limit fills, partial fills, cancel |
| Integration | 6 | End-to-end sim+cli over TREE |

---

## Project Stats

| Metric | Value |
|---|---|
| Rust source lines | ~11,700 |
| Crates | 6 |
| Tests | 190 passing |
| Benchmarks | 21 |
| Dependencies | quinn 0.11, rustls 0.23, ciborium, criterion, rcgen, uuid, serde, tokio, tracing |
| Transport | TREE (ALPN: `fig/1`) |
| Wire format | 16-byte header + TLV extensions + SBE/CBOR payload |
| Max channels | 65,535 per connection |
| Session resumption | 0-RTT via TREE + FileSessionStore |

---

## License

Dual-licensed under MIT or Apache-2.0.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

