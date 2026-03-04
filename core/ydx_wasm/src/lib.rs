use crate::response::JsResponse;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use ydx_core::service::client::HttpClient;

mod response;

/// Check auth user
#[wasm_bindgen]
pub fn ydx_wasm_request_init(uuid: String) {
    HttpClient::init("".to_string(), uuid);
}

/// Check auth user
/// GET
#[wasm_bindgen]
pub fn ydx_wasm_request_signcheck() -> Promise {
    future_to_promise(async move { HttpClient::instance().signcheck().await.to_js_response() })
}

/// Signin user
/// POST
#[wasm_bindgen]
pub fn ydx_wasm_request_signin(email: String, password: String) -> Promise {
    future_to_promise(async move {
        HttpClient::instance()
            .signin(email, password)
            .await
            .to_js_response()
    })
}

/// Signout user
/// DELETE
#[wasm_bindgen]
pub fn ydx_wasm_request_signout() -> Promise {
    future_to_promise(async move { HttpClient::instance().signout().await.to_js_response() })
}
