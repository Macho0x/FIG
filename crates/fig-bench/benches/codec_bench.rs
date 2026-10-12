use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
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
        post_only: None,
        reduce_only: None,
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

fn bench_codecs(c: &mut Criterion) {
    let order = make_order();
    let report = make_execution_report();
    let cbor_order = encode_cbor(&order).unwrap();
    let sbe_order = encode_new_order_single(&order);

    let mut group = c.benchmark_group("codec");
    group.throughput(Throughput::Elements(1));

    group.bench_function("cbor_encode_order", |b| {
        b.iter(|| {
            let encoded = encode_cbor(black_box(&order)).unwrap();
            black_box(encoded);
        })
    });
    group.bench_function("cbor_decode_order", |b| {
        b.iter(|| {
            let decoded: NewOrderSingle = decode_cbor(black_box(&cbor_order)).unwrap();
            black_box(decoded);
        })
    });
    group.bench_function("sbe_encode_order", |b| {
        b.iter(|| {
            let encoded = encode_new_order_single(black_box(&order));
            black_box(encoded);
        })
    });
    group.bench_function("sbe_decode_order", |b| {
        b.iter(|| {
            let decoded = decode_new_order_single(black_box(&sbe_order)).unwrap();
            black_box(decoded);
        })
    });
    group.bench_function("cbor_encode_execution_report", |b| {
        b.iter(|| {
            let encoded = encode_cbor(black_box(&report)).unwrap();
            black_box(encoded);
        })
    });
    group.bench_function("sbe_encode_execution_report", |b| {
        b.iter(|| {
            let encoded = encode_execution_report(black_box(&report));
            black_box(encoded);
        })
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion();
    targets = bench_codecs,
}
criterion_main!(benches);
