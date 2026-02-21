use axum_auth::AuthBasic;
use axum_extra::extract::PrivateCookieJar;

use crate::{
    api::api_response::ApiResponse,
    app::app_user::AppUser,
    ctx::{ctx_cookie::CtxJar, ctx_database::CtxDatabase, ctx_uuid::CtxUuid},
    db::services::user_service::UserService,
};

/// POST - /api/auth
pub async fn basic(
    CtxJar(jar): CtxJar,
    CtxUuid(uuid): CtxUuid,
    CtxDatabase(mut conn): CtxDatabase,
    AuthBasic((id, password)): AuthBasic,
) -> Result<(PrivateCookieJar, ApiResponse), ApiResponse> {
    // find user
    let model = UserService::find_by_email(&mut conn, &id).await?;
    // check exist pass
    if password.is_none() {
        return Err(ApiResponse::new_403());
    }
    // argon2 validate pass
    if !model.password_valid(&password.unwrap()) {
        return Err(ApiResponse::new_403());
    }
    // create user session
    let cookie = AppUser::create_cookie(model.id.unwrap(), uuid)?;
    let session = jar.add(cookie);
    // result
    Ok((session, ApiResponse::new_200()))
}
