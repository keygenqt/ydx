use chrono::NaiveDateTime;
use serde::Serialize;
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{api::api_response::ApiResponse, db::models::set_model::SetModel};

/// Model response [TokenModel]
#[serde_as]
#[derive(Serialize)]
pub struct SetResponse {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl response
impl SetResponse {
    pub fn from_model(model: &SetModel) -> Result<SetResponse, ApiResponse> {
        if model.id.is_none() {
            Err(ApiResponse::new_500())?;
        }
        Ok(Self {
            id: model.id.unwrap(),
            user_id: model.user_id,
            name: model.name.clone(),
            icon: model.icon.clone(),
            description: model.description.clone(),
            created_at: model.created_at,
        })
    }
}
