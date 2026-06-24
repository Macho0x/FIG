//! Load / flood smoke harness.

use std::time::{Duration, Instant};

use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};

fn main() {
    let secs: u64 = std::env::var("FIG_LOAD_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);
    let frame = Frame::new(FrameType::Request, 1)
        .with_seq(1)
        .with_extension(Extension::text(ExtensionTag::ChannelPath, "load/ping"))
        .with_payload(b"ping".to_vec());

    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut iters = 0u64;
    while Instant::now() < deadline {
        let encoded = frame.encode().expect("encode");
        let (decoded, _) = Frame::decode(&encoded).expect("decode");
        assert_eq!(decoded.channel_id, 1);
        iters += 1;
    }
    let rate = iters as f64 / secs as f64;
    println!("fig-load: {iters} encode/decode iterations in {secs}s ({rate:.0}/s)");
}
