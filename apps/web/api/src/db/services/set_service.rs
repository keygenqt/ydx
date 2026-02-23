use crate::{
    api::api_response::ApiResponse,
    db::{db_handle_error, entities::set_entity::SetEntity, models::set_model::SetModel, schema},
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct SetService {}

#[allow(unused)]
impl SetService {
    /// Find entity by id
    pub async fn find_by_id(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<SetModel, ApiResponse> {
        let result = SetEntity::query()
            .find(id)
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(SetModel::from_entity(&result))
    }

    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        model: &SetModel,
    ) -> Result<SetModel, ApiResponse> {
        let entity = model.to_new_entity();
        let result = diesel::insert_into(schema::sets::table)
            .values(entity)
            .returning(SetEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(SetModel::from_entity(&result))
    }
}
