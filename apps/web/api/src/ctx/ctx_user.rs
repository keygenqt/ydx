use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8::PooledConnection;

use crate::api::api_response::ApiResponse;
use crate::app::app_user::AppUser;
use crate::ctx::AppContext;
use crate::ctx::ctx_cookie::CtxJar;
use crate::ctx::ctx_database::CtxDatabase;
use crate::ctx::ctx_uuid::CtxUuid;
use crate::db::services::token_service::TokenService;
use crate::utils::{self, constants};

/// Impl for route UserModel
#[allow(dead_code)]
pub struct CtxUser(pub AppUser);

impl FromRequestParts<AppContext> for CtxUser {
    type Rejection = ApiResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppContext,
    ) -> Result<Self, Self::Rejection> {
        let state = AppContext::from_ref(state);
        // get uuid
        let CtxUuid(uuid) = CtxUuid::from_request_parts(parts, &state).await?;
        // get connect
        let CtxDatabase(mut conn) = CtxDatabase::from_request_parts(parts, &state).await?;
        // get header bearer
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));
        // check session user
        let app_user = if let Some(token) = auth_header {
            // get app_user
            let app_user = jsonwebtoken::decode::<AppUser>(
                token,
                &jsonwebtoken::DecodingKey::from_secret(state.conf.server.secret.as_bytes()),
                &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256),
            )
            .map_err(|_| ApiResponse::new_401().with_message("Invalid token"))?
            .claims;
            // check token
            Self::check_token(&mut conn, &app_user, &uuid).await?;
            // result
            app_user
        } else {
            // get secret cookies
            let CtxJar(jar) = CtxJar::from_request_parts(parts, &state).await?;
            // get cookie
            let cookie = jar
                .get(constants::SESSION_NAME)
                .ok_or_else(|| ApiResponse::new_401().with_message("No session cookie"))?;
            // get user app_user
            let app_user = serde_json::from_str::<AppUser>(cookie.value())
                .map_err(|_| ApiResponse::new_401().with_message("Error parse cookie"))?;
            // check token
            Self::check_token(&mut conn, &app_user, &uuid).await?;
            // result
            app_user
        };
        // result
        Ok(CtxUser(app_user))
    }
}

impl CtxUser {
    async fn check_token(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        app_user: &AppUser,
        uuid: &String,
    ) -> Result<(), ApiResponse> {
        // get token
        let token = TokenService::find_by_id_uuid(conn, app_user.user_id, &uuid)
            .await
            .map_err(|_| ApiResponse::new_403())?;
        // check device uuid
        if app_user.device_uuid != *uuid {
            TokenService::clear_by_id(conn, app_user.user_id).await?;
            Err(ApiResponse::new_403().with_message("Token device mismatch"))?;
        }
        // check token expires
        if token.expires_at < utils::local_time::chrono_time_now() {
            TokenService::clear_by_id_uuid(conn, app_user.user_id, &uuid).await?;
            Err(ApiResponse::new_403())?;
        }
        Ok(())
    }
}
