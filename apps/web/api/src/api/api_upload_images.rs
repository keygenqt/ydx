use axum::extract::{FromRequest, Request};
use axum_extra::extract::Multipart;
use image::imageops::FilterType;

use crate::api::api_response::ApiResponse;
use crate::domain::image::entity::EntityTypeImage;
use crate::domain::image::model::ImageRawModel;

pub trait ImageLimit {
    const MAX_SIZE: usize;
    const TARGET_SIZE: (u32, u32);
    const QUALITY: u8;
    const ENTRY_TYPE: EntityTypeImage;
}

pub struct UserIconLimit;
pub struct SetIconLimit;
pub struct CardImageLimit;

impl ImageLimit for UserIconLimit {
    const MAX_SIZE: usize = 5 * 1024 * 1024;
    const TARGET_SIZE: (u32, u32) = (512, 512);
    const QUALITY: u8 = 90;
    const ENTRY_TYPE: EntityTypeImage = EntityTypeImage::User;
}

impl ImageLimit for SetIconLimit {
    const MAX_SIZE: usize = 5 * 1024 * 1024;
    const TARGET_SIZE: (u32, u32) = (1200, 630);
    const QUALITY: u8 = 90;
    const ENTRY_TYPE: EntityTypeImage = EntityTypeImage::Set;
}

impl ImageLimit for CardImageLimit {
    const MAX_SIZE: usize = 10 * 1024 * 1024;
    const TARGET_SIZE: (u32, u32) = (1000, 1000);
    const QUALITY: u8 = 80;
    const ENTRY_TYPE: EntityTypeImage = EntityTypeImage::Card;
}

macro_rules! define_image_extractor {
    ($name:ident, $limit:ty) => {
        #[derive(Debug)]
        pub struct $name(pub ImageRawModel);

        impl<S> FromRequest<S> for $name
        where
            S: Send + Sync,
        {
            type Rejection = ApiResponse;

            async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
                let mut multipart = Multipart::from_request(req, state)
                    .await
                    .map_err(|e| ApiResponse::new_400().with_message(&e.to_string()))?;

                while let Some(field) = multipart
                    .next_field()
                    .await
                    .map_err(|e| ApiResponse::new_400().with_message(&e.to_string()))?
                {
                    let filename = match field.file_name() {
                        Some(name) => name.to_string(),
                        None => continue,
                    };

                    let content_type = field
                        .content_type()
                        .map(|ct| ct.to_string())
                        .unwrap_or_else(|| "application/octet-stream".to_string());

                    if !content_type.starts_with("image/") {
                        return Err(ApiResponse::new_400()
                            .with_message(&format!("File {} is not an image", filename)));
                    }

                    let data = match field.bytes().await {
                        Ok(bytes) => bytes.to_vec(),
                        Err(e) => {
                            return Err(ApiResponse::new_400()
                                .with_message(&format!("Failed to read file {}: {}", filename, e)));
                        }
                    };

                    if data.len() > <$limit>::MAX_SIZE {
                        return Err(ApiResponse::new_400().with_message(&format!(
                            "File too large (max {}MB)",
                            <$limit>::MAX_SIZE / 1024 / 1024
                        )));
                    }

                    let processed_data = optimize_image::<$limit>(&data)?;

                    let new_filename = format!(
                        "{}_{}x{}.webp",
                        chrono::Utc::now().timestamp(),
                        <$limit>::TARGET_SIZE.0,
                        <$limit>::TARGET_SIZE.1,
                    );

                    return Ok($name(ImageRawModel::new(
                        <$limit>::ENTRY_TYPE,
                        new_filename,
                        processed_data,
                    )));
                }

                Err(ApiResponse::new_400().with_message("No file uploaded"))
            }
        }
    };
}

define_image_extractor!(ApiUserIcon, UserIconLimit);
define_image_extractor!(ApiSetIcon, SetIconLimit);
define_image_extractor!(ApiCardImage, CardImageLimit);

fn optimize_image<L: ImageLimit>(data: &[u8]) -> Result<Vec<u8>, ApiResponse> {
    let img = match image::load_from_memory(data) {
        Ok(img) => img,
        Err(e) => return Err(ApiResponse::new_400().with_message(&format!("Invalid image: {}", e))),
    };

    let (target_w, target_h) = L::TARGET_SIZE;
    let resized = img.resize_to_fill(target_w, target_h, FilterType::Lanczos3);

    let encoder = webp::Encoder::from_image(&resized)
        .map_err(|_| ApiResponse::new_500().with_message("Failed to encode WebP"))?;

    Ok(encoder.encode(L::QUALITY as f32).to_vec())
}
