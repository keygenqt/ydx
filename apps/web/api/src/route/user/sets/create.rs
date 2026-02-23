use serde::Deserialize;
use validator::Validate;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    db::{
        models::set_model::SetModel, responses::set_response::SetResponse,
        services::set_service::SetService,
    },
};

#[derive(Deserialize, Debug, Validate)]
pub struct CreateParams {
    icon: Option<String>,
    #[validate(length(
        min = 1,
        max = 255,
        message = "Set name must be between 1 and 255 characters"
    ))]
    name: String,
    #[validate(length(max = 500, message = "Description too long (max 500 chars)"))]
    description: Option<String>,
}

/// POST - Create set
/// /api/sets
pub async fn post(
    CtxUser(user): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<CreateParams>,
) -> Result<ApiJson<SetResponse>, ApiResponse> {
    // create user in database
    let model = SetModel::new(user.user_id, params.name, params.icon, params.description);
    let model = SetService::create(&mut conn, &model).await?;
    // result
    Ok(ApiJson(SetResponse::from_model(&model)?))
}
