use ::safer_ffi::prelude::*;
use ydx_core::{service::client::HttpClient, sffi_response};

/// Init http client
#[ffi_export]
pub fn ydx_macos_request_init(base_url: &repr_c::String, uuid: &repr_c::String) {
    HttpClient::init(base_url, uuid);
}

/// Check auth user
/// GET
#[ffi_export]
pub fn ydx_macos_request_signcheck() -> repr_c::String {
    repr_c::String::from(sffi_response!(HttpClient::instance().signcheck()).to_str())
}

/// Signin user
/// POST
#[ffi_export]
pub fn ydx_macos_request_signin(
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
pub fn ydx_macos_request_signup(
    email: &repr_c::String,
    password: &repr_c::String,
) -> repr_c::String {
    repr_c::String::from(
        sffi_response!(HttpClient::instance().signup(email.to_string(), password.to_string()))
            .to_str(),
    )
}

/// Signout user
/// DELETE
#[ffi_export]
pub fn ydx_macos_request_signout() -> repr_c::String {
    repr_c::String::from(sffi_response!(HttpClient::instance().signout()).to_str())
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("build/headers/ydx_macos.h")?
        .generate()
}
