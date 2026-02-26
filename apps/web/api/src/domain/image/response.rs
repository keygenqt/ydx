use serde::Serialize;

use crate::{api::api_response::ApiResponse, domain::image::model::ImageModel};

/// Model response [ImageModel]
#[derive(Serialize)]
pub struct ImageResponse {
    pub mime: String,
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub path: String,
}

/// Impl response
impl ImageResponse {
    pub fn from_model(model: &ImageModel) -> Result<ImageResponse, ApiResponse> {
        Ok(Self {
            mime: model.mime.clone(),
            filename: model.filename.clone(),
            width: model.width,
            height: model.height,
            path: model.path.clone(),
        })
    }
}
