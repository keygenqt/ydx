use serde::Deserialize;
use validator::Validate;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::{
        card::{model::CardModel, response::CardResponse, service::CardService},
        set::service::SetService,
    },
};

#[derive(Deserialize, Debug, Validate)]
pub struct CreateParams {
    set_id: i32,
    image: Option<String>,
    #[validate(length(
        min = 1,
        max = 500,
        message = "Term must be between 1 and 500 characters"
    ))]
    term: String,
    #[validate(length(
        min = 1,
        max = 500,
        message = "Definition must be between 1 and 500 characters"
    ))]
    definition: String,
}

/// POST - Create card of set
/// /api/cards
pub async fn post(
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<CreateParams>,
) -> Result<ApiJson<CardResponse>, ApiResponse> {
    // check exist set
    let _ = SetService::find_by_id(&mut conn, params.set_id).await?;
    // create user in database
    let model = CardModel::new(params.set_id, params.term, params.definition, params.image);
    let model = CardService::create(&mut conn, &model).await?;
    // result
    Ok(ApiJson(CardResponse::from_model(&model)?))
}
