//! Synchronous FIG client over TREE for FFI language bindings.

use std::ffi::CStr;
use std::net::SocketAddr;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

use fig_core::frame::Frame;
use fig_core::transport::{client_config, FigClient, FigConnection};
use tokio::runtime::Runtime;

use crate::{into_buffer, FigBuffer};

/// Opaque connected FIG client (one TREE connection).
pub struct FigClientHandle {
    rt: Runtime,
    conn: Arc<Mutex<Option<Arc<FigConnection>>>>,
}

/// List of encoded frames returned from request/subscribe calls.
#[repr(C)]
pub struct FigFrameList {
    pub frames: *mut FigBuffer,
    pub count: usize,
}

#[no_mangle]
pub unsafe extern "C" fn fig_frame_list_free(list: FigFrameList) {
    if list.frames.is_null() || list.count == 0 {
        return;
    }
    let raw = std::slice::from_raw_parts_mut(list.frames, list.count);
    for buf in raw.iter() {
        crate::fig_buffer_free(FigBuffer {
            data: buf.data,
            len: buf.len,
        });
    }
    drop(Box::from_raw(raw));
}

fn parse_addr(addr: *const c_char) -> Result<SocketAddr, i32> {
    if addr.is_null() {
        return Err(-1);
    }
    let s = unsafe { CStr::from_ptr(addr).to_str().map_err(|_| -2)? };
    s.parse().map_err(|_| -2)
}

fn parse_server_name(server_name: *const c_char) -> Result<String, i32> {
    if server_name.is_null() {
        return Ok("localhost".to_string());
    }
    Ok(unsafe {
        CStr::from_ptr(server_name)
            .to_str()
            .map_err(|_| -2)?
            .to_string()
    })
}

/// Connect to a FIG server (`host:port`). Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_client_connect(
    addr: *const c_char,
    server_name: *const c_char,
    out: *mut *mut FigClientHandle,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let addr = match parse_addr(addr) {
        Ok(a) => a,
        Err(e) => return e,
    };
    let name = match parse_server_name(server_name) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let rt = match Runtime::new() {
        Ok(r) => r,
        Err(_) => return -3,
    };
    let conn = match rt.block_on(async {
        let cfg = client_config().map_err(|e| e.to_string())?;
        let client = FigClient::new(cfg).map_err(|e| e.to_string())?;
        client.connect(addr, &name).await.map_err(|e| e.to_string())
    }) {
        Ok(c) => c,
        Err(_) => return -4,
    };
    *out = Box::into_raw(Box::new(FigClientHandle {
        rt,
        conn: Arc::new(Mutex::new(Some(Arc::new(conn)))),
    }));
    0
}

#[no_mangle]
pub unsafe extern "C" fn fig_client_close(handle: *mut FigClientHandle) {
    if handle.is_null() {
        return;
    }
    drop(Box::from_raw(handle));
}

fn send_frame_and_recv_all(handle: &FigClientHandle, frame: Frame) -> Result<Vec<Frame>, i32> {
    let conn = {
        let guard = handle.conn.lock().unwrap();
        guard.as_ref().ok_or(-5)?.clone()
    };
    handle
        .rt
        .block_on(async move { conn.request_and_recv_all(frame).await.map_err(|_| -6) })
}

fn frames_to_list(frames: Vec<Frame>) -> Result<FigFrameList, i32> {
    let mut out: Vec<FigBuffer> = Vec::with_capacity(frames.len());
    for frame in frames {
        let bytes = frame.encode().map_err(|_| -7)?;
        out.push(into_buffer(bytes));
    }
    let count = out.len();
    let ptr = Box::into_raw(out.into_boxed_slice()) as *mut FigBuffer;
    Ok(FigFrameList { frames: ptr, count })
}

/// Send encoded frame bytes; receive all response frames.
#[no_mangle]
pub unsafe extern "C" fn fig_client_request_and_recv(
    handle: *mut FigClientHandle,
    frame_bytes: *const u8,
    frame_len: usize,
    out: *mut FigFrameList,
) -> i32 {
    if handle.is_null() || out.is_null() || frame_bytes.is_null() {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(frame_bytes, frame_len);
    let (frame, _) = match Frame::decode(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    let handle = &*handle;
    let responses = match send_frame_and_recv_all(handle, frame) {
        Ok(r) => r,
        Err(e) => return e,
    };
    match frames_to_list(responses) {
        Ok(list) => {
            *out = list;
            0
        }
        Err(e) => e,
    }
}

/// Send PING on channel 0.
#[no_mangle]
pub unsafe extern "C" fn fig_client_ping(handle: *mut FigClientHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }
    let handle = &*handle;
    let frame = fig_core::frame::Frame::ping();
    match send_frame_and_recv_all(handle, frame) {
        Ok(_) => 0,
        Err(e) => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn frame_list_free_handles_empty() {
        unsafe {
            fig_frame_list_free(FigFrameList {
                frames: ptr::null_mut(),
                count: 0,
            });
        }
    }
}
