use crate::{
    api::api_response::ApiResponse,
    db::{db_handle_error, entities::set_entity::SetEntity, models::set_model::SetModel},
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct SetService {}

#[allow(unused)]
impl SetService {
    /// Find entity by id
    pub async fn find_by_id(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<SetModel, ApiResponse> {
        let result = SetEntity::query()
            .find(id)
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(SetModel::from_entity(&result))
    }
}
