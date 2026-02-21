use axum::{
    Json,
    extract::{Form, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::api::{api_json::ApiJson, api_response::ApiResponse};

pub struct ApiValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ApiValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data) = Form::<T>::from_request(req, state).await.map_err(|e| {
            let response = ApiResponse::new_400().with_message(&e.to_string());
            (StatusCode::BAD_REQUEST, ApiJson(response)).into_response()
        })?;
        data.validate().map_err(|e| {
            let response = ApiResponse::new_422()
                .with_message("Error validate form")
                .with_data(serde_json::json!({
                    "errors": e.errors()
                }));
            (StatusCode::UNPROCESSABLE_ENTITY, Json(response)).into_response()
        })?;
        Ok(ApiValidatedForm(data))
    }
}
