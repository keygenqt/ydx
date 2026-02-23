use serde::Deserialize;
use validator::Validate;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_validate_form::ApiValidatedForm},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser}, db::responses::set_response::SetResponse,
};

#[derive(Deserialize, Debug, Validate)]
pub struct CreateParams {
    user_id: i32,
    icon: Option<String>,
    #[validate(length(min = 1, max = 255, message = "Set name must be between 1 and 255 characters"))]
    name: String,
    #[validate(length(max = 500, message = "Description too long (max 500 chars)"))]
    description: Option<String>,
}

/// POST - Create set
/// /api/sets
pub async fn post(
    CtxUser(_): CtxUser,
    CtxDatabase(_): CtxDatabase,
    ApiValidatedForm(params): ApiValidatedForm<CreateParams>,
) -> Result<ApiJson<SetResponse>, ApiResponse> {
    Err(ApiResponse::new_500().with_message("@todo"))
}
