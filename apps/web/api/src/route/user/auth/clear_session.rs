use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};

use crate::{
    api::api_response::ApiResponse,
    ctx::{ctx_cookie::CtxJar, ctx_database::CtxDatabase, ctx_user::CtxUser, ctx_uuid::CtxUuid},
    domain::token::service::TokenService,
    utils::constants,
};

/// DELETE - /api/signout
pub async fn delete(
    CtxUser(user): CtxUser,
    CtxJar(jar): CtxJar,
    CtxUuid(uuid): CtxUuid,
    CtxDatabase(mut conn): CtxDatabase,
) -> Result<(PrivateCookieJar, ApiResponse), ApiResponse> {
    // clear token
    let _ = TokenService::clear_by_id_uuid(&mut conn, user.user_id, &uuid).await;
    // clear cookie
    let cookie = Cookie::build(constants::SESSION_NAME).path("/");
    Ok((jar.remove(cookie), ApiResponse::new_200()))
}
