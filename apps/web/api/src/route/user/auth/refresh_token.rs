use axum_extra::extract::PrivateCookieJar;
use serde::Deserialize;
use serde_json::to_value;
use validator::Validate;

use crate::{
    api::{api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    app::app_user::AppUser,
    ctx::{
        ctx_config::CtxConfig, ctx_cookie::CtxJar, ctx_database::CtxDatabase, ctx_user::CtxUser,
        ctx_uuid::CtxUuid,
    },
    db::{responses::token_response::TokenResponse, services::token_service::TokenService},
    utils,
};

#[derive(Deserialize, Debug, Validate)]
pub struct TokenRefreshParams {
    #[validate(length(min = 64, max = 64, message = "Invalid refresh token format"))]
    refresh_token: String,
}

/// POST - /api/signrefresh
pub async fn post(
    CtxUser(ctx_user): CtxUser,
    CtxJar(jar): CtxJar,
    CtxUuid(uuid): CtxUuid,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<TokenRefreshParams>,
) -> Result<(PrivateCookieJar, ApiResponse), ApiResponse> {
    // find user device token
    let token = TokenService::find_by_id_uuid(&mut conn, ctx_user.user_id, &uuid).await?;
    // argon2 validate token
    if !token.refresh_token_valid(&params.refresh_token) {
        Err(ApiResponse::new_401().with_message("Invalid refresh_token"))?
    }
    // check time
    if token.expires_at < utils::local_time::chrono_time_now() {
        Err(ApiResponse::new_401().with_message("Invalid refresh_token"))?
    }
    // create user token
    let secret = &config.server.secret;
    let model_token = TokenService::create(&mut conn, ctx_user.user_id, &uuid, secret).await?;
    let model_token_response = TokenResponse::from_model(&model_token)?;
    let data_token = to_value(model_token_response).map_err(ApiResponse::serde_json_error)?;
    // create user session
    let cookie = AppUser::create_cookie(ctx_user.user_id, &uuid)?;
    let session = jar.add(cookie);
    // Ignore cleanup errors:
    // - New token already created
    // - Old token is one-time use only → already invalid
    // - Cron will clean up expired tokens
    // - User shouldn't see 500 for background cleanup issues
    let _ = TokenService::clear_by_id_uuid(&mut conn, ctx_user.user_id, &uuid).await;
    // result
    Ok((session, ApiResponse::new_200().with_data(data_token)))
}
