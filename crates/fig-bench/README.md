# fig-bench

Benchmark suite for the FIG protocol stack.

**Cite numbers from [docs/BENCHMARKS.md](../../docs/BENCHMARKS.md).** This crate is
the source; that document is the methodology and the published tables.

## Layers

| Layer | How to run | Use for |
|---|---|---|
| **Throughput (msgs/sec)** | `cargo run --release -p fig-bench --bin fig-throughput` | Wall-clock operations per second |
| **Tail latency** | `cargo run --release -p fig-bench --bin fig-latency` | p50 / p99 / p99.9 of one operation |
| **Criterion** | `cargo bench -p fig-bench` | Mean time + elems/s regression |
| **Flood smoke** | `FIG_LOAD_SECS=3 cargo run --release -p fig-bench --bin fig-load` | CI DoS-path guard, not a rate to publish |

```bash
# Published msgs/sec (3 s measure, 200 ms warmup)
cargo run --release -p fig-bench --bin fig-throughput

# CI / laptop smoke
FIG_THROUGHPUT_SECS=1 FIG_THROUGHPUT_WARMUP_MS=50 \
  cargo run --release -p fig-bench --bin fig-throughput

cargo run --release -p fig-bench --bin fig-latency
cargo bench -p fig-bench --bench codec_bench
cargo bench -p fig-bench --features alloc --bench alloc_bench
```

TREE throughput lifts `DoSGuard` / `ChannelRateLimiter` so the number is a
codec+QUIC ceiling. Production connections keep **10_000 msgs/sec/channel**.
Localhost is not a colo SLA.

`fig-throughput` source: [`src/throughput.rs`](src/throughput.rs).
