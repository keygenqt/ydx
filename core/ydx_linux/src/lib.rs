use ::safer_ffi::prelude::*;
use ydx_core::{sffi_response, service::client::HttpClient};

/// Init http client
#[ffi_export]
pub fn ydx_linux_request_init(base_url: char_p::Ref<'_>, uuid: char_p::Ref<'_>) {
    HttpClient::init(base_url.to_str(), uuid.to_str());
}

/// Check auth user
/// GET
#[ffi_export]
pub fn ydx_linux_request_signcheck() -> char_p::Box {
    sffi_response!(HttpClient::instance().signcheck())
}

/// Signin user
/// POST
#[ffi_export]
pub fn ydx_linux_request_signin(
    email: char_p::Ref<'_>,
    password: char_p::Ref<'_>
) -> char_p::Box {
    sffi_response!(HttpClient::instance().signin(
        email.to_str().to_string(),
        password.to_str().to_string()
    ))
}

/// Signup user
/// POST
#[ffi_export]
pub fn ydx_linux_request_signup(
    email: char_p::Ref<'_>,
    password: char_p::Ref<'_>
) -> char_p::Box {
    sffi_response!(HttpClient::instance().signup(
        email.to_str().to_string(),
        password.to_str().to_string()
    ))
}

/// Signout user
/// DELETE
#[ffi_export]
pub fn ydx_linux_request_signout() -> char_p::Box {
    sffi_response!(HttpClient::instance().signout())
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("build/headers/ydx_linux.h")?
        .generate()
}
