//! FIG benchmark utilities.
//!
//! Criterion microbenches live under `benches/`. Tail-latency percentile
//! reporting is in [`latency`] and exposed via the `fig-latency` binary.

pub mod criterion_config;
pub mod latency;
pub mod matching_latency;

#[cfg(test)]
mod tests {
    use super::latency::latency_iters;

    #[test]
    fn latency_iters_default() {
        std::env::remove_var("FIG_LATENCY_ITERS");
        assert_eq!(latency_iters(10_000), 10_000);
    }
}
