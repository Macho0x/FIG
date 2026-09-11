# FIG — Agent Guide

Operational guide for AI agents and contributors working in this repo.
Normative wire details live in [SPEC.md](SPEC.md); build workflow in
[CONTRIBUTING.md](CONTRIBUTING.md); architecture decisions in
[docs/adr/](docs/adr/).

---

## Identity

**FIG is a wire protocol** — framing, TREE transport, FSL schemas, and wire-level
credential validation. Same layer as FIX, REST, and WebSocket.

**FIG is not** an exchange, matching engine, ledger, API-key issuer, custody layer,
or account onboarding system. Venues own matching, funding, liquidation, indexing,
compliance, and credential issuance. FIG defines how already-issued credentials ride
on frames and how messages are addressed. See [SPEC.md §1.3](SPEC.md).

**One standard, any venue.** Binance-shaped, Hyperliquid-shaped, and institutional
brokers adopt the same native FIG surface. Do not create exchange-specific bindings
(`fig-binance`, `fig-hyperliquid`) or parallel protocol tracks.

| Term | Meaning |
|------|---------|
| **FIG** | Protocol (frames, channels, sessions) |
| **TREE** | Mandatory QUIC transport (ALPN `fig/1`, TLS 1.3) |
| **FSL** | Schema IDL — single source of truth for message types |
| **Gateway** | Optional migration edge (FIX / REST / WS → native FIG) |
| **Venue / broker** | Whoever runs the FIG server and owns business logic |

---

## Three-layer model

Work **top-down**. Never add Layer 3 without Layer 2 existing.

```text
┌─────────────────────────────────────────────────────────┐
│  LAYER 1 — FSL (canonical)                              │
│  Message types, enums, channel paths, gateway mappings    │
│  Source: schemas/*.fsl                                    │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│  LAYER 2 — Native FIG (required, complete)              │
│  SUBSCRIBE / REQUEST on CHANNEL_PATH over TREE            │
│  Implement: venue server + fig-exchange-sim (reference)   │
└────────────────────────┬────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────┐
│  LAYER 3 — Gateway (optional migration edge)            │
│  REST GET, WS topics, FIX → same FSL payloads             │
│  Binance @kline_5m, HL subscription JSON = aliases only │
│  Source: fig-gateways                                     │
└─────────────────────────────────────────────────────────┘
```

**Native FIG is canonical** ([ADR 0006](docs/adr/0006-broker-api-parity.md)):

- **Live push:** `SUBSCRIBE` → `STREAM_ITEM` × N
- **Small query:** `REQUEST` → `RESPONSE`
- **Large history:** `REQUEST` → `STREAM_ITEM` × N → `STREAM_CLOSE`

Gateway REST and WebSocket are **translation only** — they map legacy wire formats
to the same FSL payloads and `CHANNEL_PATH` values. They never define new semantics.

---

## Venue adoption track

Every exchange/broker follows the same path ([TODO.md §20](TODO.md)):

1. **Day 1 — Gateway:** Legacy JSON WS/REST/FIX clients hit `fig-gateway`; gateway
   translates to native FIG and proxies to the venue's FIG backend.
2. **Day 2 — Native FIG:** Performance clients connect over TREE with SBE/CBOR;
   gateway shrinks to stragglers.
3. **Day 3 — Product extensions:** Venue-specific products (vaults, signed actions,
   chain events) extend FSL and `CapabilitiesResponse` — not protocol forks.

---

## Naming and terminology

Use FIG-native names in FSL, SPEC, core, client SDKs, and bindings.
Exchange names belong **only** in gateway alias mappers and tests.

| Native FIG | Do not use in protocol/core | Gateway alias (examples) |
|------------|----------------------------|--------------------------|
| `CandleBar`, `marketdata/{sym}/candles/{interval}` | "klines" | Binance `@kline_5m`, `GET /klines` |
| `PublicTradeEvent` | Exchange-specific field names in FSL | Binance `@trade` |
| `accounts/{account}/fills` | Venue-only paths in core | HL `userFills` JSON egress |
| `ChannelPath` | Exchange routing in `fig-core` | `ROUTING_KEY` for legacy egress |

Intervals use FIG names (`1m`, `5m`, `1h`, `1d`) — not Binance interval strings.
Full path catalog: [SPEC.md §9.1](SPEC.md), [docs/QUERY.md](docs/QUERY.md),
[docs/STREAMING.md](docs/STREAMING.md).

---

## Role boundaries

| Responsibility | Owner |
|----------------|-------|
| Frame format, channels, sessions, TREE transport | FIG (`fig-core`) |
| Message types and gateway mappings | FSL (`schemas/`, `fig-fsl`) |
| Wire credential validation (`AUTH_TOKEN`, mTLS) | FIG libraries |
| Credential issuance, KYC, wallets | Venue |
| Matching, funding engine, liquidations | Venue |
| Historical index at scale | Venue |
| Gateway for legacy clients | Venue or integrator (`fig-gateways`) |
| Reference broker implementation | `fig-exchange-sim` (not production) |

When adding wallet signatures or chain events, treat them as **protocol extensions**
(FSL + SPEC) usable by any venue — not Hyperliquid-specific code in core.

