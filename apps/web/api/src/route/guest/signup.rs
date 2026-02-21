use std::sync::LazyLock;

use axum_extra::extract::PrivateCookieJar;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

use crate::{
    app::{
        app_json::AppJson, app_response::AppResponse, app_state::Db, app_validate::ValidatedForm,
    },
    db::models::token_model::TokenModel,
};

static PASSWORD_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[A-Z]+[a-z]+[0-9]+|[A-Z]+[0-9]+[a-z]+|[a-z]+[A-Z]+[0-9]+|[a-z]+[0-9]+[A-Z]+|[0-9]+[A-Z]+[a-z]+|[0-9]+[a-z]+[A-Z]+").unwrap()
});

#[derive(Deserialize, Debug, Validate)]
pub struct RegUserParams {
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
pub async fn reg_user(
    jar: PrivateCookieJar,
    Db(conn): Db,
    ValidatedForm(params): ValidatedForm<RegUserParams>,
) -> Result<AppJson<TokenModel>, AppResponse> {

    // @todo - create user

    // result
    Err(AppResponse::new_400())
}
