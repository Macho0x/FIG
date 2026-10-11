# FIG Getting Started Tutorial

This guide walks through connecting to the FIG exchange simulator, placing
an order, and subscribing to market data.

FIG is a **messaging protocol** (frames, paths, gateways). Credential issuance
(API keys, account portals) is venue infrastructure — not part of FIG. Private
frames carry an `AUTH_TOKEN` extension; your broker verifies credentials it
already issued (see §7 and [SPEC.md](../SPEC.md) §9.3).

## Prerequisites

- Rust 1.97+ (`rustup` recommended)
- Linux, macOS, or Windows (CI-tested on all three)

```bash
git clone https://github.com/Macho0x/fig.git
cd fig
cargo build --workspace
cargo test --workspace
```

## 1. Start the exchange simulator

```bash
cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem
```

The server binds to **`127.0.0.1:8443`** (UDP/TREE) by default and persists
sessions under `/tmp/fig-exchange-sessions`.

Optional mTLS mode — the server then requires a client certificate, verified
against `FIG_MTLS_CA` (without it, any client certificate is accepted; that is
development-only). `fig-cli` does not present a client certificate, so use this
with an integration that configures its own TLS client key:

```bash
FIG_MTLS=1 FIG_MTLS_CA=/path/to/client-ca.pem \
  cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem
```

## 2. Run the native client

In another terminal:

```bash
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-cli
cargo run -p fig-cli -- --help
```

The client connects to `127.0.0.1:8443` (override with `--server` or
`FIG_SERVER=host:port`) and runs **seven** demos: order entry, candle subscribe,
balance subscribe, historical candle query, account/ticker/funding/ledger plus
capabilities/instruments/fills queries, agg-trades/mark/margin streams, and
PING/PONG. Subscribe helpers read the snapshot without waiting for stream EOF.
Each step validates responses (no `STREAM_ERROR`, expected acks).

Private account paths require an `AUTH_TOKEN` extension on each frame (SPEC
§9.3). The reference simulator uses a **test harness token**
`fig-dev-{account}` (e.g. `fig-dev-DEMO-ACCT` for the CLI default account) —
not a key-issuance API. Set `FIG_DEV_OPEN=1` on the server to skip auth during
local development.

Minimal SDK equivalent of the CLI (simulator must be running):

```rust
use fig_client::FigSdkClient;
use fig_core::transport;
use quinn::Endpoint;

let mut ep = Endpoint::client("0.0.0.0:0".parse()?)?;
ep.set_default_client_config(transport::client_config_from_env()?);   // verifies TLS
let conn = ep.connect("127.0.0.1:8443".parse()?, "localhost")?.await?;
let client = FigSdkClient::new(&conn);

let caps = client.request_capabilities(1).await?;
let (_ticker, _) = client.request_ticker("AAPL", 2).await?;
println!("{} capability paths", caps.paths.len());
```

`request_*` waits for EOF (queries). `subscribe_*` reads the snapshot only.
For later `STREAM_ITEM`s, use `subscribe_live` — see [STREAMING.md](STREAMING.md).

## 3. Run the legacy gateway (optional)

Translate FIX, REST, WebSocket, and (opt-in) OUCH / ITCH into native FIG frames:

```bash
# Terminal 1: backend
cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem

# Terminal 2: gateway with backend proxy (FIX / REST / WebSocket)
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-gateways --bin fig-gateway -- --fig-backend 127.0.0.1:8443

# Same process, plus SoupBin OUCH and ITCH L2 egress
FIG_TLS_CA=.fig/dev-ca.pem cargo run -p fig-gateways --bin fig-gateway -- \
  --fig-backend 127.0.0.1:8443 \
  --ouch-addr 127.0.0.1:9877 \
  --itch-addr 127.0.0.1:9878 \
  --itch-multicast 239.255.0.1:9879
```

| Listener | Address | Purpose |
|---|---|---|
| REST | `http://127.0.0.1:8080` | HTTP GET → FIG `REQUEST` (with `--fig-backend`) |
| WebSocket | `ws://127.0.0.1:8090` | Legacy subscribe JSON → FIG `SUBSCRIBE` |
| FIX | `tcp://127.0.0.1:9876` | FIX 4.4 → FIG trading frames. Optional `--fix-dropcopy-account` egresses 35=8 |
| OUCH | `--ouch-addr` (example `127.0.0.1:9877`) | SoupBinTCP + OUCH 4.2-shaped enter/cancel/replace. Off unless set. Not NASDAQ byte-identical |
| ITCH | `--itch-addr` / `--itch-multicast` | L2 price-level egress (SoupBin replay and/or MoldUDP64). Off unless set. Not TotalView |

See [GATEWAY.md](GATEWAY.md), [STREAMING.md](STREAMING.md), and [QUERY.md](QUERY.md).

Example REST historical query (requires `--fig-backend`):

```bash
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
curl 'http://127.0.0.1:8080/.well-known/instruments'
```

Example WebSocket subscribe (requires `--fig-backend` and a WS client):

