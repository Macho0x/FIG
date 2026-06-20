use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fig_core::messages::{
    ExecType, ExecutionReport, NewOrderSingle, OrdStatus, Price, Quantity, Side,
};

use fig_gateways::fix::{
    fig_to_fix_execution_report, fix_to_fig_order, parse_fix_message, serialize_fix_message,
    FixOutboundContext,
};
use fig_gateways::rest::{parse_http_request, serialize_http_response, HttpResponse};
use fig_gateways::ws::{parse_ws_frame, serialize_ws_frame, WsFrame, WsOpcode};

// ─── FIX bench helpers ──────────────────────────────────────────

fn make_fix_new_order_single_bytes() -> Vec<u8> {
    let tags = vec![
        (8u32, "FIX.4.4".to_string()),
        (9, "200".to_string()),
        (35, "D".to_string()),
        (49, "SENDER".to_string()),
        (56, "TARGET".to_string()),
        (34, "1".to_string()),
        (52, "20240101-00:00:00".to_string()),
        (11, "ORD-001".to_string()),
        (21, "1".to_string()),
        (55, "AAPL".to_string()),
        (54, "1".to_string()),
        (38, "100".to_string()),
        (44, "150.25".to_string()),
        (40, "2".to_string()),
        (59, "0".to_string()),
        (1, "ACCT-123".to_string()),
    ];
    serialize_fix_message(&tags)
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

// ─── FIX benchmarks ─────────────────────────────────────────────

fn bench_fix_parse(c: &mut Criterion) {
    let msg = make_fix_new_order_single_bytes();
    c.bench_function("fix_parse", |b| {
        b.iter(|| {
            let parsed = parse_fix_message(black_box(&msg)).unwrap();
            black_box(parsed);
        })
    });
}

fn bench_fix_serialize(c: &mut Criterion) {
    let tags = vec![
        (8u32, "FIX.4.4".to_string()),
        (9, "200".to_string()),
        (35, "8".to_string()),
        (11, "ORD-001".to_string()),
        (37, "OX-001".to_string()),
        (17, "EX-001".to_string()),
        (150, "2".to_string()),
        (39, "2".to_string()),
        (54, "1".to_string()),
        (32, "100".to_string()),
        (31, "50.25".to_string()),
        (151, "0".to_string()),
        (14, "100".to_string()),
        (6, "50.25".to_string()),
        (55, "AAPL".to_string()),
        (60, "1700000000000000000".to_string()),
    ];
    c.bench_function("fix_serialize", |b| {
        b.iter(|| {
            let msg = serialize_fix_message(black_box(&tags));
            black_box(msg);
        })
    });
}

fn bench_fix_to_fig_order(c: &mut Criterion) {
    let msg = make_fix_new_order_single_bytes();
    let tags = parse_fix_message(&msg).unwrap();
    c.bench_function("fix_to_fig_order", |b| {
        b.iter(|| {
            let order: NewOrderSingle = fix_to_fig_order(black_box(&tags)).unwrap();
            black_box(order);
        })
    });
}

fn bench_fig_to_fix_execution_report(c: &mut Criterion) {
    let report = make_execution_report();
    c.bench_function("fig_to_fix_execution_report", |b| {
        b.iter(|| {
            let msg =
                fig_to_fix_execution_report(black_box(&report), &FixOutboundContext::default());
            black_box(msg);
        })
    });
}

// ─── REST benchmarks ────────────────────────────────────────────

fn bench_rest_parse_request(c: &mut Criterion) {
    let raw = b"POST /trading/orders HTTP/1.1\r\nContent-Type: application/json\r\nUser-Agent: fig-client/1.0\r\nAccept: application/json\r\n\r\n{\"symbol\":\"AAPL\",\"side\":\"buy\",\"qty\":100}";
    c.bench_function("rest_parse_request", |b| {
        b.iter(|| {
            let req = parse_http_request(black_box(raw.as_slice())).unwrap();
            black_box(req);
        })
    });
}

fn bench_rest_serialize_response(c: &mut Criterion) {
    let resp = HttpResponse {
        status_code: 200,
        reason: "OK".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("Content-Length".to_string(), "45".to_string()),
        ],
        body: b"{\"order_id\":\"OX-001\",\"status\":\"accepted\"}".to_vec(),
    };
    c.bench_function("rest_serialize_response", |b| {
        b.iter(|| {
            let data = serialize_http_response(black_box(&resp));
            black_box(data);
        })
    });
}

// ─── WebSocket benchmarks ───────────────────────────────────────

fn bench_ws_parse_text_frame(c: &mut Criterion) {
    let frame = WsFrame {
        fin: true,
        opcode: WsOpcode::Text,
        masked: false,
        payload: b"Hello, WebSocket world!".to_vec(),
    };
    let serialized = serialize_ws_frame(&frame);
    c.bench_function("ws_parse_text_frame", |b| {
        b.iter(|| {
            let (parsed, _consumed) = parse_ws_frame(black_box(&serialized)).unwrap();
            black_box(parsed);
        })
    });
}

fn bench_ws_serialize_text_frame(c: &mut Criterion) {
    let frame = WsFrame {
        fin: true,
        opcode: WsOpcode::Text,
        masked: false,
        payload: b"Hello, WebSocket world!".to_vec(),
    };
    c.bench_function("ws_serialize_text_frame", |b| {
        b.iter(|| {
            let data = serialize_ws_frame(black_box(&frame));
            black_box(data);
        })
    });
}

// ─── Protocol comparison benchmarks ─────────────────────────────

fn bench_protocol_comparison_new_order(c: &mut Criterion) {
    use fig_core::ext::{Extension, ExtensionTag};
    use fig_core::frame::{Frame, FrameType};
    use fig_gateways::rest::{http_to_fig_frame, parse_http_request};

    let fix_msg = make_fix_new_order_single_bytes();
    let fix_tags = parse_fix_message(&fix_msg).unwrap();
    let http_bytes = b"POST /trading/orders HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"cl_ord_id\":\"ORD-1\",\"symbol\":\"AAPL\",\"side\":\"Buy\",\"order_qty\":100,\"price\":150.25}";

    let fig_frame = Frame::new(FrameType::Request, 1)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, "trading/orders"))
        .with_payload(vec![]);

    let mut group = c.benchmark_group("protocol_comparison_new_order");
    group.bench_function("fix_parse", |b| {
        b.iter(|| {
            let tags = parse_fix_message(black_box(&fix_msg)).unwrap();
            black_box(tags);
        })
    });
    group.bench_function("fig_native_encode", |b| {
        b.iter(|| {
            let encoded = black_box(&fig_frame).encode().unwrap();
            black_box(encoded);
        })
    });
    group.bench_function("rest_parse", |b| {
        b.iter(|| {
            let req = parse_http_request(black_box(http_bytes)).unwrap();
            black_box(req);
        })
    });
    group.bench_function("fix_to_fig_order", |b| {
        b.iter(|| {
            let order = fix_to_fig_order(black_box(&fix_tags)).unwrap();
            black_box(order);
        })
    });
    group.bench_function("rest_to_fig", |b| {
        b.iter(|| {
            let req = parse_http_request(http_bytes).unwrap();
            let frame = http_to_fig_frame(&req).unwrap();
            black_box(frame);
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_fix_parse,
    bench_fix_serialize,
    bench_fix_to_fig_order,
    bench_fig_to_fix_execution_report,
    bench_rest_parse_request,
    bench_rest_serialize_response,
    bench_ws_parse_text_frame,
    bench_ws_serialize_text_frame,
    bench_protocol_comparison_new_order,
);
criterion_main!(benches);
