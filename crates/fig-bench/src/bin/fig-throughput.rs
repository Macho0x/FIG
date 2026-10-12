//! Sustained msgs/sec harness.
//!
//! ```bash
//! cargo run --release -p fig-bench --bin fig-throughput
//! FIG_THROUGHPUT_SECS=1 cargo run --release -p fig-bench --bin fig-throughput
//! ```

fn main() {
    fig_bench::throughput::run_all();
}
