use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::PrivateCookieJar;

use crate::{api::api_response::ApiResponse, ctx::AppContext};

pub struct CtxJar(pub PrivateCookieJar);

/// Impl PrivateCookieJar
impl FromRequestParts<AppContext> for CtxJar {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let key = (*state.jkey).clone();
        let jar = PrivateCookieJar::from_request_parts(parts, &key)
            .await
            .map_err(|e| ApiResponse::new_400().with_message(&e.to_string()))?;
        Ok(CtxJar(jar))
    }
}
