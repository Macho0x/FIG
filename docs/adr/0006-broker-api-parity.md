# ADR 0006: Broker ↔ Client API Parity (Native FIG First)

## Status

Accepted

## Context

Brokers expose rich client APIs over WebSocket (live streams) and REST (historical
queries). FIG must offer **native parity** on TREE before gateway adapters map
legacy protocols. Without this ordering, REST/WS become alternate transports that
drift from the canonical protocol.

§17 of the roadmap tracks: candles, trades, balances, positions, executions,
margin, and historical pulls — modeled on Binance/Hyperliquid surfaces but using
FIG terminology (`CandleBar`, `/candles/{interval}`, not Binance-only “klines”).

## Decision

1. **Native FIG is canonical** — every broker capability is expressed as:
   - Live: `SUBSCRIBE` → `STREAM_ITEM` on a declared `CHANNEL_PATH`
   - Query: `REQUEST` → `RESPONSE` (or `REQUEST` → `STREAM_ITEM*` for large ranges)
   - FSL message types in `schemas/orders.fsl` (schema id `0x01`)

2. **Gateway is translation only** — REST GET and WS topics map to the same FSL
   payloads and paths. Binance `/klines` and `@kline_*` are **gateway aliases**;
   native paths use `marketdata/{symbol}/candles/{interval}`.

3. **Reference implementation in exchange-sim** — `broker_api`, `market_data`, and
   `account_state` modules implement subscribe fan-out and query handlers before
   `fig-gateways` REST/WS catalogs wire legacy clients.

4. **Three interaction patterns** (SPEC §7.3):
   - Small query: `REQUEST` / `RESPONSE`
   - Large history: `REQUEST` / `STREAM_ITEM` × N (future)
   - Live stream: `SUBSCRIBE` / `STREAM_ITEM`

## Consequences

- Gateway REST mappings live in `fig_gateways::rest_query`; WS topic catalog in
  `fig_gateways::ws_catalog`.
- `fig-gateway --fig-backend` proxies translated GET queries to exchange-sim.
- Conformance vectors and multi-language bindings (§16) follow native FIG types.
- Tickers, funding, ledger, and FIX MD remain lower priority until core streams
  and queries are stable.

## References

- [TODO.md §17](../../TODO.md)
- [docs/STREAMING.md](../STREAMING.md)
- [docs/QUERY.md](../QUERY.md)
- [SPEC.md §7.3 / §9](../../SPEC.md)
