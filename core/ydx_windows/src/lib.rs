use ::safer_ffi::prelude::*;
use ydx_core::{service::client::HttpClient, sffi_response};

/// Init http client
#[ffi_export]
pub fn ydx_windows_request_init(base_url: &repr_c::String, uuid: &repr_c::String) {
    HttpClient::init(base_url, uuid);
}

/// Check auth user
/// GET
#[ffi_export]
pub fn ydx_windows_request_signcheck(token: &repr_c::String) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().signcheck(Some(token.to_string()))).to_str(),
    )
}

/// Signin user
/// POST
#[ffi_export]
pub fn ydx_windows_request_signin(
    email: &repr_c::String,
    password: &repr_c::String,
) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().signin(email.to_string(), password.to_string()))
            .to_str(),
    )
}

/// Signup user
/// POST
#[ffi_export]
pub fn ydx_windows_request_signup(
    email: &repr_c::String,
    password: &repr_c::String,
) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().signup(email.to_string(), password.to_string()))
            .to_str(),
    )
}

/// Get user by id
/// GET
#[ffi_export]
pub fn ydx_windows_request_get_user(
    id: &repr_c::String,
    token: &repr_c::String,
) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().get_user(id.to_string(), Some(token.to_string())))
            .to_str(),
    )
}

/// Signout user
/// DELETE
#[ffi_export]
pub fn ydx_windows_request_signout(token: &repr_c::String) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().signout(Some(token.to_string()))).to_str(),
    )
}

/// Free string allocated by Rust
#[ffi_export]
pub fn ydx_windows_free_string(ptr: *mut u8, len: usize, cap: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, cap);
        }
    }
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("build/headers/ydx_windows.h")?
        .generate()
}
