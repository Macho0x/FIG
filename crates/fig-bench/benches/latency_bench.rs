//! Tail-latency harness entry point for `cargo bench --bench latency_bench`.

fn main() {
    fig_bench::latency::run_all();
}
