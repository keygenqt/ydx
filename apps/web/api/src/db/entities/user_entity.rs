use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::{TimestampMilliSeconds, serde_as};

use crate::db::schema;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, HasQuery)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserEntity {
    pub id: i32,
    pub email: String,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub pwd_hash: String,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

#[serde_as]
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUserEntity {
    pub email: String,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub pwd_hash: String,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UpdateUserEntity {
    pub email: Option<String>,
    pub icon: Option<String>,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub pwd_hash: Option<String>,
}
