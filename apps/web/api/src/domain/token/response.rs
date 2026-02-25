use chrono::NaiveDateTime;
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{api::api_response::ApiResponse, domain::token::model::TokenModel};

/// Model response [TokenModel]
#[serde_as]
#[derive(Serialize)]
pub struct TokenResponse {
    pub user_id: i32,
    pub access_token: String,
    pub refresh_token: String,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub expires_at: NaiveDateTime,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl response
impl TokenResponse {
    pub fn from_model(model: &TokenModel) -> Result<TokenResponse, ApiResponse> {
        if model.access_token.is_none() || model.refresh_token.is_none() {
            Err(ApiResponse::new_500())?;
        }
        Ok(Self {
            user_id: model.user_id,
            access_token: model.access_token.clone().unwrap(),
            refresh_token: model.refresh_token.clone().unwrap(),
            expires_at: model.expires_at,
            created_at: model.created_at,
        })
    }
}
