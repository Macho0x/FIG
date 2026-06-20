//! End-to-end FIG client over the stable C ABI against exchange-sim.

use std::ffi::CString;
use std::ptr;

use fig_exchange_sim::server::run_server;
use fig_ffi::{
    fig_client_close, fig_client_connect, fig_client_ping, fig_client_request_and_recv,
    fig_frame_encode_request_ex, fig_frame_list_free, FigClientHandle, FigFrameList,
};
use tokio::runtime::Runtime;

#[test]
fn ffi_client_ping_and_capabilities() {
    let server_rt = Runtime::new().expect("server runtime");
    let endpoint = server_rt
        .block_on(run_server("127.0.0.1:0"))
        .expect("server");
    let addr = endpoint.local_addr().expect("local addr");

    let addr_c = CString::new(addr.to_string()).expect("addr");
    let mut handle: *mut FigClientHandle = ptr::null_mut();
    assert_eq!(
        unsafe { fig_client_connect(addr_c.as_ptr(), ptr::null(), &mut handle) },
        0
    );
    assert_eq!(unsafe { fig_client_ping(handle) }, 0);

    let path = CString::new(".well-known/capabilities").unwrap();
    let method = CString::new("GET").unwrap();
    let mut frame_buf = fig_ffi::FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(
        unsafe {
            fig_frame_encode_request_ex(
                1,
                1,
                1,
                path.as_ptr(),
                method.as_ptr(),
                ptr::null(),
                ptr::null(),
                0,
                &mut frame_buf,
            )
        },
        0
    );
    let mut responses = FigFrameList {
        frames: ptr::null_mut(),
        count: 0,
    };
    assert_eq!(
        unsafe {
            fig_client_request_and_recv(handle, frame_buf.data, frame_buf.len, &mut responses)
        },
        0
    );
    assert!(responses.count >= 1);
    unsafe {
        fig_ffi::fig_buffer_free(frame_buf);
        fig_frame_list_free(responses);
        fig_client_close(handle);
    }
}
