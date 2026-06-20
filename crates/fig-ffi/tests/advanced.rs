//! Tier 4 FFI smoke tests (compression + fragmentation).

use fig_core::frame::{Frame, FrameType};
use fig_ffi::{
    fig_frame_list_free, fig_frame_split, fig_frames_reassemble, fig_payload_compress,
    fig_payload_decompress, FigBuffer, FigFrameList,
};
use std::ptr;

#[test]
fn payload_compress_round_trip() {
    let data = b"compressible payload data here".repeat(20);
    let mut compressed = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(
        unsafe { fig_payload_compress(data.as_ptr(), data.len(), &mut compressed) },
        0
    );
    let mut decompressed = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(
        unsafe { fig_payload_decompress(compressed.data, compressed.len, &mut decompressed) },
        0
    );
    let out = unsafe { std::slice::from_raw_parts(decompressed.data, decompressed.len).to_vec() };
    assert_eq!(out, data);
    unsafe {
        fig_ffi::fig_buffer_free(compressed);
        fig_ffi::fig_buffer_free(decompressed);
    }
}

#[test]
fn frame_split_and_reassemble() {
    let frame = Frame::new(FrameType::Request, 1)
        .with_payload(b"hello fragmented world payload".repeat(10));
    let encoded = frame.encode().unwrap();
    let mut list = FigFrameList {
        frames: ptr::null_mut(),
        count: 0,
    };
    assert_eq!(
        unsafe { fig_frame_split(encoded.as_ptr(), encoded.len(), 32, &mut list) },
        0
    );
    assert!(list.count >= 2);
    let slice = unsafe { std::slice::from_raw_parts(list.frames, list.count) };
    let mut ptrs: Vec<*const u8> = Vec::with_capacity(list.count);
    let mut lens: Vec<usize> = Vec::with_capacity(list.count);
    for buf in slice {
        ptrs.push(buf.data);
        lens.push(buf.len);
    }
    let mut out = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(
        unsafe { fig_frames_reassemble(ptrs.as_ptr(), lens.as_ptr(), list.count, &mut out,) },
        0
    );
    let round = unsafe { std::slice::from_raw_parts(out.data, out.len).to_vec() };
    assert_eq!(round, encoded);
    unsafe {
        fig_frame_list_free(list);
        fig_ffi::fig_buffer_free(out);
    }
}
