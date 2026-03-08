#[cfg(any(feature = "ffi", feature = "sffi"))]
pub static RUNTIME: once_cell::sync::Lazy<tokio::runtime::Runtime> =
    once_cell::sync::Lazy::new(|| {
        tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime")
    });

#[cfg(feature = "wasm")]
#[macro_export]
macro_rules! wasm_response {
    ($future:expr) => {
        wasm_bindgen_futures::future_to_promise(async move {
            match $future.await {
                Ok(s) => Ok(wasm_bindgen::JsValue::from_str(&s)),
                Err(e) => Err(wasm_bindgen::JsValue::from_str(&e.to_string())),
            }
        })
    };
}

#[cfg(feature = "sffi")]
#[macro_export]
macro_rules! sffi_response {
    ($future:expr) => {{
        use std::ffi::CString;
        match ydx_core::service::macros::RUNTIME.block_on($future) {
            Ok(s) => match CString::new(s) {
                Ok(c_string) => {
                    safer_ffi::prelude::char_p::Box::try_from(c_string).unwrap_or_else(|_| {
                        safer_ffi::prelude::char_p::Box::try_from(
                            CString::new("Error: Conversion failed").unwrap(),
                        )
                        .unwrap()
                    })
                }
                Err(_) => safer_ffi::prelude::char_p::Box::try_from(
                    CString::new("Error: Invalid UTF-8").unwrap(),
                )
                .unwrap(),
            },
            Err(e) => {
                let error_msg = format!("Error: {}", e);
                match CString::new(error_msg) {
                    Ok(c_string) => safer_ffi::prelude::char_p::Box::try_from(c_string)
                        .unwrap_or_else(|_| {
                            safer_ffi::prelude::char_p::Box::try_from(
                                CString::new("Error: Unknown error").unwrap(),
                            )
                            .unwrap()
                        }),
                    Err(_) => safer_ffi::prelude::char_p::Box::try_from(
                        CString::new("Error: Invalid error message").unwrap(),
                    )
                    .unwrap(),
                }
            }
        }
    }};
}

#[cfg(feature = "ffi")]
#[macro_export]
macro_rules! ffi_response {
    ($future:expr) => {{
        match ydx_core::service::macros::RUNTIME.block_on($future) {
            Ok(s) => s,
            Err(e) => format!("Error: {}", e),
        }
    }};
}
