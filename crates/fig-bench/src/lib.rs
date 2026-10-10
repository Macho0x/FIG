//! FIG benchmark utilities.
//!
//! Criterion microbenches live under `benches/`. Tail-latency percentile
//! reporting is in [`latency`] and exposed via the `fig-latency` binary.

pub mod criterion_config;
pub mod latency;
pub mod matching_latency;

#[cfg(test)]
mod tests {
    use super::latency::resolve_latency_iters;

    #[test]
    fn latency_iters_resolution_is_default_when_unset() {
        // Pure function: no process-environment access, so this cannot race
        // other tests that set FIG_LATENCY_ITERS.
        assert_eq!(resolve_latency_iters(10_000, None), 10_000);
        assert_eq!(resolve_latency_iters(10_000, Some("")), 10_000);
        assert_eq!(resolve_latency_iters(10_000, Some("0")), 10_000);
        assert_eq!(resolve_latency_iters(10_000, Some("not-a-number")), 10_000);
        assert_eq!(resolve_latency_iters(10_000, Some("250")), 250);
    }
}
