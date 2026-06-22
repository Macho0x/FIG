# FIG Benchmarks

FIG has two benchmark layers:

1. **Criterion microbenches** — median/mean regression on isolated hot paths.
2. **Tail-latency harness** — HDR Histogram percentiles (p50 / p99 / p99.9) per operation.

All benchmarks run on the release profile. Numbers below are from the reference
development machine — yours will vary. For institutional tail-latency claims, run
the harness on dedicated bare metal with pinned CPUs and document your environment.

## Criterion microbenches

```bash
cargo bench -p fig-bench
cargo bench -p fig-bench --bench codec_bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
cargo bench -p fig-bench --bench transport_bench
```

Criterion reports **mean time per iteration** with confidence intervals — not
per-operation p99/p99.9. Use the tail-latency harness below for percentile SLAs.

## Tail-latency harness (HDR Histogram)

Records **one wall-clock sample per operation** and prints p50, p90, p99,
p99.9, and p99.99:

```bash
cargo run --release -p fig-bench --bin fig-latency
# or
cargo bench -p fig-bench --bench latency_bench
```

Reduce iterations for CI smoke runs:

```bash
FIG_LATENCY_ITERS=500 cargo run --release -p fig-bench --bin fig-latency
```

| Scenario | Default samples | What it measures |
|---|---|---|
| `sbe_decode_hot_path` | 100,000 | SBE `NewOrderSingle` decode — codec tail latency |
| `tree_round_trip_steady_state` | 10,000 | Persistent QUIC conn; new bidi stream per ping/pong |
| `tree_round_trip_under_load` | 5,000 | Measured stream + background traffic on separate channels |
| `matching_engine_contended` | 8,000 | 8 threads contending on one `MatchingEngine` |
| `matching_engine_cancel_hot_path` | 10,000 | Cancel latency per order |

**Note:** localhost TREE numbers include kernel + QUIC stack jitter. They are
useful for regression and relative comparison, not absolute HFT wire latency.

---

## Frame encode/decode

| Benchmark | Time | Description |
|---|---|---|
| `frame_encode` | **574 ns** | Encode a Request frame with extensions + payload |
| `frame_decode` | **649 ns** | Decode a Request frame from bytes |
| `frame_decoder_streaming` | **670 ns** | FrameDecoder with chunked input |
| `frame_encode_large_payload` | **529 ns** | Encode frame with 10KB payload |

## Codec: CBOR vs SBE

| Benchmark | Time | Description |
|---|---|---|
| `cbor_encode_order` | **927 ns** | CBOR encode NewOrderSingle |
| `cbor_decode_order` | **2.38 μs** | CBOR decode NewOrderSingle |
| `sbe_encode_order` | **361 ns** | SBE encode NewOrderSingle |
| `sbe_decode_order` | **205 ns** | SBE decode NewOrderSingle |
| `cbor_encode_execution_report` | **1.16 μs** | CBOR encode ExecutionReport |
| `sbe_encode_execution_report` | **437 ns** | SBE encode ExecutionReport |

**SBE is 2.6× faster to encode and 11.6× faster to decode than CBOR.**
SBE zero-copy decode (~205 ns) is 25–250× faster than JSON parsing (10–50 μs)
and 25–100× faster than FIX ASCII parsing (5–20 μs).

## Gateway adapters

| Benchmark | Time | Description |
|---|---|---|
| `fix_parse` | **2.37 μs** | Parse FIX NewOrderSingle (tag=value) |
| `fix_serialize` | **2.67 μs** | Serialize FIX ExecutionReport |
| `fix_to_fig_order` | **695 ns** | Convert FIX → FIG NewOrderSingle |
| `rest_parse_request` | **1.19 μs** | Parse HTTP/1.1 request |
| `rest_serialize_response` | **1.21 μs** | Serialize HTTP/1.1 response |
| `ws_parse_text_frame` | **52 ns** | Parse WebSocket text frame |
| `ws_serialize_text_frame` | **182 ns** | Serialize WebSocket text frame |

## Matching engine (reference simulator)

| Benchmark | Time | Description |
|---|---|---|
| `order_book_add` | **530 μs** | Add 1000 orders to the order book |
| `matching_engine_process_order` | **805 μs** | Process market order against full book |
| `matching_engine_cancel` | **1.21 μs** | Cancel an order |

## TREE transport

| Benchmark | Description |
|---|---|
| `tree_ping_pong_cold_start` | New server + connect + one ping/pong (handshake included) |
| `tree_ping_pong_steady_state` | Persistent QUIC conn; new bidi stream per ping/pong |

## Protocol comparison

| Metric | FIG (native SBE) | FIX ASCII | HTTP/1.1+JSON | WebSocket |
|---|---|---|---|---|
| Min header overhead | 16 bytes | 200–500 bytes | 200–800 bytes | 2–10 bytes |
| Decode speed | ~205 ns | ~5–20 μs | ~10–50 μs | N/A |
| Handshake RTTs | 1 (TREE) / 0 (resumed) | 4 (TCP+TLS+Logon) | 3–5 (DNS+TCP+TLS+HTTP) | 2–3 (upgrade+TLS) |
| Multiplexing | 65,535 channels/conn | 1 session/conn | 6 (browser) / HTTP/2 | 1/conn |
| Session resumption | 0-RTT | Full reconnect | N/A (stateless) | Full reconnect |

See also [fig-bench](../crates/fig-bench/) for benchmark source and CI smoke runs.