---

## Feature addition checklist

Follow this order for every broker/API change:

1. **FSL first** — add or extend messages in `schemas/*.fsl`; bump schema version
   per [ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md).
2. **Native path** — register `CHANNEL_PATH` in SPEC §9.1 and query/streaming docs.
3. **Reference broker** — implement subscribe and/or query in `fig-exchange-sim`
   (`broker_api`, `account_state`, `market_data`).
4. **Gateway aliases** — map legacy REST/WS/FIX shapes in `fig-gateways` (translation
   only; round-trip through native FIG).
5. **Conformance** — add native wire vectors in `tests/conformance/` (CBOR/SBE
   frames, not JSON).
6. **Client SDK** — extend `fig-client` and bindings with native `subscribe()` /
   `request()` — not exchange-named wrappers.
7. **Capabilities** — if optional per venue, advertise via `/.well-known/capabilities`;
   do not hard-code one venue's full surface as the FIG catalog.

Not every venue implements every path. Optional features are discovered at runtime,
not forked into separate protocol tracks.

---

## Where exchange-specific code belongs

| Allowed | Forbidden |
|---------|-----------|
| Gateway alias mappers in `fig-gateways` (`rest_query`, `ws_catalog`, `fix`) | Exchange-named types in `fig-core` or FSL |
| Gateway E2E tests round-tripping through native FIG | `fig-binance` / `fig-hyperliquid` crates |
| FSL types for product categories (funding, ledger, liquidations) | Venue business logic in protocol crates |
| `CapabilitiesResponse` per deployment | One venue's API as the canonical catalog |
| Reference handlers in `fig-exchange-sim` | Treating exchange-sim as production infra |

Gateway WS catalog pattern (`ws_catalog.rs`): Binance topics and Hyperliquid
subscription JSON map **to** the same native `CHANNEL_PATH` and FSL payloads
TREE clients already use. Copy this pattern for new legacy aliases.

---

## Anti-patterns

Do **not**:

- Add `BinanceOrder`, `HyperliquidSubscription`, or similar to FSL/core.
- Use exchange-native paths as canonical (`/api/v3/klines`, `@kline_5m`).
- Implement a feature only in `rest_query.rs` or `ws_catalog.rs` without a native
  FIG handler in the broker server.
- Put exchange-specific logic in `fig-core`, `fig-client`, or language bindings.
- Hand-edit `fig-core/src/messages.rs` without a matching FSL change.
- Ship REST-only or WS-only features that native FIG lacks.
- Harden `fig-exchange-sim`'s in-memory matcher for production use.
- Create per-venue SDK tracks instead of Capabilities + optional gateway aliases.

---

## Testing expectations

| Layer | What to test |
|-------|--------------|
| Conformance | Native FIG wire — `tests/conformance/vectors/` |
| Broker | Native `SUBSCRIBE` / `REQUEST` on TREE |
| Gateway | Legacy JSON in → native FIG frame → legacy JSON out |
| Legacy alias fixtures | `gateway_legacy_ws_alias_e2e.rs` — Binance/HL WS, FIX orders, REST GET |

```bash
cargo test --workspace
cargo test -p fig-conformance
cargo test -p fig-gateways --test gateway_legacy_ws_alias_e2e
cargo run -p xtask -- check-gateway   # gateway parity
```

---

## File map

| Task | Start here |
|------|------------|
| Add message type | `schemas/*.fsl` |
| Add channel path | [SPEC.md §9.1](SPEC.md), then `fig-exchange-sim` |
| Add REST alias | `crates/fig-gateways/src/rest_query.rs` |
| Add WS alias | `crates/fig-gateways/src/ws_catalog.rs` |
| Reference broker handler | `crates/fig-exchange-sim/src/broker_api.rs` |
| Client subscribe / request | `crates/fig-client/` |
| Conformance vectors | `tests/conformance/` |
| Architecture decision | New ADR in `docs/adr/` |

---

## Doc hierarchy

```
AGENTS.md (this file)  — how to work in the repo
docs/README.md         — user reading order (guides vs SPEC)
SPEC.md                — normative wire format and paths
docs/adr/              — recorded decisions and consequences
CONTRIBUTING.md        — build, FSL workflow, coding standards
TODO.md                — roadmap and completeness matrices
```

Escalate to a new ADR when adding a new interaction pattern, breaking FSL wire
layout, or introducing a product primitive that multiple venue tracks will share.

---

## Worked example: add a stream

To add e.g. open-interest streaming:

1. Define `OpenInterestUpdate` in FSL with `channel_type: stream_item`.
2. Add `marketdata/{symbol}/openinterest` to SPEC §9.1.
3. Implement `SUBSCRIBE` fan-out in `fig-exchange-sim`.
4. Add gateway aliases (Binance `@openInterest`, HL `type: "openInterest"`) in
   `ws_catalog.rs` — mapping **to** the native path.
5. Add conformance vector for the native `STREAM_ITEM` payload.
6. Extend `fig-client` with `subscribe_open_interest(symbol)`.

If step 3 is skipped, the gateway alias must not ship.
