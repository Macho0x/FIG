# FIG Benchmarks

FIG has two benchmark layers:

1. **Criterion microbenches** — mean time per iteration (100 samples by default).
2. **Tail-latency harness** — HDR Histogram percentiles (p50 / p99 / p99.9) per operation.

All benchmarks run on the **release** profile. Re-run locally before citing numbers in
production SLAs — results vary by CPU, governor, and background load.

### Reference environment (last measured)

| | |
|---|---|
| Date | 2026-06-24 |
| CPU | Intel Core i7-1165G7 @ 2.80 GHz (4C/8T) |
| OS | Linux x86_64 |
| Rust | 1.96.0 (release) |
| Criterion | 100 samples, 1 s warm-up, 3 s measurement (`criterion_config`) |
| Tail harness | Default iteration counts (see table below) |

```bash
cargo bench -p fig-bench                              # all Criterion benches
cargo bench -p fig-bench --bench codec_bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
cargo bench -p fig-bench --bench transport_bench
cargo run --release -p fig-bench --bin fig-latency    # tail percentiles
```

CI smoke runs use `--sample-size 10` and `FIG_LATENCY_ITERS=200` — regression guard
only, not for publishing latency claims.

Override Criterion sample size: `FIG_BENCH_SAMPLE_SIZE=200 cargo bench -p fig-bench --bench codec_bench`

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
| `tree_round_trip_steady_state` | 10,000 | Persistent QUIC conn; new bidi stream per ping/pong (reconnects every 128 streams) |
| `tree_round_trip_under_load` | 5,000 | Measured stream + background traffic on separate channels |
| `matching_insert_top_n{N}` | 50,000 | Non-crossing limit insert at top of bid (N = 0 / 100 / 1000 ask levels) |
| `matching_cancel_top_n1000` | 50,000 | Cancel best bid on a 1000-level book |
| `matching_cancel_deep_n1000` | 10,000 | Cancel deepest bid (linear book scan tail) |
| `matching_replace_qty_n1000` | 50,000 | Quantity-only replace on top-of-book order |
| `matching_engine_contended` | 8,000 | 8 threads contending on one `MatchingEngine` |

**Note:** localhost TREE numbers include kernel + QUIC stack jitter. They are
useful for regression and relative comparison, not absolute HFT wire latency.

### Tail-latency results (reference machine, 2026-06-24)

| Scenario | p50 | p99 | p99.9 |
|---|---|---|---|
| `sbe_decode_hot_path` | 463 ns | 609 ns | 3.7 μs |
| `tree_round_trip_steady_state` | 149 μs | 1.45 ms | 4.0 ms |
| `tree_round_trip_under_load` | 165 μs | 2.04 ms | 4.07 ms |
| `matching_insert_top_n0` | 1.71 μs | 3.91 μs | 19.9 μs |
| `matching_insert_top_n1000` | 1.09 μs | 3.21 μs | 9.8 μs |
| `matching_cancel_top_n1000` | 1.05 μs | 3.17 μs | 10.1 μs |
| `matching_cancel_deep_n1000` | 12.6 μs | 36.5 μs | 47.5 μs |
| `matching_replace_qty_n1000` | 2.65 μs | 6.99 μs | 15.5 μs |
| `matching_engine_contended` | 1.92 μs | 191 μs | 860 μs |

---

## Frame encode/decode

| Benchmark | Time | Description |
|---|---|---|
| `frame_encode` | **822 ns** | Encode a Request frame with extensions + payload |
| `frame_decode` | **632 ns** | Decode a Request frame from bytes |
| `frame_decoder_streaming` | **719 ns** | FrameDecoder with chunked input |
| `frame_encode_large_payload` | **505 ns** | Encode frame with 10KB payload |

## Codec: CBOR vs SBE

| Benchmark | Time | Description |
|---|---|---|
| `cbor_encode_order` | **790 ns** | CBOR encode NewOrderSingle |
| `cbor_decode_order` | **2.12 μs** | CBOR decode NewOrderSingle |
| `sbe_encode_order` | **559 ns** | SBE encode NewOrderSingle |
| `sbe_decode_order` | **312 ns** | SBE decode NewOrderSingle |
| `cbor_encode_execution_report` | **1.14 μs** | CBOR encode ExecutionReport |
| `sbe_encode_execution_report` | **539 ns** | SBE encode ExecutionReport |

