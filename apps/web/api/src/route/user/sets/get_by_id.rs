use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::set::{response::SetResponse, service::SetService},
};

/// GET - find by ID
/// /api/sets/{id}
pub async fn get(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<SetResponse>, ApiResponse> {
    let model = SetService::find_by_id(&mut conn, id).await?;
    Ok(ApiJson(SetResponse::from_model(&model)?))
}
