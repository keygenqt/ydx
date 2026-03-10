use js_sys::Promise;
use wasm_bindgen::prelude::*;
use ydx_core::service::client::HttpClient;
use ydx_core::wasm_response;

/// Init http client
#[wasm_bindgen]
pub fn ydx_wasm_request_init(uuid: String) {
    HttpClient::init("", &uuid);
}

/// Check auth user
/// GET
#[wasm_bindgen]
pub fn ydx_wasm_request_signcheck() -> Promise {
    wasm_response!(HttpClient::instance().signcheck(None))
}

/// Signin user
/// POST
#[wasm_bindgen]
pub fn ydx_wasm_request_signin(email: String, password: String) -> Promise {
    wasm_response!(HttpClient::instance().signin(email, password))
}

/// Signup user
/// POST
#[wasm_bindgen]
pub fn ydx_wasm_request_signup(email: String, password: String) -> Promise {
    wasm_response!(HttpClient::instance().signup(email, password))
}

/// Signout user
/// DELETE
#[wasm_bindgen]
pub fn ydx_wasm_request_signout() -> Promise {
    wasm_response!(HttpClient::instance().signout(None))
}
