use serde::Serialize;

use crate::{api::api_response::ApiResponse, domain::image::{entity::EntityTypeImage, model::ImageModel}};

/// Model response [ImageModel]
#[derive(Serialize)]
pub struct ImageResponse {
    entity_id: i32,
    entity_type: EntityTypeImage,
    path: String,
}

/// Impl response
impl ImageResponse {
    pub fn from_model(model: &ImageModel) -> Result<ImageResponse, ApiResponse> {
        if model.id.is_none() {
            Err(ApiResponse::new_500())?;
        }
        Ok(Self {
            entity_id: model.entity_id,
            entity_type: model.entity_type.clone(),
            path: model.path.clone(),
        })
    }
}
