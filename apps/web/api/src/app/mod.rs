pub mod app_config;
pub mod app_info;
pub mod app_json;
pub mod app_response;
pub mod app_state;
pub mod app_tracing;
pub mod app_user;

use std::fs;

use crate::{app::app_config::AppConfig, utils::constants};

/// Configure uploads
pub fn configure_uploads(config: &AppConfig) {
    let uploads = config.deployment.workdir.join(constants::UPLOADS_PATH);
    let images = uploads.join(constants::UPLOADS_IMAGES_PATH);
    let icons = uploads.join(constants::UPLOADS_ICONS_PATH);
    if !uploads.exists() {
        fs::create_dir_all(&uploads).expect(&format!(
            "Failed to create directory, please check access for create: {}",
            uploads.to_string_lossy()
        ));
    }
    if !images.exists() {
        fs::create_dir_all(&images).expect(&format!(
            "Failed to create directory, please check access for create: {}",
            images.to_string_lossy()
        ));
    }
    if !icons.exists() {
        fs::create_dir_all(&icons).expect(&format!(
            "Failed to create directory, please check access for create: {}",
            icons.to_string_lossy()
        ));
    }
}
