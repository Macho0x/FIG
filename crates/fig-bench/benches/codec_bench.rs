use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fig_bench::criterion_config::criterion;

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::messages::{
    ExecType, ExecutionReport, NewOrderSingle, OrdStatus, OrderType, Price, Quantity, Side,
    TimeInForce,
};
use fig_core::sbe::{decode_new_order_single, encode_execution_report, encode_new_order_single};

fn make_order() -> NewOrderSingle {
    NewOrderSingle {
        cl_ord_id: "ORD-001".to_string(),
        side: Side::Buy,
        order_qty: Quantity(100.0),
        price: Some(Price(150.25)),
        stop_price: None,
        symbol: "AAPL".to_string(),
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some("ACCT-123".to_string()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
    }
}

fn make_execution_report() -> ExecutionReport {
    ExecutionReport {
        cl_ord_id: "ORD-001".to_string(),
        order_id: "OX-001".to_string(),
        exec_id: "EX-001".to_string(),
        exec_type: ExecType::Fill,
        ord_status: OrdStatus::Filled,
        side: Side::Buy,
        last_qty: Some(Quantity(100.0)),
        last_price: Some(Price(50.25)),
        leaves_qty: Quantity(0.0),
        cum_qty: Quantity(100.0),
        avg_price: Price(50.25),
        symbol: "AAPL".to_string(),
        transact_time: 1700000000000000000,
    }
}

fn bench_cbor_encode_order(c: &mut Criterion) {
    let order = make_order();
    c.bench_function("cbor_encode_order", |b| {
        b.iter(|| {
            let encoded = encode_cbor(black_box(&order)).unwrap();
            black_box(encoded);
        })
    });
}

fn bench_cbor_decode_order(c: &mut Criterion) {
    let order = make_order();
    let encoded = encode_cbor(&order).unwrap();
    c.bench_function("cbor_decode_order", |b| {
        b.iter(|| {
            let decoded: NewOrderSingle = decode_cbor(black_box(&encoded)).unwrap();
            black_box(decoded);
        })
    });
}

fn bench_sbe_encode_order(c: &mut Criterion) {
    let order = make_order();
    c.bench_function("sbe_encode_order", |b| {
        b.iter(|| {
            let encoded = encode_new_order_single(black_box(&order));
            black_box(encoded);
        })
    });
}

fn bench_sbe_decode_order(c: &mut Criterion) {
    let order = make_order();
    let encoded = encode_new_order_single(&order);
    c.bench_function("sbe_decode_order", |b| {
        b.iter(|| {
            let decoded = decode_new_order_single(black_box(&encoded)).unwrap();
            black_box(decoded);
        })
    });
}

fn bench_cbor_encode_execution_report(c: &mut Criterion) {
    let report = make_execution_report();
    c.bench_function("cbor_encode_execution_report", |b| {
        b.iter(|| {
            let encoded = encode_cbor(black_box(&report)).unwrap();
            black_box(encoded);
        })
    });
}

fn bench_sbe_encode_execution_report(c: &mut Criterion) {
    let report = make_execution_report();
    c.bench_function("sbe_encode_execution_report", |b| {
        b.iter(|| {
            let encoded = encode_execution_report(black_box(&report));
            black_box(encoded);
        })
    });
}

criterion_group! {
    name = benches;
    config = criterion();
    targets = bench_cbor_encode_order,
        bench_cbor_decode_order,
        bench_sbe_encode_order,
        bench_sbe_decode_order,
        bench_cbor_encode_execution_report,
        bench_sbe_encode_execution_report,
}
criterion_main!(benches);
