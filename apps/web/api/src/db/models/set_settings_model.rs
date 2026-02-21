use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::db::entities::{
    set_entity::SortCardsEntity,
    set_settings_entity::{NewSetSettingsEntity, SetSettingsEntity},
};

/// Model entry [SetSettingsEntity]
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SetSettingsModel {
    pub id: Option<i32>,
    pub set_id: i32,
    pub known_limit: Option<i32>,
    pub session_limit: Option<i32>,
    pub sort_by: Option<SortCardsEntity>,
}

/// Impl model entry
#[allow(dead_code)]
impl SetSettingsModel {
    /// Convert from database entity to model
    pub fn from_entity(entity: &SetSettingsEntity) -> Self {
        Self {
            id: Some(entity.id),
            set_id: entity.set_id.clone(),
            known_limit: entity.known_limit.clone(),
            session_limit: entity.session_limit.clone(),
            sort_by: entity.sort_by.clone(),
        }
    }

    /// Convert to new entity for database insertion
    pub fn to_new_entity(&self) -> NewSetSettingsEntity {
        NewSetSettingsEntity {
            set_id: self.set_id.clone(),
            known_limit: self.known_limit.clone(),
            session_limit: self.session_limit.clone(),
            sort_by: self.sort_by.clone(),
        }
    }
}
