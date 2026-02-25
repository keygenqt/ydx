use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    domain::set::entity::{NewSetEntity, SetEntity, SortCardsEntity},
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
    pub known_limit: i32,
    pub session_limit: i32,
    pub sort_by: SortCardsEntity,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl SetModel {
    /// Create new [SetModel]
    pub fn new(user_id: i32, name: String, icon: Option<String>, desc: Option<String>) -> Self {
        Self {
            id: None,
            user_id,
            name,
            icon: icon.clone(),
            description: desc.clone(),
            known_limit: 10,
            session_limit: 20,
            sort_by: SortCardsEntity::DateAdded,
            created_at: utils::local_time::chrono_time_now(),
        }
    }

    /// Create model from [SetEntity]
    pub fn from_entity(entity: &SetEntity) -> Self {
        Self {
            id: Some(entity.id),
            user_id: entity.user_id,
            name: entity.name.clone(),
            icon: entity.icon.clone(),
            description: entity.description.clone(),
            known_limit: entity.known_limit,
            session_limit: entity.session_limit,
            sort_by: entity.sort_by,
            created_at: entity.created_at,
        }
    }

    /// Create new [NewSetEntity]
    pub fn to_new_entity(&self) -> NewSetEntity {
        NewSetEntity {
            user_id: self.user_id.clone(),
            name: self.name.clone(),
            icon: self.icon.clone(),
            description: self.description.clone(),
            known_limit: self.known_limit,
            session_limit: self.session_limit,
            sort_by: self.sort_by,
        }
    }
}
