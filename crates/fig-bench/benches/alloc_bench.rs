use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameDecoder, FrameType};

fn make_payload_frame(payload_kb: usize) -> Frame {
    Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_extension(Extension::text(ExtensionTag::RequestUri, "/orders"))
        .with_payload(vec![0u8; payload_kb * 1024])
}

fn bench_encode_allocating(c: &mut Criterion) {
    let frame = make_payload_frame(4);
    c.bench_function("encode_allocating_4kb", |b| {
        b.iter(|| {
            let encoded = frame.encode().unwrap();
            black_box(encoded);
        });
    });
}

fn bench_decode_allocating(c: &mut Criterion) {
    let encoded = make_payload_frame(4).encode().unwrap();
    c.bench_function("decode_allocating_4kb", |b| {
        b.iter(|| {
            let (decoded, _) = Frame::decode(&encoded).unwrap();
            black_box(decoded);
        });
    });
}

fn bench_decoder_reuse(c: &mut Criterion) {
    let encoded = make_payload_frame(1).encode().unwrap();
    c.bench_function("decoder_buffer_reuse_1kb", |b| {
        b.iter(|| {
            let mut decoder = FrameDecoder::with_capacity(encoded.len() * 2);
            decoder.feed(&encoded);
            let frame = decoder.decode_next().unwrap().unwrap();
            black_box(frame);
        });
    });
}

#[cfg(feature = "alloc")]
fn bench_preallocated_encode_buffer(c: &mut Criterion) {
    let frame = make_payload_frame(4);
    let mut buf = vec![0u8; frame.encoded_size()];
    c.bench_function("encode_preallocated_buffer_4kb", |b| {
        b.iter(|| {
            let encoded = frame.encode().unwrap();
            if encoded.len() <= buf.len() {
                buf[..encoded.len()].copy_from_slice(&encoded);
            }
            black_box(&buf[..encoded.len()]);
        });
    });
}

#[cfg(feature = "alloc")]
criterion_group!(
    alloc_benches,
    bench_encode_allocating,
    bench_decode_allocating,
    bench_decoder_reuse,
    bench_preallocated_encode_buffer
);

#[cfg(not(feature = "alloc"))]
criterion_group!(
    alloc_benches,
    bench_encode_allocating,
    bench_decode_allocating,
    bench_decoder_reuse
);

criterion_main!(alloc_benches);
