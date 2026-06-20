# FIG Conformance Test Vectors

Language-neutral golden fixtures every FIG SDK must pass. The Rust reference
runner lives in `crates/fig-conformance`; regenerate vectors after intentional
wire-format changes.

## Vector file format (v1)

Files live in `tests/conformance/vectors/*.json`.

```json
{
  "version": 1,
  "suite": "fig-conformance-v1",
  "vectors": [
    {
      "id": "frame.request.minimal",
      "category": "frame",
      "description": "Minimal REQUEST on channel 1, stream seq 42",
      "frame": {
        "frame_type": "Request",
        "channel_id": 1,
        "stream_seq": 42,
        "schema_id": 1,
        "extensions": []
      },
      "expected_hex": "..."
    },
    {
      "id": "cbor.new_order_single.limit_buy",
      "category": "cbor",
      "message_type": "NewOrderSingle",
      "payload": {
        "cl_ord_id": "CONF-001",
        "side": "Buy",
        "order_qty": 100.0,
        "price": 50.25,
        "symbol": "AAPL",
        "order_type": "Limit",
        "time_in_force": "Day"
      },
      "expected_hex": "..."
    }
  ]
}
```

## Categories

| Category | Validates |
|---|---|
| `frame` | 16-byte header + TLV extensions (`fig_core::frame`) |
| `cbor` | CBOR payload encoding (`fig_core::codec` + `messages`) |
| `sbe` | SBE wire layout (`fig_core::sbe`) |
| `channel` | TREE stream ID mapping (`channel_id * 4 + offset`) |

## Running

```bash
# Reference runner (Rust)
cargo test -p fig-conformance
cargo run -p fig-conformance --bin fig-conformance

# Regenerate golden hex after schema changes
cargo run -p fig-conformance --bin fig-conformance-gen -- \
  --write tests/conformance/vectors/v1.json
```

## CI

The `conformance` job in `.github/workflows/ci.yml` runs the reference runner and
fails if committed vectors drift from `fig-core` output.
