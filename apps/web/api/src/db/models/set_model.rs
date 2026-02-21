use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::set_entity::{NewSetEntity, SetEntity},
    utils::{self},
};

/// Model entry [SetEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SetModel {
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl SetModel {
    /// Convert from database entity to model
    pub fn from_entity(entity: &SetEntity) -> Self {
        Self {
            id: Some(entity.id),
            user_id: entity.user_id.clone(),
            name: entity.name.clone(),
            icon: entity.icon.clone(),
            description: entity.description.clone(),
            created_at: entity.created_at,
        }
    }

    /// Convert to new entity for database insertion
    pub fn to_new_entity(&self) -> NewSetEntity {
        NewSetEntity {
            user_id: self.user_id.clone(),
            name: self.name.clone(),
            icon: self.icon.clone(),
            description: self.description.clone(),
        }
    }
}
