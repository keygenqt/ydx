use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Key;
use std::sync::Arc;

use crate::api::api_response::ApiResponse;
use crate::app::app_user::AppUser;
use crate::ctx::AppContext;
use crate::db::db_handle_internal_error;
use crate::db::services::user_service::UserService;
use crate::utils::constants;

/// Impl for route UserModel
pub struct CtxUser(pub Arc<Option<AppUser>>);

impl FromRequestParts<AppContext> for CtxUser {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let mut state = AppContext::from_ref(state);
        // get secret cookies
        let key = (*state.jkey).clone();
        let jar = match PrivateCookieJar::<Key>::from_request_parts(parts, &key).await {
            Ok(jar) => jar,
            Err(err) => match err {},
        };
        // get auth model
        let user_auth = if let Some(data) = jar.get(constants::SESSION_NAME) {
            match serde_json::from_str::<AppUser>(data.value()) {
                Ok(model) => Some(model),
                Err(_) => None,
            }
        } else {
            None
        };
        // database check
        if let Some(user_auth) = user_auth.clone() {
            // get connect
            let conn = state
                .pool
                .get_owned()
                .await
                .map_err(db_handle_internal_error)?;
            // get or error
            let _ = UserService::find_by_id(conn, user_auth.id).await?;
        };
        // save
        state.auth = Arc::new(user_auth);
        // result
        Ok(CtxUser(state.auth))
    }
}
