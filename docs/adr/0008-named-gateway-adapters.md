# ADR 0008: Named gateway adapters

## Status

Accepted

## Context

FIG's native wire is TREE + FSL (`ADR 0006`). The gateway already translates
FIX 4.4, REST, and WebSocket JSON onto the same `CHANNEL_PATH` values. Venues
also need **binary order-entry and market-data edges** (OUCH, ITCH, and later
dialects) so existing colo clients survive a cutover.

Without a rule, those edges become a second protocol.

## Decision

1. **Native FIG is canonical.** Every capability is a FSL type + `CHANNEL_PATH`
   first; adapters never invent semantics.
2. **A named adapter is one dialect plus its session.**
   - OUCH implies SoupBinTCP.
   - ITCH implies MoldUDP64 and/or SoupBinTCP snapshot/replay.
   - FIX remains FIX 4.4 + the existing session state machine.
3. **New venue dialects are alias tables**, not protocol forks. Do not add
   `fig-nasdaq` crates or exchange-named FSL types.
4. **Listeners are opt-in.** `fig-gateway` keeps FIX `:9876`, REST `:8080`,
   WS `:8090` by default. `--ouch-addr` and `--itch-addr` are off until set.
5. **Auth is session-level.** SoupBin/FIX/OUCH login injects `AUTH_TOKEN`.
   Order bodies are not signed. Per-order wallet signatures are venue
   issuance, not FIG protocol auth (SPEC §9.3).
6. **One binary pair proves the pattern:** NASDAQ OUCH 4.2-shaped + ITCH L2
   egress. iLink, ETI, BOE, PITCH, MDP 3.0 are out of this ADR.

## Consequences

- `fig-gateways` grows `soupbin`, `ouch`, and `itch` modules next to `fix`.
- FSL `gateway fix` / `gateway rest` stay the documented mapping blocks;
  OUCH/ITCH field layouts live in those modules and in GATEWAY.md.
- Production-test clients keep using Rust / `fig-ffi` + SBE; adapters are
  how incumbent OMS/MM stacks talk to a FIG venue.

## References

- [ADR 0006](0006-broker-api-parity.md)
- [ADR 0009](0009-market-data-plane.md)
- [docs/GATEWAY.md](../GATEWAY.md)
- [SPEC.md §9.3](../../SPEC.md)
