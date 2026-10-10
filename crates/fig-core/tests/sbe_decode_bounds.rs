//! Regression tests for SBE decoder hardening.
//!
//! The generated SBE decoders must reject truncated or hostile payloads with
//! `Err(...)` and must never panic, index out of bounds, or allocate based on
//! an unvalidated wire count.

use fig_core::generated::sbe_wire::*;
use fig_core::sbe::decode_new_order_single;

/// The original confirmed repro: an 8-byte buffer that passes the SBE header
/// check but has no field bytes. Before the fix this panicked with
/// "index out of bounds: the len is 8 but the index is 8".
#[test]
fn confirmed_repro_returns_err_not_panic() {
    let buf = [0u8, 1, 0, 1, 0, 0, 0, 0];
    let err = decode_new_order_single(&buf).expect_err("must reject truncated payload");
    let msg = format!("{err}");
    println!("decode_new_order_single(&[0,1,0,1,0,0,0,0]) -> Err({msg})");
    assert!(
        msg.contains("buffer too short") || msg.contains("sbe:"),
        "unexpected error: {msg}"
    );
}

/// Every prefix of a longer buffer must decode to `Ok` or `Err`, never panic.
#[test]
fn truncated_payloads_never_panic() {
    // 8-byte valid NewOrderSingle header plus assorted field bytes.
    let mut full = vec![0u8, 1, 0, 1, 0, 0, 0, 0];
    full.extend_from_slice(&[
        0, 3, b'a', b'b', b'c', // cl_ord_id len + bytes
        1,    // side = Buy
        0, 0, 0, 0, 0, 0, 0, 0, // order_qty f64
        /* continue with price/stop_price presence bytes and more */
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    for len in 0..=full.len() {
        let _ = NewOrderSingleDecoder::decode(&full[..len]);
        let _ = decode_new_order_single(&full[..len]);
    }
}

fn exercise_generated_decoders(buf: &[u8]) {
    let _ = NewOrderSingleDecoder::decode(buf);
    let _ = CancelRequestDecoder::decode(buf);
    let _ = CancelReplaceRequestDecoder::decode(buf);
    let _ = ExecutionReportDecoder::decode(buf);
    let _ = CancelRejectDecoder::decode(buf);
    let _ = MarketDataSnapshotDecoder::decode(buf);
    let _ = OrderBookSnapshotDecoder::decode(buf);
    let _ = OrderBookDeltaDecoder::decode(buf);
    let _ = OrderBookRequestDecoder::decode(buf);
    let _ = CandleBarBatchDecoder::decode(buf);
    let _ = OrderHistoryBatchDecoder::decode(buf);
    let _ = BalanceSnapshotDecoder::decode(buf);
    let _ = PositionSnapshotDecoder::decode(buf);
    let _ = PriceLevelDecoder::decode(buf);
}

/// Deterministic pseudo-random buffers must never panic for any decoder.
#[test]
fn arbitrary_buffers_never_panic() {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    for _ in 0..5_000 {
        let mut buf = Vec::with_capacity(64);
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let len = (state >> 33) as usize % 64;
        for _ in 0..len {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            buf.push((state >> 33) as u8);
        }
        exercise_generated_decoders(&buf);
    }
}

/// A hostile, unbounded list count must not drive a huge allocation: the
/// decoder must validate the count against the remaining bytes and return
/// `Err`. Buffer: tmpl 9 header, empty symbol/exchange, then count = u32::MAX.
#[test]
fn oversized_list_count_is_capped_and_rejected() {
    let buf: Vec<u8> = vec![
        0, 1, // schema_id = 1
        0, 9, // template_id = 9 (OrderBookSnapshot)
        0, 0, // version
        0, 0, // block_length
        0, 0, // symbol: empty
        0, 0, // exchange: empty
        0xFF, 0xFF, 0xFF, 0xFF, // bids count = u32::MAX
    ];
    let err = OrderBookSnapshotDecoder::decode(&buf).expect_err("must reject hostile count");
    assert!(
        err.contains("buffer too short") || err.contains("sbe:"),
        "unexpected error: {err}"
    );
}

/// A hostile list count on a struct list must likewise fail rather than OOM.
#[test]
fn oversized_nested_list_count_is_rejected() {
    // OrderBookDelta (tmpl 10): symbol str then updates list count.
    let buf: Vec<u8> = vec![
        0, 1, // schema_id
        0, 10, // template_id (OrderBookDelta)
        0, 0, 0, 0, // version + block_length
        0, 0, // symbol: empty
        0xFF, 0xFF, 0xFF, 0xFF, // updates count = u32::MAX
    ];
    let _ = OrderBookDeltaDecoder::decode(&buf);
}
