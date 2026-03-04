use anyhow::Result;
use wasm_bindgen::prelude::*;

pub trait JsResponse {
    fn to_js_response(self) -> Result<JsValue, JsValue>;
}

impl JsResponse for Result<String> {
    fn to_js_response(self) -> Result<JsValue, JsValue> {
        self.map(|s| JsValue::from_str(s.as_str()))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
