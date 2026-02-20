use clap::builder::Styles;
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::utils::constants;

/// AppInfo response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppInfo {
    code: u16,
    pub version: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            code: 200,
            version: constants::APP_VERSION.to_string(),
        }
    }
}

pub fn about() -> String {
    format!(
        "{} - Axum REST API Demo Application.",
        "Rust Axum".bright_green().bold()
    )
}

pub fn styles() -> Styles {
    Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID)
}
