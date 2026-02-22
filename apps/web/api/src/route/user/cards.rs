use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::{models::card_model::CardModel, services::card_service::CardService},
};

/// GET - find by ID
/// /api/cards/{id}
pub async fn get_by_id(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<CardModel>, ApiResponse> {
    let result = CardService::find_by_id(&mut conn, id).await?;
    Ok(ApiJson(result))
}
