use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};

/// Create a realistic Request frame with extensions and payload.
fn make_request_frame() -> Frame {
    Frame::new(FrameType::Request, 42)
        .with_seq(100)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::RequestUri,
            "/trading/accounts/123/orders",
        ))
        .with_extension(Extension::text(
            ExtensionTag::ContentType,
            "application/cbor",
        ))
        .with_extension(Extension::u16(ExtensionTag::StatusCode, 200))
        .with_extension(Extension::u64(ExtensionTag::SequenceNum, 9999))
        .with_extension(Extension::i64(
            ExtensionTag::Timestamp,
            1700000000000000000,
        ))
        .with_payload(b"order payload data here".to_vec())
        .with_priority()
        .with_ack_requested()
}

fn bench_frame_encode(c: &mut Criterion) {
    let frame = make_request_frame();
    c.bench_function("frame_encode", |b| {
        b.iter(|| {
            let encoded = frame.encode().unwrap();
            black_box(encoded);
        })
    });
}

fn bench_frame_decode(c: &mut Criterion) {
    let frame = make_request_frame();
    let encoded = frame.encode().unwrap();
    c.bench_function("frame_decode", |b| {
        b.iter(|| {
            let (decoded, _consumed) = Frame::decode(black_box(&encoded)).unwrap();
            black_box(decoded);
        })
    });
}

fn bench_frame_decoder_streaming(c: &mut Criterion) {
    let frame = make_request_frame();
    let encoded = frame.encode().unwrap();
    c.bench_function("frame_decoder_streaming", |b| {
        b.iter(|| {
            let mut decoder = FrameDecoder::new();
            // Feed in 1KB chunks (simulating QUIC stream reads)
            for chunk in encoded.chunks(1024) {
                decoder.feed(chunk);
            }
            let result = decoder.decode_next().unwrap().unwrap();
            black_box(result);
        })
    });
}

fn bench_frame_encode_large_payload(c: &mut Criterion) {
    let large_payload = vec![0xAB; 10 * 1024]; // 10KB payload
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(42)
        .with_schema_id(0x01)
        .with_extension(Extension::text(
            ExtensionTag::RequestUri,
            "/trading/orders",
        ))
        .with_payload(large_payload);

    c.bench_function("frame_encode_large_payload", |b| {
        b.iter(|| {
            let encoded = frame.encode().unwrap();
            black_box(encoded);
        })
    });
}

criterion_group!(
    benches,
    bench_frame_encode,
    bench_frame_decode,
    bench_frame_decoder_streaming,
    bench_frame_encode_large_payload,
);
criterion_main!(benches);
