use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::db::schema;

#[derive(Debug, Serialize, Deserialize, HasQuery)]
#[diesel(table_name = schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TokenEntity {
    pub id: i32,
    pub user_id: i32,
    pub hash_refresh_token: String,
    pub device_uuid: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::tokens)]
pub struct NewTokenEntity {
    pub user_id: i32,
    pub hash_refresh_token: String,
    pub device_uuid: String,
    pub expires_at: NaiveDateTime,
}
