use ::safer_ffi::prelude::*;
use ydx_core::{service::client::HttpClient, sffi_response};

/// Init http client
#[ffi_export]
pub fn ydx_linux_request_init(base_url: char_p::Ref<'_>, uuid: char_p::Ref<'_>) {
    HttpClient::init(base_url.to_str(), uuid.to_str());
}

/// Check auth user
/// GET
#[ffi_export]
pub fn ydx_linux_request_signcheck(token: char_p::Ref<'_>) -> char_p::Box {
    sffi_response!(HttpClient::instance().signcheck(Some(token.to_str().to_string())))
}

/// Signin user
/// POST
#[ffi_export]
pub fn ydx_linux_request_signin(email: char_p::Ref<'_>, password: char_p::Ref<'_>) -> char_p::Box {
    sffi_response!(
        HttpClient::instance().signin(email.to_str().to_string(), password.to_str().to_string())
    )
}

/// Signup user
/// POST
#[ffi_export]
pub fn ydx_linux_request_signup(email: char_p::Ref<'_>, password: char_p::Ref<'_>) -> char_p::Box {
    sffi_response!(
        HttpClient::instance().signup(email.to_str().to_string(), password.to_str().to_string())
    )
}

/// Get user by id
/// GET
#[ffi_export]
pub fn ydx_linux_request_get_user(id: char_p::Ref<'_>, token: char_p::Ref<'_>) -> char_p::Box {
    sffi_response!(HttpClient::instance().get_user(id.to_str().to_string(), Some(token.to_str().to_string())))
}

/// Signout user
/// DELETE
#[ffi_export]
pub fn ydx_linux_request_signout(token: char_p::Ref<'_>) -> char_p::Box {
    sffi_response!(HttpClient::instance().signout(Some(token.to_str().to_string())))
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("build/headers/ydx_linux.h")?
        .generate()
}
