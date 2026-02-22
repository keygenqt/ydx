use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::{responses::user_response::UserResponse, services::user_service::UserService},
};

/// GET - find by ID
/// /api/users/{id}
pub async fn get_by_id(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<UserResponse>, ApiResponse> {
    let model = UserService::find_by_id(&mut conn, id).await?;
    Ok(ApiJson(UserResponse::from_model(&model)?))
}