On this machine, SBE is **~1.4× faster to encode** and **~6.8× faster to decode**
than CBOR for `NewOrderSingle`. SBE decode (~312 ns Criterion mean; ~463 ns tail
p50) is **~15× faster** than in-repo FIX parse (~4.8 μs) for the same message shape.

Industry ballparks for JSON (10–50 μs) and generic FIX stacks (5–20 μs) are not
measured here — use them only as rough context, not FIG regression baselines.

## Gateway adapters

| Benchmark | Time | Description |
|---|---|---|
| `fix_parse` | **4.84 μs** | Parse FIX NewOrderSingle (tag=value) |
| `fix_serialize` | **4.76 μs** | Serialize FIX ExecutionReport |
| `fix_to_fig_order` | **924 ns** | Convert FIX → FIG NewOrderSingle |
| `fig_to_fix_execution_report` | **10.8 μs** | FIG ExecutionReport → FIX bytes |
| `rest_parse_request` | **1.83 μs** | Parse HTTP/1.1 request |
| `rest_serialize_response` | **2.03 μs** | Serialize HTTP/1.1 response |
| `ws_parse_text_frame` | **65 ns** | Parse WebSocket text frame |
| `ws_serialize_text_frame` | **260 ns** | Serialize WebSocket text frame |

## Matching engine (reference simulator)

Criterion times below are **per benchmark iteration**. Throughput benches divide
wall time by the element count shown.

| Benchmark | Time | Description |
|---|---|---|
| `order_book_add_1000` | **991 μs** | Add 1000 orders (~991 ns/order) |
| `order_book_add_single` | **313 ns** | Add one order to an empty book |
| `matching_engine_process_order` | **1.25 ms** | 500 resting sells + one market buy |
| `matching_engine_cancel` | **1.47 μs** | Insert + cancel on a 1-order book |

## TREE transport

| Benchmark | Time | Description |
|---|---|---|
| `tree_ping_pong_cold_start` | **29.4 ms** | New server + connect + one ping/pong (handshake included) |
| `tree_ping_pong_steady_state` | **779 μs** | Persistent QUIC conn; new bidi stream per ping/pong |
| `tree_ping_pong_throughput_x16` | **~12 ms** | 16 ping/pong rounds per iteration (~750 μs/round) |

## Allocation patterns (`--features alloc`)

| Benchmark | Time | Description |
|---|---|---|
| `encode_allocating_4kb` | **502 ns** | `Frame::encode()` with 4 KB payload (allocates) |
| `decode_allocating_4kb` | **388 ns** | `Frame::decode()` from pre-encoded 4 KB frame |
| `decoder_buffer_reuse_1kb` | **599 ns** | `FrameDecoder` reused across decodes |
| `encode_into_reused_vec_4kb` | **718 ns** | `encode()` then copy into pre-sized `Vec` (encode still allocates) |

## Protocol comparison

| Metric | FIG (native SBE) | FIX ASCII (in-repo) | HTTP/1.1+JSON (in-repo) | WebSocket |
|---|---|---|---|---|
| Min header overhead | 16 bytes | 200–500 bytes | 200–800 bytes | 2–10 bytes |
| NewOrderSingle decode | ~312 ns (SBE) | ~4.8 μs (`fix_parse`) | ~1.8 μs (`rest_parse` headers only) | N/A |
| Handshake RTTs | 1 (TREE) / 0 (resumed) | 4 (TCP+TLS+Logon) | 3–5 (DNS+TCP+TLS+HTTP) | 2–3 (upgrade+TLS) |
| Multiplexing | 65,535 channels/conn | 1 session/conn | 6 (browser) / HTTP/2 | 1/conn |
| Session resumption | 0-RTT | Full reconnect | N/A (stateless) | Full reconnect |

See also [fig-bench](../crates/fig-bench/) for benchmark source and CI smoke runs.
