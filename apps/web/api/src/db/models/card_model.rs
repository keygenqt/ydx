use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::{
    db::entities::card_entity::{CardEntity, NewCardEntity},
    utils::{self},
};

/// Model entry [CardEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct CardModel {
    pub id: Option<i32>,
    pub set_id: i32,
    pub term: String,
    pub definition: String,
    pub image: Option<String>,
    #[serde(default = "utils::local_time::chrono_time_now")]
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

/// Impl model entry
#[allow(dead_code)]
impl CardModel {
    /// Create new [CardModel]
    pub fn new(set_id: i32, term: String, definition: String, image: Option<String>) -> Self {
        Self {
            id: None,
            set_id,
            term,
            definition,
            image: image.clone(),
            created_at: utils::local_time::chrono_time_now(),
        }
    }

    /// Create model from [CardEntity]
    pub fn from_entity(entity: &CardEntity) -> Self {
        Self {
            id: Some(entity.id),
            set_id: entity.set_id.clone(),
            term: entity.term.clone(),
            definition: entity.definition.clone(),
            image: entity.image.clone(),
            created_at: entity.created_at,
        }
    }

    /// Create new [NewSetEntity]
    pub fn to_new_entity(&self) -> NewCardEntity {
        NewCardEntity {
            set_id: self.set_id.clone(),
            term: self.term.clone(),
            definition: self.definition.clone(),
            image: self.image.clone(),
        }
    }
}
