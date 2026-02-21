use axum::{
    Json,
    extract::{Form, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::app::{app_json::AppJson, app_response::AppResponse};

pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(data) = Form::<T>::from_request(req, state).await.map_err(|e| {
            let response = AppResponse::new_400().with_message(&e.to_string());
            (StatusCode::BAD_REQUEST, AppJson(response)).into_response()
        })?;
        data.validate().map_err(|e| {
            let response = AppResponse::new_422()
                .with_message("Error validate form")
                .with_data(serde_json::json!({
                    "errors": e.errors()
                }));
            (StatusCode::UNPROCESSABLE_ENTITY, Json(response)).into_response()
        })?;
        Ok(ValidatedForm(data))
    }
}
