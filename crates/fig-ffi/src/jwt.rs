//! JWT bearer token FFI over `fig_core::jwt`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use fig_core::jwt::{decode_jwt, encode_jwt, verify_jwt_bearer, FigJwtClaims, JwtError};

use crate::{into_buffer, FigBuffer};

fn jwt_error_code(err: JwtError) -> i32 {
    match err {
        JwtError::InvalidFormat => -4,
        JwtError::InvalidHeader => -5,
        JwtError::InvalidPayload => -6,
        JwtError::SignatureMismatch => -7,
        JwtError::Expired => -8,
        JwtError::MissingSubject => -9,
    }
}

/// Encode HS256 JWT claims (`sub`, `exp`) into `out`. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_jwt_encode(
    sub: *const c_char,
    exp: u64,
    secret: *const c_char,
    out: *mut FigBuffer,
) -> i32 {
    if out.is_null() || sub.is_null() || secret.is_null() {
        return -1;
    }
    let sub = match CStr::from_ptr(sub).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let secret = match CStr::from_ptr(secret).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let claims = FigJwtClaims::new(sub, exp, vec![]);
    match encode_jwt(&claims, secret) {
        Ok(token) => {
            *out = into_buffer(token.into_bytes());
            0
        }
        Err(e) => jwt_error_code(e),
    }
}

/// Decode JWT and return the `sub` claim via newly allocated C string.
#[no_mangle]
pub unsafe extern "C" fn fig_jwt_decode_sub(
    token: *const c_char,
    secret: *const c_char,
    out_sub: *mut *mut c_char,
) -> i32 {
    if token.is_null() || secret.is_null() || out_sub.is_null() {
        return -1;
    }
    let token = match CStr::from_ptr(token).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let secret = match CStr::from_ptr(secret).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    match decode_jwt(token, secret) {
        Ok(claims) => match CString::new(claims.sub) {
            Ok(s) => {
                *out_sub = s.into_raw();
                0
            }
            Err(_) => -3,
        },
        Err(e) => jwt_error_code(e),
    }
}

/// Verify bearer JWT signature and expiry. Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn fig_jwt_verify_bearer(token: *const c_char, secret: *const c_char) -> i32 {
    if token.is_null() || secret.is_null() {
        return -1;
    }
    let token = match CStr::from_ptr(token).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let secret = match CStr::from_ptr(secret).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    match verify_jwt_bearer(token, secret) {
        Ok(_) => 0,
        Err(e) => jwt_error_code(e),
    }
}
