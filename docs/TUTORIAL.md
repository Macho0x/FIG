# FIG Getting Started Tutorial

This guide walks through connecting to the FIG exchange simulator, placing
an order, and subscribing to market data.

## Prerequisites

- Rust 1.75+ (`rustup` recommended)
- Linux, macOS, or Windows (CI-tested on all three)

```bash
git clone https://github.com/Macho0x/fig.git
cd fig
cargo build --workspace
cargo test --workspace
```

## 1. Start the exchange simulator

```bash
cargo run -p fig-exchange-sim
```

The server binds to **`127.0.0.1:8443`** (UDP/TREE) by default and persists
sessions under `/tmp/fig-exchange-sessions`.

Optional mTLS mode:

```bash
FIG_MTLS=1 cargo run -p fig-exchange-sim
```

## 2. Run the native client

In another terminal:

```bash
cargo run -p fig-cli
```

The client connects to `127.0.0.1:8443` (override with `FIG_SERVER=host:port`)
and runs six demos: order entry, candle subscribe, balance subscribe,
historical candle query, account/ticker/funding/ledger queries, and PING/PONG.

Private account paths require an `AUTH_TOKEN` extension. The simulator accepts
`fig-dev-{account}` (e.g. `fig-dev-DEMO-ACCT` for the CLI default account).
Set `FIG_DEV_OPEN=1` on the server to skip auth during local development.

## 3. Run the legacy gateway (optional)

Translate FIX, REST, and WebSocket into native FIG frames:

```bash
# Terminal 1: backend
cargo run -p fig-exchange-sim

# Terminal 2: gateway with backend proxy
cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443
```

| Listener | Address | Purpose |
|---|---|---|
| REST | `http://127.0.0.1:8080` | HTTP GET → FIG `REQUEST` (with `--fig-backend`) |
| WebSocket | `ws://127.0.0.1:8090` | Legacy subscribe JSON → FIG `SUBSCRIBE` |
| FIX | `tcp://127.0.0.1:9876` | FIX 4.4 → FIG trading frames |

See [GATEWAY.md](GATEWAY.md), [STREAMING.md](STREAMING.md), and [QUERY.md](QUERY.md).

Example REST historical query (requires `--fig-backend`):

```bash
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
```

Example REST order (translation demo without backend):

```bash
curl -X POST http://127.0.0.1:8080/trading/orders \
  -H 'Content-Type: application/json' \
  -d '{"cl_ord_id":"ORD-1","symbol":"AAPL","side":"Buy","order_qty":100,"price":150.25}'
```

## 4. Prometheus metrics

Export FIG counters for scraping:

```bash
cargo run -p fig-core --features observability-bin --bin fig-observability
curl http://127.0.0.1:9090/metrics
```

## 5. Compile FSL schemas

```bash
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang rust --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang go --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- compile schemas/orders.fsl --lang proto --out /tmp/fig-gen
cargo run -p fig-fsl --bin ftlc -- validate schemas/orders.fsl
```

Supported `--lang` values:

`rust`, `sbe`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`, `python`, `typescript`, `ocaml`, `zig`, `json-schema`, `fix-yaml`

After changing a schema, regenerate all targets — see
[ADR 0004 — FSL as Single Source of Truth](adr/0004-fsl-single-source-of-truth.md)
and [CONTRIBUTING.md](../CONTRIBUTING.md#schema-changes).

## 6. TCP downgrade mode

For environments without TREE support:

```rust
use fig_core::tcp::{FigTcpConnection, FigTcpServer};

// Server accepts FIG\x01 magic prefix, single-stream multiplexing
```

See `fig_core::tcp` module documentation and unit tests for round-trip examples.

## 7. Authentication

### Bearer / JWT

```rust
use fig_core::jwt::{encode_jwt, verify_jwt_bearer, FigJwtClaims};

let claims = FigJwtClaims::new("trader-1", exp_unix_secs, vec!["orders:write".into()]);
let token = encode_jwt(&claims, "shared-secret")?;
let auth = verify_jwt_bearer(&token, "shared-secret")?;
```

### OAuth2 (dev introspection)

```rust
use fig_core::oauth::{OAuthValidator, OAuthTokenInfo};

let validator = OAuthValidator::new("https://auth.example.com")
    .register_token("access-token", OAuthTokenInfo { /* … */ });
let auth = validator.validate("access-token")?;
```

### Per-channel permissions

```rust
use fig_core::ChannelAuthPolicy;

let policy = ChannelAuthPolicy::new()
    .require_for_channel(5, vec!["marketdata:read".into()]);
