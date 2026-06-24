# SBE order entry path (colo / market makers)

Use **SBE** (`application/fig+sbe`) for hot order entry; use **CBOR** for
development, scripting, and gateway JSON migration.

## When to use SBE

| Use SBE | Use CBOR |
|---------|----------|
| Colocated market makers | `fig-cli` demos, Python/FFI |
| Fixed-layout latency-sensitive entry | REST gateway JSON bodies |
| Template IDs already in `fig-core::sbe` | Exploratory integration |

## Wire checklist

1. **Transport:** TREE (QUIC) to venue FIG backend — not the REST gateway port.
2. **Frame:** `REQUEST` with `Method: POST`, `ChannelPath: trading/accounts/{account}/orders`.
3. **Content-Type extension:** `application/fig+sbe`.
4. **Schema ID:** `0x01` (`schema_id::TRADING_ORDERS`).
5. **Auth:** `AUTH_TOKEN` extension (venue-issued JWT or dev token `fig-dev-{account}`).
6. **Payload:** SBE-encoded `NewOrderSingle` (includes optional `post_only`, `reduce_only`).

## Template IDs

See generated codecs in [`crates/fig-core/src/generated/sbe_generated.rs`](../crates/fig-core/src/generated/sbe_generated.rs):

- `NewOrderSingle` — order entry
- `ExecutionReport` — fill / ack responses

Encode/decode via [`crates/fig-core/src/sbe.rs`](../crates/fig-core/src/sbe.rs):

```rust
use fig_core::messages::NewOrderSingle;
use fig_core::sbe;

let bytes = sbe::encode_new_order_single(&order);
let decoded = sbe::decode_new_order_single(&bytes)?;
```

## Runnable example

[`crates/fig-cli/src/lib.rs`](../crates/fig-cli/src/lib.rs) — `run_sbe_order_demo`.

Verified by `cargo test -p fig-cli` (`cli_sbe_order_demo_successfully`).

## Gateway note

The REST/WS gateway translates **JSON → CBOR** by default. SBE is for **native FIG**
clients connecting directly to the venue backend.
