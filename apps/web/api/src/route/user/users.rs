use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse}, ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser}, db::{models::user_model::UserModel, services::user_service::UserService}
};

/// GET - find by ID
/// /api/users/{id}
pub async fn get_by_id(
    CtxUser(user): CtxUser,
    CtxDatabase(conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<UserModel>, ApiResponse> {
    if user.is_some() {
        let result = UserService::find_by_id(conn, id).await?;
        Ok(ApiJson(result))
    } else {
        Err(ApiResponse::new_403())
    }
}
