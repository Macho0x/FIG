//! End-to-end FIG client over the stable C ABI against exchange-sim.

use std::ffi::CString;
use std::ptr;
use std::slice;

use fig_core::codec;
use fig_core::ext::{Extension, ExtensionTag};
use fig_core::frame::{Frame, FrameType};
use fig_core::messages::{
    NewOrderSingle, OrderType, PositionUpdate, Price, Quantity, Side, TimeInForce,
};
use fig_exchange_sim::server::run_server;
use fig_ffi::{
    fig_buffer_free, fig_client_close, fig_client_connect, fig_client_ping,
    fig_client_request_and_recv, fig_client_sub_close, fig_client_sub_next, fig_client_subscribe,
    fig_frame_encode_request_ex, fig_frame_encode_subscribe_auth, fig_frame_list_free, FigBuffer,
    FigClientHandle, FigFrameList, FigSubHandle,
};
use tokio::runtime::Runtime;

fn connect(addr: &str) -> *mut FigClientHandle {
    let addr_c = CString::new(addr).expect("addr");
    let mut handle: *mut FigClientHandle = ptr::null_mut();
    assert_eq!(
        unsafe { fig_client_connect(addr_c.as_ptr(), ptr::null(), &mut handle) },
        0
    );
    handle
}

fn send_frame(handle: *mut FigClientHandle, frame: &Frame) -> FigFrameList {
    let bytes = frame.encode().expect("encode");
    let mut responses = FigFrameList {
        frames: ptr::null_mut(),
        count: 0,
    };
    assert_eq!(
        unsafe { fig_client_request_and_recv(handle, bytes.as_ptr(), bytes.len(), &mut responses) },
        0
    );
    responses
}

fn order_frame(
    cl_ord_id: &str,
    side: Side,
    order_type: OrderType,
    price: Option<f64>,
    qty: f64,
) -> Frame {
    let order = NewOrderSingle {
        cl_ord_id: cl_ord_id.into(),
        side,
        order_qty: Quantity(qty),
        price: price.map(Price),
        stop_price: None,
        symbol: "AAPL".into(),
        order_type,
        time_in_force: TimeInForce::Day,
        expire_time: None,
        account: Some("TEST".into()),
        strategy_id: None,
        security_id: None,
        id_source: None,
        security_exchange: None,
        post_only: None,
        reduce_only: None,
    };
    let payload = codec::encode_cbor(&order).expect("cbor");
    Frame::new(FrameType::Request, 2)
        .with_seq(1)
        .with_schema_id(1)
        .with_extension(Extension::text(
            ExtensionTag::ChannelPath,
            "trading/accounts/TEST/orders",
        ))
        .with_extension(Extension::text(ExtensionTag::Method, "POST"))
        .with_payload(payload)
}

#[test]
fn ffi_client_ping_and_capabilities() {
    let server_rt = Runtime::new().expect("server runtime");
    let endpoint = server_rt
        .block_on(run_server("127.0.0.1:0"))
        .expect("server");
    let addr = endpoint.local_addr().expect("local addr");

    let handle = connect(&addr.to_string());
    assert_eq!(unsafe { fig_client_ping(handle) }, 0);

    let path = CString::new(".well-known/capabilities").unwrap();
    let method = CString::new("GET").unwrap();
    let mut frame_buf = FigBuffer {
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
        fig_buffer_free(frame_buf);
        fig_frame_list_free(responses);
        fig_client_close(handle);
    }
}

#[test]
fn ffi_subscribe_live_positions_on_fill() {
    let server_rt = Runtime::new().expect("server runtime");
    let endpoint = server_rt
        .block_on(run_server("127.0.0.1:0"))
        .expect("server");
    let addr = endpoint.local_addr().expect("local addr").to_string();

    let sub_client = connect(&addr);
    let trader = connect(&addr);

    let path = CString::new("accounts/TEST/positions").unwrap();
    let token = CString::new("fig-dev-TEST").unwrap();
    let mut sub_frame = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(
        unsafe {
            fig_frame_encode_subscribe_auth(
                1,
                1,
                path.as_ptr(),
                path.as_ptr(),
                token.as_ptr(),
                &mut sub_frame,
            )
        },
        0
    );

    let mut snapshot = FigFrameList {
        frames: ptr::null_mut(),
        count: 0,
    };
    let mut sub: *mut FigSubHandle = ptr::null_mut();
    assert_eq!(
        unsafe {
            fig_client_subscribe(
                sub_client,
                sub_frame.data,
                sub_frame.len,
                &mut snapshot,
                &mut sub,
            )
        },
        0
    );
    fig_buffer_free(sub_frame);
    assert!(!sub.is_null());
    unsafe { fig_frame_list_free(snapshot) };

    let sell = send_frame(
        trader,
        &order_frame("FFI-PD-1", Side::Sell, OrderType::Limit, Some(100.0), 5.0),
    );
    unsafe { fig_frame_list_free(sell) };
    let buy = send_frame(
        trader,
        &order_frame("FFI-PD-2", Side::Buy, OrderType::Market, None, 5.0),
    );
    unsafe { fig_frame_list_free(buy) };

    let mut live_buf = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };
    assert_eq!(unsafe { fig_client_sub_next(sub, 5_000, &mut live_buf) }, 0);
    let bytes = unsafe { slice::from_raw_parts(live_buf.data, live_buf.len) };
    let (frame, _) = Frame::decode(bytes).expect("decode live");
    assert_eq!(frame.frame_type, FrameType::StreamItem);
    let update: PositionUpdate = codec::decode_cbor(&frame.payload).expect("position");
    assert_eq!(update.account, "TEST");

    unsafe {
        fig_buffer_free(live_buf);
        fig_client_sub_close(sub);
        fig_client_close(sub_client);
        fig_client_close(trader);
    }
}
