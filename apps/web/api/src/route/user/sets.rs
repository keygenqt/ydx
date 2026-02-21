use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::{models::set_model::SetModel, services::set_service::SetService},
};

/// GET - find by ID
/// /api/sets/{id}
pub async fn get_by_id(
    CtxUser(user): CtxUser,
    CtxDatabase(conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<SetModel>, ApiResponse> {
    if user.is_some() {
        let result = SetService::find_by_id(conn, id).await?;
        Ok(ApiJson(result))
    } else {
        Err(ApiResponse::new_403())
    }
}
