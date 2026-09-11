# FIG API & Documentation Index

Central reference for FIG crates, modules, and project documentation.

## Project documentation

| Document | Description |
|---|---|
| [TUTORIAL.md](TUTORIAL.md) | Step-by-step getting started |
| [PROTOCOL.md](PROTOCOL.md) | Design rationale, channel directions, migration |
| [STREAMING.md](STREAMING.md) | Live `SUBSCRIBE` paths and gateway WS mapping |
| [QUERY.md](QUERY.md) | Historical `REQUEST`/`RESPONSE` paths and REST GET mapping |
| [GATEWAY.md](GATEWAY.md) | Legacy FIX/REST/WebSocket gateway deployment |
| [SECURITY_AUDIT.md](SECURITY_AUDIT.md) | Security checklist (1.0 sign-off recorded) |
| [TODO.md §18](../TODO.md#18-fig-10-release-criteria) | 1.0 release criteria · [§0 backlog](../TODO.md#0-active-backlog-fig-repo) |
| [DEPLOYMENT.md](DEPLOYMENT.md) | HA deployment and operations |
| [PUBLISHING.md](PUBLISHING.md) | Build and install SDKs |
| [adr/README.md](adr/README.md) | Architecture decision records |
| [adr/0004-fsl-single-source-of-truth.md](adr/0004-fsl-single-source-of-truth.md) | FSL schema evolution & multi-language codegen |
| [adr/0006-broker-api-parity.md](adr/0006-broker-api-parity.md) | Native FIG first; broker API parity policy |
| [../SPEC.md](../SPEC.md) | Normative wire format specification |
| [../TODO.md](../TODO.md) | Implementation roadmap |

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
| [`session`](../crates/fig-core/src/session.rs) | Session model; `MemorySessionStore`, `FileSessionStore`, optional `RedisSessionStore` (`session-redis` feature) |
| [`transport`](../crates/fig-core/src/transport.rs) | TREE transport, `FigConnection`, 0-RTT, migration, replay cache |
| [`tcp`](../crates/fig-core/src/tcp.rs) | TCP downgrade mode (`FIG\x01` magic prefix) |
| [`migration`](../crates/fig-core/src/migration.rs) | Connection migration tokens and channel reconstruction |
| [`codec`](../crates/fig-core/src/codec.rs) | CBOR helpers; JSON ↔ CBOR for gateways |
| [`replay`](../crates/fig-core/src/replay.rs) | 0-RTT resumption token replay cache |
| [`sbe`](../crates/fig-core/src/sbe.rs) | Zero-alloc SBE encoder/decoder |
| [`compression`](../crates/fig-core/src/compression.rs) | zstd payload compression |
| [`fragment`](../crates/fig-core/src/fragment.rs) | Payload fragmentation and reassembly |
| [`control`](../crates/fig-core/src/control.rs) | CONTROL frame dispatcher (PING, AUTH_REFRESH, SEQ_RESET, …) |
| [`auth`](../crates/fig-core/src/auth.rs) | Wire auth: token/mTLS types, constant-time verify, refresh hooks — not credential issuance |
| [`jwt`](../crates/fig-core/src/jwt.rs) | JWT bearer decode/verify (HS256 for tests/conformance) |
| [`oauth`](../crates/fig-core/src/oauth.rs) | OAuth2/OIDC dev introspection stub (venues wire real IdP) |
| [`channel_auth`](../crates/fig-core/src/channel_auth.rs) | Per-channel permission policy |
| [`rate_limit`](../crates/fig-core/src/rate_limit.rs) | Token-bucket rate limiter |
| [`dos`](../crates/fig-core/src/dos.rs) | Connection-level DoS guard and flood detector |
| [`trace`](../crates/fig-core/src/trace.rs) | W3C traceparent ↔ TRACE_ID extension |
| [`observability`](../crates/fig-core/src/observability.rs) | Tracing spans, metrics, Prometheus export |
| [`messages`](../crates/fig-core/src/messages.rs) | Trading message types; `channel_path` constants for §17 paths |
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
| [`rest_query`](../crates/fig-gateways/src/rest_query.rs) | REST GET → native FIG `REQUEST` (candles, fills, funding, …) |
| [`ws`](../crates/fig-gateways/src/ws.rs) | WebSocket RFC 6455 frame mapping |
| [`ws_catalog`](../crates/fig-gateways/src/ws_catalog.rs) | Legacy WS topic → native FIG `SUBSCRIBE` (Binance/HL reference aliases) |
| [`ws_listener`](../crates/fig-gateways/src/ws_listener.rs) | HTTP upgrade handler for `fig-gateway` WS listener |
| [`backend`](../crates/fig-gateways/src/backend.rs) | `proxy_frame` (REQUEST/EOF), `BackendSession` (live SUBSCRIBE) |
| [`sse`](../crates/fig-gateways/src/sse.rs) | Server-Sent Events ↔ STREAM_ITEM |

### Binaries

| Binary | Command | Purpose |
|---|---|---|
| `fig-gateway` | `cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443` | REST `:8080`, WS `:8090`, FIX `:9876` |

E2E alias round-trip: `cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e`
(Binance / Hyperliquid / FIX / REST fixture tables in one test file).

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
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang go --out /tmp/out
cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.fsl
```

Supported `--lang` values: `rust`, `sbe`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`, `python`, `typescript`, `ocaml`, `zig`, `json-schema`, `fix-yaml`.

---

## `fig-exchange-sim` — reference server

| Module | Purpose |
|---|---|
| [`server`](../crates/fig-exchange-sim/src/server.rs) | FIG server: `FigServer::accept_0rtt`, held-open subscribe fan-out |
| [`broker_api`](../crates/fig-exchange-sim/src/broker_api.rs) | §17 query/subscribe routing and post-fill fan-out |
| [`market_data`](../crates/fig-exchange-sim/src/market_data.rs) | Candles, trades, BBO, ticker aggregation |
| [`account_state`](../crates/fig-exchange-sim/src/account_state.rs) | Balances, positions, funding, ledger |
| [`auth`](../crates/fig-exchange-sim/src/auth.rs) | Simulator test harness: `fig-dev-{account}` path scoping (not production issuance) |
| [`matching`](../crates/fig-exchange-sim/src/matching.rs) | Price-time matching engine |
| [`orderbook`](../crates/fig-exchange-sim/src/orderbook.rs) | Limit order book |

Default listen address: `127.0.0.1:8443` (UDP/TREE).

---

## `fig-cli` — demo client

Native FIG client connecting to `127.0.0.1:8443` by default. Uses `FigSdkClient`
for seven validated demo flows plus an SBE order demo. Attaches simulator test token
`fig-dev-DEMO-ACCT` on private paths (see SPEC §9.3 wire semantics; not a
key-issuance API).

## `fig-client` — SDK merge helpers

| Module | State type | Streams |
|---|---|---|
| `order_book` | `OrderBookState` | book snapshot/delta |
| `candles` | `CandleState` | candle bars |
| `account` | `AccountCache` | balances, positions, margin |
| `agg_trades` | `AggTradeState` | aggregate trades |
| `funding` | `FundingState` | funding payments |
| `ledger` | `LedgerState` | ledger updates |
| `liquidations` | `LiquidationState` | user + public liquidations |
| `client` | `FigSdkClient`, `LiveSubscription` | subscribe/request wrappers; held-open live recv |

```bash
cargo run -p fig-cli
cargo run -p fig-cli -- --server 127.0.0.1:8443
cargo test -p fig-cli
```

---

## `fig-bench` — benchmarks

Criterion microbenches plus a tail-latency harness (`fig-latency` / `latency_bench`).

| Suite | Role |
|---|---|
| `frame_bench`, `codec_bench`, `gateway_bench`, `matching_bench`, `transport_bench`, `alloc_bench` | Criterion mean/median regression |
| `fig-latency`, `latency_bench` | HDR Histogram p50 / p99 / p99.9 per operation |

```bash
cargo bench -p fig-bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
cargo run --release -p fig-bench --bin fig-latency
FIG_LATENCY_ITERS=1000 cargo run --release -p fig-bench --bin fig-latency
```

See [BENCHMARKS.md](BENCHMARKS.md) for methodology and reference numbers.
