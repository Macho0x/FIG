//! JWT FFI round-trip and verification tests.

use fig_ffi::{
    fig_buffer_free, fig_jwt_decode_sub, fig_jwt_encode, fig_jwt_verify_bearer, FigBuffer,
};
use std::ffi::CString;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn jwt_encode_decode_and_verify() {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;
    let sub = CString::new("trader-1").unwrap();
    let secret = CString::new("fig-jwt-secret").unwrap();
    let mut out = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };

    unsafe {
        assert_eq!(
            fig_jwt_encode(sub.as_ptr(), exp, secret.as_ptr(), &mut out),
            0
        );
        assert!(!out.data.is_null());
        assert!(out.len > 0);

        let token_bytes = std::slice::from_raw_parts(out.data, out.len);
        let token = CString::new(token_bytes).unwrap();
        fig_buffer_free(out);

        let mut decoded_sub: *mut i8 = ptr::null_mut();
        assert_eq!(
            fig_jwt_decode_sub(token.as_ptr(), secret.as_ptr(), &mut decoded_sub),
            0
        );
        let sub_out = CString::from_raw(decoded_sub);
        assert_eq!(sub_out.to_str().unwrap(), "trader-1");

        assert_eq!(fig_jwt_verify_bearer(token.as_ptr(), secret.as_ptr()), 0);
    }
}

#[test]
fn jwt_verify_rejects_wrong_secret() {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;
    let sub = CString::new("user").unwrap();
    let secret_a = CString::new("secret-a").unwrap();
    let secret_b = CString::new("secret-b").unwrap();
    let mut out = FigBuffer {
        data: ptr::null_mut(),
        len: 0,
    };

    unsafe {
        assert_eq!(
            fig_jwt_encode(sub.as_ptr(), exp, secret_a.as_ptr(), &mut out),
            0
        );
        let token_bytes = std::slice::from_raw_parts(out.data, out.len);
        let token = CString::new(token_bytes).unwrap();
        fig_buffer_free(out);
        assert!(fig_jwt_verify_bearer(token.as_ptr(), secret_b.as_ptr()) < 0);
    }
}
