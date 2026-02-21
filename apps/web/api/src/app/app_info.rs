use colored::Colorize;
use serde::Serialize;

use crate::utils::constants;

/// AppInfo data
#[derive(Serialize)]
pub struct AppInfo {
    pub code: u16,
    pub version: String,
    #[serde(skip_serializing)]
    pub description: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            code: 200,
            version: constants::APP_VERSION.to_string(),
            description: format!(
                "{} - Axum REST API Demo Application.",
                "Rust Axum".bright_green().bold()
            ),
        }
    }
}
