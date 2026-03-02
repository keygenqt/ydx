use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use ydx_core::service::client::HttpClient;

/// Check auth user
#[wasm_bindgen]
pub fn ydx_wasm_request_signcheck(uuid: String) -> Promise {
    future_to_promise(async {
        match HttpClient::new(uuid).get_signcheck().await {
            Ok(result) => Ok(JsValue::from_str(&result)),
            Err(e) => Err(JsValue::from_str(&format!("Error: {}", e))),
        }
    })
}
