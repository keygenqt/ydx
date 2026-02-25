use std::path::PathBuf;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};
use tokio::fs;

use crate::{
    api::api_response::ApiResponse,
    domain::image::entity::{EntityTypeImage, ImageEntity, NewImageEntity},
    utils::{self, constants},
};

/// Model entry [ImageEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageModel {
    pub id: Option<i32>,
    pub entity_type: EntityTypeImage,
    pub entity_id: i32,
    pub path: String,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl ImageModel {
    /// Create new [ImageModel]
    pub fn new(entity_type: EntityTypeImage, entity_id: i32, path: String) -> Self {
        Self {
            id: None,
            entity_type: entity_type.clone(),
            entity_id,
            path: path.clone(),
            created_at: utils::local_time::chrono_time_now(),
        }
    }

    /// Create model from [ImageEntity]
    pub fn from_entity(entity: &ImageEntity) -> Self {
        Self {
            id: Some(entity.id),
            entity_type: entity.entity_type,
            entity_id: entity.entity_id,
            path: entity.path.clone(),
            created_at: entity.created_at,
        }
    }

    /// Create new [NewImageEntity]
    pub fn to_new_entity(&self) -> NewImageEntity {
        NewImageEntity {
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            path: self.path.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ImageRawModel {
    pub entity_type: EntityTypeImage,
    filename: String,
    data: Vec<u8>,
}

impl ImageRawModel {
    /// Create new [ImageRawModel]
    pub fn new(entity_type: EntityTypeImage, filename: String, data: Vec<u8>) -> ImageRawModel {
        ImageRawModel {
            entity_type,
            filename,
            data,
        }
    }

    /// Save image
    pub async fn save(&self, workdir: &PathBuf) -> Result<String, ApiResponse> {
        let uploads = PathBuf::from(constants::UPLOADS_PATH);
        let file_path = match self.entity_type {
            EntityTypeImage::Card => uploads.join(constants::UPLOADS_IMAGES_PATH),
            _ => uploads.join(constants::UPLOADS_ICONS_PATH),
        }
        .join(&self.filename);
        // save file
        fs::write(&workdir.join(&file_path), &self.data)
            .await
            .map_err(|e| {
                ApiResponse::new_500().with_message(&format!("Failed to save file: {}", e))
            })?;
        // result
        Ok(format!("/{}", file_path.to_string_lossy()))
    }
}
