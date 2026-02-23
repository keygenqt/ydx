use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::models::card_model::CardModel,
};

/// POST - Create set
/// /api/sets
pub async fn post(
    CtxUser(_): CtxUser,
    CtxDatabase(_): CtxDatabase,
) -> Result<ApiJson<CardModel>, ApiResponse> {
    Err(ApiResponse::new_500().with_message("@todo"))
}
