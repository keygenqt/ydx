use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

use crate::api::api_json::ApiJson;

/// Application common response
#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

/// Impl axum response
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = (status, ApiJson(self.clone())).into_response();
        response.extensions_mut().insert(Arc::new(self.clone()));
        response
    }
}

/// Impl response states
impl ApiResponse {
    pub fn new(status: StatusCode, message: &str, data: Option<serde_json::Value>) -> Self {
        Self {
            code: status.as_u16(),
            message: message.to_string(),
            data,
        }
    }

    pub fn new_200() -> Self {
        Self::new(StatusCode::OK, "Success", None)
    }

    pub fn new_400() -> Self {
        Self::new(StatusCode::BAD_REQUEST, "Bad request", None)
    }

    pub fn new_404() -> Self {
        Self::new(StatusCode::NOT_FOUND, "Not found", None)
    }

    pub fn new_403() -> Self {
        Self::new(StatusCode::FORBIDDEN, "Access denied", None)
    }

    pub fn new_409() -> Self {
        Self::new(StatusCode::CONFLICT, "Conflict", None)
    }

    pub fn new_422() -> Self {
        Self::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "Unprocessable content",
            None,
        )
    }

    pub fn new_500() -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
            None,
        )
    }
}

/// Impl response builders
impl ApiResponse {
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Impl response errors
impl ApiResponse {
    pub fn serde_json_error(e: serde_json::Error) -> Self {
        Self::new_500()
            .with_message("JSON processing error")
            .with_data(json!({
                "error": "json_serialization",
                "detail": e.to_string()
            }))
    }
}