```bash
# Binance-shaped alias → native `marketdata/AAPL/ticker`
printf '{"method":"SUBSCRIBE","params":["aapl@ticker"]}\n' \
  | websocat ws://127.0.0.1:8090
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

`rust`, `sbe`, `sbe-go`, `sbe-cpp`, `sbe-csharp`, `sbe-typescript` (`sbe-ts`), `sbe-zig`, `go`, `proto`, `sbe-xml`, `cpp`, `csharp`, `python`, `typescript`, `ocaml`, `zig`, `java`, `json-schema`, `fix-yaml`, `protocol-cpp`, `protocol-zig`

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

## 7. Authentication (wire semantics)

FIG defines **how credentials travel on frames** and **how receivers scope them
to paths** — the same layer as FIX Logon or REST `Authorization:`. Key
generation, storage, and admin UI are **not** FIG; your venue issues credentials
out of band and verifies them when private frames arrive.

### Validating bearer / JWT (reference helpers)

`fig_core::jwt` provides encode/decode for tests and conformance — production
venues verify JWTs their own identity service signed:

```rust
use fig_core::jwt::{encode_jwt, verify_jwt_bearer, FigJwtClaims};

let claims = FigJwtClaims::new("trader-1", exp_unix_secs, vec!["orders:write".into()]);
let token = encode_jwt(&claims, "shared-secret")?;
let auth = verify_jwt_bearer(&token, "shared-secret")?;
// Attach `token` in ExtensionTag::AuthToken on private frames; map claims.sub → {account} in path
```

### OAuth2 (dev introspection stub)

```rust
use fig_core::oauth::{OAuthValidator, OAuthTokenInfo};

let validator = OAuthValidator::new("https://auth.example.com")
    .register_token("access-token", OAuthTokenInfo { /* … */ });
let auth = validator.validate("access-token")?;
```

Production venues wire real OAuth/OIDC introspection; FIG does not define the IdP.

### Per-channel permissions (optional)

```rust
use fig_core::ChannelAuthPolicy;

let policy = ChannelAuthPolicy::new()
    .require_for_channel(5, vec!["marketdata:read".into()]);
assert!(policy.authorize(5, &auth));
```

### Exchange simulator test tokens

The reference server (`fig-exchange-sim`) checks a predictable dev token so
integration tests can run without a credential store. This is **not** part of
the FIG protocol product:

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
cargo run --release -p fig-bench --bin fig-latency
```

Criterion microbenches cover frame codec, TREE round-trip latency, gateway
adapter comparison (FIX vs REST vs native FIG), matching engine throughput,
and allocation patterns.

The `fig-latency` binary reports **tail latency percentiles** (p50 / p99 /
p99.9) using HDR Histogram — one sample per operation. Use
`FIG_LATENCY_ITERS=1000` for a quick smoke run. See [BENCHMARKS.md](BENCHMARKS.md).

## 9. Docker

```bash
docker build -t fig-exchange-sim .
docker run --rm -p 8443:8443/udp fig-exchange-sim
```

The container's simulator presents a self-signed certificate. Local clients either pin it
(`--print-ca` on the simulator, then `FIG_TLS_CA` on the client) or, for development only, set
`FIG_TLS_INSECURE=1`. Production deployments use `FIG_TLS_CA` with the venue CA
([DEPLOYMENT.md](DEPLOYMENT.md)).

## Next steps

- [docs/README.md](README.md) — what to read next (SUBSCRIBE vs REQUEST)
- Read [SPEC.md](../SPEC.md) for the full protocol specification (§9 stream catalog, §9.3 auth)
- Read [PROTOCOL.md](PROTOCOL.md) for worked subscribe/query sequences
- Read [STREAMING.md](STREAMING.md) and [QUERY.md](QUERY.md) for broker API parity
- See [API.md](API.md) for the module index
- Run integration tests: `cargo test -p fig-exchange-sim --test integration`

## 10. Broker API stream and query inventory

Native FIG paths implemented by `fig-exchange-sim`. Gateway WS/REST columns show
legacy aliases when running `fig-gateway --fig-backend`. OUCH (`--ouch-addr`)
enters, cancels, and replaces on `trading/accounts/{acct}/orders`. ITCH
(`--itch-addr`, `--itch-multicast`) egresses `book` and `trades` for `AAPL`,
`BTC`, `ETH`, and `BTCUSDT`, plus `marketdata/session`, as L2 price levels.
Neither adapter adds a native path.

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
| Trading session | `marketdata/session` | `TradingSessionStatus` | FIX 35=h; ITCH `S` |
| Security status | `marketdata/{sym}/status` | `SecurityStatus` | FIX 35=f |
| Executions | `trading/accounts/{acct}/executions` | `ExecutionReport` | HL `orderUpdates`; FIX drop-copy 35=8 |
| Balances | `accounts/{acct}/balances` | `BalanceSnapshot`, `BalanceUpdate` | HL `spotState` |
| Positions | `accounts/{acct}/positions` | `PositionSnapshot`, `PositionUpdate` | HL `clearinghouseState` |
| Margin | `accounts/{acct}/margin` | `MarginUpdate` | HL `margin` |
| Funding | `accounts/{acct}/funding` | `FundingPayment` | HL `userFunding` |
| Ledger | `accounts/{acct}/ledger` | `LedgerUpdate` | HL `ledgerUpdates` |
| User liq | `accounts/{acct}/liquidations` | `UserLiquidation` | HL `liquidation` |
| Order lists | `trading/accounts/{acct}/orderlists` | `OrderListStatus` | `@orderlists` |

