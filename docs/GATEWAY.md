# Gateway Deployment Guide

How to run FIG alongside legacy FIX, REST, WebSocket, OUCH, and ITCH clients.

## Overview

The `fig-gateway` binary is a **translation bridge** for migration testing. It
accepts legacy connections, converts them to FIG frames, and proxies to a remote
FIG backend when `--fig-backend` is set ([ADR 0008](adr/0008-named-gateway-adapters.md)):

- **REST GET** — one-shot `proxy_frame` (finish send, read until EOF)
- **WebSocket SUBSCRIBE** — one `BackendSession` (`FigConnection`) per WS client;
  subscribe without finishing send, then forward later `STREAM_ITEM`s until
  `STREAM_CLOSE` or the socket closes
- **FIX TCP** — session + D/F/G application messages; optional drop-copy 35=8
- **OUCH / SoupBinTCP** — opt-in (`--ouch-addr`); binary order entry
- **ITCH L2 egress** — opt-in (`--itch-addr` SoupBin, `--itch-multicast` MoldUDP64);
  price-level messages only ([ADR 0009](adr/0009-market-data-plane.md))

Library adapters (`fig_gateways::fix`, `rest`, `ws`, `sse`, `ouch`, `itch`) can
be embedded in your own gateway service that forwards to a FIG server.

```text
FIX clients ──► :9876 ──┐
REST clients ──► :8080 ─┼──► fig-gateway ──► FIG backend (TREE)
WS clients  ──► :8090 ─┘
OUCH / ITCH ──► opt-in --ouch-addr / --itch-addr (off by default)
```

**Not served:** NASDAQ TotalView-ITCH (order-by-order), CME MDP 3.0 / FAST,
Cboe BOE/PITCH, Eurex ETI/EOBI, iLink 3. Those are other named adapters, not
this binary. Native FIG market data remains TREE L2.

## Quick Start

```bash
# Terminal 1: FIG exchange simulator
cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem

# Terminal 2: Gateway proxies REST GET + live WS to the simulator
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

Without `--fig-backend`, REST/FIX still translate frames in-process but do not
hit a live book. Native FIG clients (`fig-cli`) connect to `:8443` directly.

## CLI options

| Flag | Default | Description |
|---|---|---|
| `--rest-addr` | `127.0.0.1:8080` | REST HTTP listener |
| `--ws-addr` | `127.0.0.1:8090` | WebSocket HTTP upgrade listener |
| `--fix-addr` | `127.0.0.1:9876` | FIX TCP listener |
| `--fix-dropcopy-account` | (none) | After Logon, SUBSCRIBE executions for this account and egress FIX 35=8 |
| `--ouch-addr` | (none) | SoupBinTCP + OUCH 4.2-shaped listener (off unless set) |
| `--itch-addr` | (none) | SoupBinTCP ITCH L2 egress (off unless set) |
| `--itch-multicast` | (none) | MoldUDP64 ITCH L2 datagram egress (off unless set) |
| `--fig-backend` | (none) | Proxy REST GET (`proxy_frame`) and live WS SUBSCRIBE (`BackendSession`) to native FIG |

Example:

```bash
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-gateways --bin fig-gateway -- \
  --rest-addr 0.0.0.0:8080 \
  --fix-addr 0.0.0.0:9876
```

## REST Gateway

- JSON request bodies are converted to CBOR via `json_to_cbor`.
- Responses are converted back with `cbor_to_json`.
- SSE streaming endpoints map to FIG `STREAM_ITEM` frames via `fig_gateways::sse`.

Example:

```bash
curl -X POST http://127.0.0.1:8080/trading/orders \
  -H 'Content-Type: application/json' \
  -d '{"cl_ord_id":"ORD-1","symbol":"AAPL","side":"Buy","order_qty":100,"price":150.25}'
