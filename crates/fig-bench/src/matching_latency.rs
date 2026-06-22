//! Per-operation matching-engine latency (HDR Histogram).
//!
//! Isolated samples for insert-top, cancel-by-id, and replace on a pre-warmed book.

use std::time::Instant;

use fig_core::messages::{
    CancelReplaceRequest, CancelRequest, NewOrderSingle, OrderType, Price, Quantity, Side,
    TimeInForce,
};
use fig_exchange_sim::matching::MatchingEngine;

use crate::latency::{latency_iters, LatencyReport};

pub fn make_limit_order(id: &str, side: Side, price: f64) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: id.to_string(),
        side,
        order_qty: Quantity(10.0),
        price: Some(Price(price)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    }
}

/// Populate the ask side with `n_levels` resting sells starting at `base_price`.
fn prewarm_asks(engine: &mut MatchingEngine, n_levels: usize, base_price: f64) {
    for i in 0..n_levels {
        let order = make_limit_order(
            &format!("ASK-{i}"),
            Side::Sell,
            base_price + i as f64 * 0.01,
        );
        engine.process_new_order(&order);
    }
}

/// Populate the bid side with `n_levels` resting buys descending from `base_price`.
fn prewarm_bids(engine: &mut MatchingEngine, n_levels: usize, base_price: f64) {
    for i in 0..n_levels {
        let order = make_limit_order(&format!("BID-{i}"), Side::Buy, base_price - i as f64 * 0.01);
        engine.process_new_order(&order);
    }
}

