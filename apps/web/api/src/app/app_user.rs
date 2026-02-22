use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};

use crate::{
    api::api_response::ApiResponse,
    utils::{constants, local_time},
};

/// User data session
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppUser {
    pub user_id: i32,
    pub device_uuid: String,
}

impl AppUser {
    pub fn create_cookie(
        user_id: i32,
        device_uuid: String,
    ) -> Result<Cookie<'static>, ApiResponse> {
        // get value string
        let model_json = serde_json::to_string(&AppUser {
            user_id,
            device_uuid: device_uuid,
        })
        .map_err(ApiResponse::serde_json_error)?;
        // create cookie session
        let mut token_hash = Cookie::new(constants::SESSION_NAME, model_json);
        token_hash.set_http_only(true);
        token_hash.set_path("/");
        token_hash.set_expires(local_time::cookie_time_expires());
        Ok(token_hash)
    }
}
