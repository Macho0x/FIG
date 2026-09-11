# FIG Protocol Guide

Deep dive beyond [SPEC.md](../SPEC.md): design rationale, channel model, and
worked examples for production integrations.

## Design Goals

FIG unifies FIX session semantics, REST resource routing, and WebSocket
streaming over a single multiplexed TREE transport. Key goals:

- **Zero-RTT resumption** — sessions survive disconnects via resumption tokens.
- **Schema-native payloads** — CBOR for flexibility, SBE for zero-alloc hot paths.
- **Channel isolation** — each logical stream maps to a TREE bidirectional or
  unidirectional stream with independent sequence numbers.
- **Native FIG first** — REST and WebSocket are gateway edges over the same FSL
  types; see [ADR 0006](adr/0006-broker-api-parity.md).

## Connection Lifecycle

1. TLS 1.3 handshake over TREE (ALPN `fig/1`).
2. Optional 0-RTT session resumption with stored resumption token.
   `FigServer::accept_0rtt` returns the first application frame if one arrived —
   the reference broker dispatches it (do not drop it).
3. Control channel (ID 0) for PING/PONG, SETTINGS, AUTH_REFRESH, SEQ_RESET.
4. Application channels opened via STREAM_OPEN or implicit gateway mapping.

`SUBSCRIBE` streams stay open after the snapshot. Later `STREAM_ITEM`s are sent
to **that subscriber's** `FigConnection`. `REQUEST` streams half-close after
the response (finish + EOF). Gateway REST GET uses the request path; live WS
uses a held-open `BackendSession`.

## Channel Directions

| Direction | Client offset | Server offset | Use case |
|---|---|---|---|
| Bidirectional | 0 | 1 | Request/response, FIX sessions |
| Send-only | 2 | 3 | Client order entry firehose |
| Recv-only | 3 | 2 | Server market data push |

Stream ID formula: `channel_id × 4 + offset`.

## Connection Migration

When the underlying IP changes (mobile clients, load balancer failover):

1. Call `FigConnection::prepare_migration()` before disconnect.
2. Reconnect via TREE migration or fresh handshake with resumption token.
3. Call `FigConnection::apply_migration()` to restore channel sequence state.

See `fig_core::migration` for token format and validation. Session stores
(`MemorySessionStore`, `FileSessionStore`, optional `RedisSessionStore` with
`fig-core` feature `session-redis`) persist channel and sequence state across
restarts. See [DEPLOYMENT.md](DEPLOYMENT.md) for multi-node layouts.

## Security Primitives (wire validation)

FIG libraries include **validators** for credentials already issued by your
venue. They do not implement key generation, admin APIs, or account onboarding.

| Module | Purpose |
|---|---|
| `fig_core::jwt` | JWT bearer decode/verify (HS256 for tests; venues use RS256 + KMS) |
| `fig_core::oauth` | OAuth2/OIDC dev introspection stub |
| `fig_core::channel_auth` | Per-channel permission requirements |
| `fig_core::rate_limit` | Token-bucket per-channel rate limiting |
| `fig_core::dos` | Connection-level DoS guard and flood detection |

Private account paths require `AUTH_TOKEN` whose principal matches the
`{account}` segment in `ChannelPath`. See SPEC §9.3. Credential lifecycle
(issue, rotate, revoke) is venue infrastructure — outside FIG.

## Worked Example: Order Entry

```text
Client                              Server
  | STREAM_OPEN (channel 1)            |
  |---------------------------------->|
  | REQUEST /orders (SBE payload)    |
  |---------------------------------->|
  | STREAM_ITEM (ExecutionReport)      |
  |<----------------------------------|
  | RESPONSE 200                       |
  |<----------------------------------|
```

## Worked Example: Public Market Data Subscribe (Candles)

Subscribe to live OHLCV bars on `marketdata/BTC/candles/5m`. No auth required.

```text
Client (channel 3)                    Exchange (fig-exchange-sim)
  |                                    |
  | STREAM_OPEN (session mode)         |
  |----------------------------------->|
  | SUBSCRIBE                          |
  |   RoutingKey: marketdata/BTC/candles/5m
  |   ChannelPath: marketdata/BTC/candles/5m
  |   SchemaId: 0x01
  |----------------------------------->|
  | STREAM_ITEM (CandleBarEvent)       |  partial bar, is_snapshot optional
  |<-----------------------------------|
  | STREAM_ITEM (CandleBarEvent)       |  bar update on each trade
  |<-----------------------------------|
  | STREAM_ITEM (CandleBarEvent)       |  final bar when interval closes
  |<-----------------------------------|
  | UNSUBSCRIBE                        |
  |----------------------------------->|
  | STREAM_CLOSE                       |
  |<-----------------------------------|
```

**Frame details:**

1. Open a session channel (`ChannelMode::Session`, schema `0x01`).
2. Send `SUBSCRIBE` with both `ROUTING_KEY` and `CHANNEL_PATH` set to the native
   path (gateway WS clients use legacy keys like `btcusdt@kline_5m` — mapped by
   `fig_gateways::ws_catalog::binance_topic_to_subscribe`).
3. Decode each `STREAM_ITEM` payload as CBOR `CandleBarEvent` (contains
   `CandleBar` + `is_final`).
4. On sequence gap, issue `CandleBarRequest` for the missing window (see below),
   then resume the subscription.

**Gap fill after disconnect:**

```text
  | REQUEST GET marketdata/BTC/candles/5m  (CandleBarRequest body)
  |----------------------------------->|
  | RESPONSE (CandleBarBatch)          |
  |<-----------------------------------|
  | SUBSCRIBE (same path)              |
  |----------------------------------->|
```

