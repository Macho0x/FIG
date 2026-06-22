//! Tail-latency percentile harness (p50 / p99 / p99.9).
//!
//! ```bash
//! cargo run --release -p fig-bench --bin fig-latency
//! FIG_LATENCY_ITERS=1000 cargo run --release -p fig-bench --bin fig-latency
//! ```

fn main() {
    fig_bench::latency::run_all();
}