/// Insert a non-crossing buy at the top of the bid book (rests, no match).
pub fn matching_insert_top(n_levels: usize) -> LatencyReport {
    let iters = latency_iters(50_000);
    let name = format!("matching_insert_top_n{n_levels}");
    let mut report = LatencyReport::new_ns(name);

    let mut engine = MatchingEngine::new();
    prewarm_asks(&mut engine, n_levels, 150.0);

    for i in 0..iters {
        let order = make_limit_order(
            &format!("TOP-{i}"),
            Side::Buy,
            149.99 - (i as f64 % 500.0) * 0.0001,
        );
        let t0 = Instant::now();
        let result = engine.process_new_order(&order);
        std::hint::black_box(result);
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

/// Cancel the best bid after pre-warming `n_levels` bids plus one target order.
pub fn matching_cancel_top(n_levels: usize) -> LatencyReport {
    let iters = latency_iters(50_000);
    let name = format!("matching_cancel_top_n{n_levels}");
    let mut report = LatencyReport::new_ns(name);

    for i in 0..iters {
        let mut engine = MatchingEngine::new();
        prewarm_bids(&mut engine, n_levels, 100.0);
        let target_id = format!("TOP-CXL-{i}");
        engine.process_new_order(&make_limit_order(&target_id, Side::Buy, 100.0 + 0.01));

        let cancel = CancelRequest {
            cl_ord_id: format!("CXL-{i}"),
            orig_cl_ord_id: target_id,
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_qty: None,
        };

        let t0 = Instant::now();
        let result = engine.process_cancel(&cancel);
        std::hint::black_box(result);
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

/// Cancel the deepest bid (worst price) on a pre-warmed book.
pub fn matching_cancel_deep(n_levels: usize) -> LatencyReport {
    let iters = latency_iters(if n_levels >= 1000 { 10_000 } else { 50_000 });
    let name = format!("matching_cancel_deep_n{n_levels}");
    let mut report = LatencyReport::new_ns(name);

    let depth = n_levels.max(1);

    for i in 0..iters {
        let mut engine = MatchingEngine::new();
        prewarm_bids(&mut engine, depth, 100.0);
        let deep_id = format!("DEEP-{i}");
        let deep_price = 100.0 - depth as f64 * 0.01;
        engine.process_new_order(&make_limit_order(&deep_id, Side::Buy, deep_price));

        let cancel = CancelRequest {
            cl_ord_id: format!("CXL-DEEP-{i}"),
            orig_cl_ord_id: deep_id,
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_qty: None,
        };

        let t0 = Instant::now();
        let result = engine.process_cancel(&cancel);
        std::hint::black_box(result);
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

/// Replace quantity only (same price) on a resting top-of-book order.
pub fn matching_replace_qty(n_levels: usize) -> LatencyReport {
    let iters = latency_iters(50_000);
    let name = format!("matching_replace_qty_n{n_levels}");
    let mut report = LatencyReport::new_ns(name);

    for i in 0..iters {
        let mut engine = MatchingEngine::new();
        prewarm_bids(&mut engine, n_levels, 100.0);
        let orig_id = format!("REP-{i}");
        engine.process_new_order(&make_limit_order(&orig_id, Side::Buy, 100.05));

        let replace = CancelReplaceRequest {
            cl_ord_id: format!("REP-NEW-{i}"),
            orig_cl_ord_id: orig_id,
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_qty: Quantity(20.0),
            price: Some(Price(100.05)),
        };

        let t0 = Instant::now();
        let result = engine.process_replace(&replace);
        std::hint::black_box(result);
        report.record_ns(t0.elapsed().as_nanos() as u64);
    }

    report
}

/// Eight threads contending on one engine — mutex queueing tail latency.
pub fn matching_engine_contended() -> LatencyReport {
    use std::sync::{Arc, Mutex};

    let iters = latency_iters(8_000);
    let threads = 8;
    let per_thread = iters / threads;
    let mut report = LatencyReport::new_ns("matching_engine_contended");

    let engine = Arc::new(Mutex::new(MatchingEngine::new()));
    {
        let mut eng = engine.lock().expect("lock");
        prewarm_asks(&mut eng, 200, 150.0);
    }

    let handles: Vec<_> = (0..threads)
        .map(|t| {
            let engine = Arc::clone(&engine);
            std::thread::spawn(move || {
                let mut local =
                    hdrhistogram::Histogram::<u64>::new_with_bounds(1, 60_000_000_000, 3)
                        .expect("histogram");
                for i in 0..per_thread {
                    let order = make_limit_order(
                        &format!("T{t}-BUY-{i}"),
                        Side::Buy,
                        149.0 - (i as f64) * 0.001,
                    );
                    let t0 = Instant::now();
                    {
                        let mut eng = engine.lock().expect("lock");
                        let result = eng.process_new_order(&order);
                        std::hint::black_box(result);
                    }
                    let _ = local.record(t0.elapsed().as_nanos() as u64);
                }
                local
            })
        })
        .collect();

    for h in handles {
        let local = h.join().expect("thread");
        report.merge_histogram(&local);
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use fig_exchange_sim::matching::CancelOutcome;

    fn with_low_iters(f: impl FnOnce()) {
        std::env::set_var("FIG_LATENCY_ITERS", "50");
        f();
        std::env::remove_var("FIG_LATENCY_ITERS");
    }

    #[test]
    fn insert_top_produces_samples() {
        with_low_iters(|| {
            let r = matching_insert_top(100);
            assert_eq!(r.len(), 50);
        });
    }

    #[test]
    fn cancel_top_succeeds() {
        with_low_iters(|| {
            let r = matching_cancel_top(100);
            assert_eq!(r.len(), 50);
        });
    }

    #[test]
    fn cancel_deep_succeeds() {
        with_low_iters(|| {
            let r = matching_cancel_deep(1000);
            assert_eq!(r.len(), 50);
        });
    }

    #[test]
    fn replace_qty_succeeds() {
        with_low_iters(|| {
            let r = matching_replace_qty(100);
            assert_eq!(r.len(), 50);
        });
    }

    #[test]
    fn cancel_top_actually_cancels() {
        let mut engine = MatchingEngine::new();
        let id = "T1".to_string();
        engine.process_new_order(&make_limit_order(&id, Side::Buy, 100.0));
        let cancel = CancelRequest {
            cl_ord_id: "C1".to_string(),
            orig_cl_ord_id: id,
            symbol: "AAPL".to_string(),
            side: Side::Buy,
            order_qty: None,
        };
        assert!(matches!(
            engine.process_cancel(&cancel),
            CancelOutcome::Cancelled(_)
        ));
    }
}
