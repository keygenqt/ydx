use axum::extract::Path;

use crate::{
    app::{
        app_json::AppJson,
        app_response::AppResponse,
        app_state::{AuthModel, Db},
    },
    db::{models::user_model::UserModel, services::user_service::UserService},
};

/// GET - find by ID
/// /api/users/{id}
pub async fn get_by_id(
    AuthModel(user): AuthModel,
    Db(conn): Db,
    Path(id): Path<i32>,
) -> Result<AppJson<UserModel>, AppResponse> {
    if user.is_some() {
        let result = UserService::find_by_id(conn, id).await?;
        Ok(AppJson(result))
    } else {
        Err(AppResponse::new_403())
    }
}
