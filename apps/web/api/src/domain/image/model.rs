use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use tokio::fs;
use uuid::Uuid;

use crate::{api::api_response::ApiResponse, app::app_config::AppConfig, utils::constants};

pub trait ImageType {
    const TARGET_SIZE: (u32, u32);
    const QUALITY: u8;
    const PATH: &'static str;
}

pub struct UserIcon;
pub struct SetIcon;
pub struct CardImage;

impl ImageType for UserIcon {
    const TARGET_SIZE: (u32, u32) = (512, 512);
    const QUALITY: u8 = 90;
    const PATH: &'static str = "users";
}

impl ImageType for SetIcon {
    const TARGET_SIZE: (u32, u32) = (1200, 630);
    const QUALITY: u8 = 90;
    const PATH: &'static str = "sets";
}

impl ImageType for CardImage {
    const TARGET_SIZE: (u32, u32) = (1000, 1000);
    const QUALITY: u8 = 80;
    const PATH: &'static str = "cards";
}

pub struct ImageModel {
    pub mime: String,
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub path: String,
    full_path: String,
    data: Vec<u8>,
}

impl ImageModel {
    /// Create new [ImageModel]
    pub async fn new<T: ImageType>(config: &AppConfig, data: &[u8]) -> Result<Self, ApiResponse> {
        let mime = "image/webp".to_string();
        let (width, height) = T::TARGET_SIZE;
        let filename = format!(
            "{}_{}x{}.webp",
            Uuid::new_v4(),
            width,
            height
        );
        let path = format!("/{}/{}/{}", constants::UPLOADS_PATH, T::PATH, filename);
        let full_path = Self::get_full_path(&config.deployment.workdir, &path).await?;
        let image = Self::optimize_data::<T>(data)?;
        Ok(Self {
            mime,
            filename,
            width,
            height,
            path,
            full_path,
            data: image,
        })
    }

    /// Convert and crop image
    fn optimize_data<T: ImageType>(data: &[u8]) -> Result<Vec<u8>, ApiResponse> {
        // load image
        let img = match image::load_from_memory(data) {
            Ok(img) => img,
            Err(e) => {
                return Err(ApiResponse::new_400().with_message(&format!("Invalid image: {}", e)));
            }
        };
        // resize with crop
        let (width, height) = T::TARGET_SIZE;
        let resized = img.resize_to_fill(width, height, FilterType::Lanczos3);
        // convert to WebP
        let encoder = webp::Encoder::from_image(&resized)
            .map_err(|_| ApiResponse::new_500().with_message("Failed to encode WebP"))?;
        // result
        Ok(encoder.encode(T::QUALITY as f32).to_vec())
    }

    /// Save image
    pub async fn save(&self) -> Result<(), ApiResponse> {
        fs::write(&self.full_path, &self.data).await.map_err(|e| {
            ApiResponse::new_500().with_message(&format!("Failed to save file: {}", e))
        })
    }

    /// Create if not exist directory for upload
    async fn get_full_path(workdir: &PathBuf, path: &String) -> Result<String, ApiResponse> {
        let full_path = workdir.join(&path.trim_start_matches('/'));
        if let Some(parent) = Path::new(&full_path).parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                ApiResponse::new_500().with_message(&format!("Failed to create directory: {}", e))
            })?;
        }
        Ok(full_path.to_string_lossy().to_string())
    }
}
