use crate::{api::api_json::ApiJson, app::app_info::AppInfo};

/// GET - / & /api
pub async fn get() -> ApiJson<AppInfo> {
    ApiJson(AppInfo::default())
}
