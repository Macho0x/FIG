<p align="center">
  <img src="assets/FIG%20Light%20Mode%20transparent%20(small).png" alt="FIG — Fast Interchange Gateway" width="320">
</p>

# FIG — Fast Interchange Gateway

**One encrypted connection** for orders, market data, and private account streams —
with optional FIX, REST, and WebSocket gateways for migration.

- **Brokers & venues** — one native FIG server surface; legacy clients via `fig-gateway`
- **Clients & integrators** — connect once; trade and subscribe on channel paths, not three protocols
- **Everyone** — schema-defined messages (SBE/CBOR), sequenced streams, 0-RTT reconnect over TREE (QUIC)

[![CI](https://github.com/Macho0x/fig/actions/workflows/ci.yml/badge.svg)](https://github.com/Macho0x/fig/actions/workflows/ci.yml)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

> Schema-native, multiplexed, zero-RTT binary protocol for trading systems.

**FIG is a wire protocol** — not a matching engine, account portal, or API-key issuer.
It specifies how messages move and how credentials ride on the wire (like FIX Logon or
an HTTP `Authorization` header). Your venue issues tokens; FIG libraries validate them
on the wire. See [Authentication](#authentication) below.

**Terms:** **FIG** = protocol · **TREE** = QUIC transport (TLS 1.3, ALPN `fig/1`) · **FSL** = schema IDL ([SPEC.md](SPEC.md) §1).

---

## Choose your path

| I am… | Start here | You will… |
|---|---|---|
| **New to FIG** | [Docs map](docs/README.md) | Pick a guide; don't mix SUBSCRIBE and REQUEST I/O |
| **Broker / venue** | [Gateway guide](docs/GATEWAY.md) · [Architecture](#architecture--migration) | Run a FIG backend + optional gateway; issue auth in *your* stack |
| **Client / integrator** | [Tutorial](docs/TUTORIAL.md) · [Quick start by language](#quick-start-by-language) | Connect with `fig-cli` or a binding; use channel paths from [SPEC.md](SPEC.md) |
| **Migrating from FIX/REST/WS** | [Try it](#try-it-in-60-seconds) (gateway row) | Point legacy clients at `:9876` / `:8080` / `:8090`; proxy with `--fig-backend` |

---

## Who uses FIG?

| Persona | What you get |
|---|---|
| **Venues & brokers** | One native connection for order entry, live market data, and private account streams |
| **Integration teams** | Replace FIX + REST + WebSocket glue with one client library and one auth model |
| **Gateway operators** | Proxy legacy clients to a FIG backend with `fig-gateway` (`--fig-backend`) |
| **Trading bot authors (TS)** | Native FIG on **Bun** via `bun:ffi` over `fig-ffi` — no WASM; browsers use the gateway |
| **Schema / tooling authors** | FSL schemas compile to Rust, SBE, JSON Schema, and FIX mappings |

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
| Heavy parse cost on hot paths | SBE decode **~312 ns** on reference hardware ([BENCHMARKS.md](docs/BENCHMARKS.md)) |

Fewer connections and smaller headers on the wire; faster decode on order hot paths.
Full Criterion medians and tail-latency percentiles (p99/p99.9): [docs/BENCHMARKS.md](docs/BENCHMARKS.md).

### At a glance

| | FIG (SBE) | FIX ASCII (in-repo) | HTTP + JSON (in-repo) |
|---|---|---|---|
| Order decode | **312 ns** | ~4.8 μs | ~1.8 μs (headers) |
| Min header | **16 B** | 200–500 B | 200–800 B |
| Reconnect | **0-RTT** | full Logon | stateless |

Industry ballparks for generic JSON/FIX stacks appear in [BENCHMARKS.md](docs/BENCHMARKS.md) as context only — not FIG regression baselines.

---

## Try it in 60 seconds

```bash
git clone https://github.com/Macho0x/fig.git && cd fig
cargo run -p fig-exchange-sim          # terminal 1 — server on 127.0.0.1:8443
cargo run -p fig-cli                   # terminal 2 — seven demos, one connection
```

**Migration day 1** — legacy gateway in front of the same backend (REST `:8080`, WS `:8090`, FIX `:9876`):

```bash
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

Walkthrough: [docs/TUTORIAL.md](docs/TUTORIAL.md). You do **not** need to move all clients to native FIG at once.

---

## One connection, three flows

Same TREE connection (`127.0.0.1:8443` in the simulator). Each logical stream is a
**channel** (bidirectional TREE stream) identified by path extensions on the frame:

```text
channel 1  →  POST   trading/accounts/DEMO-ACCT/orders     →  ExecutionReport stream
channel 2  →  SUBSCRIBE  marketdata/AAPL/candles/5m        →  CandleBarEvent stream
channel 3  →  SUBSCRIBE  accounts/DEMO-ACCT/balances       →  BalanceSnapshot, BalanceUpdate
```

- Example **#1** — [Live market data](#1-live-market-data-public-subscribe)
- Example **#2** — [Order entry](#2-order-entry)
- Example **#3** — [Private account stream](#3-private-account-stream-wire-auth)
- Example **#4** — [Historical query, then resume live](#4-historical-query-then-resume-live)

Full sequences: [docs/PROTOCOL.md](docs/PROTOCOL.md).

---

## Authentication

| | |
|---|---|
| **Public market data** | No token — subscribe on `marketdata/...` paths |
| **Private paths** | `AUTH_TOKEN` extension on every frame; principal must match `{account}` in `ChannelPath` ([SPEC §9.3](SPEC.md)) |
| **Your venue** | Issues, rotates, and revokes credentials (JWT/OAuth patterns in spec) — FIG only **verifies** |
| **Local dev** | Simulator accepts test token `fig-dev-{account}`; set `FIG_DEV_OPEN=1` on the server to skip auth |

---

## What's included vs reference

| Component | Role |
|---|---|
| [`fig-core`](crates/fig-core/), [SPEC](SPEC.md), [conformance](tests/conformance/) | Protocol implementation — CI-tested (~360 workspace tests) |
| [`fig-exchange-sim`](crates/fig-exchange-sim/) | **Reference** venue for learning — not a production exchange |
| [`fig-gateway`](crates/fig-gateways/) | **Migration bridge** — run standalone or embed adapters |
| Auth / API keys / matching | **Your** venue stack — outside the protocol |

---

## Capabilities today

What the reference simulator and CLI exercise end-to-end (parity roadmap: [TODO.md §17](TODO.md)):

- Order entry (limit/market), cancel, execution reports
- Public market data subscribe (candles, ticker, agg trades, mark price, …)
- Private account streams (balances, positions, margin) with wire auth
- Historical query (GET + cursor) then resume live on the same connection
- Instrument catalog and capabilities (`GET /.well-known/instruments`, `/.well-known/capabilities`)
- Fill history on `accounts/{account}/fills`
- FIX / REST / WebSocket gateway translation paths
- PING/PONG, session resumption, channel sequencing

---

## Worked examples

Native FIG on **one TREE connection** (TLS + multiplexed channels). If you know legacy APIs, the mapping is:

| You know… | FIG native | `CHANNEL_PATH` example |
|-----------|------------|-------------------------|
| **WebSocket** `SUBSCRIBE` / push stream | `SUBSCRIBE` → `STREAM_ITEM` | `marketdata/AAPL/candles/5m` |
| **REST** `GET` / `POST` | `REQUEST` → `RESPONSE` | `marketdata/AAPL/ticker`, `trading/…/orders` |
| **FIX** NewOrderSingle (35=D) | `REQUEST` + CBOR `NewOrderSingle` | `trading/accounts/{account}/orders` |
| **FIX** drop copy / MD incremental | `SUBSCRIBE` → `STREAM_ITEM` | `trading/accounts/{account}/executions` |
| **Hyperliquid** `allMids` | `GET` → `AllMidsBatch` | `marketdata/ticker/all` |

**Wire notation** (SPEC §4 names → Rust — not strings on the wire):

| Wire (SPEC) | Rust | SDK helper |
|-------------|------|------------|
| `SUBSCRIBE` | `FrameType::Subscribe` | `subscribe_frame`, `subscribe_*` |
| `REQUEST` | `FrameType::Request` | `request_frame`, `request_*`, `post_order` |
| `RESPONSE` | `FrameType::Response` | one-shot reply to `REQUEST` |
| `STREAM_ITEM` | `FrameType::StreamItem` | server push (decode CBOR payload) |

Recommended path: [`fig-client`](crates/fig-client/) (`FigSdkClient`) — same wire as below, less boilerplate. Full demo: [`fig-cli`](crates/fig-cli/src/lib.rs) (`run_demos`).

**SUBSCRIBE vs REQUEST:** snapshot helpers (`subscribe_*`) do not wait for stream EOF.
Use `subscribe_live` + `LiveSubscription::next_frame` for later `STREAM_ITEM`s.
`send_and_read` is **REQUEST only** — using it on a live subscribe hangs.

```rust
use fig_client::{dev_auth_token, FigSdkClient};
use fig_core::messages::*;
use fig_core::transport;
use quinn::Endpoint;

// Connect once (like opening a FIX session or a single WS connection).
let client_cfg = transport::client_config()?;
let mut ep = Endpoint::client("0.0.0.0:0".parse()?)?;
ep.set_default_client_config(client_cfg);
let conn = ep.connect("127.0.0.1:8443".parse()?, "localhost")?.await?;
let client = FigSdkClient::new(&conn);
let account = "DEMO-ACCT";
```

### 1. Live market data (public subscribe)

WebSocket-style subscribe (e.g. Binance `@kline_5m`) or a FIX market-data request — server pushes `STREAM_ITEM`s. No `AUTH_TOKEN`.

```rust
// wire: SUBSCRIBE → STREAM_ITEM (CandleBarEvent) — snapshot, then drop the live handle
let (candles, _frames) = client.subscribe_candles("AAPL", "5m", 2).await?;
if let Some(bar) = candles.current() {
    println!("close={}", bar.close.0);
}

// wire: SUBSCRIBE → STREAM_ITEM (BestBidOffer)
let (bbo, _frames) = client.subscribe_bbo("AAPL", 3).await?;
println!("mid={:?}", bbo.implied_mid());

// Hold the recv stream for later bars (do not use send_and_read):
use fig_client::frames::subscribe_frame;
let path = "marketdata/AAPL/candles/5m";
let sub = subscribe_frame(2, 1, path, Some(path), None)?;
let (_snapshot, mut live) = client.subscribe_live(sub).await?;
while let Some(frame) = live.next_frame().await? {
    println!("live {:?}", frame.frame_type);
}

// wire: REQUEST GET → RESPONSE (AllMidsBatch)
let (mids, _batch) = client.request_all_mids(4).await?;
println!("AAPL mid={:?}", mids.mid("AAPL"));
```

### 2. Order entry

`REQUEST` POST — same job as `POST /accounts/{id}/orders` or FIX MsgType `D`. One-shot; execution reports come back on that request stream.

```rust
let order = NewOrderSingle {
    cl_ord_id: "CLI-001".into(),
    side: Side::Buy,
    symbol: "AAPL".into(),
    order_qty: Quantity(100.0),
    price: Some(Price(150.25)),
    order_type: OrderType::Limit,
    time_in_force: TimeInForce::Day,
    account: Some(account.into()),
    stop_price: None,
    expire_time: None,
    strategy_id: None,
    security_id: None,
    id_source: None,
    security_exchange: None,
    post_only: None,
    reduce_only: None,
};
// wire: REQUEST POST → STREAM_ITEM (ExecutionReport)
let frames = client.post_order(account, &order, 1).await?;
// Decode ExecutionReport from frames (or use OrdersState for live merge)
```

### 3. Private account stream (wire auth)

Authenticated user-data WebSocket or private FIX drop copy — **`AUTH_TOKEN` required** on every channel.

```rust
// wire: SUBSCRIBE → STREAM_ITEM (BalanceSnapshot, then BalanceUpdate)
let (balances, _frames) = client.subscribe_balances(account, 5).await?;
println!("USD={:?}", balances.balances.get("USD"));

// wire: SUBSCRIBE → STREAM_ITEM (ExecutionReport)
let (orders, _frames) = client.subscribe_executions(account, 6).await?;
println!("open orders={}", orders.open_count());

// Keep recv open for later STREAM_ITEMs (fills, position deltas):
use fig_client::frames::subscribe_frame;
let path = format!("accounts/{account}/positions");
let token = dev_auth_token(account);
let sub = subscribe_frame(7, 1, &path, Some(&path), Some(&token))?;
let (_snapshot, mut live) = client.subscribe_live(sub).await?;
while let Some(frame) = live.next_frame().await? {
    println!("live {:?}", frame.frame_type);
}
```

### 4. Historical query, then resume live

Like `GET /candles?limit=100` then a live WebSocket subscribe — both on the **same TREE connection**.

```rust
// wire: REQUEST GET → RESPONSE (CandleBarBatch)
let (batch, _) = client.request_candles(
    CandleBarRequest {
        symbol: "AAPL".into(),
        interval: "5m".into(),
        start_time: None,
        end_time: None,
        limit: Some(100),
        cursor: None,
    },
    7,
).await?;
println!("history bars={}", batch.bars.len());

// wire: SUBSCRIBE → STREAM_ITEM (live candles, same connection)
use fig_client::frames::subscribe_frame;
let path = "marketdata/AAPL/candles/5m";
let sub = subscribe_frame(8, 1, path, Some(path), None)?;
let (_snapshot, mut live) = client.subscribe_live(sub).await?;
while let Some(frame) = live.next_frame().await? {
    println!("live {:?}", frame.frame_type);
}
```

### 5. Legacy migration (keep FIX / REST / WS clients)

Run [`fig-gateway`](crates/fig-gateways/) at the edge; translate legacy shapes to native FIG frames and proxy to your FIG backend ([docs/GATEWAY.md](docs/GATEWAY.md)).

```rust
use fig_gateways::rest::{parse_http_request};
use fig_gateways::rest_query::http_get_to_fig_request;
use fig_gateways::ws_catalog::legacy_ws_json_to_fig_subscribe;
use fig_gateways::fix::{parse_fix_message, fix_to_fig_order};

// wire: REST GET → REQUEST (FrameType::Request + Method GET)
let get = parse_http_request(
    b"GET /marketdata/AAPL/ticker HTTP/1.1\r\nHost: localhost\r\n\r\n",
)?;
let ticker_req = http_get_to_fig_request(&get)?;

// wire: WS JSON SUBSCRIBE → SUBSCRIBE (FrameType::Subscribe)
let ws_sub = legacy_ws_json_to_fig_subscribe(
    r#"{"method":"SUBSCRIBE","params":["aapl@ticker"]}"#,
    1,
)?;

// Forward translated frames to your native FIG backend (fig-gateway --fig-backend)
// REST GET: proxy_frame(addr, ticker_req).await?
// WS SUBSCRIBE: BackendSession::connect(addr) then send_frame + recv_frame
// use fig_gateways::backend::{proxy_frame, BackendSession};

// wire: FIX 35=D → CBOR NewOrderSingle, then REQUEST POST at gateway
// let fix = parse_fix_message(b"8=FIX.4.4\x0135=D\x0111=CLI-001\x01...")?;
// let order = fix_to_fig_order(&fix.tags)?;
```

**Low-level wire:** build `Frame::new(FrameType::Subscribe, …)` (or `Request`, etc.) with `fig_core` — see [SPEC §4](SPEC.md). Python/Go/C++ use the same frame types via [`fig-ffi`](crates/fig-ffi/) / [`bindings/`](bindings/README.md).

---

## Architecture & migration

**Day 1 — gateway only:** legacy FIX/REST/WS clients hit `fig-gateway`; gateway speaks FIG to your backend.

**Day N — native clients:** bots and integrations connect directly over TREE; gateway shrinks to stragglers.

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

### Venue adoption (one standard)

FIG is **wire infrastructure**, not an exchange. Every broker follows the same path:
Day 1 gateway for legacy clients, Day 2 native FIG over TREE for performance clients.
Binance and Hyperliquid wire shapes are **gateway aliases only** — see [AGENTS.md](AGENTS.md)
and [docs/GATEWAY.md](docs/GATEWAY.md#venue-adoption--day-1-gateway). Crypto perps use
the shared instrument model ([ADR 0007](docs/adr/0007-crypto-instrument-model.md)); colo
order entry uses [SBE order path](docs/SBE_ORDER_PATH.md).

---

## Quick start by language

| Language | Get connected | Docs |
|---|---|---|
| **Rust** | `cargo run -p fig-cli` | [`fig-core`](crates/fig-core/), [`fig-cli`](crates/fig-cli/src/main.rs) |
| **Python** | `cargo build -p fig-python` then import `fig` | [`fig-python`](crates/fig-python/) |
| **Go / C# / TS / Java / OCaml / Zig / C++** | `cargo build -p fig-ffi` + binding README | [`bindings/`](bindings/README.md) |

**Browser:** no in-browser FIG stack — use [`fig-gateway`](crates/fig-gateways/) REST/WebSocket against your FIG backend.

**FFI bindings** share [`fig-ffi`](crates/fig-ffi/include/fig.h) for connect, request, subscribe, and SBE encode/decode on the hot path. Per-language generated SBE for complex messages is evolving; prefer FFI wire codecs until your binding's conformance coverage matches Rust.

---

## Language SDKs

Roadmap and parity definition: [TODO.md §0](TODO.md#0-active-backlog-fig-repo) (active backlog) ·
[§16–21](TODO.md) reference. Status key: **✅** shipped · **🔶** partial · **⬜** planned · **🏛** venue-owned.

| Language | SDK status | Package / path | FSL codegen (`ftlc`) | Native FIG client |
|---|---|---|---|---|
| **Rust** | ✅ Reference | [`fig-core`](crates/fig-core/), [`fig-cli`](crates/fig-cli/) | ✅ full (Rust + SBE) | Tier 1–4 — `subscribe_live` + `LiveSubscription` |
| **Python** | ✅ Reference binding | [`fig-python`](crates/fig-python/) (PyO3) | ✅ full + CBOR via PyO3 | `request()` (EOF); `subscribe()` snapshot; `subscribe_live()` / `FigPySubscription.next()` |
| **C++** | ✅ FFI wrapper | [`bindings/cpp`](bindings/cpp/) → [`fig-ffi`](crates/fig-ffi/) | ✅ types + generated SBE | `fig::Client::subscribe` + `Subscription::next` over `fig.h` |
| **C#** | ✅ FFI wrapper | [`bindings/csharp`](bindings/csharp/) → `fig-ffi` | ✅ types + `SbeGenerated.cs` | `FigClient.Subscribe` + `FigSubscription.Next` |
| **Go** | ✅ FFI wrapper | [`bindings/go`](bindings/go/) → `fig-ffi` | ✅ types + `sbe_generated.go` | `Client.Subscribe` + `Subscription.Next` |
| **Java** | ✅ FFI wrapper | [`bindings/java`](bindings/java/) → `fig-ffi` | ✅ `--lang java` | JNI `figClientSubscribe` / `figClientSubNext` |
| **TypeScript** | 🔶 Compile smoke | [`bindings/typescript`](bindings/typescript/) → `fig-ffi` | ✅ types + generated SBE | Bun `version()` smoke; browsers/Node use [gateway](docs/GATEWAY.md) |
| **OCaml** | 🔶 Compile smoke | [`bindings/ocaml`](bindings/ocaml/) → `fig-ffi` | ✅ records + variant enums | `fig_version` / JWT smoke — not a live-sub SDK |
| **Zig** | 🔶 Compile smoke | [`bindings/zig`](bindings/zig/) → `fig-ffi` | ✅ types + generated SBE | `@cImport` smoke — not a live-sub SDK |

**C ABI:** [`fig-ffi`](crates/fig-ffi/include/fig.h) + [bindings/README.md](bindings/README.md).

---

## Documentation

Index with “if you want to…” paths: **[docs/README.md](docs/README.md)**.

| Document | Description |
|---|---|
| [docs/README.md](docs/README.md) | What to read, in what order; SUBSCRIBE vs REQUEST |
| [AGENTS.md](AGENTS.md) | Contributor guide — one standard, gateway alias rules |
| [SPEC.md](SPEC.md) | Normative protocol specification |
| [docs/TUTORIAL.md](docs/TUTORIAL.md) | Getting started and CLI walkthrough |
| [docs/PROTOCOL.md](docs/PROTOCOL.md) | Worked sequences and integration patterns |
| [docs/STREAMING.md](docs/STREAMING.md) | Live subscribe paths and WS catalog |
| [docs/QUERY.md](docs/QUERY.md) | Historical queries and REST GET mapping |
| [docs/GATEWAY.md](docs/GATEWAY.md) | Legacy gateway deployment and alias E2E tests |
| [docs/SBE_ORDER_PATH.md](docs/SBE_ORDER_PATH.md) | Colo SBE order entry path |
| [docs/adr/0007-crypto-instrument-model.md](docs/adr/0007-crypto-instrument-model.md) | Crypto perp instrument catalog (FSL) |
| [docs/BENCHMARKS.md](docs/BENCHMARKS.md) | Criterion microbenches + tail-latency harness |
| [docs/API.md](docs/API.md) | Crate and module index |
| [TODO.md §0](TODO.md#0-active-backlog-fig-repo) | Active backlog (priority-ordered open work) |
| [TODO.md §18](TODO.md#18-fig-10-release-criteria) | 1.0 release criteria + venue adoption (§19–21) |
| [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) | HA deployment and operations |
| [docs/PUBLISHING.md](docs/PUBLISHING.md) | Build and install SDKs |
| [schemas/orders.fsl](schemas/orders.fsl) | Example FSL schema |
| [TODO.md](TODO.md) | Implementation roadmap |

```bash
cargo test --workspace
cargo test -p fig-cli                              # SDK demos + SBE order demo
cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e
cargo bench -p fig-bench                           # Criterion microbenches (medians)
cargo run --release -p fig-bench --bin fig-latency # tail latency p99/p99.9
```

---

## License

Dual-licensed under MIT or Apache-2.0. See [CONTRIBUTING.md](CONTRIBUTING.md).
