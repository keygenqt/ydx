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
#[diesel(table_name = schema::sets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SetEntity {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = schema::sets)]
pub struct NewSetEntity {
    pub user_id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = schema::sets)]
pub struct UpdateSetEntity {
    pub name: Option<String>,
    pub icon: Option<Option<String>>,
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = schema::sql_types::SortCards)]
pub enum SortCardsEntity {
    Random,
    Alphabetical,
    DateAdded,
}

impl std::fmt::Display for SortCardsEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortCardsEntity::Random => write!(f, "random"),
            SortCardsEntity::Alphabetical => write!(f, "alphabetical"),
            SortCardsEntity::DateAdded => write!(f, "date_added"),
        }
    }
}

impl ToSql<schema::sql_types::SortCards, Pg> for SortCardsEntity {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            SortCardsEntity::Random => out.write_all(b"random")?,
            SortCardsEntity::Alphabetical => out.write_all(b"alphabetical")?,
            SortCardsEntity::DateAdded => out.write_all(b"date_added")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<schema::sql_types::SortCards, Pg> for SortCardsEntity {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        match value.as_bytes() {
            b"random" => Ok(SortCardsEntity::Random),
            b"alphabetical" => Ok(SortCardsEntity::Alphabetical),
            b"date_added" => Ok(SortCardsEntity::DateAdded),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
