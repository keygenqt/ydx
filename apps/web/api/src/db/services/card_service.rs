use crate::{
    api::api_response::ApiResponse, db::{db_handle_error, entities::card_entity::CardEntity, models::card_model::CardModel}
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct CardService {}

#[allow(unused)]
impl CardService {
    /// Find entity by id
    pub async fn find_by_id(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<CardModel, ApiResponse> {
        let result = CardEntity::query()
            .find(id)
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(CardModel::from_entity(&result))
    }
}
