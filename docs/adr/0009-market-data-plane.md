# ADR 0009: Market-data plane

## Status

Accepted

## Context

Native FIG market data is **aggregated L2** on TREE:
`OrderBookSnapshot` / `OrderBookDelta` as `list<PriceLevel>`, plus BBO,
trades, and candles. Institutional and US-cash venues also speak
**order-by-order multicast** (NASDAQ TotalView-ITCH, CME MDP 3.0, Eurex
EOBI). TREE is reliable and point-to-point; those plants are UDP fan-out
with a separate rewind session.

Mapping TotalView Add/Execute/Cancel/Delete onto today's `PriceLevel`
would drop order identity. Building a second native transport would
fork the protocol.

## Decision

1. **FIG native MD stays TREE L2.** The catalog path
   `marketdata/{symbol}/book` does not become MBO.
2. **ITCH in this repo is L2 egress**, not TotalView:
   - Snapshot + incremental **price-level** messages (`L`) derived from
     `OrderBookSnapshot` / `OrderBookDelta`.
   - Optional trade (`T`) and system-event (`S`) messages.
   - **No synthetic order references.** Do not invent Add/Delete per
     order from aggregated quantity.
3. **Session package:** SoupBinTCP on `--itch-addr` (sequenced replay,
   used by CI). MoldUDP64 on `--itch-multicast` (datagram fan-out).
   Both carry the same payload bytes.
4. **ITCH ingress and native L3/MBO are out of scope** until a later ADR
   adds `BookOrder` types and a sim MBO book. Until then, FIG does not
   claim TotalView, MDP, or EOBI replacement.
5. **MDP 3.0 / FAST** stay unbuilt. This ADR is the documented MD-plane
   answer for the §21 multicast row: hybrid — FIG for orders/accounts/L2
   on TREE; incumbent multicast plants remain the HFT MD plane.

## Consequences

- Gateway ITCH output can drive an L2 book reconstruct in tests; it
  cannot drive an order-by-order FPGA handler.
- Capabilities advertise the existing L2 path only.
- A future L3 path would be a new `CHANNEL_PATH`
  (`marketdata/{symbol}/book/orders`), not a silent change to L2.

## References

- [ADR 0008](0008-named-gateway-adapters.md)
- [TODO.md §21](../../TODO.md)
- [SPEC.md §9.1](../../SPEC.md)
