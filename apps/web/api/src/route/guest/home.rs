use crate::app::{app_info::AppInfo, app_json::AppJson};

/// GET - / & /api
pub async fn app_info() -> AppJson<AppInfo> {
    AppJson(AppInfo::default())
}
