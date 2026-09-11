//! Tier 4 FFI: compression, fragmentation, 0-RTT, migration.

use std::ffi::CStr;
use std::net::SocketAddr;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

use fig_core::codec::{decode_cbor, encode_cbor};
use fig_core::compression::{compress_payload, decompress_payload};
use fig_core::fragment::{reassemble_fragments, split_frame};
use fig_core::frame::Frame;
use fig_core::migration::MigrationToken;
use fig_core::session::Session;
use fig_core::transport::{client_config, FigClient};
use tokio::runtime::Runtime;

use crate::client::{FigClientHandle, FigFrameList};
use crate::{into_buffer, FigBuffer};

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

/// Compress payload bytes with zstd. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_payload_compress(
    data: *const u8,
    data_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || data.is_null() {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(data, data_len);
    match compress_payload(bytes) {
        Ok(compressed) => {
            *out = into_buffer(compressed);
            0
        }
        Err(_) => -2,
    }
}

/// Decompress zstd payload bytes. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_payload_decompress(
    data: *const u8,
    data_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || data.is_null() {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(data, data_len);
    match decompress_payload(bytes) {
        Ok(decompressed) => {
            *out = into_buffer(decompressed);
            0
        }
        Err(_) => -2,
    }
}

/// Split an encoded frame into fragments (max payload size per fragment).
#[no_mangle]
pub unsafe extern "C" fn fig_frame_split(
    frame_bytes: *const u8,
    frame_len: usize,
    max_payload: usize,
    out: *mut FigFrameList,
) -> i32 {
    if out.is_null() || frame_bytes.is_null() || max_payload == 0 {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(frame_bytes, frame_len);
    let (frame, _) = match Frame::decode(bytes) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    let fragments = match split_frame(&frame, max_payload) {
        Ok(f) => f,
        Err(_) => return -3,
    };
    match super::client::frames_to_list(fragments) {
        Ok(list) => {
            *out = list;
            0
        }
        Err(e) => e,
    }
}

/// Reassemble fragment frames (encoded bytes concatenated in `frames` array).
#[no_mangle]
pub unsafe extern "C" fn fig_frames_reassemble(
    frame_ptrs: *const *const u8,
    frame_lens: *const usize,
    count: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || frame_ptrs.is_null() || frame_lens.is_null() || count == 0 {
        return -1;
    }
    let ptrs = std::slice::from_raw_parts(frame_ptrs, count);
    let lens = std::slice::from_raw_parts(frame_lens, count);
    let mut fragments = Vec::with_capacity(count);
    for i in 0..count {
        let bytes = std::slice::from_raw_parts(ptrs[i], lens[i]);
        let (frame, _) = match Frame::decode(bytes) {
            Ok(v) => v,
            Err(_) => return -2,
        };
        fragments.push(frame);
    }
    let frame = match reassemble_fragments(fragments) {
        Ok(f) => f,
        Err(_) => return -3,
    };
    let encoded = match frame.encode() {
        Ok(b) => b,
        Err(_) => return -4,
    };
    *out = into_buffer(encoded);
    0
}

/// Connect with optional 0-RTT resumption token.
#[no_mangle]
pub unsafe extern "C" fn fig_client_connect_0rtt(
    addr: *const c_char,
    server_name: *const c_char,
    resumption_token: *const u8,
    token_len: usize,
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
    let token = if resumption_token.is_null() || token_len == 0 {
        None
    } else {
        Some(std::slice::from_raw_parts(resumption_token, token_len))
    };
    let rt = match Runtime::new() {
        Ok(r) => Arc::new(r),
        Err(_) => return -3,
    };
    let (conn, session) = match rt.block_on(async {
        let cfg = client_config().map_err(|e| e.to_string())?;
        let client = FigClient::new(cfg).map_err(|e| e.to_string())?;
        client
            .connect_0rtt(addr, &name, token)
            .await
            .map_err(|e| e.to_string())
    }) {
        Ok(v) => v,
        Err(_) => return -4,
    };
    *out = Box::into_raw(Box::new(FigClientHandle {
        rt,
        conn: Arc::new(Mutex::new(Some(Arc::new(conn)))),
        session: Arc::new(Mutex::new(session)),
    }));
    0
}

/// Export resumption token for an active session (if present).
#[no_mangle]
pub unsafe extern "C" fn fig_client_export_resumption_token(
    handle: *mut FigClientHandle,
    out: *mut FigBuffer,
) -> i32 {
    if handle.is_null() || out.is_null() {
        return -1;
    }
    let handle = &*handle;
    let session = {
        let guard = handle.session.lock().unwrap();
        guard.clone()
    };
    let Some(session) = session else {
        return -2;
    };
    match session.resumption_token() {
        Ok(token) => {
            *out = into_buffer(token);
            0
        }
        Err(_) => -3,
    }
}

/// Prepare migration token CBOR bytes from active session + connection.
#[no_mangle]
pub unsafe extern "C" fn fig_client_migration_prepare(
    handle: *mut FigClientHandle,
    out: *mut FigBuffer,
) -> i32 {
    if handle.is_null() || out.is_null() {
        return -1;
    }
    let handle = &*handle;
    let session = {
        let guard = handle.session.lock().unwrap();
        guard.clone()
    };
    let Some(session) = session else {
        return -2;
    };
    let conn = {
        let guard = handle.conn.lock().unwrap();
        match guard.as_ref() {
            Some(c) => c.clone(),
            None => return -3,
        }
    };
    let token = handle
        .rt
        .block_on(async { conn.prepare_migration(&session).await });
    match encode_cbor(&token) {
        Ok(bytes) => {
            *out = into_buffer(bytes);
            0
        }
        Err(_) => -4,
    }
}

/// Apply migration token CBOR and reopen channels on the connection.
#[no_mangle]
pub unsafe extern "C" fn fig_client_migration_apply(
    handle: *mut FigClientHandle,
    token_bytes: *const u8,
    token_len: usize,
) -> i32 {
    if handle.is_null() || token_bytes.is_null() {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(token_bytes, token_len);
    let token: MigrationToken = match decode_cbor(bytes) {
        Ok(t) => t,
        Err(_) => return -2,
    };
    let handle = &*handle;
    let conn = {
        let guard = handle.conn.lock().unwrap();
        match guard.as_ref() {
            Some(c) => c.clone(),
            None => return -3,
        }
    };
    let mut session = {
        let guard = handle.session.lock().unwrap();
        match guard.clone() {
            Some(s) => s,
            None => return -4,
        }
    };
    match handle
        .rt
        .block_on(async { conn.apply_migration(&token, &mut session, false).await })
    {
        Ok(()) => {
            *handle.session.lock().unwrap() = Some(session);
            0
        }
        Err(_) => -5,
    }
}

/// Restore session from resumption token bytes (without connecting).
#[no_mangle]
pub unsafe extern "C" fn fig_session_from_resumption_token(
    token_bytes: *const u8,
    token_len: usize,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || token_bytes.is_null() {
        return -1;
    }
    let bytes = std::slice::from_raw_parts(token_bytes, token_len);
    let session = match Session::from_resumption_token(bytes) {
        Ok(s) => s,
        Err(_) => return -2,
    };
    match encode_cbor(&session) {
        Ok(encoded) => {
            *out = into_buffer(encoded);
            0
        }
        Err(_) => -3,
    }
}