```

With `--fig-backend`, GET queries round-trip through native FIG:

```bash
curl 'http://127.0.0.1:8080/.well-known/capabilities'
curl 'http://127.0.0.1:8080/.well-known/instruments'
curl 'http://127.0.0.1:8080/marketdata/AAPL/ticker'
```

## FIX Gateway

- Logon (35=A) maps to STREAM_OPEN + AUTH extension.
- ResendRequest (35=2) maps to CONTROL(RESEND) via `fix_session`.
- Application messages map to FIG Request/StreamItem frames.

Use `fig_gateways::fix_session::FixSession` for production-grade FIX session
state management (sequence numbers, heartbeats, gap fill). Application
messages (35=8 and mass-cancel/session/security-status) are stored and
replayed on ResendRequest (35=2) with `PossDupFlag=Y`; admin messages are
gap-filled (35=4). `--fix-dropcopy-account` SUBSCRIBEs
`trading/accounts/{account}/executions` after Logon and egresses 35=8.

FIX 35=q / 35=r map to `MassCancelRequest` / `MassCancelReport`. 35=g / 35=h
map to trading-session status; 35=e / 35=f map to security status.

## OUCH (SoupBinTCP)

Opt-in `--ouch-addr`. Session is SoupBinTCP (`L` login, `A` accept, `S`
sequenced, `U` unsequenced, `H`/`R` heartbeats). **FIG OUCH is 4.2-shaped,
not NASDAQ byte-identical.**

Inbound unsequenced payloads:

| Type | Layout |
|---|---|
| `O` Enter | token 14 ASCII, side `B`/`S`, shares u32 BE, stock 8 ASCII, price u64 BE (`px * 10_000`), TIF `0` Day / `3` IOC, account 16 ASCII |
| `X` Cancel | token 14, shares u32 BE (0 = remaining) |
| `U` Replace | existing 14, replacement 14, shares u32 BE, price u64 BE |

Outbound sequenced: `A` accepted, `E` executed, `C` canceled, `J` rejected.
Login username (SoupBin 6-byte field) is the FIG account; `AUTH_TOKEN` is
`fig-dev-{account}`. E2E: `cargo test -p fig-gateways --test gateway_ouch_itch_e2e`.

## ITCH L2 egress

Opt-in `--itch-addr` (SoupBin replay) and/or `--itch-multicast` (MoldUDP64).
Price-level only — no Add/Delete order references ([ADR 0009](adr/0009-market-data-plane.md)).

With `--fig-backend`, the adapter subscribes to `book` and `trades` for
`AAPL`, `BTC`, `ETH`, and `BTCUSDT`, plus `marketdata/session`. Other symbols
are not egressed.

| Type | Meaning |
|---|---|
| `S` | System event (`O` start, `S` open, `H` halt, `C` close) |
| `R` | Stock directory (8-byte stock) |
| `L` | Price level: stock, side `B`/`S`, price u64, qty u32, level u8, action `S`/`U`/`D` |
| `T` | Trade |

MoldUDP64: 10-byte session (`FIGITCH01`), u64 sequence, u16 count, then
`u16 length + payload` per message. Same payload bytes as SoupBin `S`.

## WebSocket & SSE

- **WebSocket**: `fig-gateway` listens on `--ws-addr` (default `8090`). Legacy subscribe JSON is translated via `fig_gateways::ws_catalog`. With `--fig-backend`, each WS client keeps one TREE connection and receives follow-up `STREAM_ITEM`s (fills, book deltas) until the socket closes.

```json
{"method":"SUBSCRIBE","params":["aapl@ticker"]}
```

maps to native `SUBSCRIBE` on `marketdata/aapl/ticker`. Hyperliquid-shaped JSON
(`{"method":"subscribe","subscription":{"type":"trades","coin":"AAPL"}}`) maps
`coin` to an uppercase symbol on the same native catalog. Round-trip:
`cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e`.
- **SSE**: use `fig_gateways::sse` (`parse_sse_chunk`, `sse_to_fig_stream_item`) for REST streaming endpoints.

## Production Checklist

- [ ] Replace self-signed certs with proper PKI or mTLS (`FIG_MTLS=1` on exchange-sim).
- [ ] Multi-node: shared sessions via `RedisSessionStore` (`fig-core` feature `session-redis`). See [DEPLOYMENT.md](DEPLOYMENT.md).
- [ ] Enable rate limiting and DoS guards (`DoSGuard`, `ChannelRateLimiter`, public MD `MdRateLimiter`).
- [ ] Export metrics: `fig-observability` or `fig-gateway --metrics-addr`.
- [ ] Liveness: `fig-gateway --health-addr` for orchestrators.
- [ ] Wire gateway adapters to forward translated frames to your FIG backend (`--fig-backend`).
- [ ] Run gateway and backend in separate network zones with firewall rules.
- [ ] Use [docker-compose.yml](../docker-compose.yml) as a reference stack.

## Docker

Build and run the exchange simulator container:

```bash
docker build -t fig-exchange-sim .
docker run --rm -p 8443:8443/udp fig-exchange-sim
```

Clients verify TLS by default: pin the simulator's certificate with `--print-ca` +
`FIG_TLS_CA`, or use `FIG_TLS_INSECURE=1` in development only.

Full stack (sim + gateway + metrics): `docker compose up`. See [DEPLOYMENT.md](DEPLOYMENT.md).

See [Dockerfile](../Dockerfile) for the multi-stage build definition.

## Venue adoption — Day 1 gateway

Every exchange/broker follows the same track ([AGENTS.md](../AGENTS.md)): **native FIG
is canonical**. The gateway translates legacy wire formats (FIX, REST, WebSocket JSON,
OUCH, ITCH L2) to the same FSL payloads and `CHANNEL_PATH` values — there is no
per-venue protocol fork. OUCH and ITCH are named adapters ([ADR 0008](adr/0008-named-gateway-adapters.md),
[ADR 0009](adr/0009-market-data-plane.md)), not extra native paths.

```text
Legacy clients ──► fig-gateway ──► FIG backend (TREE :8443)
  REST :8080 · WebSocket :8090 · FIX :9876          on by default
  OUCH --ouch-addr · ITCH --itch-addr / --itch-multicast   off by default
  mappers: ws_catalog.rs, rest_query.rs, ouch.rs, itch.rs
