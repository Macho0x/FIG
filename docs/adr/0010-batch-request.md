# ADR 0010: Batch / atomic multi-action REQUEST

## Status

Accepted

## Context

FIG `REQUEST` is one payload, one `RESPONSE` or `STREAM_ERROR` (SPEC §4).
Crypto-perp venues commonly submit several trading actions as a **single
interaction** (place hedge + cancel the previous quote, or a small basket of
limits) so the book never shows a partial update.

`OrderListStatus` is a different product: linked/OCO lifecycle on
`trading/accounts/{account}/orderlists`. Mass cancel (`35=q/r`) is a bulk
cancel of resting orders, not an ordered list of mixed actions.

A new frame type would be a SPEC `1.0.0` wire break. Product FSL (TWAP,
vaults, builder codes, chain events) is venue surface and is out of this ADR
([AGENTS.md](../../AGENTS.md), [TODO.md §20.1](../../TODO.md)).

The active P3 track is **crypto-perps**. Institutional FIX dialect work is
parked.

## Decision

1. **Same `REQUEST` frame.** Batch is a catalog message + `CHANNEL_PATH`, not a
   new frame type. SPEC §4 is unchanged: one `REQUEST` still yields exactly one
   `RESPONSE` or `STREAM_ERROR` on that channel.

2. **Path:** `POST trading/accounts/{account}/batch`.
   Payload `BatchRequest`; response `BatchReport`. Existing single-action
   paths (`…/orders`, `…/cancel`, `…/replace`, `…/masscancel`) stay 1:1.

3. **Allowed actions (ordered):** `NewOrderSingle`, `CancelRequest`,
   `CancelReplaceRequest`. Queries, `SUBSCRIBE`, mass cancel, and mixed
   accounts are not batch items.

4. **Atomicity is all-or-nothing.** Actions apply in list order; later items
   observe earlier ones in the same batch. If any item would reject, the venue
   MUST restore pre-batch state. `BatchReport.outcome = rejected`. The failing
   item carries the concrete reason; every other item is `rejected` with
   `BATCH_ROLLED_BACK`. There is no silent partial apply.

5. **`STREAM_ERROR` is envelope-only:** decode failure, empty list, more than
   the advertised max, auth, account mismatch, or a `kind` that does not match
   its payload. Per-action trading rejects stay inside `BatchReport`.

6. **Correlation:** `batch_id` on the envelope (same `ClientOrderId` grammar).
   Each action keeps its own `cl_ord_id`. Applied actions still fan out
   `ExecutionReport` on `trading/accounts/{account}/executions`.

7. **Reference cap:** 16 actions (`fig_exchange_sim::matching::MAX_BATCH_ACTIONS`).
   Venues MAY advertise a different max; they MUST reject over-cap batches with
   `STREAM_ERROR` / `BATCH_TOO_LARGE`.

8. **Gateways.** REST `POST /trading/accounts/{account}/batch` maps to the
   native path. Hyperliquid-style action arrays are **aliases** onto
   `BatchRequest` — no HL types in FSL. FIX, OUCH, and ITCH stay one message
   per action. Adapters MUST NOT fold N FIX/OUCH messages into one batch
   (that would invent atomicity the dialect does not have) and MUST NOT split
   a batch into N native `REQUEST`s (that would drop atomicity).

9. **Not this ADR:** best-effort / partial-commit modes, TWAP, vaults, builder
   codes, chain/L1 events, per-order wallet signatures.

## Consequences

- Additive FSL under SPEC `1.0.0` (`BatchRequest`, `BatchReport`, `BatchAction`,
  `BatchResult`).
- `fig-exchange-sim` snapshots the matcher, applies, and restores on reject.
- Capabilities advertise `trading/accounts/{account}/batch` as
  `request_response` (auth required).
- Conformance SBE/CBOR vectors pin the envelope; CLI/SDK helpers post a batch
  as one `REQUEST`.

## References

- [ADR 0004](0004-fsl-single-source-of-truth.md) — additive FSL
- [ADR 0006](0006-broker-api-parity.md) — native path first
- [ADR 0007](0007-crypto-instrument-model.md) — crypto track, product FSL deferred
- [ADR 0008](0008-named-gateway-adapters.md) — adapters do not invent semantics
- [SPEC.md §4 / §7.3 / §9](../../SPEC.md)
