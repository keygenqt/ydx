use std::sync::Arc;

use diesel_async::{AsyncPgConnection, pooled_connection::bb8::PooledConnection};

use crate::{
    api::{
        api_json::ApiJson,
        api_response::ApiResponse,
        api_upload_images::{ApiCardImage, ApiSetIcon, ApiUserIcon},
    },
    app::{app_config::AppConfig, app_user::AppUser},
    ctx::{ctx_config::CtxConfig, ctx_database::CtxDatabase, ctx_user::CtxUser},
    domain::image::{
        model::{ImageModel, ImageRawModel},
        response::ImageResponse,
        service::ImageService,
    },
};

/// POST - /api/images/user
pub async fn post_user(
    CtxUser(user): CtxUser,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiUserIcon(image): ApiUserIcon,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    upload_image(user, config, image, &mut conn).await
}

/// POST - /api/images/set
pub async fn post_set(
    CtxUser(user): CtxUser,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiSetIcon(image): ApiSetIcon,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    upload_image(user, config, image, &mut conn).await
}

/// POST - /api/images/card
pub async fn post_card(
    CtxUser(user): CtxUser,
    CtxConfig(config): CtxConfig,
    CtxDatabase(mut conn): CtxDatabase,
    ApiCardImage(image): ApiCardImage,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    upload_image(user, config, image, &mut conn).await
}

/// Upload image to filesystem and create database record
async fn upload_image(
    user: AppUser,
    config: Arc<AppConfig>,
    image: ImageRawModel,
    conn: &mut PooledConnection<'static, AsyncPgConnection>,
) -> Result<ApiJson<ImageResponse>, ApiResponse> {
    let path = image.save(&config.deployment.workdir).await?;
    let model = ImageModel::new(image.entity_type.clone(), user.user_id, path);
    let model = ImageService::create(conn, &model).await?;
    Ok(ApiJson(ImageResponse::from_model(&model)?))
}
