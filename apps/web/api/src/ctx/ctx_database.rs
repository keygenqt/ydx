use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;

use crate::api::api_response::ApiResponse;
use crate::db::db_handle_internal_error;
use crate::ctx::AppContext;

/// Impl for route AsyncPgConnection
pub struct CtxDatabase(pub bb8::PooledConnection<'static, AsyncPgConnection>);

impl FromRequestParts<AppContext> for CtxDatabase {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let state = AppContext::from_ref(state);
        let conn = state
            .pool
            .get_owned()
            .await
            .map_err(db_handle_internal_error)?;
        Ok(CtxDatabase(conn))
    }
}
