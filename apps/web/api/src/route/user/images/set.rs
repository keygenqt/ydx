use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_upload_images::ApiSetIcon},
    ctx::ctx_user::CtxUser,
    domain::image::response::ImageResponse,
};

/// POST - /api/images/set/{id}
pub async fn post(
    CtxUser(_): CtxUser,
    ApiSetIcon(image): ApiSetIcon,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    // save image
    image.save().await?;
    // result
    Ok(ApiJson(ImageResponse::from_model(&image)?))
}
