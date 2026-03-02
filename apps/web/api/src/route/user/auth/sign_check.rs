use crate::{api::api_response::ApiResponse, ctx::ctx_user::CtxUser};

/// GET - /api/signcheck
pub async fn get(
    CtxUser(_): CtxUser,
) -> Result<ApiResponse, ApiResponse> {
    Ok(ApiResponse::new_200())
}
