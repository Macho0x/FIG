//! Shared Criterion settings for local/doc runs. CI passes `--sample-size N` on the CLI.

use std::time::Duration;

use criterion::Criterion;

/// Default Criterion config: enough samples for stable medians without all-day runs.
pub fn criterion() -> Criterion {
    let mut c = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .sample_size(100);

    if let Ok(n) = std::env::var("FIG_BENCH_SAMPLE_SIZE") {
        if let Ok(n) = n.parse() {
            c = c.sample_size(n);
        }
    }

    c
}

/// Slow benches (QUIC handshake, full matching scenarios).
pub fn criterion_slow() -> Criterion {
    let mut c = criterion()
        .sample_size(30)
        .measurement_time(Duration::from_secs(5));
    if let Ok(n) = std::env::var("FIG_BENCH_SAMPLE_SIZE") {
        if let Ok(n) = n.parse() {
            c = c.sample_size(n);
        }
    }
    c
}
