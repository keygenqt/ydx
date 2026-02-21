use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::api::api_response::ApiResponse;
use crate::ctx::AppContext;

/// Impl for client device uuid
pub struct CtxUuid(pub String);

impl FromRequestParts<AppContext> for CtxUuid {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let uuid = parts
            .headers
            .get("x-device-uuid")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| ApiResponse::new_400().with_message("Missing X-Device-UUID header"))?
            .to_string();
        Ok(CtxUuid(uuid))
    }
}
