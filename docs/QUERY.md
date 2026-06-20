# FIG Query Guide (Historical & Snapshot Pulls)

Historical and snapshot data use native FIG **`REQUEST` / `RESPONSE`** frames.
REST `GET` endpoints are gateway adapters over the same FSL request/response types.

## Interaction pattern

```text
Client                          Exchange
  │                                │
  │── REQUEST (GET + path) ───────►│  ChannelPath + optional CBOR body
  │◄── RESPONSE (200 + CBOR) ──────│  *Batch or summary type
```

Large ranges may use `REQUEST` → multiple `STREAM_ITEM` frames (future).

## Query paths (§17.0b)

| Query | Native `CHANNEL_PATH` | Request type | Response type |
|---|---|---|---|
| Historical candles | `marketdata/{symbol}/candles/{interval}` | `CandleBarRequest` | `CandleBarBatch` |
| Public trade history | `marketdata/{symbol}/trades` | `TradeHistoryRequest` | `PublicTradeBatch` |
| Account summary | `accounts/{account}` | (empty GET) | `AccountSummary` |
| Margin summary | `accounts/{account}/margin` | (empty GET) | `MarginSummary` |
| Fill history | `accounts/{account}/fills` | `FillHistoryRequest` | `FillHistoryBatch` |

## Request fields

Common optional fields on batch requests:

| Field | Aliases (gateway) | Description |
|---|---|---|
| `start_time` | `start`, `startTime` | Inclusive start (nanoseconds since epoch) |
| `end_time` | `end`, `endTime` | Inclusive end |
| `limit` | `limit` | Max rows (default 500) |

## REST gateway mapping

`fig_gateways::rest_query::http_get_to_fig_request` maps:

- Native: `GET /marketdata/BTC/candles/5m?start=…&limit=100`
- Binance alias: `GET /api/v3/klines?symbol=BTCUSDT&interval=5m&startTime=…`

Both produce the same FIG `ChannelPath` and `CandleBarRequest` CBOR payload.

Run the gateway with backend proxy:

```bash
cargo run -p fig-exchange-sim &
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
```

Without `--fig-backend`, the gateway returns a translation demo (no live data).

## Pagination

Batch responses include `has_more` and optional `next_cursor`. Clients should
pass cursor values on subsequent requests when implemented in FSL.

## Auth

Private queries (`accounts/{account}/…`) require the same scopes as private
streams. Cross-account reads are rejected.

## See also

- [STREAMING.md](STREAMING.md) — live `SUBSCRIBE` paths
- [ADR 0006](adr/0006-broker-api-parity.md) — native-first policy
- [PROTOCOL.md](PROTOCOL.md) — frame sequences
