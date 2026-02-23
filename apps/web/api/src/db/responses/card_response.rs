use chrono::NaiveDateTime;
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{api::api_response::ApiResponse, db::models::{card_model::CardModel, set_model::SetModel}};

/// Model response [TokenModel]
#[serde_as]
#[derive(Serialize)]
pub struct CardResponse {
    pub id: i32,
    pub set_id: i32,
    pub term: String,
    pub definition: String,
    pub image: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl response
impl CardResponse {
    pub fn from_model(model: &CardModel) -> Result<CardResponse, ApiResponse> {
        if model.id.is_none() {
            Err(ApiResponse::new_500())?;
        }
        Ok(Self {
            id: model.id.unwrap(),
            set_id: model.set_id,
            term: model.term.clone(),
            definition: model.definition.clone(),
            image: model.image.clone(),
            created_at: model.created_at,
        })
    }
}