```

Binance `@kline_5m` and Hyperliquid `subscription.type` JSON are **reference aliases**
in `fig-gateways`, not separate FIG tracks. New venues add their own alias mappers
pointing at the same native paths documented in [SPEC.md §9.1](../SPEC.md).

### Topology with `--fig-backend`

```bash
# Terminal 1: venue FIG backend (or exchange-sim for testing)
cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem

# Terminal 2: gateway proxies legacy JSON/FIX to native FIG.
# Add --ouch-addr / --itch-addr / --itch-multicast to turn the binary edges on.
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-gateways --bin fig-gateway -- \
  --fig-backend 127.0.0.1:8443 \
  --ouch-addr 127.0.0.1:9877 \
  --itch-addr 127.0.0.1:9878 \
  --itch-multicast 239.255.0.1:9879
```

### Reference WebSocket alias coverage

| Legacy alias source | Example input | Native FIG path |
|---|---|---|
| Binance WS | `@trade`, `@depth`, `@kline_5m` | `marketdata/{symbol}/trades`, `/book`, `/candles/{interval}` |
| Hyperliquid WS | `trades`, `l2Book`, `userFunding` | same native paths (see `ws_catalog.rs`) |
| FIG native | — | `SUBSCRIBE` with `ChannelPath` directly |

Full mapping: [`crates/fig-gateways/src/ws_catalog.rs`](../crates/fig-gateways/src/ws_catalog.rs).

### Alias round-trip tests

One integration test file exercises **reference fixture tables** (not separate protocol tracks):

```bash
cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e
```

| Test | Fixture table | Legacy shape |
|---|---|---|
| `legacy_ws_binance_aliases_round_trip_through_backend` | Binance WS | `{"method":"SUBSCRIBE","params":["…@…"]}` |
| `legacy_ws_hyperliquid_aliases_round_trip_through_backend` | Hyperliquid WS | `{"method":"subscribe","subscription":{…}}` |
| `legacy_fix_order_aliases_round_trip_through_backend` | FIX 4.4 | `35=D` NewOrderSingle → native order `REQUEST` |
| `legacy_rest_get_aliases_round_trip_through_backend` | Native REST | `GET /.well-known/capabilities`, `GET /accounts/{acct}`, `GET /.well-known/instruments` |
| `legacy_ws_follow_up_event_after_fill` | Hyperliquid WS | Subscribe positions, fill on a second client, follow-up WS JSON |

WS subscribe snapshots use `proxy_subscribe_snapshot` / `BackendSession` (held-open
TREE stream). REST GET and FIX orders still use `proxy_frame`. Each case maps to
native FIG and asserts `StreamItem` / `Response` (no `StreamError`). See
[AGENTS.md](../AGENTS.md).

Performance clients (colo MMs) connect directly to the backend with SBE or CBOR —
see [SBE_ORDER_PATH.md](SBE_ORDER_PATH.md).

