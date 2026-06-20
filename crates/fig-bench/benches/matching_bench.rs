use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fig_core::messages::{
    CancelRequest, NewOrderSingle, OrderType, Price, Quantity, Side, TimeInForce,
};

use fig_exchange_sim::matching::MatchingEngine;
use fig_exchange_sim::orderbook::{OrderBook, RestingOrder};

fn make_limit_order(id: &str, side: Side, symbol: &str, price: f64, qty: f64) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: id.to_string(),
        side,
        order_qty: Quantity(qty),
        price: Some(Price(price)),
        stop_price: None,
        symbol: symbol.to_string(),
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

fn make_market_order(id: &str, side: Side, symbol: &str, qty: f64) -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: id.to_string(),
        side,
        order_qty: Quantity(qty),
        price: None,
        stop_price: None,
        symbol: symbol.to_string(),
        order_type: OrderType::Market,
        time_in_force: TimeInForce::Ioc,
        expire_time: None,
        account: None,
        strategy_id: None,
            security_id: None,
            id_source: None,
            security_exchange: None,
    }
}

fn make_resting_order(id: &str, side: Side, symbol: &str, price: f64, qty: f64) -> RestingOrder {
    RestingOrder {
        order_id: format!("OX-{}", id),
        cl_ord_id: id.to_string(),
        side,
        symbol: symbol.to_string(),
        price: Price(price),
        qty: Quantity(qty),
        leaves_qty: Quantity(qty),
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: None,
        seq: 0,
    }
}

fn bench_order_book_add(c: &mut Criterion) {
    c.bench_function("order_book_add", |b| {
        b.iter(|| {
            let mut book = OrderBook::new("AAPL".to_string());
            for i in 0..1000 {
                let price = 100.0 + (i as f64) * 0.01;
                let order =
                    make_resting_order(&format!("ORD-{}", i), Side::Buy, "AAPL", price, 10.0);
                book.add_order(order);
            }
            black_box(book);
        })
    });
}

fn bench_matching_engine_process_order(c: &mut Criterion) {
    // Pre-populate the engine with a full book
    let mut engine = MatchingEngine::new();
    for i in 0..500 {
        let sell = make_limit_order(
            &format!("S-{}", i),
            Side::Sell,
            "AAPL",
            150.0 + (i as f64) * 0.01,
            10.0,
        );
        engine.process_new_order(&sell);
    }

    let market_buy = make_market_order("MB-1", Side::Buy, "AAPL", 100.0);

    // We need to reset the engine for each iteration — clone the setup data
    // Instead, create a fresh engine each iteration
    c.bench_function("matching_engine_process_order", |b| {
        b.iter(|| {
            let mut eng = MatchingEngine::new();
            // Populate with sells
            for i in 0..500 {
                let sell = make_limit_order(
                    &format!("S-{}", i),
                    Side::Sell,
                    "AAPL",
                    150.0 + (i as f64) * 0.01,
                    10.0,
                );
                eng.process_new_order(&sell);
            }
            let result = eng.process_new_order(black_box(&market_buy));
            black_box(result);
        })
    });
}

fn bench_matching_engine_cancel(c: &mut Criterion) {
    c.bench_function("matching_engine_cancel", |b| {
        b.iter(|| {
            let mut engine = MatchingEngine::new();
            let order = make_limit_order("CXL-1", Side::Buy, "AAPL", 100.0, 50.0);
            engine.process_new_order(&order);

            let cancel = CancelRequest {
                cl_ord_id: "CXL-CMD".to_string(),
                orig_cl_ord_id: "CXL-1".to_string(),
                symbol: "AAPL".to_string(),
                side: Side::Buy,
                order_qty: None,
            };
            let cancelled = engine.process_cancel(black_box(&cancel));
            black_box(cancelled);
        })
    });
}

criterion_group!(
    benches,
    bench_order_book_add,
    bench_matching_engine_process_order,
    bench_matching_engine_cancel,
);
criterion_main!(benches);
