use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::{entities::set_entity::SortCardsEntity, schema};

#[derive(Debug, Serialize, Deserialize, HasQuery)]
#[diesel(table_name = schema::sets_settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SetSettingsEntity {
    pub id: i32,
    pub set_id: i32,
    pub known_limit: Option<i32>,
    pub session_limit: Option<i32>,
    pub sort_by: Option<SortCardsEntity>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::sets_settings)]
pub struct NewSetSettingsEntity {
    pub set_id: i32,
    pub known_limit: Option<i32>,
    pub session_limit: Option<i32>,
    pub sort_by: Option<SortCardsEntity>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::sets_settings)]
pub struct UpdateSetSettingsEntity {
    pub known_limit: Option<Option<i32>>,
    pub session_limit: Option<Option<i32>>,
    pub sort_by: Option<Option<SortCardsEntity>>,
}
