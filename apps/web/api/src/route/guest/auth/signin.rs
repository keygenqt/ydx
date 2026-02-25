use axum_extra::extract::PrivateCookieJar;
use serde::Deserialize;
use serde_json::to_value;
use validator::Validate;

use crate::{
    api::{api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    app::app_user::AppUser,
    ctx::{
        ctx_config::CtxConfig, ctx_cookie::CtxJar, ctx_database::CtxDatabase, ctx_uuid::CtxUuid,
    },
    domain::{
        token::{model::TokenModel, response::TokenResponse, service::TokenService},
        user::service::UserService,
    },
};

#[derive(Deserialize, Debug, Validate)]
pub struct SignInParams {
    #[validate(email(message = "Invalid email format"))]
    email: String,
    #[validate(length(
        min = 8,
        max = 16,
        message = "Password must be between 8 and 16 characters"
    ))]
    password: String,
}

/// POST - /api/signin
pub async fn post(
    CtxJar(jar): CtxJar,
    CtxUuid(uuid): CtxUuid,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<SignInParams>,
) -> Result<(PrivateCookieJar, ApiResponse), ApiResponse> {
    // find user
    let model = UserService::find_by_email(&mut conn, &params.email)
        .await
        .map_err(|_| ApiResponse::new_401().with_message("Invalid email or password"))?;
    let user_id = model.id.unwrap();
    // argon2 validate pass
    if !model.password_valid(&params.password) {
        Err(ApiResponse::new_401().with_message("Invalid email or password"))?
    }
    // clear user token
    let _ = TokenService::clear_by_id_uuid(&mut conn, user_id, &uuid).await?;
    // create user token
    let model = TokenModel::new(user_id, uuid.clone(), config.server.secret.clone());
    let model = TokenService::create(&mut conn, &model).await?;
    let response = TokenResponse::from_model(&model)?;
    let data_token = to_value(response).map_err(ApiResponse::serde_json_error)?;
    // create user session
    let cookie = AppUser::create_cookie(user_id, &uuid)?;
    let session = jar.add(cookie);
    // result
    Ok((session, ApiResponse::new_200().with_data(data_token)))
}
