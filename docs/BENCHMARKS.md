# FIG Benchmarks

Three layers. Use the right one for the claim you are making.

| Layer | Command | What it answers |
|---|---|---|
| **Throughput (msgs/sec)** | `cargo run --release -p fig-bench --bin fig-throughput` | How many operations finish in a wall-clock second |
| **Tail latency** | `cargo run --release -p fig-bench --bin fig-latency` | p50 / p99 / p99.9 of **one** operation |
| **Criterion microbench** | `cargo bench -p fig-bench` | Mean time per iteration (and elems/s when `Throughput::Elements` is set) |

`fig-load` is a **flood smoke** (`FIG_LOAD_SECS` encode/decode loops). CI uses it as a DoS-path guard. Do not cite it as msgs/sec.

All of these run on the **release** profile. Re-run on the target host before citing numbers in an SLA. Results move with CPU, governor, background load, and kernel version.

---

## How to run

```bash
# Published msgs/sec (default 3 s measure, 200 ms warmup)
cargo run --release -p fig-bench --bin fig-throughput

# CI / laptop smoke
FIG_THROUGHPUT_SECS=1 FIG_THROUGHPUT_WARMUP_MS=50 \
  cargo run --release -p fig-bench --bin fig-throughput

# Tail percentiles
cargo run --release -p fig-bench --bin fig-latency
FIG_LATENCY_ITERS=200 cargo run --release -p fig-bench --bin fig-latency

# Criterion (mean time + elems/s)
cargo bench -p fig-bench
cargo bench -p fig-bench --bench codec_bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
cargo bench -p fig-bench --bench transport_bench -- tree_ping_pong_pipeline_persistent
```

| Variable | Default | Used by |
|---|---|---|
| `FIG_THROUGHPUT_SECS` | `3` | `fig-throughput` measurement window |
| `FIG_THROUGHPUT_WARMUP_MS` | `200` | `fig-throughput` warmup |
| `FIG_LATENCY_ITERS` | per-scenario | `fig-latency` sample count |
| `FIG_BENCH_SAMPLE_SIZE` | `100` (slow benches `30`) | Criterion |
| `FIG_LOAD_SECS` | `5` (CI `3`) | `fig-load` flood smoke |

CI smokes Criterion with `--sample-size 10`, `FIG_LATENCY_ITERS=200`, `FIG_LOAD_SECS=3`, and `FIG_THROUGHPUT_SECS=1`. Those runs are regression guards, not numbers to publish.

---

## Methodology (throughput)

`fig-throughput` runs each scenario for a **fixed duration** after a warmup, then prints `ops / wall_seconds`.

- **CPU / codec / matching:** inner batches of 1024 ops between `Instant::now` checks so the clock is not on the 300 ns path. `std::hint::black_box` keeps LLVM from deleting the work.
- **Matching `matching_insert_cancel`:** one non-crossing limit insert plus one cancel on a 1000-level ask book. The book depth stays constant. One op = one insert/cancel pair.
- **TREE `tree_round_trip_persistent`:** ping then pong on **one** channel. The server keeps reading that channel (`recv_frame` after the first `accept_frame`). Rate ≈ 1 / localhost RTT.
- **TREE `tree_pipeline_persistent_x32`:** send 32 pings, then receive 32 pongs, on that same channel. This is the sustained-stream msgs/sec number, not 1/RTT.
- **TREE `tree_round_trip_new_stream`:** a new bidirectional QUIC stream per ping (the older Criterion `tree_ping_pong_throughput_x16` shape). Measures stream-open cost as well as RTT.

TREE benches call `FigConnection::with_limits` to lift `DoSGuard` (default 100_000 msgs/sec/connection) and `ChannelRateLimiter` (default **10_000 msgs/sec/channel**). Production connections keep those caps. A rate that exceeds 10_000 /s on one channel is a ceiling, not what a default venue process will admit.

Localhost TREE includes kernel + QUIC stack jitter. Useful for regression and relative comparison, not absolute HFT wire latency. Co-lo evidence on realistic topology remains a separate P3 row.

**One op** is one encode, one decode, one insert/cancel pair, or one ping/pong (pipeline counts each ping).

Criterion mean time converts to an *implied* msgs/sec as `1e9 / mean_ns`. Prefer `fig-throughput` for that claim: it measures a full second of steady looping instead of a small Criterion sample.

---

## Throughput results (msgs/sec)

### Reference environment

| | |
|---|---|
| Date | 2026-10-12 |
| CPU | 11th Gen Intel Core i7-1165G7 @ 2.80 GHz (4C/8T, turbo 4.7 GHz) |
| OS | Linux 7.2.5 x86_64 |
| Rust | 1.99.0 (release) |
| Measure | 3.0 s, 200 ms warmup (`fig-throughput` defaults) |

Same laptop as the 2026-06-24 Criterion / tail tables below. Do not mix a
throughput rate from this table with a Criterion mean from a different day
and call it a speedup.

### CPU / codec / matching

