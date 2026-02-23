use std::sync::LazyLock;

use axum_extra::extract::PrivateCookieJar;
use regex::Regex;
use serde::Deserialize;
use serde_json::to_value;
use validator::Validate;

use crate::{
    api::{api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    app::app_user::AppUser,
    ctx::{
        ctx_config::CtxConfig, ctx_cookie::CtxJar, ctx_database::CtxDatabase, ctx_uuid::CtxUuid,
    },
    db::{
        models::user_model::UserModel,
        responses::token_response::TokenResponse,
        services::{token_service::TokenService, user_service::UserService},
    },
};

static PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[A-Z]+[a-z]+[0-9]+|[A-Z]+[0-9]+[a-z]+|[a-z]+[A-Z]+[0-9]+|[a-z]+[0-9]+[A-Z]+|[0-9]+[A-Z]+[a-z]+|[0-9]+[a-z]+[A-Z]+").unwrap()
});

#[derive(Deserialize, Debug, Validate)]
pub struct SignUpParams {
    #[validate(email(message = "Invalid email format"))]
    email: String,
    #[validate(length(
        min = 8,
        max = 16,
        message = "Password must be between 8 and 16 characters"
    ))]
    #[validate(regex(
        path = "PASSWORD_REGEX",
        message = "Password must contain at least one uppercase letter, one lowercase letter, and one number"
    ))]
    password: String,
}

/// POST - /api/signup
pub async fn post(
    CtxJar(jar): CtxJar,
    CtxUuid(uuid): CtxUuid,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<SignUpParams>,
) -> Result<(PrivateCookieJar, ApiResponse), ApiResponse> {
    // check exist email
    if let Ok(_) = UserService::find_by_email(&mut conn, &params.email).await {
        Err(ApiResponse::new_409().with_message("User with this email already exists"))?;
    }
    // create user in database
    let model_params = UserModel::new(&params.email, &params.password);
    let model_create = UserService::create(&mut conn, &model_params).await?;
    // create user token
    let user_id = model_create
        .id
        .ok_or_else(|| ApiResponse::new_500().with_message("User ID is required"))?;
    let secret = &config.server.secret;
    let model_token = TokenService::create(&mut conn, user_id, &uuid, &secret).await?;
    let model_token_response = TokenResponse::from_model(&model_token)?;
    let data_token = to_value(model_token_response).map_err(ApiResponse::serde_json_error)?;
    // create user session
    let cookie = AppUser::create_cookie(model_create.id.unwrap(), &uuid)?;
    let session = jar.add(cookie);
    // result
    Ok((session, ApiResponse::new_200().with_data(data_token)))
}
