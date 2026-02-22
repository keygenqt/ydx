use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::{models::set_model::SetModel, services::set_service::SetService},
};

/// GET - find by ID
/// /api/sets/{id}
pub async fn get_by_id(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<SetModel>, ApiResponse> {
    let result = SetService::find_by_id(&mut conn, id).await?;
    Ok(ApiJson(result))
}
