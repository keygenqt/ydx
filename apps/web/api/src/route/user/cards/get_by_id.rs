use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::card::{response::CardResponse, service::CardService},
};

/// GET - find by ID
/// /api/cards/{id}
pub async fn get(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    Path(id): Path<i32>,
) -> Result<ApiJson<CardResponse>, ApiResponse> {
    let model = CardService::find_by_id(&mut conn, id).await?;
    Ok(ApiJson(CardResponse::from_model(&model)?))
}
