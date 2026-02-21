use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use std::sync::Arc;

use crate::api::api_response::ApiResponse;
use crate::app::app_user::AppUser;
use crate::ctx::AppContext;
use crate::ctx::ctx_cookie::CtxJar;
use crate::ctx::ctx_database::CtxDatabase;
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
        let CtxJar(jar) = CtxJar::from_request_parts(parts, &state).await?;
        // get auth model
        let app_user = if let Some(data) = jar.get(constants::SESSION_NAME) {
            match serde_json::from_str::<AppUser>(data.value()) {
                Ok(model) => Some(model),
                Err(_) => None,
            }
        } else {
            None
        };
        // database check
        if let Some(app_user) = app_user.clone() {
            // get connect
            let CtxDatabase(mut conn) = CtxDatabase::from_request_parts(parts, &state).await?;
            // get or error
            let _ = UserService::find_by_id(&mut conn, app_user.user_id).await?;
            // @todo - check tokens
        };
        // save
        state.auth = Arc::new(app_user);
        // result
        Ok(CtxUser(state.auth))
    }
}
