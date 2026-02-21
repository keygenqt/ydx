use axum_extra::extract::{PrivateCookieJar, cookie::Cookie};

use crate::{api::api_response::ApiResponse, utils::constants};

/// DELETE - /api/logout
pub async fn clear_session(jar: PrivateCookieJar) -> (PrivateCookieJar, ApiResponse) {
    let cookie = Cookie::build(constants::SESSION_NAME).path("/");
    (jar.remove(cookie), ApiResponse::new_200())
}
