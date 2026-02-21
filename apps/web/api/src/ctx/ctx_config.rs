use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use std::sync::Arc;

use crate::api::api_response::ApiResponse;
use crate::app::app_config::AppConfig;
use crate::ctx::AppContext;

/// Impl for route AppConfig
#[allow(unused)]
pub struct CtxConfig(pub Arc<AppConfig>);

impl FromRequestParts<AppContext> for CtxConfig {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let state = AppContext::from_ref(state);
        Ok(CtxConfig(state.conf))
    }
}
