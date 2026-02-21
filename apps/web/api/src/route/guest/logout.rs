use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};

use crate::{app::app_response::AppResponse, utils::constants};

/// DELETE - /api/logout
pub async fn clear_session(jar: PrivateCookieJar) -> (PrivateCookieJar, AppResponse) {
    let cookie = Cookie::build(constants::SESSION_NAME).path("/");
    (jar.remove(cookie), AppResponse::new_200())
}
