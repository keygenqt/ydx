use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_upload_images::ApiCardImage},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::{card::service::CardService, image::response::ImageResponse},
};

/// POST - /api/images/card/{id}
pub async fn post(
    Path(id): Path<i32>,
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    ApiCardImage(image): ApiCardImage,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    CardService::find_by_id(&mut conn, id)
        .await
        .map_err(|_| ApiResponse::new_400().with_message(&format!("Invalid card id: {}", id)))?;
    // save image
    image.save().await?;
    // result
    Ok(ApiJson(ImageResponse::from_model(&image)?))
}
