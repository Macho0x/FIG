# TODO — FIG Protocol Roadmap

**Central source of truth** for all outstanding FIG work.

| Section | Purpose |
|---|---|
| **[§0](#0-active-backlog-fig-repo)** | **Active backlog** — open items by priority (start here) |
| [§18](#18-fig-10-release-criteria) | 1.0 release gate, review packet, sign-off |
| [§19–22](#19-venue-adoption--shared-foundation) | Venue adoption (FIG + adopter checklists) |
| [§16–17](#16-multi-language-sdk-parity) | SDK parity and broker API coverage reference |
| [Appendix](#appendix-a-completed-rust-core-115) | Completed Rust core (§1–15) |

Tracking work against [SPEC.md](SPEC.md). **Rust core (§1–15) is complete.**
§16–17 reference stacks are complete except items in §0. **§18** gates 1.0.
**§19–21** describe production venue adoption.

See [API.md](docs/API.md) and [PROTOCOL.md](docs/PROTOCOL.md) for integration guidance.

**Legend:** ✅ Done · 🔶 Partial · ⬜ Not started · 🏛 Venue-owned (outside FIG repo)

**Priority tiers (§0):** P0 ship 1.0 · P1 security · P2 SDK polish · P3 track work · P4 long horizon

---

## 0. Active backlog (FIG repo)

**Start here.** Every open FIG-repo item appears **once**, ordered by priority.
Detail sections (§18–21) add context; do not duplicate status here.

**Tracks:** `both` = any adopter · `crypto` = Hyperliquid-shaped (§20) ·
`institutional` = CME-shaped (§21) · `process` = human sign-off, not code.

### 0.1 P0 — Ship FIG 1.0

| Status | Item | Track | Detail |
|---|---|---|---|
| ✅ | Engineering sign-off | process | [§18.11](#1811-sign-off) — Macho0x, 2026-07-29, `v1.0.0` |
| ✅ | Security sign-off | process | [§18.11](#1811-sign-off) — Macho0x, 2026-07-29, `v1.0.0` |
| ✅ | Run review packet (or green CI equivalent) | process | [§18.8](#188-review-packet-engineering--security) — green 2026-07-29 |
| ✅ | Ticket or accept every P1–P4 row before sign-off | process | [§18.7](#187-post-10-outstanding-fig-project) — accepted; tracked in §0 |
| ✅ | Bump SPEC to **1.0.0** stable + tag release | both | [SPEC.md](SPEC.md) `1.0.0` · tag `v1.0.0` |

### 0.2 P1 — Security and correctness

| Status | Item | Track | Detail |
|---|---|---|---|
| ✅ | `FigServer::accept_0rtt` reads resumption token from first app frame | both | `fig-core::transport::try_restore_session_from_first_frame` |
| ✅ | Fuzz `frame_decode` in required CI | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ⬜ | Shared `RedisReplayCache` for HA 0-RTT | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ✅ | 0-RTT replay on server accept path (criteria row) | both | `FigServer::accept_0rtt` + session restore — [§18.2](#182-security) |

### 0.3 P2 — Reference client and SDK polish

| Status | Item | Track | Detail |
|---|---|---|---|
| ✅ | `fig-client`: funding, ledger, liquidations, agg trades merge helpers | both | `fig-client::{FundingState,LedgerState,AggTradeState,LiquidationState}` |
| ✅ | `fig-cli` demos use `FigSdkClient` subscribe/request helpers | both | `cli_demos_complete_successfully` |
| 🔶 | Tier 1 SDK: `FigSdkClient` as primary `request()` path | both | [§17.2](#172-cross-cutting-protocol-both-sides) |
| ✅ | Tier 3 SDK: pub/sub merge wrappers complete | both | perps + account merge helpers in `fig-client` |
| ✅ | §16 binding compile smoke for new stream types (Go/C++/C++) | both | [§17.6](#176-codegen-conformance--testing) |
| ⬜ | Tier-based public MD rate limits | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ⬜ | JWT RS256/KMS reference + key rotation runbook | both | [§18.7](#187-post-10-outstanding-fig-project) |
| 🔶 | Throughput benchmarks (msgs/sec) | both | [Appendix §12](#12-benchmarks-fig-bench) |

### 0.4 P3 — Shared adoption (both tracks)

| Status | Item | Track | Detail |
|---|---|---|---|
| ⬜ | Published artifacts (crates, bindings) + compatibility policy | both | [§19.1](#191-fig-project--protocol-release-maturity) |
| ⬜ | Co-lo / tail-latency evidence on realistic topology | both | [§19.1](#191-fig-project--protocol-release-maturity) |
| ✅ | Per-language SBE hex parity CI | both | [§19.2](#192-fig-project--client-ecosystem) |
| ✅ | SBE-default order path documentation + examples | both | [docs/SBE_ORDER_PATH.md](docs/SBE_ORDER_PATH.md), `run_sbe_order_demo` |
| 🔶 | Native TREE clients at Tier 3–4 in Java/C++/C# | both | [§19.2](#192-fig-project--client-ecosystem) |

### 0.5 P3 — Crypto-perps track (pick one track at a time)

**Deferred to Full §20.1** (venue product surface, not protocol forks): EIP-712 wallet auth,
TWAP, vault/strategy accounts, builder codes, batch multi-action `REQUEST`, chain/L1 streams.

| Status | Item | Track | Detail |
|---|---|---|---|
| ✅ | Extended instrument model (long symbols, perp IDs, margin asset metadata) | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ✅ | Gateway legacy alias E2E (Binance/HL/FIX/REST fixtures) | crypto | `gateway_legacy_ws_alias_e2e.rs` |
| ⬜ | Cryptographic action auth (wallet signatures, EIP-712) | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ⬜ | FSL: TWAP, vault/strategy accounts, spot vs perp products | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ⬜ | FSL: builder codes, referral tiers, bridge deposit/withdraw events | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ⬜ | Batch / atomic multi-action `REQUEST` contract | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ⬜ | Chain / L1 event stream domain in FSL | crypto | [§20.1](#201-fig-project--wire-and-protocol-gaps) |
| ✅ | Post-only, reduce-only, venue policy flags in SPEC | crypto | SPEC §9.4, `schemas/orders.fsl`, exchange-sim matcher |

### 0.6 P3 — Institutional track (pick one track at a time)

| Status | Item | Track | Detail |
|---|---|---|---|
| ⬜ | FIX gateway: full institutional dialect matrix | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FIX: mass cancel, trading session status, security status | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FIX: allocations, trade capture, multi-leg, IOI | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FSL: market state & control (halts, auctions, pre-open, price limits) | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FSL: open interest, settlement prices, block trades | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | Member entitlement model on wire (beyond path scoping) | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | MDP 3.0 / FAST adapter or multicast distribution ADR | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FIX session cert harness (resend/gap-fill vs OMS stacks) | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | Co-lo latency SLA benchmark program | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | FSL: regulatory / surveillance (bust/correct, give-up, reporting) | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |
| ⬜ | Tier 4 advanced frames in all production SDKs | institutional | [§21.1](#211-fig-project--wire-and-protocol-gaps) |

### 0.7 P4 — Long horizon / optional

| Status | Item | Track | Detail |
|---|---|---|---|
| ⬜ | `AccountCache` in FFI/Python | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ⬜ | Native TREE transport per language | both | [§16.6](#166-pure-protocol-libraries-track-b-option-2) |
| ⬜ | Wire schema split `0x02` market / `0x03` account | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ✅ | Redis session round-trip in CI service container | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ⬜ | crates.io / npm publish automation | both | [§18.7](#187-post-10-outstanding-fig-project) |
| ⬜ | External penetration test | both | [§18.2](#182-security) (optional for 1.0) |
| ⬜ | Per-language generated SBE hex CI (optional criterion) | both | [§18.3](#183-client-sdk) |
| 🔶 | FSL codegen: per-lang native CBOR serializers (§16.2) | both | [§16.2](#162-fsl-codegen-parity-track-a) |

### 0.8 Adopter checklist (not FIG repo — 🏛 venue-owned)

Do **not** schedule these in the FIG backlog. Venues own matching, clearing,
chain settlement, certification, and compliance. See [§19.3](#193-venue-owned--business-stack-both-venue-types),
[§20.2](#202-venue-owned--backend-hyperliquid-shaped), [§21.2](#212-venue-owned--market-infrastructure-cme-shaped).

---

## 18. FIG 1.0 Release Criteria

[SPEC.md](SPEC.md) is **1.0.0 (stable)** as of the 2026-07-29 Engineering +
Security sign-off (§18.11). Required criteria below were ✅ at release; optional
and post-1.0 rows remain tracked in [§0](#0-active-backlog-fig-repo) / §18.7.

### 18.1 Protocol and reference implementation

| Status | Criterion | Blocker | Evidence |
|---|---|---|---|
| ✅ | SPEC §1–15 Rust core complete | Required | [Appendix §1–15](#appendix-a-completed-rust-core-115) |
| ✅ | Broker API parity (§17) native paths | Required | exchange-sim + `cargo run -p xtask -- check-gateway` |
| ✅ | Conformance vectors CI | Required | `cargo run -p fig-conformance --bin fig-conformance` |
| ✅ | Workspace tests pass | Required | `cargo test --workspace` |

### 18.2 Security

| Status | Criterion | Blocker | Evidence |
|---|---|---|---|
| ✅ | 0-RTT replay protection (client path) | Required | `fig_core::replay`, client `connect_0rtt`, unit tests |
| 🔶 | 0-RTT replay protection (server accept path) | Post-1.0* | `FigServer::accept_0rtt` token read ✅; Redis HA replay cache deferred |
| ✅ | Frame decoder fuzz target | Required | `crates/fig-core/fuzz/frame_decode`, [CONTRIBUTING.md](CONTRIBUTING.md) |
| ✅ | Frame decoder fuzz in required CI | Post-1.0* | §18.7 |
| ✅ | Load / flood smoke test | Required | `fig-load` bin, CI `FIG_LOAD_SECS=3` |
| ✅ | mTLS + rate limits available | Required | `FIG_MTLS`, `ChannelRateLimiter`, `DoSGuard` |
| ⬜ | External penetration test | Optional | Manual — [SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md) |

\*Post-1.0 items do not block 1.0 if accepted or ticketed in §18.7.

### 18.3 Client SDK

| Status | Criterion | Blocker | Evidence |
|---|---|---|---|
| ✅ | `fig-client` merge helpers (book, candles, account, mids, bbo, trades, mark, orders) | Required | `crates/fig-client` tests |
| 🔶 | `fig-cli` uses `fig-client` as primary demo path | Post-1.0* | `run_demos` via `FigSdkClient` — `cli_demos_complete_successfully` |
| ✅ | Python + FFI exposure | Required | `PyOrderBookState`, `PyMidsState`, … + `fig_*` FFI |
| ⬜ | Per-language generated SBE hex CI | Optional | FFI conformance required; native SBE compile-smoke only |

### 18.4 Operations

| Status | Criterion | Blocker | Evidence |
|---|---|---|---|
| ✅ | `RedisSessionStore` (`session-redis` feature) | Required | CI `REDIS_URL` service + `round_trip_session`; CLI `--session-store redis` |
| ✅ | Redis session round-trip in CI service container | Post-1.0* | §18.7 |
| ✅ | `docker-compose.yml` (sim + gateway + metrics) | Required | repo root |
| ✅ | Gateway `--health-addr` / `--metrics-addr` | Required | `fig-gateway` flags |
| ✅ | [DEPLOYMENT.md](docs/DEPLOYMENT.md) | Required | HA topology documented |

### 18.5 Documentation

| Status | Criterion | Blocker | Evidence |
|---|---|---|---|
| ✅ | Doc drift resolved (no stale protobuf/etcd) | Required | API, PROTOCOL, GATEWAY, SPEC §3.3 / §11.2 |
| ✅ | [PUBLISHING.md](docs/PUBLISHING.md) | Required | install paths |
| ✅ | Benchmarks from measured runs | Required | [BENCHMARKS.md](docs/BENCHMARKS.md) |

### 18.6 1.0 vs post-1.0 (sign-off reference)

Use during Engineering / Security review. **1.0** means the reference Rust stack +
FFI/Python wire path is production-ready for **protocol adoption**; **post-1.0**
items do not block the SPEC version bump if accepted or ticketed in §18.7.

| Area | In 1.0 (required / done) | Post-1.0 (deferred → §18.7) |
|---|---|---|
| **Release gate** | Criteria tables ✅ + sign-off + SPEC → `1.0.0` | — |
| **Wire codecs** | CBOR + SBE only (SPEC §3.3, §11.2) | Protobuf wire restoration (not planned) |
| **Rust core** | §1–15 framing, transport, sessions, auth, observability | Tier 4 frames in all SDKs (ACK_RANGE, …) |
| **Broker API** | §9.1 catalog in exchange-sim + gateway adapters | Order-list stream polish; every edge path in `fig-client` |
| **`fig-client`** | book, candles, account, mids, BBO, trades, mark, orders/executions | funding, ledger, liquidations, agg trades, persistent multiplexer SDK |
| **Bindings** | `fig-ffi` + `fig-python` conformance CI; Go/C++/C# compile smoke + SBE hex via FFI | Native TREE per language; full per-lang generated SBE hex matrix |
| **0-RTT replay** | Client `connect_0rtt` + `MemoryReplayCache` | Server `accept_0rtt` token read + shared Redis replay cache (HA) |
| **Sessions HA** | `RedisSessionStore`, compose, `DurableSessionStore` | Redis round-trip in CI service container |
| **Security** | mTLS, rate limits, load smoke, fuzz target exists | Fuzz in required CI; external pen test; JWT key rotation runbook |
| **Schema IDs** | Single wire `schema_id = 0x01` (documented) | Split `0x02` market / `0x03` account domains |
| **Ops** | docker-compose, gateway health/metrics | crates.io / npm publish automation |
| **Venue-owned** | Wire auth contract (§9.3) | API key issuance, RS256/KMS, matching engine |

### 18.7 Post-1.0 outstanding (FIG project)

Actionable follow-ups from §18.6. **Canonical ordered list: [§0](#0-active-backlog-fig-repo).**
Ticket or accept each P1–P4 row before sign-off.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | See §0.2 (P1 security) | High | Accepted at 1.0 sign-off (2026-07-29) — tracked in §0 |
| ⬜ | See §0.3 (P2 SDK polish) | Medium | Accepted at 1.0 sign-off (2026-07-29) — tracked in §0 |
| ⬜ | See §0.4–0.7 (P3–P4) | Mixed | Accepted at 1.0 sign-off (2026-07-29) — tracked in §0 |

### 18.8 Review packet (Engineering + Security)

Run from repo root before signing:

```bash
cargo test --workspace
cargo test -p fig-cli                    # includes cli_integration e2e
cargo test -p fig-client
cargo test -p fig-core replay
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo run -p xtask -- codegen --check
cargo run -p xtask -- check-gateway
cargo run -p fig-conformance --bin fig-conformance
cargo test -p fig-ffi --test binding_conformance
cargo test -p fig-python --test binding_conformance
FIG_LOAD_SECS=3 cargo run --release -p fig-bench --bin fig-load
# optional: cargo fuzz run frame_decode -- -max_total_time=60
# optional: REDIS_URL=redis://127.0.0.1:6379 cargo test -p fig-core round_trip_session -- --ignored
```

### 18.9 Engineering review (SPEC alignment)

| Area | SPEC reference | Reviewer question | Automated evidence |
|---|---|---|---|
| Transport + framing | SPEC §2–8 | Frames, channels, extensions match spec? | conformance vectors + `fig-core` tests |
| Payload codecs | SPEC §3.3, §11.2 | CBOR + SBE only documented and implemented? | exchange-sim multi-codec tests |
| Sessions + 0-RTT | SPEC §10, §10.1 | Replay window documented and enforced on client? | `replay::tests`, SPEC §10.1 |
| Broker API surface | SPEC §9.1 | High-priority paths in exchange-sim + gateway? | `check-gateway`, integration tests |
| Reference client | SPEC §9.1 | `fig-cli` demos exercise native paths? | `cli_demos_complete_successfully` |
| HA sessions | DEPLOYMENT.md | Redis store + compose topology accurate? | `session-redis` feature, compose file |

### 18.10 Security review ([SECURITY_AUDIT.md](docs/SECURITY_AUDIT.md))

| Area | Checklist item | Status | Notes for reviewer |
|---|---|---|---|
| Transport | TLS 1.3, mTLS, cert rotation | ✅ | Verify prod deploy uses `FIG_MTLS=1` |
| Auth | Constant-time compare, JWT, path policy | ✅ | RS256/KMS is venue responsibility |
| DoS | Rate limiter, flood guard, load smoke | ✅ | `fig-load` in CI |
| 0-RTT | Replay cache | 🔶 | Server accept path ✅; shared Redis HA deferred — §18.7 |
| Fuzz | Frame decoder | 🔶 | Target exists; not required CI |
| External | Pen test | ⬜ | Optional for 1.0 |

### 18.11 Sign-off

Record **name**, **date**, and **git SHA** (or release tag) when approving.
Approving means: required criteria in §18.1–18.5 are ✅, review packet was run
(or CI equivalent is green), and post-1.0 rows in §18.7 are **accepted or ticketed**.

| Role | Name | Date | Git SHA / tag | Status |
|---|---|---|---|---|
| Engineering | Macho0x | 2026-07-29 | v1.0.0 | Approved |
| Security | Macho0x | 2026-07-29 | v1.0.0 | Approved |

Both rows are **Approved**. [SPEC.md](SPEC.md) is **1.0.0** (stable); release
tagged `v1.0.0`.

---

## 19. Venue Adoption — Shared Foundation

Work required before **either** Hyperliquid-shaped (§20) or CME-shaped (§21) venues
would treat FIG as production-grade wire infrastructure. Items marked 🏛 are
**venue-owned** — FIG defines the wire contract; the venue builds the business stack.

### 19.1 FIG project — protocol release maturity

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Complete §18 sign-off + SPEC **1.0.0** stable | High | [§0.1](#01-p0--ship-fig-10) |
| ✅ | All post-1.0 items accepted or ticketed | High | [§0.1](#01-p0--ship-fig-10) |

Remaining §19.1 items are in [§0.4](#04-p3--shared-adoption-both-tracks).

### 19.2 FIG project — client ecosystem

Open items: [§0.3](#03-p2--reference-client-and-sdk-polish) (P2) and
[§0.4](#04-p3--shared-adoption-both-tracks) (P3).

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `fig-client` complete merge + request surface | Medium | Perps merge helpers + `FigSdkClient` demos; Tier 1 polish in §0.3 |
| 🔶 | Native TREE clients at Tier 3–4 in Java/C++/C# | Medium | FFI smoke exists; see §0.4 |

### 19.3 Venue-owned — business stack (both venue types)

| Status | Item | Priority | Notes |
|---|---|---|---|
| 🏛 | Production matching engine (latency, determinism, surveillance) | High | `fig-exchange-sim` is reference only |
| 🏛 | Risk / margin / liquidation at scale | High | Sim has basic margin |
| 🏛 | Clearing & ledger (CCP or on-chain) | High | Outside SPEC §1.3 |
| 🏛 | Credential issuance, entitlements, rotation, admin | High | FIG verifies `AUTH_TOKEN` only |
| 🏛 | Durable historical data platform | High | Sim uses in-memory stores |
| 🏛 | SRE, incident response, multi-region capacity | High | Compose is a template |
| 🏛 | Member / client certification program | High | Protocol conformance ≠ venue cert |

### 19.4 Suggested sequencing (both tracks)

| Wave | Scope | Target |
|---|---|---|
| **Wave 1** | §18 1.0 + [§0](#0-active-backlog-fig-repo) P1–P2 hardening | Weeks–months — protocol adoption ready |
| **Wave 2** | §20 crypto-perps OR §21 institutional (pick track) | Months–year+ |
| **Wave 3** | Second venue track + shared §19.2 SDK depth | Ongoing |

---

## 20. Venue Adoption — Hyperliquid-Shaped Crypto Perps

**API-shape fit:** High — §17 catalog + `ws_catalog.rs` Hyperliquid mappings cover
mids, L2, funding, ledger, liquidations, mark price, etc. **Production fit:** Low
without venue backend + auth + product extensions.

Adoption model: **Day 1 gateway** (JSON WS/REST → FIG) → **Day 2 native FIG/SBE**
for colocated market makers → **Day 3 FSL extensions** for venue-specific products.

### 20.1 FIG project — wire and protocol gaps

**Ordered backlog:** [§0.5](#05-p3--crypto-perps-track-pick-one-track-at-a-time). Detail below.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | Cryptographic action auth (wallet signatures, EIP-712, chain-specific) | High | Today: `AUTH_TOKEN` / JWT / mTLS — not signed L1 actions |
| ⬜ | FSL: TWAP, vault/strategy accounts, spot vs perp products | High | Not in `schemas/*.fsl` |
| ⬜ | FSL: builder codes, referral tiers, bridge deposit/withdraw events | Medium | HL product surface |
| ⬜ | Batch / atomic multi-action `REQUEST` contract | Medium | HL batches L1 actions; FIG is frame-per-request |
| ⬜ | Chain / L1 event stream domain in FSL | Medium | Block height, finality, deposit confirmation, withdrawal status |
| ✅ | Extended instrument model (long symbols, perp identifiers, margin asset metadata) | High | `schemas/instruments.fsl`, ADR 0007, `Symbol` max_len 32 |
| ✅ | Post-only, reduce-only, venue policy flags + surveillance hooks in SPEC | Medium | SPEC §9.4, exchange-sim matcher |
| ✅ | Gateway legacy alias E2E test suite (Binance/HL/FIX fixtures) | Medium | `gateway_legacy_ws_alias_e2e.rs` |
| ✅ | `fig-client` + Python/FFI: funding, ledger, liquidations, agg trades | Medium | `fig-client` merge helpers + subscribe APIs |

### 20.2 Venue-owned — backend (Hyperliquid-shaped)

| Status | Item | Priority | Notes |
|---|---|---|---|
| 🏛 | Matching + order book at venue scale | High | Replace in-memory sim matcher |
| 🏛 | On-chain or hybrid settlement (deposits, withdrawals, state roots) | High | Core to HL model |
| 🏛 | Funding rate engine + oracle/index pipeline | High | FIG streams exist; production oracle doesn't |
| 🏛 | Liquidation engine + insurance fund / ADL | High | Push events exist; production logic doesn't |
| 🏛 | Indexer for historical `REQUEST` at petabyte scale | High | Durable backing for §17.0b paths |
| 🏛 | Wallet linking, sub-accounts, account abstraction | Medium | Path model is `accounts/{account}` |
| 🏛 | Compliance (geo-fencing, sanctions, KYC tiers) | High | Venue policy |
| 🏛 | Public API backward compatibility during migration | High | Gateway keeps JSON WS for retail |

### 20.3 Milestones (Hyperliquid-shaped)

| Phase | Deliverable | FIG repo | Venue |
|---|---|---|---|
| **P1** | Gateway in front of existing JSON API | ✅ `fig-gateway` + `--fig-backend`, HL alias E2E | Proxy to real matching engine |
| **P2** | Native FIG/TREE + SBE for colo MMs | ✅ SBE order path doc + demo, instrument model, order flags | Production order entry SLA |
| **P3** | Venue-specific products on FIG wire | §20.1 FSL + auth extensions (deferred) | Vaults, signed actions, chain events |

**Rough effort:** §17 API surface ~70% for Binance/Hyperliquid-shaped perps;
production venue **12–24+ months** venue engineering on top of §19.

---

## 21. Venue Adoption — CME-Shaped Institutional

**API-shape fit:** Low for core Globex MD distribution; **moderate** for FIX drop-copy /
order-entry modernization. FIG unifies FIX + REST + WS on one binary protocol — useful
for **new connectivity** or **internal crossing**, not replacing MDP 3.0 multicast.

### 21.1 FIG project — wire and protocol gaps

**Ordered backlog:** [§0.6](#06-p3--institutional-track-pick-one-track-at-a-time). Detail below.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ⬜ | FIX gateway: full institutional dialect matrix | High | Today: subset (Logon, D/F/G/8/9, some MD) — see `fix.rs` |
| ⬜ | FIX: mass cancel, trading session status, security status | High | Not in gateway |
| ⬜ | FIX: allocations, trade capture, multi-leg, IOI | High | Institutional order types |
| ⬜ | FSL: market state & control (halts, auctions, pre-open, price limits) | High | Not in catalog |
| ⬜ | FSL: open interest, settlement prices, block trades | High | Not in catalog |
| ⬜ | FSL: regulatory / surveillance (bust/correct, give-up, trade reporting) | Medium | Out of current scope |
| ⬜ | Member entitlement model on wire (beyond path scoping) | High | Channel arbitration, drop-copy entitlements |
| ⬜ | MDP 3.0 / FAST adapter OR documented multicast distribution layer | High | FIG is TREE point-to-point; CME uses UDP multicast fan-out |
| ⬜ | FIX session cert harness (resend/gap-fill matrix vs OMS stacks) | High | Partial resend wiring |
| ⬜ | Co-lo latency SLA benchmark program | High | Dedicated hardware, jitter budgets — not localhost claims |
| ⬜ | Tier 4 advanced frames in all production SDKs | Medium | §16.4 core ✅; not all langs at Tier 4 |
| ⬜ | Wire schema split `0x02` market / `0x03` account | Low | Institutions care about domain versioning — §18.7 |

### 21.2 Venue-owned — market infrastructure (CME-shaped)

| Status | Item | Priority | Notes |
|---|---|---|---|
| 🏛 | Globex matching + market supervision | High | FIG does not replace |
| 🏛 | CME Clearing / SPAN-scale margin | High | Outside FIG |
| 🏛 | Member connectivity cert labs (per OMS/EMS) | High | Months per member |
| 🏛 | Drop copy legal/ops entitlements | High | Not just wire format |
| 🏛 | Audit trail & retention (legal hold) | High | Compliance |
| 🏛 | Disaster recovery & market reopen procedures | High | Venue ops |
| 🏛 | Cross-asset (rates, equities, FX, options combos) | High | FSL instrument model is equity-ish |

### 21.3 Realistic adoption scenarios (institutional)

| Scenario | FIG role | Primary §21 work |
|---|---|---|
| Internal crossing network between members | Native FIG + FIX adapter | §21.1 FIX completeness + §19.3 matching |
| Parallel API for fintech / retail wrappers | Gateway + subset native FIG | §17 + gateway; keep MDP for pro MD |
| Drop copy modernization | `SUBSCRIBE` executions + FIX 8 egress | §21.1 exec type matrix + entitlements |
| Greenfield regional exchange | Fuller FIG adoption | §19 + §21.1 + venue clearing stack |

### 21.4 Milestones (CME-shaped)

| Phase | Deliverable | FIG repo | Venue |
|---|---|---|---|
| **P1** | FIX drop-copy / order-entry bridge | Expand `fix.rs`, cert tests | Member OMS integration |
| **P2** | Market-state FSL domain + halts/auctions | New FSL + exchange-sim reference | Production supervision |
| **P3** | MD distribution strategy (multicast adapter or hybrid) | §21.1 MDP/FAST or ADR | Globex-scale fan-out |

**Rough effort:** Protocol surface ~**25–35%** for CME-equivalent connectivity;
institutional cert + MD distribution largely **greenfield** relative to current FIG.

---

## 22. Venue Adoption — Side-by-Side Summary

| Dimension | Hyperliquid-shaped (§20) | CME-shaped (§21) |
|---|---|---|
| **§17 API catalog fit** | High | Low (different MD/order models) |
| **Gateway migration value** | High (JSON WS → FIG) | Medium (FIX subset only) |
| **Native FIG as primary wire** | Plausible for colo MM clients | Unlikely for core Globex MD |
| **Biggest FIG-project gap** | Signed-action auth + product FSL | FIX completeness + multicast MD |
| **Biggest venue gap** | Chain + matching at scale | Clearing, cert, surveillance, MDP |
| **Sensible first milestone** | Gateway + native FIG orders/MD | FIX drop-copy / order-entry bridge |
| **Depends on** | §18 1.0 + §19 shared foundation | §18 1.0 + §19 shared foundation |

---

## 16. Multi-Language SDK Parity

Bring non-Rust languages to full SPEC parity with Rust. Rust has (1) rich
**FSL codegen** (`RustCodegen` + `--lang sbe`) and (2) a complete **`fig-core`
runtime** (~20 modules). All `ftlc --lang` targets emit typed enums, nested structs,
type aliases, message metadata, and CBOR field manifests; wire codecs delegate to
`fig-ffi` / `fig-python` rather than per-language SBE rewrites.

Schema evolution policy: [ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md).

### Parity gap (current state)

| Capability | Rust | Python / C++ / C# / Go / TS / OCaml / Zig / Java |
|---|---|---|
| FSL typed enums | ✅ | ✅ all `ftlc` targets |
| Nested struct types | ✅ | ✅ all `ftlc` targets |
| Type aliases + constraints | ✅ | ✅ constraint docs in all targets |
| Message metadata constants | ✅ | ✅ first-class in Go/TS/Java/Zig; OCaml values |
| CBOR / SBE / Protobuf serializers | ✅ | ✅ `fig-ffi` + `fig-python` CBOR; per-lang SBE codegen (Go/C++/C#/TS/Zig) |
| SBE encode/decode codegen | ✅ (`--lang sbe`) | ✅ `ftlc --lang sbe-{go,cpp,csharp,typescript,zig}` via `sbe_target_codegen.rs` |
| Protocol runtime (frames, channels, transport) | ✅ `fig-core` | ✅ `fig-ffi` Tier 1–4 + pure protocol libs (C++/Zig) |
| Native client SDK | ✅ `fig-cli` | ✅ `fig-python` reference + FFI bindings all langs + JWT helpers |

SPEC §11.2 claims aligned via ADR 0004 — see
[ADR 0004](docs/adr/0004-fsl-single-source-of-truth.md) for schema evolution policy.

### Definition of "full spec"

Align with [SPEC.md §13 Compliance Tiers](SPEC.md):

| Tier | Capability | Deliverable |
|---|---|---|
| **1 — Core** | Frame encode/decode, REQUEST/RESPONSE, extensions | Frame + ext library |
| **2 — Session** | STREAM_OPEN/CLOSE, seq nums, CONTROL, auth | Channel + session + control |
| **3 — Pub/Sub** | SUBSCRIBE, ROUTING_KEY, STREAM_ITEM | Streaming client API |
| **4 — Advanced** | ACK_RANGE, FLOW_CONTROL, FRAGMENT, COMPRESSION, migration | Full transport features |

**Full spec parity** = Tier 4 client + FSL codegen parity + passes cross-language
conformance tests against Rust reference vectors.

### Strategy (two tracks)

**Track A — FSL codegen parity** (extend `ftlc` / `target_codegen.rs`):
typed enums, nested types, type aliases, message metadata constants, per-language
CBOR/SBE serializers. `ftlc --lang proto` is an optional codegen target, not a
wire codec. Do not rewrite `fig-core` nine times without shared
conformance vectors.

**Track B — Protocol runtime parity** (pick per language):

1. **Rust core + FFI bindings** (recommended default) — `fig-ffi` (cbindgen → C ABI)
   + PyO3 / P/Invoke / cgo / ctypes / `@cImport`. Fastest path to behavioral parity.
2. **Generated protocol library** (pure per language) — frame/ext/channel/control
   generated from normative spec. Best for C++ / Zig (performance-sensitive, FFI-averse).
3. **Gateway proxy** (operational parity, not native) — Rust service embeds
   `fig-gateways`, forwards to FIG backend. Good for migration; not Tier 4 native FIG.

---

### 16.1 Conformance Foundation

Language-neutral test vectors that every SDK must pass. Rust `fig-exchange-sim`
integration tests are the reference behavior.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Conformance test vector format spec | High | `tests/conformance/README.md` + `vectors/v1.json` |
| ✅ | Frame encode/decode vectors | High | Round-trip against `fig-core::frame` golden output |
| ✅ | CBOR payload vectors | High | `NewOrderSingle`, `ExecutionReport`, … — snake_case fields, serde enum strings (`"Buy"`) |
| ✅ | Historical / query REQUEST-RESPONSE vectors | High | `frame.request.order_history_paginated` + candle/open-orders frames |
| ✅ | `CandleBar` CBOR/SBE payload vectors | High | `cbor.candle_bar.snapshot` + `sbe.candle_bar*` / `sbe.symbol_ticker` in `v1.json`; `fig_cbor_encode_candle_bar` + binding conformance |
| ✅ | Account / position / balance stream vectors | High | balance + position + order book snapshot/delta CBOR vectors |
| ✅ | SBE payload vectors | High | `schema_id=0x01`, template IDs; verify vs `sbe_generated.rs` |
| ✅ | Channel stream-ID mapping vectors | Medium | `channel_id * 4 + offset` client/server cases |
| ✅ | End-to-end integration scripts | Medium | `tests/conformance/e2e_driver.sh` + `test_e2e_driver_order_to_execution` |
| ✅ | CI matrix for conformance runners | Medium | Rust reference + per-language SDK jobs |

---

### 16.2 FSL Codegen Parity (Track A)

Mirror [`RustCodegen`](crates/fig-fsl/src/codegen.rs) for every `--lang` target.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Typed enum generation (all targets) | High | Go/C++/C#/Python/TS/OCaml/Zig/Java |
| ✅ | Nested struct types (all targets) | High | `type_defs` in all targets |
| ✅ | Type aliases with constraint docs | Medium | `ClientOrderId`, `Price`, `Quantity`, … |
| ✅ | Message metadata constants | Medium | Go const blocks; TS/Java/Zig exports; OCaml values |
| 🔶 | Python: dataclasses + CBOR serializers | High | FSL types + runtime CBOR via `fig-python` (PyO3) |
| 🔶 | Go: structs + JSON/CBOR tags | High | `json` + `cbor` struct tags + field manifests |
| 🔶 | C++: structs + CBOR/JSON helpers | High | FSL types; wire CBOR via `fig-ffi` |
| 🔶 | C#: classes + System.Text.Json + CBOR | High | FSL types; wire CBOR via `fig-ffi` |
| 🔶 | TypeScript: interfaces + CBOR encode/decode | Medium | FSL types + `bindings/typescript/fig.ts` over `fig-ffi` |
| 🔶 | OCaml: records + variant enums + CBOR | Low | FSL types + ctypes; wire via `fig-ffi` |
| 🔶 | Zig: structs + CBOR helpers | Medium | FSL types + `@cImport`; wire via `fig-ffi` |
| ✅ | SBE encode/decode codegen per language | Medium | `ftlc --lang sbe-{go,cpp,csharp,typescript,zig}` → `bindings/` via `sbe_target_codegen.rs` + xtask |
| ✅ | Verify `sbe-xml` vs Rust `--lang sbe` wire compatibility | Medium | Cross-validation test in `target_codegen_tests.rs` |
| ✅ | Java codegen target (`--lang java`) | Low | `JavaCodegen` → `Generated.java` |
| ✅ | Update SPEC §11.2 claims to match reality | Low | ADR 0004 + SPEC §11.2 |
| ✅ | Codegen tests per target (orders.fsl) | High | `target_codegen_tests.rs` — 14 tests |

---

### 16.3 `fig-ffi` + Language Bindings (Track B, Option 1)

Wrap `fig-core` once; expose stable C ABI; bind per language.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `crates/fig-ffi` crate | High | cbindgen → `fig.h`; Tier 1–4: client, compression, fragmentation, 0-RTT, migration, stream decode; `jwt.rs` + `sbe.rs` (NewOrderSingle, CandleBar, SymbolTicker) |
| ✅ | FFI API surface spec | High | Frame + CBOR + client + advanced features in `fig.h` |
| ✅ | `fig-python` (PyO3 / maturin) | High | Reference SDK — codec, client (`connect`/`request`/`subscribe` + auth), binding conformance |
| ✅ | `fig-csharp` (P/Invoke) | Medium | `bindings/csharp/Fig` — encode + `FigClient` + JWT + SBE generated |
| ✅ | `fig-go` (cgo) | Medium | `bindings/go/fig` — encode + `Client` + JWT + SBE generated |
| ✅ | `fig-cpp` (header + link staticlib) | Medium | RAII `fig::Client` + auth encode + JWT + SBE generated |
| ✅ | `fig-ocaml` (ctypes) | Low | `bindings/ocaml/fig.ml` over `fig.h` + JWT helpers |
| ✅ | `fig-zig` (`@cImport fig.h`) | Low | `bindings/zig/fig.zig` + JWT + SBE generated |
| ✅ | TypeScript / Bun (`bun:ffi`) | Medium | `bindings/typescript/fig.ts` — connect, request, subscribe, stream decode, JWT, SBE generated |
| ✅ | `fig-java` (JNI) | Low | `bindings/java/FigNative.java` + `native/fig_jni.c` + JWT helpers |
| ✅ | Perps merge FFI handles (funding, ledger, liquidations, agg trades) | Medium | `fig_funding_*`, `fig_ledger_*`, `fig_liquidation_*`, `fig_agg_trades_*` |
| ✅ | Binding conformance tests | High | Python + `fig-ffi` run §16.1 vectors; `advanced` + `client_integration` + `stream_state` tests |

High-level client API (all bindings):

```text
FigClient::connect(addr, tls_config)
FigClient::open_channel(path, schema_id)
FigClient::send_request(channel, payload, content_type)
FigClient::recv_stream() → stream of frames
FigClient::ping() / close()
```

---

### 16.4 SDK Feature Rollout (SPEC Tiers)

Roll out incrementally per binding; do not expose all 20 `fig-core` modules at once.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Tier 1 — frames, REQUEST/RESPONSE, CBOR payloads | High | All FFI bindings + `fig-python` |
| ✅ | Tier 2 — STREAM_OPEN/CLOSE, JWT auth, PING/PONG, seq nums | High | Connect/subscribe/request + JWT encode/decode/verify in all bindings |
| ✅ | Tier 3 — SUBSCRIBE, market data + account streams, correlation | Medium | SUBSCRIBE + auth + `fig_frame_payload` / CBOR stream decode helpers |
| ✅ | Tier 4 — 0-RTT resumption, migration, fragmentation, zstd | Low | `fig_client_connect_0rtt`, migration prepare/apply, compress/split/reassemble in `fig-ffi` |

---

### 16.5 Server + Gateway (multi-codec & proxy)

End-to-end parity requires server and gateway changes, not just client SDKs.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Exchange-sim multi-codec dispatch | High | CBOR/SBE on orders, cancel, queries via `ContentType` |
| ✅ | SBE payload decode in exchange-sim | High | NewOrderSingle + CancelRequest + ExecutionReport responses |
| ✅ | Protobuf payload decode in exchange-sim | Medium | Optional `ftlc --lang proto` codegen only — not on-the-wire |
| ✅ | Multi-codec integration tests | High | `test_sbe_new_order_single` + `multi_codec` unit test |
| ✅ | Wire `fig-gateway` to proxy to FIG backend | Medium | `--fig-backend` proxies REST GET + WS `SUBSCRIBE` to exchange-sim |
| ✅ | Production gateway service template | Medium | `examples/production-gateway` + gateway `--health-addr` / `--metrics-addr` |

---

### 16.6 Pure Protocol Libraries (Track B, Option 2)

For languages where FFI is unacceptable — optional alternative to §16.3.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Generated frame encode/decode (spec-driven) | Medium | `protocol_codegen.rs` → frame header encode/decode |
| ✅ | Generated extension tag library | Medium | All 29 extension tags in `bindings/cpp/pure/fig_protocol.hpp` + `bindings/zig/pure/protocol.zig` |
| ✅ | Generated channel manager | Medium | Stream ID formula, seq nums in pure protocol libs |
| ✅ | Generated control frame dispatcher | Medium | PING/PONG + control dispatch in pure protocol libs |
| ⬜ | TREE transport binding per language | Low | Native FIG client over UDP/TLS (ALPN `fig/1`); or TCP downgrade (`FIG\x01`) where TREE is unavailable |
| ✅ | C++ pure protocol library | Medium | `bindings/cpp/pure/fig_protocol.hpp` via `protocol_codegen.rs` |
| ✅ | Zig pure protocol library | Low | `bindings/zig/pure/protocol.zig` via `protocol_codegen.rs` |

---

### 16.7 Per-Language Recommended Path

| Language | Codegen (§16.2) | Runtime path | Notes |
|---|---|---|---|
| Python | enums + CBOR serializers | PyO3 → `fig-core` | Best ROI; ship first |
| C# | classes + System.Text.Json | P/Invoke → `fig-ffi` | Common in trading |
| C++ | full structs + SBE | Generated protocol or cbindgen | Low-latency HFT |
| Go | structs + tags + CBOR | cgo or pure Go port | `--lang go` exists |
| TypeScript | interfaces + CBOR | N-API / Deno FFI → `fig-ffi` (Node, Bun, Deno) | No WASM; browser uses gateway |
| OCaml | variant enums + CBOR | ctypes → `fig-ffi` | Quant/research |
| Zig | struct layout + comptime | `@cImport fig.h` or pure Zig | Comptime-friendly codegen |
| Java | add `--lang java` first | JNI → `fig-ffi` | SPEC lists; not in `ftlc` yet |

---

### 16.8 Anti-Patterns (do not do)

- **Do not** ship gateway REST GET endpoints without native FIG `REQUEST`/`RESPONSE` for that query — REST is translation only (§17.0b).
- **Do not** ship gateway WS streams without native FIG `SUBSCRIBE` + FSL message for that stream — WS is translation only (§17).
- **Do not** treat REST gateway as native FIG parity — without `--fig-backend`, gateway translates in-process only.
- **Do not** hand-port `fig-core` eight times without §16.1 conformance vectors — wire drift is guaranteed.
- **Do not** stop at flat dataclasses/structs — that is ~5% of Rust capability.
- **Do not** assume `sbe-xml` and Rust `--lang sbe` are wire-identical without verification.

---

### 16.9 Target End State

For each non-Rust language:

```text
ftlc --lang {lang}  →  types + enums + nested types + CBOR/SBE codecs + message constants
{fig}-{lang} SDK    →  Tier 4 client (connect, channels, auth, streaming, migration)
conformance tests   →  same vectors as Rust, CI green
exchange-sim        →  accepts CBOR + SBE + Protobuf from all SDKs
```

**Minimal viable parity** (recommended first milestone):

```text
FSL codegen (full) + PyO3/FFI client (Tier 2) + CBOR only + conformance tests
```

---

### 16.10 Priority Summary (§16)

| Priority | Items |
|---|---|
| **High** | ✅ §16.1 conformance vectors · §16.2 enum/nested/serializer + SBE codegen · §16.3 `fig-ffi` + all bindings · §16.4 Tier 1–4 SDK · §16.5 multi-codec exchange-sim |
| **Medium** | ✅ C# / Go / C++ bindings · SBE per-language codegen · gateway proxy · TypeScript SDK · pure protocol libs (C++) |
| **Low** | ✅ OCaml / Zig bindings · Tier 4 advanced features · Java target · pure Zig protocol lib · ⬜ TREE transport per language |

**§16 complete ✅** — optional follow-ups: [§0.7](#07-p4--long-horizon--optional) (TREE per language, §16.2 CBOR serializers).

---

## 17. Broker ↔ Client API Parity

FIG must support the **full broker API surface** that modern venues expose — **live
streams** (WebSocket) **and historical/query pulls** (REST GET). Reference APIs:
[Binance Spot REST](https://developers.binance.com/docs/binance-spot-api-docs/rest-api),
[Binance WebSocket Streams / User Data](https://developers.binance.com/docs/binance-spot-api-docs/web-socket-streams),
[Hyperliquid WebSocket subscriptions](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket/subscriptions).

**Goal:** a FIG-native broker serves typed messages over TREE; clients use native
FIG frames (`SUBSCRIBE` for push, `REQUEST` for pull); legacy REST/WS gateways map
to the **same FSL messages** — not parallel JSON schemas.

### Native FIG is canonical (REST & WS are edge adapters)

**Every capability in §17.0 (live) and §17.0b (historical) must work in native FIG
first.** REST GET and WebSocket are legacy wire formats that gateways translate to
and from FIG frames on TREE.

| Layer | Role | Completeness rule |
|---|---|---|
| **Native FIG** (TREE + frames + FSL) | **Source of truth** | **Push:** `SUBSCRIBE` → `STREAM_ITEM` · **Pull:** `REQUEST` → `RESPONSE` (or `REQUEST` → `STREAM_ITEM` × N for large ranges) |
| **Gateway REST** | **Adapter** | HTTP `GET`/`POST` ↔ native FIG `REQUEST`/`RESPONSE`; query params ↔ payload or `CHANNEL_PATH` |
| **Gateway WS / SSE** | **Adapter** | Topics / SSE ↔ native FIG `SUBSCRIBE` / `STREAM_ITEM` |
| **Client SDK** | **Native first** | `FigClient::subscribe(…)` and `FigClient::request(…)` over TREE |

```text
                         NATIVE FIG (required, complete)
  Client ──TREE──► SUBSCRIBE ──► STREAM_ITEM × N     (live)
         ──TREE──► REQUEST ──► RESPONSE             (small query)
         ──TREE──► REQUEST ──► STREAM_ITEM × N       (large historical range)
         ▲                    │
         │                    │ same FSL payloads
  Legacy ├──REST GET─────────┤
         └──WS/SSE───────────┘   fig-gateway (translate only)
```

**Native FIG must cover everything** means:

1. **Live parity (§17.0)** — each WS stream has native `SUBSCRIBE` + FSL `stream_item`.
2. **Historical parity (§17.0b)** — each REST GET has native `REQUEST`/`RESPONSE` +
   FSL `request_response` (or `request_stream` for paginated dumps per SPEC §7.3).
3. **Broker implements on FIG first** — exchange-sim handles native frames; gateway
   re-exports, not the other way around.
4. **Pull + push work together** — e.g. `SUBSCRIBE` live candles + `CandleBarRequest`
   backfill after reconnect gap; gateway `GET …/candles/…` is not a separate feature set.
5. **Conformance proves native wire** — golden vectors are FIG CBOR/SBE frames;
   gateway tests round-trip *through* native FIG.
6. **No REST-only or WS-only features** — incumbent aliases (e.g. Binance `GET /klines`,
   `@kline_5m`) map to native FIG `/candles` paths and `CandleBar` — not separate semantics.

**Roles:**

| Role | Responsibility |
|---|---|
| **Broker / exchange** | Own stream truth **and** queryable history: snapshots, deltas, bar store, account state |
| **Client / trader** | `SUBSCRIBE` for live; `REQUEST` for historical; merge, paginate, gap-fill on reconnect |
| **Gateway** (optional edge) | Map legacy REST GET / WS ↔ native FIG; never add queries or streams FIG lacks |

**Three interaction classes (all required on native FIG):**

| Class | Native FIG pattern | Binance analogue | FIG today |
|---|---|---|---|
| **Live push** | `SUBSCRIBE` → `STREAM_ITEM × N` | WS candle stream (Binance: `@kline_*`) | ✅ native broker paths; client SDK merge helpers in §16 |
| **Point / batch query** | `REQUEST` → `RESPONSE` | REST `GET …/candles/{interval}` (Binance: `/klines`) | ✅ candles, trades, ticker, account, margin, fills, funding, ledger |
| **Large range query** | `REQUEST` → `STREAM_ITEM × N` → close | REST paginated history | ✅ all batch queries via `stream_paginated_batch` when >50 rows |

**Two data domains:**

| Domain | Auth | Live (§17.0) | Historical (§17.0b) |
|---|---|---|---|
| **Public market data** | Usually none | trades, depth, candles, ticker | candles, trades, depth snapshot at time T |
| **Private account** | Required | balances, positions, executions | order history, fill history, ledger |

Normative detail belongs in [SPEC.md](SPEC.md) §7.3 (interaction patterns), §9
(addressing), new FSL schema files, and ADR 0006 (broker API parity). Do not hand-edit
generated Rust types.

---

### 17.0 Live streaming coverage matrix

Map incumbent broker WS offerings → **native FIG** channel paths + FSL messages.
Use this as the completeness checklist: **if a row is ⬜, native FIG is incomplete**
(gateway WS for that stream is blocked until the native path exists).

#### Public market data (unauthenticated)

| Incumbent stream | Binance example | Hyperliquid example | Native FIG `CHANNEL_PATH` | FSL message | Native FIG | Gateway WS |
|---|---|---|---|---|---|---|
| Order book snapshot | REST `depth` / WS snapshot | `l2Book` snapshot | `marketdata/{symbol}/book` | `OrderBookSnapshot` | ✅ | ✅ `@depth` → quotes |
| Order book delta | `@depth` diff | `l2Book` updates | `marketdata/{symbol}/book` | `OrderBookDelta` | ✅ incremental refresh | ✅ `@depth` → quotes |
| Best bid/offer | `@bookTicker` | `bbo` | `marketdata/{symbol}/bbo` | `BestBidOffer` | ✅ | ✅ `@bookTicker` |
| Trade tape | `@trade` | `trades` | `marketdata/{symbol}/trades` | `PublicTrade` | ✅ (`PublicTradeEvent`) | ✅ `@trade` |
| Aggregate trades | `@aggTrade` | — | `marketdata/{symbol}/aggtrades` | `AggregateTrade` | ✅ | ✅ `@aggTrade` |
| Candles (OHLCV bars) | Binance: `@kline_{interval}` | — (build from trades) | `marketdata/{symbol}/candles/{interval}` | `CandleBar` | ✅ (`CandleBarEvent`) | ✅ `@kline_*` |
| 24h ticker | `@ticker` | — | `marketdata/{symbol}/ticker` | `SymbolTicker` | ✅ subscribe + GET | ✅ `@ticker` |
| Mini ticker | `@miniTicker` | `allMids` | `marketdata/ticker/all` or per-symbol | `MiniTicker` / `AllMids` | ✅ | ✅ `@miniTicker` |
| Mark / index / funding rate | futures mark price | `activeAssetCtx` | `marketdata/{symbol}/mark` | `MarkPriceUpdate` | ✅ | ✅ `@markPrice` |
| Liquidation feed (public) | `@forceOrder` | public liquidation trades | `marketdata/liquidations` | `LiquidationTrade` | ✅ | ✅ `@forceOrder` |

#### Private user / account (authenticated)

| Incumbent stream | Binance User Data event | Hyperliquid subscription | Native FIG `CHANNEL_PATH` | FSL message | Native FIG | Gateway WS |
|---|---|---|---|---|---|---|
| Order / execution updates | `executionReport` | `orderUpdates` / fills in `userEvents` | `trading/accounts/{account}/executions` | `ExecutionReport` | ✅ `SUBSCRIBE` + fill fan-out | ✅ `@executionReport` + Hyperliquid `orderUpdates` |
| Account balance snapshot | `outboundAccountPosition` | `spotState.balances` | `accounts/{account}/balances` | `BalanceSnapshot` | ✅ snapshot on subscribe | ✅ `@balance` + Hyperliquid `spotState` |
| Balance delta | `balanceUpdate` | balance WS (`subscribeBalance`) | `accounts/{account}/balances` | `BalanceUpdate` | ✅ on fill | ✅ Hyperliquid `balanceUpdate` |
| Open orders snapshot | (via REST / WS API) | order state in user channel | `trading/accounts/{account}/orders/open` | `OpenOrdersSnapshot` | ✅ | ✅ Hyperliquid `openOrders` |
| Position snapshot | futures account | `clearinghouseState` / `subscribePosition` | `accounts/{account}/positions` | `PositionSnapshot` | ✅ snapshot on subscribe | ✅ Hyperliquid `clearinghouseState` |
| Position delta | — | position WS updates | `accounts/{account}/positions` | `PositionUpdate` | ✅ fill fan-out | ✅ Hyperliquid passthrough |
| Margin / account summary | account info REST | clearinghouse margin summary | `accounts/{account}/margin` | `MarginSummary` / `MarginUpdate` | ✅ GET + live stream | ✅ passthrough |
| User fills stream | trade in `executionReport` | `userFills` | `accounts/{account}/fills` | `UserFill` or reuse `ExecutionReport` | ✅ executions sub + `FillHistoryRequest` GET (reuse `ExecutionReport`) | ✅ Hyperliquid `userFills` → executions |
| Funding payments | — | `userFundings` | `accounts/{account}/funding` | `FundingPayment` | ✅ sub + history GET | ✅ WS catalog |
| Ledger (deposit/withdraw/transfer) | — | `userNonFundingLedgerUpdates` | `accounts/{account}/ledger` | `LedgerUpdate` | ✅ sub + history GET + fee on fill | ✅ WS catalog |
| Liquidation (user) | — | `liquidation` in `userEvents` | `accounts/{account}/liquidations` | `UserLiquidation` | ✅ on balance breach | ✅ Hyperliquid passthrough |
| List / OCO status | `listStatus` | — | `trading/accounts/{account}/orderlists` | `OrderListStatus` | ✅ subscribe + fan-out on rest | ✅ `@orderlists` + Hyperliquid `listStatus` |
| Stream lifecycle | `eventStreamTerminated` | subscription ack + snapshot flag | control / `STREAM_CLOSE` | session event | ✅ `STREAM_CLOSE` on UNSUBSCRIBE + `is_snapshot` | ✅ `fig_stream_close_to_legacy_json` + subscribe ack |

**Legend:** ✅ native FIG complete · 🔶 partial · ⬜ not started · **Gateway WS** blocked until native FIG ✅

---

### 17.0b Historical / query coverage matrix (REST-style pull)

**Terminology:** FIG uses **candles** (`/candles/{interval}`, `CandleBar`) — not "klines"
(Binance-only). Gateway maps incumbent `/klines` and `@kline_*` aliases to native FIG paths.

Map incumbent broker **REST GET** (and RPC query) endpoints → **native FIG
`REQUEST`/`RESPONSE`**. Use this checklist: **if Native FIG is ⬜, gateway REST GET
for that query is blocked.**

| Query | Binance REST example | Hyperliquid / other | Native FIG `CHANNEL_PATH` | Request → Response FSL | Native FIG | Gateway REST |
|---|---|---|---|---|---|---|
| Historical candles | Binance alias: `GET /api/v3/klines` | candle snapshot API | `marketdata/{symbol}/candles/{interval}` | `CandleBarRequest` → `CandleBarBatch` | ✅ | ✅ `/klines` alias + native path |
| Historical public trades | `GET /api/v3/aggTrades` | — | `marketdata/{symbol}/trades` | `TradeHistoryRequest` → `PublicTradeBatch` | ✅ | ✅ `/api/v3/aggTrades` alias |
| Order book snapshot at time | `GET /api/v3/depth` | `l2Book` snapshot REST | `marketdata/{symbol}/book` | `OrderBookRequest` → `OrderBookSnapshot` | ✅ `at_time` + book history store | ✅ `/api/v3/depth` alias |
| 24h ticker snapshot | `GET /api/v3/ticker/24hr` | — | `marketdata/{symbol}/ticker` | `TickerRequest` → `SymbolTicker` | ✅ | ✅ `/api/v3/ticker/24hr` alias |
| Exchange info / symbols | `GET /api/v3/exchangeInfo` | meta endpoints | `/.well-known/capabilities` | `CapabilitiesRequest` → `CapabilitiesResponse` | ✅ | ✅ `/api/v3/exchangeInfo` alias |
| Account snapshot | `GET /api/v3/account` | clearinghouse state | `accounts/{account}` | `AccountSummaryRequest` → `AccountSummary` / `MarginSummary` | ✅ | ✅ `/api/v3/account` alias |
| Open orders | `GET /api/v3/openOrders` | open orders REST | `trading/accounts/{account}/orders/open` | `OpenOrdersRequest` → `OpenOrdersSnapshot` | ✅ | ✅ `/api/v3/openOrders` alias |
| Order history | `GET /api/v3/allOrders` | — | `trading/accounts/{account}/orders` | `OrderHistoryRequest` → `OrderHistoryBatch` | ✅ | ✅ `/api/v3/allOrders` alias |
| User trade / fill history | `GET /api/v3/myTrades` | user fills REST | `accounts/{account}/fills` | `FillHistoryRequest` → `FillHistoryBatch` | ✅ | ✅ `/api/v3/myTrades` alias |
| Ledger / deposits / withdrawals | — | ledger REST | `accounts/{account}/ledger` | `LedgerHistoryRequest` → `LedgerHistoryBatch` | ✅ | ✅ native path + REST alias |
| Funding history | `GET /fundingRate` (futures) | `userFundings` history | `accounts/{account}/funding` | `FundingHistoryRequest` → `FundingHistoryBatch` | ✅ | ✅ native path + REST alias |
| Position snapshot | futures account | clearinghouse REST | `accounts/{account}/positions` | `PositionRequest` → `PositionSnapshot` | ✅ GET + subscribe snapshot | ✅ native path + REST alias |

**Native FIG interaction patterns for historical data:**

| Result size | Pattern | FIG frames | FSL `channel_type` |
|---|---|---|---|
| Small (≤ few KB) | Request-Response | `REQUEST` → `RESPONSE` | `request_response` |
| Large / paginated | Request-Stream | `REQUEST` → `STREAM_ITEM` × N → `STREAM_CLOSE` | `request_stream` (SPEC §9) |
| Live ongoing | Pub/Sub | `SUBSCRIBE` → `STREAM_ITEM` × N | `stream_item` (§17.0) |

**Pagination (all historical requests):** `start_time`, `end_time`, `limit`, `cursor`
in request payload; `next_cursor` + `has_more` in response — mirror Binance time/id
pagination. Gap-fill after live disconnect: client sends `CandleBarRequest` or
`FillHistoryRequest` for the missing window, then resumes `SUBSCRIBE`.

**Legend:** **Gateway REST** = HTTP GET mapping only; **blocked until native FIG ✅**

---

### 17.1 Schema & Protocol Specification

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | ADR 0006: broker ↔ client API parity | High | [0006-broker-api-parity.md](docs/adr/0006-broker-api-parity.md) |
| ✅ | Split FSL schemas by domain | High | `orders.fsl` + `marketdata.fsl` + `account.fsl` merged at codegen |
| ✅ | Well-known schema IDs in SPEC | High | §9.1 domain IDs `0x01–0x03`; wire schema `0x01` + template IDs |
| ✅ | Unified `CHANNEL_PATH` tree | High | Core §17 paths + capabilities + open orders + order history |
| ✅ | Interaction type on path | High | `path_policy.rs` + SPEC §9.1 interaction classes |
| ✅ | Unified `ROUTING_KEY` convention | High | Dot-separated topics; normative in SPEC §9.1 |
| ✅ | Snapshot-on-subscribe contract | High | `is_snapshot` + book `sequence` on `MarketDataSnapshot` |
| ✅ | `.well-known/capabilities` stream catalog | High | `CapabilitiesResponse` in exchange-sim + gateway REST |
| ✅ | Private stream auth model | High | `fig-dev-{account}` + `FIG_DEV_OPEN`; integration test `test_private_auth_required` |
| ✅ | `subscriptionId` / correlation | Medium | `CORRELATION_ID` extension = channel ID on SUBSCRIBE/STREAM_ITEM |
| ✅ | Multi-stream per connection | Medium | Documented in SPEC §9.1; exchange-sim + fig-cli demos |
| ✅ | UNSUBSCRIBE semantics | Medium | `FrameType::Unsubscribe` + session store cleanup |
| ✅ | Schema evolution policy | Medium | ADR 0004/0006 + optional `is_snapshot` stream fields |

#### FSL messages — public market data (`marketdata.fsl`)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `CandleInterval` type | High | In `orders.fsl` (pending schema split) |
| ✅ | `CandleBar` | High | Type + `CandleBarEvent` stream wrapper |
| ✅ | `CandleBarRequest` → `CandleBarBatch` | High | Historical + gap backfill |
| ✅ | `TradeHistoryRequest` → `PublicTradeBatch` | High | Trade tape query |
| ✅ | `OrderBookRequest` → `MarketDataSnapshot` | High | GET `marketdata/{symbol}/book` with `sequence` + `is_snapshot` |
| ✅ | `TickerRequest` → `SymbolTicker` | Medium | 24h stats snapshot query |
| ✅ | `PublicTrade` | High | Type + `PublicTradeEvent` stream wrapper |
| ✅ | `AggregateTrade` | Medium | Type + `AggregateTradeEvent` stream + history query |
| ✅ | `BestBidOffer` | High | BBO stream |
| ✅ | `OrderBookSnapshot` | High | Dedicated type; subscribe snapshot |
| ✅ | `OrderBookDelta` | High | Incremental book updates on fill |
| ✅ | `SymbolTicker` | Medium | Stream + GET |
| ✅ | `MiniTicker` / `AllMids` | Medium | `AllMidsBatch` GET + `marketdata/ticker/all` sub |
| ✅ | `MarkPriceUpdate` | Low | Stream + GET |
| ✅ | `LiquidationTrade` | Low | Public liquidation stream (large fills) |

#### FSL messages — private account (`account.fsl`)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `BalanceSnapshot` | High | Snapshot on subscribe |
| ✅ | `BalanceUpdate` | High | Delta on fill |
| ✅ | `MarginSummary` | High | GET on `accounts/{account}/margin` |
| ✅ | `PositionSnapshot` | High | Snapshot on subscribe |
| ✅ | `PositionUpdate` | High | Fill fan-out wired |
| ✅ | `OpenOrdersSnapshot` | Medium | `OpenOrdersRequest` → snapshot query |
| ✅ | `FundingPayment` | Medium | Stream + `FundingHistoryBatch` |
| ✅ | `LedgerUpdate` | Medium | Stream + `LedgerHistoryBatch` + fee on fill |
| ✅ | `UserLiquidation` | Medium | Stream on balance breach |
| ✅ | `OrderListStatus` | Low | FSL type defined (stream wiring optional) |
| ✅ | `PageInfo` | Medium | Shared pagination type in FSL |

#### Trading stream gaps (extend `trading.orders.fsl`)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `ExecutionReport` | — | `SUBSCRIBE` on `trading/accounts/{account}/executions` + fill fan-out |
| ✅ | `UserFill` vs reuse `ExecutionReport` | Medium | Reusing `ExecutionReport` + `FillHistoryBatch` by design |
| ✅ | Order update stream | High | New/Canceled/Fill on executions sub + order history |
| ✅ | `OrderHistoryRequest` → `OrderHistoryBatch` | High | GET + `request_stream` when >50 rows |
| ✅ | `FillHistoryRequest` → `FillHistoryBatch` | High | Query on `accounts/{account}/fills` |
| ✅ | `OpenOrdersRequest` → `OpenOrdersSnapshot` | Medium | GET `trading/accounts/{account}/orders/open` |

#### FSL messages — historical / query (cross-schema)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Shared pagination types | High | `PageInfo` in FSL; batches inline `next_cursor` + `has_more`; cursor on requests + REST `?cursor=` |
| ✅ | `CapabilitiesRequest` → `CapabilitiesResponse` | Medium | `.well-known/capabilities` |
| ✅ | `LedgerHistoryRequest` → `LedgerHistoryBatch` | Medium | Implemented |
| ✅ | `FundingHistoryRequest` → `FundingHistoryBatch` | Low | Implemented |
| ✅ | `*Batch` list response types | High | Candle, trade, fill, funding, ledger batches in `orders.fsl` |
| ✅ | `gateway rest` GET for every §17.0b row | High | `rest_query.rs` + `cargo xtask check-gateway` CI gate |

---

### 17.2 Cross-Cutting Protocol (Both Sides)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Public `SUBSCRIBE` (no auth) | High | MD paths in exchange-sim; rate limit by tier pending |
| ✅ | Private `SUBSCRIBE` (auth required) | High | `auth.rs` + `test_private_auth_required` |
| ✅ | Snapshot then delta | High | `is_snapshot` on candles/BBO/ticker/balances/book; SPEC §9.1 |
| ✅ | Sequence / gap detection | High | Book `sequence` + `SEQUENCE_NUM` ext; gap-fill in STREAMING.md |
| 🔶 | Tier 1 SDK: native `request()` | High | See [§0.3](#03-p2--reference-client-and-sdk-polish) |
| ✅ | Session resume restores subscriptions | High | `Method: RESUME` + `.well-known/resume` + session store |
| ✅ | Heartbeat independent of data | — | PING/PONG in §3; used by `fig-cli` |
| 🔶 | Tier 3 SDK parity definition | Medium | See [§0.3](#03-p2--reference-client-and-sdk-polish) |

---

### 17.3 Public Market Data — Broker & Client

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | L2 quote streaming | — | `OrderBookSnapshot` + `OrderBookDelta` on subscribe/fill; client merge helpers pending (§16) |
| ✅ | `handle_subscribe` for all MD paths | High | `book`, `bbo`, `trades`, `candles`, `ticker` in `broker_api` |
| ✅ | Trade tape fan-out | High | `MarketDataHub::on_trade` + `post_fill_updates` |
| ✅ | Candle aggregator | High | OHLCV from trades; partial + final bars |
| ✅ | BBO stream | High | `update_bbo` + subscribe path |
| ✅ | Ticker aggregator | Medium | 24h rolling window from trade tape in `MarketDataHub` |
| ✅ | `fig-cli` MD demo suite | High | Candles + ticker query in `fig-cli`; book/trades/BBO via subscribe |
| ✅ | Client merge: order book | High | `fig-client::OrderBookState` |
| ✅ | Client merge: candles | High | `fig-client::CandleState` |
| ✅ | SDK helpers | High | `fig-client::FigSdkClient` subscribe/request APIs |

---

### 17.4 Private Account & Trading — Broker & Client

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `AccountSummary` REQUEST | — | GET on `accounts/{account}` |
| ✅ | `SUBSCRIBE accounts/{account}/balances` | High | Snapshot + `BalanceUpdate` on fill |
| ✅ | `SUBSCRIBE accounts/{account}/margin` | High | Snapshot + live `MarginUpdate` on subscribe and fill (`test_margin_subscribe_snapshot`) |
| ✅ | `SUBSCRIBE accounts/{account}/positions` | High | Snapshot on subscribe + `PositionUpdate` on fill (`test_position_delta_on_fill`) |
| ✅ | `SUBSCRIBE trading/…/executions` | High | `post_fill_updates` execution fan-out |
| ✅ | `SUBSCRIBE trading/…/fills` | Medium | Same as executions sub; dedicated path optional |
| ✅ | `SUBSCRIBE accounts/{account}/funding` | Medium | Subscribe path wired |
| ✅ | `SUBSCRIBE accounts/{account}/ledger` | Medium | Subscribe + fee ledger on fill |
| ✅ | `SUBSCRIBE accounts/{account}/liquidations` | Low | Snapshot on subscribe + push on balance breach |
| ✅ | Account state on order fill | High | Balance + execution + ledger updates on fill |
| ✅ | `fig-cli` private stream demo | High | Balance subscribe + authenticated queries in `fig-cli` |
| ✅ | Client account cache | High | `fig-client::AccountCache` |

---

### 17.4b Historical / Query API — Native FIG (Broker & Client)

**REST GET is not a separate product.** Each row in §17.0b is implemented as native
FIG `REQUEST` on TREE; the REST gateway translates HTTP → same frames.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | `AccountSummary` query | — | GET on `accounts/{account}` |
| ✅ | `MarketDataSnapshot` query | — | GET `marketdata/{symbol}/book` with `at_time` historical lookup |
| ✅ | `handle_request` router for query paths | High | `broker_api::handle_query_request` |
| ✅ | Historical candle store + handler | High | `MarketDataHub::query_candles` |
| ✅ | Public trade history handler | High | `MarketDataHub::query_trades` |
| ✅ | Order / fill history handler | High | `FillHistoryRequest` + `OrderHistoryRequest` handlers + integration tests |
| ✅ | Open orders query | Medium | GET `trading/accounts/{account}/orders/open` |
| ✅ | Large-range `request_stream` | Medium | All batch queries use `stream_paginated_batch` when >50 rows |
| ✅ | Pagination enforcement | High | `limit` + `cursor`/`next_cursor` on history batches; REST query params wired |
| ✅ | `fig-cli` historical demo | High | Candle + funding + ledger queries in `fig-cli` |
| ✅ | SDK `request_candles`, `request_fills`, … | High | `fig-client::FigSdkClient` |
| ✅ | Gap-fill workflow | High | Documented in STREAMING.md + PROTOCOL.md; `OrderBookRequest` / `CandleBarRequest` backfill |
| ✅ | Auth on private queries | High | Same auth as private streams |

---

### 17.5 Gateway Egress (REST + WebSocket)

**Gateway is not a shortcut.** Implement native FIG first (§17.3–17.4, §17.4b); then
add mappings here. REST `GET` ↔ native `REQUEST`/`RESPONSE`; WS topic ↔ native
`SUBSCRIBE`/`STREAM_ITEM` — **identical FSL payloads**.

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Native FIG completeness gate | High | `cargo xtask check-gateway` + integration tests |
| ✅ | REST GET catalog spec | High | Native paths use `/candles/{interval}`; gateway maps Binance `/klines` alias |
| ✅ | FSL `gateway rest` for all query types | High | Core query types in `orders.fsl` |
| ✅ | Query param ↔ CBOR request mapping | High | `rest_query.rs`: `?start=&end=&limit=` → request fields |
| ✅ | WS topic catalog spec | High | `ws_catalog.rs`; Binance `@kline_*` / Hyperliquid topics |
| ✅ | Inbound WS → FIG proxy | High | `legacy_ws_json_to_fig_subscribe` |
| ✅ | Gateway `--fig-backend` for queries + streams | Medium | REST GET via `proxy_frame`; WS holds `BackendSession` for live `STREAM_ITEM`s |
| ✅ | FIX market data (MD entries) | Low | `fig_bbo_to_fix_md_snapshot`, `fig_candle_bar_to_fix_md_snapshot`, `fig_order_book_to_fix_md_snapshot` |

---

### 17.6 Codegen, Conformance & Testing

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Regenerate all schemas (`cargo xtask codegen`) | High | §17 types in `orders.fsl`; CI `codegen --check` green |
| ✅ | SBE templates for hot-path streams | High | `fig-core::sbe_stream` encoders (templates 9, 21–22, 27, 30) |
| ✅ | CBOR golden vectors per message | High | 28 vectors incl. snapshot/delta pairs + paginated history frame |
| ✅ | SBE golden vectors per message | High | `sbe.candle_bar*`, `sbe.symbol_ticker`, `sbe.order_book_snapshot`, `sbe.balance_snapshot` |
| ✅ | Snapshot + delta vector pairs | High | `cbor.order_book_snapshot.demo` + `cbor.order_book_delta.demo` |
| ✅ | Multi-codec exchange-sim dispatch | High | CBOR/SBE via `ContentType`; `multi_codec.rs` + integration test |
| ✅ | E2E: public MD subscribe suite | High | BBO/trades/candles/book/agg-trade/mark tests |
| ✅ | E2E: private account subscribe suite | High | Auth + balance/margin/position/executions/orderlists |
| ✅ | E2E: native historical REQUEST suite | High | Ticker/capabilities/open-orders/order-history/fill-history/pagination |
| ✅ | E2E: gateway REST GET round-trip | High | `gateway_legacy_ws_alias_e2e` REST fixtures |
| ✅ | E2E: gateway WS alias round-trip | Medium | Binance + Hyperliquid fixture tables |
| ✅ | E2E: gateway FIX order alias round-trip | Medium | FIX `35=D` → native order `REQUEST` |
| ✅ | §16 binding tests for new types | Medium | post_only + InstrumentCatalogResponse FFI conformance |

---

### 17.7 Exchange Simulator (Reference Broker)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Subscription registry (all stream types) | High | `StreamSubscription` + `AccountSubscription` |
| ✅ | Account state engine | High | `account_state::AccountHub` |
| ✅ | Push account updates on fill | High | `post_fill_updates` balance + execution fan-out |
| ✅ | Candle + trade tape from matcher | High | `market_data::MarketDataHub::on_trade` |
| ✅ | Historical bar + trade tape store | High | In-memory `closed_candles` + trade tape |
| ✅ | Integration tests per §17.0b row | High | capabilities, open orders, order book, order history stream, ticker, auth |
| ✅ | Hyperliquid-style snapshot flag | Medium | `is_snapshot` on candles/BBO/ticker/balances/margin/mark |

---

### 17.8 Documentation

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | SPEC.md: native FIG is canonical transport | High | §9 intro + ADR 0006 cross-ref; gateway edge rules |
| ✅ | SPEC.md §9 stream catalog | High | Full §17.0 public + private path tables in §9.1 |
| ✅ | SPEC.md private vs public auth | High | §9.3 scoping rules (token match, cross-account reject) |
| ✅ | SPEC.md §7.3 historical patterns | High | §9.2 request/response + request_stream + gap-fill |
| ✅ | `docs/QUERY.md` or STREAMING.md § query | High | [QUERY.md](docs/QUERY.md) + [STREAMING.md](docs/STREAMING.md) |
| ✅ | `docs/WS_GATEWAY.md` or GATEWAY.md § WS catalog | High | `ws_catalog.rs` + alias E2E in [GATEWAY.md](docs/GATEWAY.md) |
| ✅ | [AGENTS.md](AGENTS.md) contributor guide | High | One standard; gateway alias placement rules |
| ✅ | [SBE_ORDER_PATH.md](docs/SBE_ORDER_PATH.md) | Medium | Colo SBE order entry + `run_sbe_order_demo` |
| ✅ | ADR 0007 crypto instrument model | Medium | [0007-crypto-instrument-model.md](docs/adr/0007-crypto-instrument-model.md) |
| ✅ | ADR 0006 broker API parity | High | [0006-broker-api-parity.md](docs/adr/0006-broker-api-parity.md) |
| ✅ | PROTOCOL.md worked examples | High | Public MD subscribe + private account + historical query |
| ✅ | TUTORIAL.md streaming steps | Medium | §10 stream/query inventory tables |
| ✅ | README.md message / stream table | Medium | Broker API section with live + query tables |

---

### 17.9 Parity Checklist (Broker ↔ Client)

Aligned with §17.0 / §17.0b matrix status. **Gateway WS/REST/FIX** = reference alias
fixtures in `gateway_legacy_ws_alias_e2e.rs`; native FIG remains canonical.

| Stream category | Broker publishes | Client subscribes | Gateway WS | In FSL | In exchange-sim |
|---|---|---|---|---|---|
| **Historical / query (§17.0b)** | | | | | |
| Historical candles | ✅ | ✅ | — | ✅ | ✅ |
| Public trade history | ✅ | ✅ | — | ✅ | ✅ |
| Fill history | ✅ | ✅ | — | ✅ | ✅ |
| Funding / ledger history | ✅ | ✅ | — | ✅ | ✅ |
| Account / margin query | ✅ GET | ✅ | — | ✅ | ✅ |
| Ticker snapshot query | ✅ GET | ✅ | — | ✅ | ✅ |
| Order / open-order history | ✅ | ✅ | — | ✅ | ✅ |
| Capabilities / exchange info | ✅ | ✅ | — | ✅ | ✅ |
| Gateway REST GET | ✅ | N/A | — | ✅ | ✅ |
| **Live streaming (§17.0)** | | | | | |
| Candles (OHLCV bars) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Public trades | ✅ | ✅ | ✅ | ✅ | ✅ |
| BBO | ✅ | ✅ | ✅ | ✅ | ✅ |
| 24h ticker | ✅ | ✅ | ✅ | ✅ | ✅ |
| Order book (quotes) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Order / execution updates | ✅ | ✅ | ✅ | ✅ | ✅ |
| Balances (snapshot + delta) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Positions (snapshot) | ✅ | ✅ | ✅ | ✅ | ✅ |
| Position delta | ✅ | ✅ | ✅ | ✅ | ✅ |
| Margin live stream | ✅ | ✅ | ✅ | ✅ | ✅ |
| Funding / ledger live | ✅ | ✅ | ✅ | ✅ | ✅ |
| Aggregate trades / mini ticker / mark / liquidations | ✅ | ✅ | ✅ | ✅ | ✅ |
| Snapshot-on-subscribe | ✅ | ✅ | ✅ | ✅ | ✅ |
| Session resume / gap fill | ✅ | ✅ | ✅ | — | ✅ |
| Conformance vectors | ✅ | ✅ | — | — | ✅ |

---

### 17.10 Priority Summary (§17)

**§17 native broker + gateway catalog: complete ✅**

Remaining SDK polish: [§0.3](#03-p2--reference-client-and-sdk-polish).

---

---

# Appendix A: Completed Rust core (§1–15)

**Status: complete ✅** — archived detail for archaeology. Active work is in [§0](#0-active-backlog-fig-repo).

## 1. Transport (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | TREE transport via quinn 0.11 | — | ALPN `fig/1`, self-signed certs, 0-RTT |
| ✅ | FrameDecoder for streaming TREE reads | — | Buffered, handles partial frames |
| ✅ | FigConnection wrapper (open/send/recv/close) | — | Per-channel TREE streams |
| ✅ | 0-RTT session resumption end-to-end | — | FigClient::connect_0rtt + FigServer::accept_0rtt with rejection fallback; TODO: production replay protection |
| ✅ | TCP downgrade mode (`FIG\x01` magic prefix) | Medium | Spec §2.1; FigTcpConnection/FigTcpServer over plain TCP |
| ✅ | Connection migration handling | Low | `migration` module + `FigConnection::prepare_migration` / `apply_migration` |
| ✅ | TREE stream reset → channel CLOSED transition | — | StreamReset/StreamStopped errors detected, force_close_channel transitions to Closed |

---

## 2. Wire Format (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | 16-byte fixed header encode/decode | — | All fields, round-trip tested |
| ✅ | TLV extension encode/decode | — | All 29 extension tags |
| ✅ | All 13 frame types | — | CONTROL through REDIRECT |
| ✅ | Frame flags (ACK_REQUESTED, COMPRESSED, FRAGMENTED, LAST_FRAGMENT) | — | Defined and tested |
| ✅ | Payload fragmentation | Medium | FRAGMENTED + LAST_FRAGMENT split/reassembly via FragmentReassembler |
| ✅ | Payload compression | Medium | zstd compression via compression module; COMPRESSED flag wired |
| ✅ | Reserved field validation (must be 0) | Low | Spec §3.1; decoder rejects non-zero reserved and flag bit 7 |

---

## 3. Control Frames (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | PING / PONG | — | Heartbeat |
| ✅ | GOAWAY | — | Graceful shutdown with reason |
| ✅ | SETTINGS | — | Tier advertisement |
| ✅ | AUTH_REFRESH | — | Frame constructor + token extraction + verify_refresh_token + control dispatcher |
| ✅ | SEQ_RESET | — | Frame constructor + payload parsing + control dispatcher + ChannelManager::reset_seq |
| ✅ | RESEND | Medium | Frame::resend + control dispatcher + FIX ResendRequest conversion |

---

## 4. Channel Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Channel open/close/remove | — | 15 tests |
| ✅ | Sequence numbers (increment, wraparound, duplicate detection) | — | Spec-compliant |
| ✅ | TREE stream ID mapping (client/server parity) | — | `channel_id * 4 + offset` |
| ✅ | Credit-based flow control | — | Per-channel credits, consume/grant, exhaustion error |
| ✅ | Channel reconstruction after reconnect | — | ChannelManager::reconstruct from stored session, reopen TREE streams |
| ✅ | Channel ID leak prevention on stream reset | High | force_close_channel on StreamReset/StreamStopped in FigConnection |
| ✅ | Unidirectional channels | Low | `ChannelDirection` + `open_unidirectional_channel` + direction checks |

---

## 5. Session Management (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Session struct (UUID, auth token, channels, seq tracking) | — | 14 tests |
| ✅ | MemorySessionStore | — | HashMap-backed |
| ✅ | FileSessionStore | — | JSON file persistence, 6 tests |
| ✅ | 0-RTT resumption integration | — | TREE 0-RTT wired through transport; FileSessionStore persists sessions |
| ✅ | Redis session store (`session-redis` feature) | Low | `RedisSessionStore` + `DurableSessionStore::from_env` |
| ✅ | Session expiry / TTL | Medium | Session::is_expired + store TTL on get/purge_expired |
| ✅ | Session migration (connection migration) | Low | `migration::apply_migration` restores session seq state across IP changes |

---

## 6. Codec (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | CBOR encode/decode via ciborium | — | 6 tests |
| ✅ | SBE encode/decode for trading messages | — | 79 tests, zero-alloc |
| ✅ | SBE encode: NewOrderSingle, ExecutionReport, CancelRequest | — | Hot path |
| ✅ | SBE encode: remaining message types | — | CancelReplace, MarketDataSnapshot, MarketDataIncrementalRefresh, CancelReject (generated from FSL) |
| ✅ | SBE message header (schema ID, version, template ID) | — | Generated SBE headers compliant with Spec §10 |
| ✅ | Protobuf codec | Low | Removed — CBOR + SBE only |
| ✅ | JSON codec (for REST gateway) | Medium | json_to_cbor/cbor_to_json in fig-core codec module |

---

## 7. Authentication (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | AuthToken enum (Token, Mtls, DevToken) | — | 12 tests |
| ✅ | Token verification | — | Constant-time comparison |
| ✅ | mTLS certificate CN extraction | — | |
| ✅ | Dev tokens for testing | — | |
| ✅ | JWT support | Medium | HS256 encode/decode + verify_jwt_bearer in jwt module |
| ✅ | Per-channel auth | Medium | ChannelAuthPolicy with per-channel permission requirements |
| ✅ | AUTH_REFRESH flow | — | Token refresh (client→server), verify_refresh_token, control dispatcher |
| ✅ | OAuth2 / OIDC integration | Low | `OAuthValidator` with dev token introspection registry |

---

## 8. Gateway Adapters (fig-gateways)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | FIX 4.4 parse/serialize | — | 8 tests, checksum computation |
| ✅ | FIX → FIG message conversion | — | NewOrderSingle, Cancel, ExecutionReport |
| ✅ | REST HTTP/1.1 parse/serialize | — | 7 tests |
| ✅ | WebSocket RFC 6455 frame parse/serialize | — | 14 tests, all opcodes, masking |
| ✅ | FIX session state machine | — | FixSession with states (LoggedOut/LogonSent/LoggedIn/LogoutSent), MsgSeqNum tracking, Heartbeat, ResendRequest, GapFill |
| ✅ | REST JSON ↔ CBOR body conversion | Medium | Delegates to fig_core::codec json_to_cbor/cbor_to_json |
| ✅ | WebSocket → FIG stream mapping | Medium | ws_to_fig_frame / fig_to_ws_frame with CONTENT_TYPE |
| ✅ | FIX Logon (35=A) → STREAM_OPEN + AUTH | — | logon_to_stream_open + stream_open_to_logon conversion functions |
| ✅ | FIX ResendRequest (35=2) → CONTROL(RESEND) | Medium | resend_request_to_control + control_to_resend_request |
| ✅ | REST SSE → STREAM_ITEM streaming | Low | `sse` module: parse_sse_chunk / sse_to_fig_stream_item |
| ✅ | Historical / query REST gateway mappings | High | See **§17.0b**, **§17.5** — `rest_query.rs` + `--fig-backend` proxy |
| ✅ | Full WS stream catalog (MD + user data) | High | See **§17.8** — `ws_catalog.rs` Binance/Hyperliquid → FIG `SUBSCRIBE` |
| ✅ | Gateway process (standalone binary) | Medium | fig-gateway binary: REST + FIX TCP listeners |

---

## 9. FSL Schema Language (fig-fsl)

| Status | Item | Priority | Notes |
|---|----|----|----|
| ✅ | FSL parser (tokenizer + recursive descent) | — | 12 tests, full orders.fsl |
| ✅ | AST with all node types | — | Serde round-trip |
| ✅ | Rust codegen | — | 5 tests |
| ✅ | ftlc CLI (compile, validate) | — | 10 tests |
| ✅ | Enum codegen | — | from_value/to_value with explicit discriminators |
| ✅ | Inline struct codegen | — | ENCODED_LEN constant, fixed-size struct generation |
| ✅ | Go codegen target | Medium | GoCodegen in target_codegen.rs + ftlc --lang go |
| ✅ | Python codegen target | Low | PythonCodegen + ftlc --lang python |
| ✅ | TypeScript codegen target | Low | TypeScriptCodegen + ftlc --lang typescript |
| ✅ | C++ codegen target | Medium | CppCodegen → generated.hpp |
| ✅ | C# codegen target | Medium | CsharpCodegen → Generated.cs |
| ✅ | OCaml codegen target | Low | OcamlCodegen + ftlc --lang ocaml |
| ✅ | Zig codegen target | Low | ZigCodegen + ftlc --lang zig |
| ✅ | Protobuf `.proto` codegen | Medium | ProtoCodegen + ftlc --lang proto |
| ✅ | SBE `.xml` codegen | Medium | SbeXmlCodegen + ftlc --lang sbe-xml |
| ✅ | JSON Schema `.json` codegen | Low | JsonSchemaCodegen + ftlc --lang json-schema |
| ✅ | FIX mapping `.yaml` codegen | Low | FixYamlCodegen + ftlc --lang fix-yaml |
| ✅ | FSL → SBE Rust encode/decode impls | — | Generated from FSL alongside hand-written, cross-validated, 7 messages |

---

## 10. Exchange Simulator (fig-exchange-sim)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Order book (price-time priority) | — | 8 tests |
| ✅ | Matching engine (market + limit orders) | — | 8 tests |
| ✅ | FIG server over TREE | — | Order entry, cancel, market data, account query |
| ✅ | Integration tests (end-to-end) | — | 22 tests (orders, MD, account, candles, auth, ticker, capabilities, open orders, book seq, agg trades, mark price, margin, position delta, UNSUBSCRIBE, session resume, request_stream, …) |
| ✅ | Market data streaming (push updates) | Medium | Subscription registry + build_market_data_push on trade |
| ✅ | Historical data handlers in exchange-sim | High | See **§17.4b** — `broker_api::handle_query_request` |
| ✅ | Candle / OHLCV bar streaming | High | See **§17.3** — `market_data` + `SubscriptionKind::Candles` |
| ✅ | Private account streaming (balance, margin, positions) | High | See **§17.4** — `account_state` + `handle_account_subscribe` |
| ✅ | Execution / order update subscription | High | See **§17.4** — `post_fill_updates` execution fan-out |
| ✅ | CancelReplace (order modification) | Low | `/replace` path wired to `process_replace` |
| ✅ | Order book depth streaming | Low | `push_book_depth` on every book change |
| ✅ | Multi-symbol support in server | Low | Symbol from routing key/order; no hardcoded default |
| ✅ | Session resumption in server | — | FileSessionStore wired into handle_connection, sessions persist across restarts |

---

## 11. Observability (fig-core)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Tracing spans (8 span constructors) | — | encode, decode, channel, session, connection, gateway |
| ✅ | Atomic metrics counters (10 counters) | — | frames_sent/recv, channels, sessions, errors, etc. |
| ✅ | Metrics snapshot | — | For Prometheus-style export |
| ✅ | OpenTelemetry integration | Medium | init_tracing() + tracing-subscriber env-filter; OTel-ready |
| ✅ | Prometheus metrics endpoint | Medium | fig-observability binary serves /metrics |
| ✅ | Frame-level tracing (per-frame span) | Low | `span_encode`/`span_decode` in Frame::encode/decode hot path |
| ✅ | Distributed trace context propagation | Medium | W3C traceparent via trace module + TRACE_ID extension |

---

## 12. Benchmarks (fig-bench)

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | Frame encode/decode benchmarks | — | 4 benchmarks |
| ✅ | CBOR vs SBE codec benchmarks | — | 6 benchmarks |
| ✅ | Gateway adapter benchmarks | — | 8 benchmarks |
| ✅ | Matching engine benchmarks | — | 3 benchmarks |
| ✅ | TREE transport benchmarks (round-trip latency) | Medium | cold-start + steady-state in transport_bench; CI smoke-tests steady-state |
| ✅ | Tail-latency harness (p99 / p99.9) | Medium | `fig-latency` binary + `latency_bench`; HDR Histogram per-operation samples |
| ✅ | Comparison benchmarks vs FIX/REST/WS | Medium | protocol_comparison_new_order group in gateway_bench |
| 🔶 | Throughput benchmarks (msgs/sec) | Medium | `tree_ping_pong_throughput_x16` restored in transport_bench |
| ✅ | Memory allocation benchmarks | Low | `alloc_bench.rs`; run with `--features alloc` |

If you want the full transport throughput benchmark back in CI later, we'll need a proper server-side multi-frame read path first.

---

## 13. Documentation

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | README.md | — | Full description, benchmarks, architecture |
| ✅ | SPEC.md | — | RFC-style protocol specification (553 lines) |
| ✅ | CONTRIBUTING.md | — | Development guidelines |
| ✅ | schemas/orders.fsl | — | Complete example schema |
| ✅ | API docs (rustdoc) | Medium | docs/API.md + module index; cargo doc --workspace |
| ✅ | Tutorial / getting started guide | Medium | docs/TUTORIAL.md step-by-step guide |
| ✅ | Protocol guide (deep dive) | Low | docs/PROTOCOL.md |
| ✅ | Gateway deployment guide | Low | docs/GATEWAY.md |
| ✅ | Architecture Decision Records (ADRs) | Low | docs/adr/README.md + 0004 FSL source of truth |

---

## 14. CI/CD

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | GitHub Actions CI | — | build, test, clippy, fmt --check |
| ✅ | Cross-platform CI (macOS, Windows) | Low | `cross-platform` matrix job in ci.yml |
| ✅ | Benchmark regression CI | Medium | CI smoke-runs frame/transport (ping-pong only)/gateway/alloc benches |
| ✅ | Coverage reporting | Medium | cargo llvm-cov job in CI workflow |
| ✅ | Release workflow | Low | `.github/workflows/release.yml` — GitHub Release on every push to `main` (auto patch bump) |
| ✅ | Docker image | Low | Dockerfile for fig-exchange-sim |

---

## 15. Security

| Status | Item | Priority | Notes |
|---|---|---|---|
| ✅ | TLS 1.3 via TREE | — | Mandatory, integrated |
| ✅ | Self-signed cert generation | — | For development |
| ✅ | mTLS | Medium | server_config_mtls + FIG_MTLS=1 in exchange-sim |
| ✅ | Certificate rotation | Medium | RotatingServerCerts with reload + rebuild config |
| ✅ | Rate limiting | Medium | ChannelRateLimiter token-bucket per channel |
| ✅ | DoS protection | Low | `DoSGuard` + `FloodDetector` in dos module |
| ✅ | Security audit | Low | docs/SECURITY_AUDIT.md pre-1.0 checklist |

---