assert!(policy.authorize(5, &auth));
```

### Exchange simulator dev tokens

Private streams and queries on `fig-exchange-sim` require an `AUTH_TOKEN`
extension matching `fig-dev-{account}`. Example for account `DEMO-ACCT`:

```rust
frame.with_extension(Extension::text(
    ExtensionTag::AuthToken,
    "fig-dev-DEMO-ACCT",
));
```

Set `FIG_DEV_OPEN=1` when running the simulator to disable this check locally.

## 8. Run benchmarks

```bash
cargo bench -p fig-bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
```

Benchmark groups include frame codec, TREE round-trip latency, gateway
adapter comparison (FIX vs REST vs native FIG), matching engine throughput,
and allocation patterns.

## 9. Docker

```bash
docker build -t fig-exchange-sim .
docker run --rm -p 8443:8443/udp fig-exchange-sim
```

## Next steps

- Read [SPEC.md](../SPEC.md) for the full protocol specification (§9 stream catalog, §9.3 auth)
- Read [PROTOCOL.md](PROTOCOL.md) for worked subscribe/query sequences
- Read [STREAMING.md](STREAMING.md) and [QUERY.md](QUERY.md) for broker API parity
- See [API.md](API.md) for the module index
- Run integration tests: `cargo test -p fig-exchange-sim --test integration`

## 10. Broker API stream and query inventory

Native FIG paths implemented by `fig-exchange-sim`. Gateway WS/REST columns show
legacy aliases when running `fig-gateway --fig-backend`.

### Live streams (`SUBSCRIBE` → `STREAM_ITEM`)

| Category | Native path | FSL payload | Gateway WS (examples) |
|---|---|---|---|
| Book snapshot/delta | `marketdata/{sym}/book` | `OrderBookSnapshot`, `OrderBookDelta` | `@depth` |
| BBO | `marketdata/{sym}/bbo` | `BestBidOffer` | `@bookTicker` |
| Trades | `marketdata/{sym}/trades` | `PublicTradeEvent` | `@trade` |
| Agg trades | `marketdata/{sym}/aggtrades` | `AggregateTradeEvent` | `@aggTrade` |
| Candles | `marketdata/{sym}/candles/{iv}` | `CandleBarEvent` | `@kline_{iv}` |
| Ticker | `marketdata/{sym}/ticker` | `SymbolTicker` | `@ticker` |
| All mids | `marketdata/ticker/all` | `MiniTicker` | `@miniTicker` |
| Mark price | `marketdata/{sym}/mark` | `MarkPriceUpdate` | `@markPrice` |
| Public liq | `marketdata/liquidations` | `LiquidationTrade` | `@forceOrder` |
| Executions | `trading/accounts/{acct}/executions` | `ExecutionReport` | HL `orderUpdates` |
| Balances | `accounts/{acct}/balances` | `BalanceSnapshot`, `BalanceUpdate` | HL `spotState` |
| Positions | `accounts/{acct}/positions` | `PositionSnapshot`, `PositionUpdate` | HL `clearinghouseState` |
| Margin | `accounts/{acct}/margin` | `MarginUpdate` | HL `margin` |
| Funding | `accounts/{acct}/funding` | `FundingPayment` | HL `userFunding` |
| Ledger | `accounts/{acct}/ledger` | `LedgerUpdate` | HL `ledgerUpdates` |
| User liq | `accounts/{acct}/liquidations` | `UserLiquidation` | HL `liquidation` |

Private rows require `AUTH_TOKEN: fig-dev-{acct}` (or production JWT/mTLS).

### Historical queries (`REQUEST` → `RESPONSE` or `STREAM_ITEM` × N)

| Query | Native path | Request → response |
|---|---|---|
| Capabilities | `/.well-known/capabilities` | `CapabilitiesRequest` → `CapabilitiesResponse` |
| Candles | `marketdata/{sym}/candles/{iv}` | `CandleBarRequest` → `CandleBarBatch` |
| Trades | `marketdata/{sym}/trades` | `TradeHistoryRequest` → `PublicTradeBatch` |
| Agg trades | `marketdata/{sym}/aggtrades` | `AggregateTradeRequest` → `AggregateTradeBatch` |
| Book snapshot | `marketdata/{sym}/book` | `OrderBookRequest` → `OrderBookSnapshot` |
| Ticker | `marketdata/{sym}/ticker` | `TickerRequest` → `SymbolTicker` |
| Account | `accounts/{acct}` | GET → `AccountSummary` |
| Margin | `accounts/{acct}/margin` | GET → `MarginSummary` |
| Open orders | `trading/accounts/{acct}/orders/open` | `OpenOrdersRequest` → `OpenOrdersSnapshot` |
| Order history | `trading/accounts/{acct}/orders` | `OrderHistoryRequest` → `OrderHistoryBatch` (stream if large) |
| Fills | `accounts/{acct}/fills` | `FillHistoryRequest` → `FillHistoryBatch` |
| Funding hist | `accounts/{acct}/funding` | `FundingHistoryRequest` → `FundingHistoryBatch` |
| Ledger hist | `accounts/{acct}/ledger` | `LedgerHistoryRequest` → `LedgerHistoryBatch` |

Example gateway REST query:

```bash
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
curl 'http://127.0.0.1:8080/.well-known/capabilities'
```

See [PROTOCOL.md](PROTOCOL.md) for frame-by-frame worked examples.

