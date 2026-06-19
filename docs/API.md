# FIG API & Documentation Index

Central reference for FIG crates, modules, and project documentation.

## Project documentation

| Document | Description |
|---|---|
| [TUTORIAL.md](TUTORIAL.md) | Step-by-step getting started |
| [PROTOCOL.md](PROTOCOL.md) | Design rationale, channel directions, migration |
| [GATEWAY.md](GATEWAY.md) | Legacy FIX/REST gateway deployment |
| [SECURITY_AUDIT.md](SECURITY_AUDIT.md) | Pre-1.0 security checklist |
| [adr/README.md](adr/README.md) | Architecture decision records |
| [../SPEC.md](../SPEC.md) | Normative wire format specification |
| [../TODO.md](../TODO.md) | Implementation roadmap (complete) |

Generate Rust API docs:

```bash
cargo doc --workspace --no-deps --open
```

---

## `fig-core` — protocol engine

| Module | Purpose |
|---|---|
| [`frame`](../crates/fig-core/src/frame.rs) | 16-byte header encode/decode, streaming `FrameDecoder` |
| [`ext`](../crates/fig-core/src/ext.rs) | TLV extension tags (29 tags) |
| [`channel`](../crates/fig-core/src/channel.rs) | Channel lifecycle, unidirectional streams, sequence numbers |
| [`session`](../crates/fig-core/src/session.rs) | Session model; `MemorySessionStore`, `FileSessionStore`, `RedisSessionStore`, `EtcdSessionStore` |
| [`transport`](../crates/fig-core/src/transport.rs) | TREE transport, `FigConnection`, 0-RTT, migration |
| [`tcp`](../crates/fig-core/src/tcp.rs) | TCP downgrade mode (`FIG\x01` magic prefix) |
| [`migration`](../crates/fig-core/src/migration.rs) | Connection migration tokens and channel reconstruction |
| [`codec`](../crates/fig-core/src/codec.rs) | CBOR helpers; JSON ↔ CBOR for gateways |
| [`protobuf`](../crates/fig-core/src/protobuf.rs) | Protobuf wire encoding for schema-evolving payloads |
| [`sbe`](../crates/fig-core/src/sbe.rs) | Zero-alloc SBE encoder/decoder |
| [`compression`](../crates/fig-core/src/compression.rs) | zstd payload compression |
| [`fragment`](../crates/fig-core/src/fragment.rs) | Payload fragmentation and reassembly |
| [`control`](../crates/fig-core/src/control.rs) | CONTROL frame dispatcher (PING, AUTH_REFRESH, SEQ_RESET, …) |
| [`auth`](../crates/fig-core/src/auth.rs) | Token and mTLS authentication |
| [`jwt`](../crates/fig-core/src/jwt.rs) | HS256 JWT encode/decode |
| [`oauth`](../crates/fig-core/src/oauth.rs) | OAuth2/OIDC dev token validator |
| [`channel_auth`](../crates/fig-core/src/channel_auth.rs) | Per-channel permission policy |
| [`rate_limit`](../crates/fig-core/src/rate_limit.rs) | Token-bucket rate limiter |
| [`dos`](../crates/fig-core/src/dos.rs) | Connection-level DoS guard and flood detector |
| [`trace`](../crates/fig-core/src/trace.rs) | W3C traceparent ↔ TRACE_ID extension |
| [`observability`](../crates/fig-core/src/observability.rs) | Tracing spans, metrics, Prometheus export |
| [`messages`](../crates/fig-core/src/messages.rs) | Trading message types (NewOrderSingle, …) |
| [`error`](../crates/fig-core/src/error.rs) | `FrameError`, `ChannelError`, `SessionError`, `FigError` |

### Binaries

| Binary | Command | Purpose |
|---|---|---|
| `fig-observability` | `cargo run -p fig-core --features observability-bin --bin fig-observability` | Prometheus `/metrics` on `:9090` |

---

## `fig-gateways` — legacy adapters

| Module | Purpose |
|---|---|
| [`fix`](../crates/fig-gateways/src/fix.rs) | FIX 4.4 parse/serialize, FIG message conversion |
| [`fix_session`](../crates/fig-gateways/src/fix_session.rs) | FIX session state machine (logon, heartbeat, resend) |
| [`rest`](../crates/fig-gateways/src/rest.rs) | HTTP/1.1 parse/serialize, JSON ↔ CBOR |
| [`ws`](../crates/fig-gateways/src/ws.rs) | WebSocket RFC 6455 frame mapping |
| [`sse`](../crates/fig-gateways/src/sse.rs) | Server-Sent Events ↔ STREAM_ITEM |

### Binaries

| Binary | Command | Purpose |
|---|---|---|
| `fig-gateway` | `cargo run -p fig-gateways --bin fig-gateway` | REST `:8080` + FIX `:9876` translation demo |

---

## `fig-fsl` — schema language

| Module | Purpose |
|---|---|
| [`parser`](../crates/fig-fsl/src/parser.rs) | FSL tokenizer and recursive-descent parser |
| [`ast`](../crates/fig-fsl/src/ast.rs) | Schema AST (messages, types, gateway mappings) |
| [`codegen`](../crates/fig-fsl/src/codegen.rs) | Rust struct codegen |
| [`sbe_codegen`](../crates/fig-fsl/src/sbe_codegen.rs) | SBE Rust encode/decode codegen |
| [`target_codegen`](../crates/fig-fsl/src/target_codegen.rs) | Go, Python, TS, OCaml, Zig, C++, C#, Proto, SBE XML, JSON Schema, FIX YAML |

### CLI (`ftlc`)

```bash
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.usl --lang go --out /tmp/out
cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.usl
```

Supported `--lang` values: `rust`, `sbe`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`, `python`, `typescript`, `ocaml`, `zig`, `json-schema`, `fix-yaml`.

---

## `fig-exchange-sim` — reference server

| Module | Purpose |
|---|---|
| [`server`](../crates/fig-exchange-sim/src/server.rs) | FIG server: orders, cancels, replace, market data, accounts |
| [`matching`](../crates/fig-exchange-sim/src/matching.rs) | Price-time matching engine |
| [`orderbook`](../crates/fig-exchange-sim/src/orderbook.rs) | Limit order book |

Default listen address: `127.0.0.1:8443` (UDP/TREE).

---

## `fig-cli` — demo client

Native FIG client connecting to `127.0.0.1:8443`. Demonstrates order entry,
market data subscription, account query, and PING/PONG over TREE.

---

## `fig-bench` — benchmarks

Six Criterion suites: `frame_bench`, `codec_bench`, `gateway_bench`, `matching_bench`, `transport_bench`, `alloc_bench`.

```bash
cargo bench -p fig-bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
```
