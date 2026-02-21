use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::token_entity::{NewTokenEntity, TokenEntity},
    utils::{self},
};

/// Model entry [TokenEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenModel {
    pub id: Option<i32>,
    pub user_id: i32,
    pub refresh_token: String,
    pub device_uuid: String,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub expires_at: NaiveDateTime,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl TokenModel {
    /// Convert from database entity to model
    pub fn from_entity(entity: &TokenEntity) -> Self {
        Self {
            id: Some(entity.id),
            user_id: entity.user_id.clone(),
            refresh_token: entity.refresh_token.clone(),
            device_uuid: entity.device_uuid.clone(),
            expires_at: entity.expires_at,
            created_at: entity.created_at,
        }
    }

    /// Convert to new entity for database insertion
    pub fn to_new_entity(&self) -> NewTokenEntity {
        NewTokenEntity {
            user_id: self.user_id.clone(),
            refresh_token: self.refresh_token.clone(),
            device_uuid: self.device_uuid.clone(),
            expires_at: self.expires_at,
        }
    }
}
