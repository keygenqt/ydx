use chrono::NaiveDateTime;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::db::schema;

#[derive(Debug, Serialize, Deserialize, HasQuery)]
#[diesel(table_name = schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImageEntity {
    pub id: i32,
    pub entity_type: EntityTypeImage,
    pub entity_id: i32,
    pub path: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::images)]
pub struct NewImageEntity {
    pub entity_type: EntityTypeImage,
    pub entity_id: i32,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::sql_types::EntityTypeImage)]
pub enum EntityTypeImage {
    User,
    Set,
    Card,
}

impl std::fmt::Display for EntityTypeImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityTypeImage::User => write!(f, "user"),
            EntityTypeImage::Set => write!(f, "set"),
            EntityTypeImage::Card => write!(f, "card"),
        }
    }
}

impl ToSql<schema::sql_types::EntityTypeImage, Pg> for EntityTypeImage {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            EntityTypeImage::User => out.write_all(b"user")?,
            EntityTypeImage::Set => out.write_all(b"set")?,
            EntityTypeImage::Card => out.write_all(b"card")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::EntityTypeImage, Pg> for EntityTypeImage {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        match value.as_bytes() {
            b"user" => Ok(EntityTypeImage::User),
            b"set" => Ok(EntityTypeImage::Set),
            b"card" => Ok(EntityTypeImage::Card),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
