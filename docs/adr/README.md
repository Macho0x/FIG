# Architecture Decision Records (ADR)

Index of FIG architecture decisions. Each ADR captures **context, decision,
and consequences** for choices with long-term impact.

| ADR | Title | Status |
|---|---|---|
| 0001 | [TREE as Primary Transport](#adr-0001-tree-as-primary-transport) | Accepted |
| 0002 | [Dual Codec Strategy (CBOR + SBE)](#adr-0002-dual-codec-strategy-cbor--sbe) | Accepted |
| 0003 | [Channel-Based Multiplexing](#adr-0003-channel-based-multiplexing) | Accepted |
| 0004 | [FSL as Single Source of Truth](0004-fsl-single-source-of-truth.md) | Accepted |
| 0005 | [Multi-Language Runtime Strategy](0005-multi-language-runtime-strategy.md) | Accepted |
| 0006 | [Broker ↔ Client API Parity](0006-broker-api-parity.md) | Accepted |
| 0007 | [Crypto Instrument Model](0007-crypto-instrument-model.md) | Accepted |

---

# ADR 0001: TREE as Primary Transport

## Status

Accepted

## Context

FIG needs multiplexed, encrypted, low-latency transport with connection migration
and 0-RTT resumption. Candidates: raw TCP+TLS, HTTP/3, custom transport wrapper.

## Decision

Use TREE via the `quinn` crate (RFC 9000 foundation) with ALPN `fig/1`.
Map FIG channels to TREE streams using `channel_id × 4 + direction_offset`.

## Consequences

- Built-in TLS 1.3, migration, and stream multiplexing.
- Plain TCP downgrade available for constrained environments (`FIG\x01` magic).
- Requires UDP reachability; TCP-only networks use downgrade mode.

---

# ADR 0002: Dual Codec Strategy (CBOR + SBE)

## Status

Accepted

## Context

Trading systems need both schema evolution (REST/gateway) and zero-allocation
hot paths (order entry).

## Decision

CBOR for self-describing gateway payloads; SBE generated from FSL for production
trading messages. Protobuf is optional **codegen** (`ftlc --lang proto`), not an
on-wire FIG codec.

## Consequences

- Two encode/decode paths to maintain, cross-validated in tests.
- Gateways translate JSON↔CBOR; clients choose SBE or CBOR per channel schema ID.

---

# ADR 0003: Channel-Based Multiplexing

## Status

Accepted

## Context

FIX uses separate sessions per asset class; REST uses HTTP/1.1 keep-alive;
WebSocket uses single stream. FIG must unify these.

## Decision

Logical channels with independent sequence numbers, credit-based flow control,
and optional unidirectional direction. Control channel ID 0 for heartbeat/settings.

## Consequences

- Channel manager complexity but clean separation of concerns.
- Gateway adapters map FIX session / REST request / WS stream to channel semantics.

---

See also: [ADR 0004 — FSL as Single Source of Truth](0004-fsl-single-source-of-truth.md)
for schema evolution and multi-language codegen policy.
