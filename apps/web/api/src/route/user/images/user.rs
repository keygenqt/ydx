use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_upload_images::ApiUserIcon},
    ctx::ctx_user::CtxUser,
    domain::image::response::ImageResponse,
};

/// POST - /api/images/user/{id}
pub async fn post(
    Path(id): Path<i32>,
    CtxUser(user): CtxUser,
    ApiUserIcon(image): ApiUserIcon,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    // check user id
    if id != user.user_id {
        return Err(ApiResponse::new_403().with_message("Access denied"));
    }
    // save image
    image.save().await?;
    // result
    Ok(ApiJson(ImageResponse::from_model(&image)?))
}
