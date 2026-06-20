# FIG Streaming Guide

Native FIG live data uses **TREE channels**, `SUBSCRIBE` frames, and `STREAM_ITEM`
payloads. WebSocket and SSE are gateway edges that map to the same FSL types.

## Interaction pattern

```text
Client                          Exchange
  │                                │
  │── STREAM_OPEN (channel N) ────►│
  │── SUBSCRIBE routing_key ──────►│  (+ ChannelPath extension)
  │◄── STREAM_ITEM (snapshot) ─────│  optional is_snapshot: true
  │◄── STREAM_ITEM (updates) ──────│
  │── STREAM_CLOSE ───────────────►│
```

## Public market data paths

| Stream | Native `CHANNEL_PATH` | FSL `stream_item` | Gateway WS alias |
|---|---|---|---|
| Order book / quotes | `marketdata/{symbol}/quotes` | `MarketDataSnapshot` / incremental | Binance `@depth` |
| Best bid/offer | `marketdata/{symbol}/bbo` | `BestBidOffer` | Binance `@bookTicker` |
| Public trades | `marketdata/{symbol}/trades` | `PublicTradeEvent` | Binance `@trade` |
| Candles (OHLCV) | `marketdata/{symbol}/candles/{interval}` | `CandleBarEvent` | Binance `@kline_{interval}` |

Intervals use FIG names: `1m`, `5m`, `1h`, `1d` (not “klines”).

## Private account paths

| Stream | Native `CHANNEL_PATH` | FSL payload |
|---|---|---|
| Executions / fills | `trading/accounts/{account}/executions` | `ExecutionReport` |
| Balances | `accounts/{account}/balances` | `BalanceSnapshot` / `BalanceUpdate` |
| Positions | `accounts/{account}/positions` | `PositionSnapshot` / `PositionUpdate` |

Private streams require auth (`AUTH_TOKEN` extension or mTLS). Account in the
path must match the authenticated principal.

## Subscribe example (conceptual)

1. Open channel with `ChannelMode::Session` and schema id `0x01`.
2. Send `SUBSCRIBE` with:
   - `RoutingKey`: `marketdata/BTC/candles/5m` (or legacy `btcusdt@kline_5m` at gateway)
   - `ChannelPath`: `marketdata/BTC/candles/5m`
3. Receive `STREAM_ITEM` frames with CBOR `CandleBarEvent` bodies.

## Gateway WebSocket

Use `fig_gateways::ws_catalog`:

- `legacy_ws_json_to_fig_subscribe` — Binance/Hyperliquid JSON → FIG `SUBSCRIBE`
- `fig_stream_item_to_legacy_json` — FIG `STREAM_ITEM` → legacy JSON envelope

See [GATEWAY.md](GATEWAY.md) for deployment and [QUERY.md](QUERY.md) for historical pulls.

## Gap fill

If a live candle or book stream detects a sequence gap:

1. Issue native `REQUEST` with `CandleBarRequest` (or book snapshot request).
2. Resume `SUBSCRIBE` after backfill completes.

Native FIG is the source of truth; do not backfill over REST-only APIs.
