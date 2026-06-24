#![no_main]

use libfuzzer_sys::fuzz_target;
use fig_core::frame::{Frame, FrameDecoder};

fuzz_target!(|data: &[u8]| {
    let _ = Frame::decode(data);
    let mut decoder = FrameDecoder::new();
    decoder.feed(data);
    while decoder.decode_next().transpose().ok().flatten().is_some() {}
});
