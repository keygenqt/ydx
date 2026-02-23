use serde::Deserialize;
use validator::Validate;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser}, db::responses::card_response::CardResponse,
};

#[derive(Deserialize, Debug, Validate)]
pub struct CreateParams {
    set_id: i32,
    image: Option<String>,
    #[validate(length(min = 1, max = 500, message = "Term must be between 1 and 500 characters"))]
    term: String,
    #[validate(length(min = 1, max = 500, message = "Definition must be between 1 and 500 characters"))]
    definition: String,
}

/// POST - Create card of set
/// /api/cards
pub async fn post(
    CtxUser(_): CtxUser,
    CtxDatabase(_): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<CreateParams>,
) -> Result<ApiJson<CardResponse>, ApiResponse> {
    Err(ApiResponse::new_500().with_message("@todo"))
}