## Worked Example: Private Account Subscribe (Balances + Executions)

Account `DEMO-ACCT` requires `AUTH_TOKEN` on subscribe frames, scoped to that
account in the path. The simulator uses test token `fig-dev-DEMO-ACCT`; production
venues verify JWT or mTLS per SPEC §9.3 (credentials issued outside FIG).

```text
Client (channel 5)                    Exchange
  |                                    |
  | SUBSCRIBE                          |
  |   ChannelPath: accounts/DEMO-ACCT/balances
  |   AuthToken: fig-dev-DEMO-ACCT
  |----------------------------------->|
  | STREAM_ITEM (BalanceSnapshot)      |  snapshot on subscribe
  |<-----------------------------------|
  |                                    |
  | SUBSCRIBE (channel 6)              |
  |   ChannelPath: trading/accounts/DEMO-ACCT/executions
  |   AuthToken: fig-dev-DEMO-ACCT
  |----------------------------------->|
  |                                    |
  | REQUEST (NewOrderSingle on ch 1)   |
  |----------------------------------->|
  | STREAM_ITEM (ExecutionReport)      |  ch 6 — fill notification
  |<-----------------------------------|
  | STREAM_ITEM (BalanceUpdate)        |  ch 5 — balance delta on fill
  |<-----------------------------------|
  | STREAM_ITEM (PositionUpdate)       |  ch 7 if positions subscribed
  |<-----------------------------------|
```

**Auth rules:**

- Token account suffix MUST match path `{account}` or the broker rejects the frame.
- Set `FIG_DEV_OPEN=1` on the simulator to skip auth during local development.
- Margin, funding, ledger, and liquidations use the same auth model on their
  respective paths (see [STREAMING.md](STREAMING.md)).

## Worked Example: Historical Query (Fill History)

```text
Client                              Exchange
  | REQUEST GET accounts/DEMO-ACCT/fills
  |   AuthToken: fig-dev-DEMO-ACCT
  |   Payload: FillHistoryRequest { limit: 100, … }
  |----------------------------------->|
  | RESPONSE 200 (FillHistoryBatch)    |
  |<-----------------------------------|
```

Large order history responses use `request_stream`:

```text
  | REQUEST GET trading/accounts/DEMO-ACCT/orders
  |----------------------------------->|
  | STREAM_ITEM (OrderHistoryBatch)    |  chunk 1
  |<-----------------------------------|
  | STREAM_ITEM (OrderHistoryBatch)    |  chunk 2
  |<-----------------------------------|
  | STREAM_CLOSE                       |
  |<-----------------------------------|
```

See [QUERY.md](QUERY.md) for REST gateway mapping and pagination fields.

## Codec Selection

| Payload | When to use |
|---|---|
| SBE | Production trading hot path (fixed schema) |
| CBOR | Self-describing, gateway translation, stream payloads |
| Protobuf | Schema-evolving enterprise integrations |
| JSON | REST gateway edge only |

## Session resume and gap fill

- **Resume:** persisted subscriptions are replayed when the client sends `SUBSCRIBE` with `Method: RESUME` and `ChannelPath: .well-known/resume` (same `SESSION_ID` as before disconnect when available).
- **Gap fill:** after a sequence gap on a live book or candle stream, issue `OrderBookRequest` or `CandleBarRequest` for a snapshot, then resume the subscription.
- **`request_stream`:** large historical responses (order history when >50 rows) arrive as multiple `STREAM_ITEM` frames plus `STREAM_CLOSE` instead of one `RESPONSE`.

## Further Reading

- [SPEC.md](../SPEC.md) — normative wire format (§9 stream catalog, §9.3 auth)
- [TUTORIAL.md](TUTORIAL.md) — getting started and stream inventory
- [STREAMING.md](STREAMING.md) — live subscribe paths and WS catalog
- [QUERY.md](QUERY.md) — historical REQUEST paths and REST GET mapping
- [GATEWAY.md](GATEWAY.md) — legacy protocol deployment (Day 1 gateway)
- [SBE_ORDER_PATH.md](SBE_ORDER_PATH.md) — colo SBE order entry
- [adr/](adr/) — architecture decision records ([0004: FSL source of truth](adr/0004-fsl-single-source-of-truth.md), [0006: broker API parity](adr/0006-broker-api-parity.md), [0007: crypto instruments](adr/0007-crypto-instrument-model.md))

## Gateway alias mapping (reference venues)

Native FIG clients use `CHANNEL_PATH` directly. The gateway maps **legacy wire shapes**
to those paths — see [AGENTS.md](../AGENTS.md). Binance and Hyperliquid are reference
implementations in `fig-gateways`; venues do not fork the protocol.

| Native `CHANNEL_PATH` | Binance WS alias | Hyperliquid WS alias | FIX alias |
|---|---|---|---|
| `marketdata/{sym}/trades` | `@trade` | `trades` | — |
| `marketdata/{sym}/book` | `@depth` | `l2Book`, `bbo` | — |
| `marketdata/{sym}/candles/{iv}` | `@kline_{iv}` | `candle` | — |
| `trading/accounts/{acct}/executions` | `@executionReport` | `orderUpdates` | `ExecutionReport` (egress) |
| `trading/accounts/{acct}/orders` POST | — | — | `35=D` NewOrderSingle |
| `accounts/{acct}/funding` | — | `userFunding` | — |
| `marketdata/ticker/all` | `@miniTicker` | `allMids` | — |

Alias round-trip: `cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e`.
