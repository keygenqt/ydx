use axum::extract::Path;

use crate::{
    api::{api_json::ApiJson, api_response::ApiResponse, api_upload_images::ApiSetIcon},
    ctx::{ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::{image::response::ImageResponse, set::service::SetService},
};

/// POST - /api/images/set/{id}
pub async fn post(
    Path(id): Path<i32>,
    CtxUser(_): CtxUser,
    CtxDatabase(mut conn): CtxDatabase,
    ApiSetIcon(image): ApiSetIcon,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    SetService::find_by_id(&mut conn, id)
        .await
        .map_err(|_| ApiResponse::new_400().with_message(&format!("Invalid set id: {}", id)))?;
    // save image
    image.save().await?;
    // result
    Ok(ApiJson(ImageResponse::from_model(&image)?))
}
