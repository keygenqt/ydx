use axum::extract::Path;

use crate::{
    app::{
        app_json::AppJson,
        app_response::AppResponse,
        app_state::{AuthModel, Db},
    },
    db::{models::set_model::SetModel, services::set_service::SetService},
};

/// GET - find by ID
/// /api/sets/{id}
pub async fn get_by_id(
    AuthModel(user): AuthModel,
    Db(conn): Db,
    Path(id): Path<i32>,
) -> Result<AppJson<SetModel>, AppResponse> {
    if user.is_some() {
        let result = SetService::find_by_id(conn, id).await?;
        Ok(AppJson(result))
    } else {
        Err(AppResponse::new_403())
    }
}
