use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use regex::Regex;
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationError};

use crate::api::{api_json::ApiJson, api_response::ApiResponse};

pub struct ApiValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ApiValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|e| {
            let response = ApiResponse::new_400().with_message(&e.to_string());
            (StatusCode::BAD_REQUEST, ApiJson(response)).into_response()
        })?;
        data.validate().map_err(|e| {
            let response = ApiResponse::new_422()
                .with_message("Error validate json")
                .with_data(serde_json::json!({
                    "errors": e.errors()
                }));
            (StatusCode::UNPROCESSABLE_ENTITY, Json(response)).into_response()
        })?;
        Ok(ApiValidatedJson(data))
    }
}

/// check file exist
pub fn validate_file_exists(file: &str) -> Result<(), ValidationError> {
    let full_path = std::path::Path::new(file.trim_start_matches('/'));
    if !full_path.exists() {
        return Err(ValidationError::new("file_not_found")
            .with_message("Icon file not found on server".into()));
    }
    Ok(())
}

/// check by regex
pub fn validate_image_path(file: &str) -> Result<(), ValidationError> {
    let regex =
        Regex::new(r"^/uploads/(users|sets|cards)/[0-9a-fA-F\-]{36}_\d+x\d+\.webp$").unwrap();
    if !regex.is_match(file) {
        return Err(
            ValidationError::new("file_error").with_message("Invalid image path format".into())
        );
    }
    Ok(())
}

/// check image by regex and exist
pub fn validate_image(file: &str) -> Result<(), ValidationError> {
    validate_image_path(file)?;
    validate_file_exists(file)?;
    Ok(())
}
