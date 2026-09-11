# FIG Streaming Guide

Native FIG live data uses **TREE channels**, `SUBSCRIBE` frames, and `STREAM_ITEM`
payloads. WebSocket and SSE are gateway edges that map to the same FSL types.
Native FIG is the source of truth — see [ADR 0006](adr/0006-broker-api-parity.md).

## Interaction pattern

```text
Client                          Exchange
  │                                │
  │── STREAM_OPEN (channel N) ────►│
  │── SUBSCRIBE routing_key ──────►│  (+ ChannelPath extension)
  │◄── STREAM_ITEM (snapshot) ─────│  optional is_snapshot: true
  │◄── STREAM_ITEM (updates) ──────│  same TREE stream, held open
  │── UNSUBSCRIBE ────────────────►│  optional (new request stream)
  │── STREAM_CLOSE ───────────────►│
```

The reference broker writes the snapshot/ack and **leaves the subscribe send
stream open**. Later `STREAM_ITEM`s (fills, book deltas, balance updates) are
`send_frame`d to **that subscriber's** `FigConnection`, not piggybacked on the
trader's order stream. `FigSdkClient::subscribe_*` reads the snapshot without
waiting for EOF; hold `LiveSubscription` to read follow-up frames. REQUEST
streams still half-close after the response.

## Public market data paths

| Stream | Native `CHANNEL_PATH` | FSL `stream_item` | Gateway WS alias |
|---|---|---|---|
| Order book snapshot / delta | `marketdata/{symbol}/book` | `OrderBookSnapshot`, `OrderBookDelta` | Binance `@depth` |
| Order book / quotes (legacy) | `marketdata/{symbol}/quotes` | `MarketDataSnapshot`, incremental | Binance `@depth` |
| Best bid/offer | `marketdata/{symbol}/bbo` | `BestBidOffer` | Binance `@bookTicker` |
| Public trades | `marketdata/{symbol}/trades` | `PublicTradeEvent` | Binance `@trade` |
| Aggregate trades | `marketdata/{symbol}/aggtrades` | `AggregateTradeEvent` | Binance `@aggTrade` |
| Candles (OHLCV) | `marketdata/{symbol}/candles/{interval}` | `CandleBarEvent` | Binance `@kline_{interval}` |
| 24h ticker | `marketdata/{symbol}/ticker` | `SymbolTicker` | Binance `@ticker` |
| Mini ticker / all mids | `marketdata/ticker/all` | `MiniTicker`, `AllMids` | Binance `@miniTicker`, HL `allMids` |
| Mark / index price | `marketdata/{symbol}/mark` | `MarkPriceUpdate` | Binance `@markPrice`, HL `activeAssetCtx` |
| Public liquidations | `marketdata/liquidations` | `LiquidationTrade` | Binance `@forceOrder` |

Intervals use FIG names: `1m`, `5m`, `1h`, `1d` (not “klines”).

## Private account paths

Private paths require `AUTH_TOKEN` on every frame, with the authenticated
principal matching `{account}` in the path (SPEC §9.3). FIG defines this wire
contract; **credential issuance** (API keys, JWT signing, admin UI) is venue
infrastructure — not part of the protocol.

| Stream | Native `CHANNEL_PATH` | FSL payload |
|---|---|---|
| Executions / fills | `trading/accounts/{account}/executions` | `ExecutionReport` |
| Balances | `accounts/{account}/balances` | `BalanceSnapshot` / `BalanceUpdate` |
| Positions | `accounts/{account}/positions` | `PositionSnapshot` / `PositionUpdate` |
| Margin | `accounts/{account}/margin` | `MarginSummary` (GET) / `MarginUpdate` (stream) |
| Funding payments | `accounts/{account}/funding` | `FundingPayment` |
| Ledger | `accounts/{account}/ledger` | `LedgerUpdate` |
| User liquidations | `accounts/{account}/liquidations` | `UserLiquidation` (push on breach) |
| Order lists | `trading/accounts/{account}/orderlists` | `OrderListStatus` |

### SDK merge helpers (`fig-client`)

Use `FigSdkClient::subscribe_*` and merge states for live streams:

- `OrderBookState` — `subscribe_order_book`
- `AggTradeState` — `subscribe_agg_trades`
- `FundingState` — `subscribe_funding`
- `LedgerState` — `subscribe_ledger`
- `LiquidationState` — `subscribe_liquidations`
- `AccountCache` — `subscribe_positions` / `subscribe_balances`
- `subscribe_ticker`, `subscribe_orderlists` — snapshot helpers

Hold `LiveSubscription` (from `subscribe_live`) when you need frames after the
initial snapshot. Batch apply via `FigSdkClient::apply_account_stream_frames` or
`apply_all_stream_frames`.

## Subscribe example (conceptual)

1. Open channel with `ChannelMode::Session` and schema id `0x01`.
2. Send `SUBSCRIBE` with:
   - `RoutingKey`: `marketdata/BTC/candles/5m` (or legacy `btcusdt@kline_5m` at gateway)
   - `ChannelPath`: `marketdata/BTC/candles/5m`
3. Receive `STREAM_ITEM` frames with CBOR `CandleBarEvent` bodies.

For private streams, attach `AUTH_TOKEN` on the frame; the receiver verifies a
credential your venue already issued and rejects path/account mismatches.

## Gateway WebSocket

Use `fig_gateways::ws_catalog`:

- `legacy_ws_json_to_fig_subscribe` — Binance/Hyperliquid JSON → FIG `SUBSCRIBE`
- `fig_stream_item_to_legacy_json` — FIG `STREAM_ITEM` → legacy JSON envelope
- `binance_topic_to_subscribe` / `hyperliquid_subscribe_to_fig` — topic mapping
- `backend::BackendSession` — persistent TREE connection for live WS follow-up frames

Run with backend proxy:

```bash
cargo run -p fig-exchange-sim &
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e
```

See [GATEWAY.md](GATEWAY.md) for deployment and [QUERY.md](QUERY.md) for historical pulls.

## Gap fill

If a live candle or book stream detects a sequence gap:

1. Send `CONTROL(SEQ_RESET)` on the affected channel if the peer advertised a reset point.
2. Issue native `REQUEST` with `CandleBarRequest` or `OrderBookRequest` (snapshot with `sequence` + `is_snapshot: true`).
3. Resume `SUBSCRIBE` after backfill completes, or send `SUBSCRIBE` with `Method: RESUME` and `ChannelPath: .well-known/resume` to restore persisted subscriptions.

Native FIG is the source of truth; do not backfill over REST-only APIs.

## Session resume

Subscriptions are persisted on the server session store. After reconnect, send:

```text
SUBSCRIBE  Method=RESUME  ChannelPath=.well-known/resume
```

The broker replays persisted `ChannelPath` / `RoutingKey` pairs as snapshot `STREAM_ITEM` frames before live updates continue.

## Large historical ranges (`request_stream`)

When a query result exceeds 50 rows (exchange-sim default), the broker emits multiple `STREAM_ITEM` chunks followed by `STREAM_CLOSE` instead of a single `RESPONSE`. Clients must merge chunks until `StreamClose`. Order history uses this path today; other batch types may follow.

## See also

- [PROTOCOL.md](PROTOCOL.md) — worked subscribe sequences
- [SPEC.md](../SPEC.md) §9.1 — normative path catalog
- [TUTORIAL.md](TUTORIAL.md) — hands-on stream inventory
