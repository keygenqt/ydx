use crate::app::{app_info::AppInfo, app_json::AppJson};

/// GET home page
/// / & /api
pub async fn index() -> AppJson<AppInfo> {
    AppJson(AppInfo::default())
}
