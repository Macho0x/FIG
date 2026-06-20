# FIG — Fast Interchange Gateway

> A schema-native, multiplexed, zero-RTT binary protocol for trading systems.
> Unifies FIX, REST, and WebSocket over TREE (QUIC).

[![CI](https://github.com/Macho0x/fig/actions/workflows/ci.yml/badge.svg)](https://github.com/Macho0x/fig/actions/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

**FIG is a wire protocol** — one binary format for orders, market data, and
account streams over a single encrypted connection. Gateway adapters translate
legacy FIX/REST/WebSocket clients for migration; they are not the product.

FIG specifies how messages move and how credentials ride on the wire (like FIX
Logon or an HTTP `Authorization` header). API key issuance, account portals,
and matching engines belong in your venue stack, outside the protocol.

---

## Who uses FIG?

| Persona | What you get |
|---|---|
| **Venues & brokers** | One native connection for order entry, live market data, and private account streams |
| **Integration teams** | Replace FIX + REST + WebSocket glue with one client library and one auth model |
| **Gateway operators** | Proxy legacy clients to a FIG backend with `fig-gateway` (`--fig-backend`) |
| **Schema / tooling authors** | FSL schemas compile to Rust, SBE, Protobuf, JSON Schema, and FIX mappings |

---

## Why FIG?

| Problem with status quo | FIG solution |
|---|---|
| FIX needs 4 RTTs to connect (TCP + TLS + Logon) | TREE 0-RTT session resumption — 1 RTT new, 0 RTT resumed |
| FIX is ASCII, 200–500 bytes header overhead | Binary 16-byte fixed header + compact TLV extensions |
| REST is stateless — no session, no sequencing | Three channel modes: stateless, session, affinity |
| WebSocket has no built-in schema or semantics | Schema-native framing — every frame carries a Schema ID |
| Each protocol needs its own auth, error, observability | One auth model, one error model, one tracing pipeline |
| No multiplexing — one session per FIX/TCP connection | 65,535 concurrent channels per TREE connection |
| JSON parsing is 10–50 μs; FIX ASCII parsing is 5–20 μs | SBE zero-copy decode: **~205 ns** |

### At a glance

| | FIG (SBE) | FIX ASCII | HTTP + JSON |
|---|---|---|---|
| Order decode | **205 ns** | 5–20 μs | 10–50 μs |
| Min header | **16 B** | 200–500 B | 200–800 B |
| Reconnect | **0-RTT** | full Logon | stateless |

Full Criterion results: [docs/BENCHMARKS.md](docs/BENCHMARKS.md).

**Terms:** **FIG** = protocol · **TREE** = QUIC transport (TLS 1.3, ALPN `fig/1`) · **FSL** = schema IDL ([SPEC.md](SPEC.md) §1).

---

## Try it in 60 seconds

```bash
git clone https://github.com/Macho0x/fig.git && cd fig
cargo run -p fig-exchange-sim          # terminal 1 — server on 127.0.0.1:8443
cargo run -p fig-cli                   # terminal 2 — six demos, one connection
```

Optional legacy gateway (REST `:8080`, WS `:8090`, FIX `:9876`):

```bash
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

Walkthrough: [docs/TUTORIAL.md](docs/TUTORIAL.md).

---

## Worked examples

Rust snippets using `fig_core` — same patterns as [`fig-cli`](crates/fig-cli/src/main.rs).
Each frame is sent on a TREE bidirectional stream (`conn.open_bi()` → `encode()` →
`FrameDecoder`). Full sequences: [docs/PROTOCOL.md](docs/PROTOCOL.md).

```rust
use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::*;
```

### 1. Live market data (public subscribe)

```rust
let candles = Frame::new(FrameType::Subscribe, 2)
    .with_extension(Extension::text(ExtensionTag::RoutingKey, "marketdata/AAPL/candles/5m"))
    .with_extension(Extension::text(ExtensionTag::ChannelPath, "marketdata/AAPL/candles/5m"));
// → STREAM_ITEM (CandleBarEvent)
```

### 2. Order entry

```rust
let order = NewOrderSingle {
    cl_ord_id: "CLI-001".into(), side: Side::Buy, symbol: "AAPL".into(),
    order_qty: Quantity(100.0), price: Some(Price(150.25)),
    order_type: OrderType::Limit, time_in_force: TimeInForce::Day,
    account: Some("DEMO-ACCT".into()),
    stop_price: None, expire_time: None, strategy_id: None,
    security_id: None, id_source: None, security_exchange: None,
};
let order_frame = Frame::new(FrameType::Request, 1)
    .with_extension(Extension::text(ExtensionTag::ChannelPath, "trading/accounts/DEMO-ACCT/orders"))
    .with_extension(Extension::text(ExtensionTag::Method, "POST"))
    .with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-DEMO-ACCT"))
    .with_payload(codec::encode_cbor(&order)?);
// → STREAM_ITEM (ExecutionReport)
```

### 3. Private account stream (wire auth)

```rust
let balances = Frame::new(FrameType::Subscribe, 3)
    .with_extension(Extension::text(ExtensionTag::RoutingKey, "accounts/DEMO-ACCT/balances"))
    .with_extension(Extension::text(ExtensionTag::ChannelPath, "accounts/DEMO-ACCT/balances"))
    .with_extension(Extension::text(ExtensionTag::AuthToken, "fig-dev-DEMO-ACCT"));
// → STREAM_ITEM (BalanceSnapshot, then BalanceUpdate)
```

Simulator test token only — see [SPEC.md §9.3](SPEC.md). `FIG_DEV_OPEN=1` skips auth locally.

### 4. Historical query, then resume live

```rust
let history = Frame::new(FrameType::Request, 4)
    .with_extension(Extension::text(ExtensionTag::ChannelPath, "marketdata/AAPL/candles/5m"))
    .with_extension(Extension::text(ExtensionTag::Method, "GET"))
    .with_payload(codec::encode_cbor(&CandleBarRequest {
        symbol: "AAPL".into(),
        interval: "5m".into(),
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    })?);
// → RESPONSE (CandleBarBatch); then re-use example 1 SUBSCRIBE on the same connection
```

### 5. Legacy migration (gateway adapters)

Translate REST or WebSocket client shapes to native FIG frames before proxying
to your backend ([docs/GATEWAY.md](docs/GATEWAY.md)).

```rust
use fig_gateways::rest::{HttpRequest, parse_http_request};
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws_catalog::legacy_ws_json_to_fig_subscribe;

let get = parse_http_request(b"GET /marketdata/AAPL/ticker HTTP/1.1\r\nHost: localhost\r\n\r\n")?;
let ticker = http_get_to_fig_request(&get)?;

let ws_sub = legacy_ws_json_to_fig_subscribe(
    r#"{"method":"SUBSCRIBE","params":["aapl@ticker"]}"#,
    1,
)?;
// Forward `ticker` / `ws_sub` to FIG backend via fig_gateways::backend
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  FIG client (fig-cli, fig-core, bindings)                    │
└────────────────────────┬────────────────────────────────────┘
                         │ TREE — TLS 1.3, 0-RTT, 65k channels
┌────────────────────────▼────────────────────────────────────┐
│  FIG server (your venue or fig-exchange-sim reference)       │
│  frames · channels · sessions · SBE/CBOR · auth on wire      │
└────────────────────────┬────────────────────────────────────┘
                         │ optional
         ┌───────────────▼───────────────────┐
         │  Gateway: FIX · REST · WebSocket   │
         │  legacy clients → native FIG       │
         └───────────────────────────────────┘
```

Reference crates: [`fig-core`](crates/fig-core/) (protocol) · [`fig-gateways`](crates/fig-gateways/) (adapters) · [`fig-exchange-sim`](crates/fig-exchange-sim/) (demo server) · [`fig-fsl`](crates/fig-fsl/) (schemas). Module index: [docs/API.md](docs/API.md).

---

## Documentation

| Document | Description |
|---|---|
| [SPEC.md](SPEC.md) | Normative protocol specification |
| [docs/TUTORIAL.md](docs/TUTORIAL.md) | Getting started and CLI walkthrough |
| [docs/PROTOCOL.md](docs/PROTOCOL.md) | Worked sequences and integration patterns |
| [docs/STREAMING.md](docs/STREAMING.md) | Live subscribe paths and WS catalog |
| [docs/QUERY.md](docs/QUERY.md) | Historical queries and REST GET mapping |
| [docs/GATEWAY.md](docs/GATEWAY.md) | Legacy gateway deployment |
| [docs/BENCHMARKS.md](docs/BENCHMARKS.md) | Full Criterion results |
| [docs/API.md](docs/API.md) | Crate and module index |
| [schemas/orders.fsl](schemas/orders.fsl) | Example FSL schema |
| [TODO.md](TODO.md) | Implementation roadmap |

```bash
cargo test --workspace          # ~360 tests
cargo bench -p fig-bench        # performance suites
```

---

## License

Dual-licensed under MIT or Apache-2.0. See [CONTRIBUTING.md](CONTRIBUTING.md).
