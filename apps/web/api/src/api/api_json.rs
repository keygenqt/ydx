use axum::Json;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};

use crate::api::api_response::ApiResponse;

/// Application custom Json
#[derive(FromRequest)]
#[from_request(via(Json), rejection(ApiResponse))]
pub struct ApiJson<T>(pub T);

impl<T> IntoResponse for ApiJson<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
