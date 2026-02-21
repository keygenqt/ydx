use axum_auth::AuthBasic;
use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};

use crate::{
    app::{app_response::AppResponse, app_state::Db, app_user::AppUser},
    db::services::user_service::UserService,
    utils::{constants, local_time},
};

/// POST - /api/auth
pub async fn basic(
    jar: PrivateCookieJar,
    Db(conn): Db,
    AuthBasic((id, password)): AuthBasic,
) -> Result<(PrivateCookieJar, AppResponse), AppResponse> {
    // find user
    let result = UserService::find_by_email(conn, id).await?;
    // check exist pass
    if password.is_none() {
        return Err(AppResponse::new_403());
    }
    // argon2 validate pass
    if !result.password_valid(&password.unwrap()) {
        return Err(AppResponse::new_403());
    }
    // get value string
    let model_json = serde_json::to_string(&AppUser {
        id: result.id.unwrap(),
        email: result.email,
    })
    .unwrap();
    // create cookie session
    let mut token_hash = Cookie::new(constants::SESSION_NAME, model_json);
    token_hash.set_http_only(true);
    token_hash.set_path("/");
    token_hash.set_expires(local_time::cookie_time_expires());

    // @todo token db

    // result
    Ok((jar.add(token_hash), AppResponse::new_200()))
}
