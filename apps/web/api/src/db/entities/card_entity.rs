use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::db::schema;

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::cards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CardEntity {
    pub id: i32,
    pub set_id: i32,
    pub term: String,
    pub definition: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::cards)]
pub struct NewCardEntity {
    pub set_id: i32,
    pub term: String,
    pub definition: String,
    pub image: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::cards)]
pub struct UpdateCardEntity {
    pub term: Option<String>,
    pub definition: Option<String>,
    pub image: Option<Option<String>>,
}