| Scenario | ops | msgs/sec | Implied ns/op | One op |
|---|---:|---:|---:|---|
| `frame_encode` | 9,726,976 | **3,242,158** | 308 | Encode Request (header + TLV + payload) |
| `frame_decode` | 9,886,720 | **3,295,566** | 303 | `Frame::decode` of that Request |
| `sbe_encode_new_order` | 8,363,008 | **2,787,418** | 359 | SBE `NewOrderSingle` encode |
| `sbe_decode_new_order` | 8,817,664 | **2,939,100** | 340 | SBE `NewOrderSingle` decode |
| `cbor_encode_new_order` | 4,635,648 | **1,545,006** | 647 | CBOR `NewOrderSingle` encode |
| `cbor_decode_new_order` | 1,395,712 | **465,188** | 2,150 | CBOR `NewOrderSingle` decode |
| `fix_parse_new_order` | 1,419,264 | **473,049** | 2,114 | In-repo FIX 4.4 NewOrderSingle parse |
| `matching_insert_cancel` | 2,504,704 | **834,393** | 1,199 | Non-crossing insert + cancel, 1000-level book |

Relative rates on this run (same harness, same second):

- SBE encode is **1.8×** CBOR encode; SBE decode is **6.3×** CBOR decode.
- SBE decode is **6.2×** the in-repo FIX parser (not a generic FIX stack).

CPU ops are multiples of 1,024 (inner batch). The last batch can overshoot the
window by up to 1,023 ops; the rate uses actual wall time.

### TREE (localhost loopback)

DoS / channel caps **lifted**. Production default is 10,000 msgs/sec/channel.

| Scenario | ops | msgs/sec | Implied μs/op | What it is |
|---|---:|---:|---:|---|
| `tree_round_trip_persistent` | 22,366 | **7,455** | 134 | Ping/pong on one reused channel (≈ 1/RTT) |
| `tree_pipeline_persistent_x32` | 172,768 | **57,580** | 17 | 32 in-flight pings on that channel |
| `tree_round_trip_new_stream` | 5,000 | **1,667** | 600 | New bidi QUIC stream per ping |

Cite `tree_pipeline_persistent_x32` for sustained-stream msgs/sec. Cite
`tree_round_trip_persistent` for round-trip bound. Cite
`tree_round_trip_new_stream` only when the claim is stream-open cost.

`57,580 /s` is above the production 10,000/channel cap — it is a ceiling with
limiters off, not a default-process rate. Localhost is not a colo SLA.

---

## Tail-latency harness (HDR Histogram)

Records **one wall-clock sample per operation** and prints p50, p90, p99,
p99.9, and p99.99:

```bash
cargo run --release -p fig-bench --bin fig-latency
# or
cargo bench -p fig-bench --bench latency_bench
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

Criterion times below (2026-06-24) are **per iteration**. Groups now also
report elems/s.

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

Criterion mean times on this laptop (2026-06-24) implied SBE **~1.4×** CBOR
encode and **~6.8×** CBOR decode. The 2026-10-12 `fig-throughput` run (same
chip, same process) is the number to cite for msgs/sec: SBE encode **1.8×**
CBOR, SBE decode **6.3×** CBOR, SBE decode **6.2×** in-repo FIX parse. Tail
p50 for SBE decode is 463 ns (`fig-latency`); that is a different question
than sustained msgs/sec.

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

The `fig-throughput` `matching_insert_cancel` row is the msgs/sec form of a
steady insert/cancel on a pre-warmed book (see table above).

## TREE transport

| Benchmark | Time | Description |
|---|---|---|
| `tree_ping_pong_cold_start` | **29.4 ms** | New server + connect + one ping/pong (handshake included) |
| `tree_ping_pong_steady_state` | **779 μs** | Persistent QUIC conn; new bidi stream per ping/pong |
| `tree_ping_pong_throughput_x16` | **~12 ms** | 16 new-stream ping/pong rounds per iteration (~750 μs/round) |
| `tree_ping_pong_pipeline_persistent_x32` | Criterion + **57,580 /s** (`fig-throughput`) | 32 pipelined ping/pongs on one persistent channel |

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
| NewOrderSingle decode (Criterion mean) | ~312 ns | ~4.8 μs (`fix_parse`) | ~1.8 μs (`rest_parse` headers only) | N/A |
| NewOrderSingle decode (msgs/sec) | **2.94 M/s** | **0.47 M/s** | headers only, not JSON body | N/A |
| Handshake RTTs | 1 (TREE) / 0 (resumed) | 4 (TCP+TLS+Logon) | 3–5 (DNS+TCP+TLS+HTTP) | 2–3 (upgrade+TLS) |
| Multiplexing | 65,535 channels/conn | 1 session/conn | 6 (browser) / HTTP/2 | 1/conn |
| Session resumption | 0-RTT | Full reconnect | N/A (stateless) | Full reconnect |

See [fig-bench](../crates/fig-bench/) for source. CI: `.github/workflows/ci.yml` job `benchmarks`.