Private rows require `AUTH_TOKEN` on each frame, scoped to `{acct}` in the path
(production: JWT bearer or mTLS; simulator test harness: `fig-dev-{acct}`).

### Historical queries (`REQUEST` → `RESPONSE` or `STREAM_ITEM` × N)

| Query | Native path | Request → response |
|---|---|---|
| Capabilities | `/.well-known/capabilities` | `CapabilitiesRequest` → `CapabilitiesResponse` |
| Instruments | `/.well-known/instruments` | GET → `InstrumentCatalogResponse` |
| Candles | `marketdata/{sym}/candles/{iv}` | `CandleBarRequest` → `CandleBarBatch` |
| Trades | `marketdata/{sym}/trades` | `TradeHistoryRequest` → `PublicTradeBatch` |
| Agg trades | `marketdata/{sym}/aggtrades` | `AggregateTradeRequest` → `AggregateTradeBatch` |
| Book snapshot | `marketdata/{sym}/book` | `OrderBookRequest` → `OrderBookSnapshot` |
| Ticker | `marketdata/{sym}/ticker` | `TickerRequest` → `SymbolTicker` |
| Account | `accounts/{acct}` | GET → `AccountSummary` |
| Margin | `accounts/{acct}/margin` | GET → `MarginSummary` |
| Trading session | `marketdata/session` | `TradingSessionStatusRequest` → `TradingSessionStatus` |
| Security status | `marketdata/{sym}/status` | `SecurityStatusRequest` → `SecurityStatus` |
| Open orders | `trading/accounts/{acct}/orders/open` | `OpenOrdersRequest` → `OpenOrdersSnapshot` |
| Mass cancel | `trading/accounts/{acct}/orders/masscancel` | `MassCancelRequest` → `MassCancelReport` (FIX 35=q / 35=r; not an OUCH message) |
| Order history | `trading/accounts/{acct}/orders` | `OrderHistoryRequest` → `OrderHistoryBatch` (stream if large) |
| Fills | `accounts/{acct}/fills` | `FillHistoryRequest` → `FillHistoryBatch` |
| Funding hist | `accounts/{acct}/funding` | `FundingHistoryRequest` → `FundingHistoryBatch` |
| Ledger hist | `accounts/{acct}/ledger` | `LedgerHistoryRequest` → `LedgerHistoryBatch` |

Example gateway REST query:

```bash
curl 'http://127.0.0.1:8080/marketdata/AAPL/candles/5m?limit=10'
curl 'http://127.0.0.1:8080/.well-known/capabilities'
curl 'http://127.0.0.1:8080/.well-known/instruments'
```

See [PROTOCOL.md](PROTOCOL.md) for frame-by-frame worked examples.

## 11. Perps walkthrough (native FIG)

This section uses **native FIG paths** — the same surface any crypto or equity venue
implements. Gateway aliases (Binance `@markPrice`, Hyperliquid `activeAssetCtx`) map
to these paths; see [AGENTS.md](../AGENTS.md).

With the exchange simulator running and `fig-cli` as reference:

```bash
cargo run -p fig-exchange-sim -- --print-ca .fig/dev-ca.pem   # terminal 1
cargo test -p fig-cli           # exercises FigSdkClient perps streams
```

Native flows the CLI demos exercise via `FigSdkClient`:

| Action | Native path | SDK helper |
|---|---|---|
| Mark price stream | `marketdata/BTC/mark` | `subscribe` + `MarkPriceUpdate` |
| Aggregate trades | `marketdata/BTC/aggtrades` | `subscribe_agg_trades` |
| Funding payments | `accounts/{acct}/funding` | `subscribe_funding` |
| Ledger updates | `accounts/{acct}/ledger` | `subscribe_ledger` |
| Post-only limit order | `trading/accounts/{acct}/orders` POST | `NewOrderSingle { post_only: Some(true), … }` |
| Capabilities | `/.well-known/capabilities` GET | `request_capabilities` |
| Instrument catalog | `/.well-known/instruments` GET | `request_instruments` |
| Fill history | `accounts/{acct}/fills` GET | `request_fills` |

Merge helpers (`FundingState`, `LedgerState`, `AggTradeState`, `LiquidationState`)
accumulate `STREAM_ITEM` payloads — see [STREAMING.md](STREAMING.md).

SBE order entry for colo clients: `cargo test -p fig-cli cli_sbe_order_demo_successfully`
and [SBE_ORDER_PATH.md](SBE_ORDER_PATH.md).

