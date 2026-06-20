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

All five patterns run over **one TREE connection** with per-channel multiplexing.
Native paths are canonical; gateways map legacy FIX/REST/WS shapes to the same
frames. Full sequences: [docs/PROTOCOL.md](docs/PROTOCOL.md).

### 1. Live market data (public subscribe)

No auth. Stream OHLCV bars as trades arrive.

```text
Client                              Exchange
  SUBSCRIBE  ChannelPath: marketdata/AAPL/candles/5m
             RoutingKey:  marketdata/AAPL/candles/5m
  ───────────────────────────────────────────────►
  STREAM_ITEM  CandleBarEvent (partial bar)
  ◄───────────────────────────────────────────────
  STREAM_ITEM  CandleBarEvent (bar close)
  ◄───────────────────────────────────────────────
```

Gateway WS alias: `aapl@kline_5m` → same path ([docs/STREAMING.md](docs/STREAMING.md)).

### 2. Order entry (request + execution stream)

Session-oriented flow; SBE on the hot path in production.

```text
Client                              Exchange
  REQUEST POST  ChannelPath: trading/accounts/DEMO-ACCT/orders
                Payload: NewOrderSingle (AAPL, Buy, 100 @ 150.25)
  ───────────────────────────────────────────────►
  STREAM_ITEM   ExecutionReport (fill or ack)
  ◄───────────────────────────────────────────────
  RESPONSE      200
  ◄───────────────────────────────────────────────
```

FIX equivalent: MsgType `D` → `ExecutionReport` (35=8). See [docs/GATEWAY.md](docs/GATEWAY.md).

### 3. Private account stream (wire auth)

`AUTH_TOKEN` on every private frame; principal must match `{account}` in the path.
The simulator uses test token `fig-dev-{account}` — not a key-issuance API.

```text
Client                              Exchange
  SUBSCRIBE  ChannelPath: accounts/DEMO-ACCT/balances
             AuthToken:   fig-dev-DEMO-ACCT
  ───────────────────────────────────────────────►
  STREAM_ITEM  BalanceSnapshot (is_snapshot: true)
  ◄───────────────────────────────────────────────
  STREAM_ITEM  BalanceUpdate (on change)
  ◄───────────────────────────────────────────────
```

Spec: [SPEC.md §9.3](SPEC.md). Set `FIG_DEV_OPEN=1` on the sim to skip auth locally.

### 4. Historical query (request-response)

Pull a batch, then resume live subscribe on the same connection.

```text
Client                              Exchange
  REQUEST GET  ChannelPath: marketdata/AAPL/candles/5m
               Payload: CandleBarRequest { limit: 100 }
  ───────────────────────────────────────────────►
  RESPONSE     CandleBarBatch
  ◄───────────────────────────────────────────────
  SUBSCRIBE    marketdata/AAPL/candles/5m   (resume live)
  ───────────────────────────────────────────────►
```

REST alias: `GET /marketdata/AAPL/candles/5m?limit=100` via `fig-gateway`.

### 5. Legacy migration (gateway proxy)

Existing REST or WebSocket clients talk to `fig-gateway`; the gateway forwards
native FIG frames to your venue. No client rewrite required for incremental rollout.

```text
REST client                         fig-gateway                    FIG backend
  GET /marketdata/AAPL/ticker  ──►  REQUEST (native path)  ──►  exchange-sim
  ◄── JSON ◄──────────────────  RESPONSE (CBOR→JSON)  ◄──────

WS client                           fig-gateway                    FIG backend
  {"method":"SUBSCRIBE",           SUBSCRIBE (mapped path)  ──►  STREAM_ITEM
   "params":["aapl@ticker"]}  ──►  ───────────────────────  ◄──  (legacy JSON out)
```

Deploy notes: [docs/GATEWAY.md](docs/GATEWAY.md).

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
