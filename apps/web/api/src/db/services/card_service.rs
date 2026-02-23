use crate::{
    api::api_response::ApiResponse,
    db::{
        db_handle_error, entities::card_entity::CardEntity, models::card_model::CardModel, schema,
    },
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct CardService {}

#[allow(unused)]
impl CardService {
    /// Find entity by id
    pub async fn find_by_id(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<CardModel, ApiResponse> {
        let result = CardEntity::query()
            .find(id)
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(CardModel::from_entity(&result))
    }

    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        model: &CardModel,
    ) -> Result<CardModel, ApiResponse> {
        let entity = model.to_new_entity();
        let result = diesel::insert_into(schema::cards::table)
            .values(entity)
            .returning(CardEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(CardModel::from_entity(&result))
    }
}
