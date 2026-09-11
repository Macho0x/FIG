# FIG Query Guide (Historical & Snapshot Pulls)

Historical and snapshot data use native FIG **`REQUEST` / `RESPONSE`** frames
(or **`REQUEST` → `STREAM_ITEM` × N** for large ranges). REST `GET` endpoints
are gateway adapters over the same FSL request/response types.

## Interaction pattern

```text
Client                          Exchange
  │                                │
  │── REQUEST (GET + path) ───────►│  ChannelPath + optional CBOR body
  │◄── RESPONSE (200 + CBOR) ──────│  *Batch or summary type
```

Large ranges use `request_stream`:

```text
  │── REQUEST (GET + path) ───────►│
  │◄── STREAM_ITEM (batch chunk) ───│  × N
  │◄── STREAM_CLOSE ───────────────│
```

Order history (>50 rows in exchange-sim) uses this pattern today.

## Query paths (§17.0b)

| Query | Native `CHANNEL_PATH` | Request type | Response type |
|---|---|---|---|
| Capabilities / exchange info | `/.well-known/capabilities` | `CapabilitiesRequest` | `CapabilitiesResponse` |
| Instrument catalog | `/.well-known/instruments` | (empty GET) | `InstrumentCatalogResponse` |
| Historical candles | `marketdata/{symbol}/candles/{interval}` | `CandleBarRequest` | `CandleBarBatch` |
| Public trade history | `marketdata/{symbol}/trades` | `TradeHistoryRequest` | `PublicTradeBatch` |
| Aggregate trade history | `marketdata/{symbol}/aggtrades` | `AggregateTradeRequest` | `AggregateTradeBatch` |
| Order book snapshot | `marketdata/{symbol}/book` | `OrderBookRequest` | `OrderBookSnapshot` |
| 24h ticker snapshot | `marketdata/{symbol}/ticker` | `TickerRequest` | `SymbolTicker` |
| All mids / mini ticker | `marketdata/ticker/all` | `AllMidsRequest` | `AllMidsBatch` |
| Mark price | `marketdata/{symbol}/mark` | `MarkPriceRequest` | `MarkPriceUpdate` |
| Account summary | `accounts/{account}` | (empty GET) | `AccountSummary` |
| Margin summary | `accounts/{account}/margin` | (empty GET) | `MarginSummary` |
| Open orders | `trading/accounts/{account}/orders/open` | `OpenOrdersRequest` | `OpenOrdersSnapshot` |
| Order history | `trading/accounts/{account}/orders` | `OrderHistoryRequest` | `OrderHistoryBatch` (+ stream) |
| Fill history | `accounts/{account}/fills` | `FillHistoryRequest` | `FillHistoryBatch` |
| Funding history | `accounts/{account}/funding` | `FundingHistoryRequest` | `FundingHistoryBatch` |
| Ledger history | `accounts/{account}/ledger` | `LedgerHistoryRequest` | `LedgerHistoryBatch` |
| Position snapshot | `accounts/{account}/positions` | `PositionRequest` | `PositionSnapshot` |

## Request fields

Common optional fields on batch requests:

| Field | Aliases (gateway) | Description |
|---|---|---|
| `start_time` | `start`, `startTime` | Inclusive start (nanoseconds since epoch) |
| `end_time` | `end`, `endTime` | Inclusive end |
| `limit` | `limit` | Max rows (default 500 in gateway) |
| `cursor` | `cursor` | Pagination cursor from prior response |

## REST gateway mapping

`fig_gateways::rest_query::http_get_to_fig_request` maps:

- Native: `GET /marketdata/BTC/candles/5m?start=…&limit=100`
- Binance alias: `GET /api/v3/klines?symbol=BTCUSDT&interval=5m&startTime=…`
- Instruments: `GET /.well-known/instruments`
- Open orders: `GET /trading/accounts/DEMO-ACCT/orders/open`
- Order history: `GET /trading/accounts/DEMO-ACCT/orders?limit=100`
- Fills: `GET /accounts/DEMO-ACCT/fills`

All produce FIG `ChannelPath` and optional CBOR request bodies.

Run the gateway with backend proxy:

```bash
cargo run -p fig-exchange-sim &
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
curl 'http://127.0.0.1:8080/.well-known/capabilities'
curl 'http://127.0.0.1:8080/.well-known/instruments'
```

Without `--fig-backend`, the gateway returns a translation demo (no live data).

## Native SDK (same paths, no HTTP)

```rust
let caps = client.request_capabilities(1).await?;
assert!(!caps.paths.is_empty());

let catalog = client.request_instruments(2).await?;
println!("{} instruments", catalog.instruments.len());

let fills = client
    .request_fills(
        account,
        FillHistoryRequest {
            account: account.to_string(),
            symbol: None,
            start_time: None,
            end_time: None,
            limit: Some(50),
            cursor: None,
        },
        3,
    )
    .await?;
println!("{} fills", fills.fills.len());
```

`request_*` uses `send_and_read` (finish send, read until EOF). That is correct
for GET/POST and **wrong** for live `SUBSCRIBE` — see [STREAMING.md](STREAMING.md).

## Pagination

Batch responses include `has_more` and optional `next_cursor` on each batch type.
The shared `PageInfo` struct in FSL documents the pattern; cursor propagation
across all handlers is still partial in the reference broker.

## Auth

Private queries (`accounts/{account}/…`, `trading/accounts/{account}/…`) use
the same **wire auth** as private streams: `AUTH_TOKEN` on each frame, principal
must match `{account}` in the path. Cross-account reads are rejected. See SPEC
§9.3.

The reference simulator checks `fig-dev-{account}` as a **test harness** only.
Production venues verify bearer/JWT/mTLS credentials their own systems issued;
FIG does not define key generation or storage.

## See also

- [STREAMING.md](STREAMING.md) — live `SUBSCRIBE` paths
- [PROTOCOL.md](PROTOCOL.md) — worked REQUEST sequences
- [SPEC.md](../SPEC.md) §9.2 — historical interaction patterns
- [ADR 0006](adr/0006-broker-api-parity.md) — native-first policy
