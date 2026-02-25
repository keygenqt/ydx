use crate::{
    api::api_response::ApiResponse,
    db::{db_handle_error, schema},
    domain::image::{entity::ImageEntity, model::ImageModel},
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct ImageService {}

#[allow(unused)]
impl ImageService {
    /// Find entity by id
    pub async fn find_by_id(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<ImageModel, ApiResponse> {
        let result = ImageEntity::query()
            .find(id)
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(ImageModel::from_entity(&result))
    }

    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        model: &ImageModel,
    ) -> Result<ImageModel, ApiResponse> {
        let entity = model.to_new_entity();
        let result = diesel::insert_into(schema::images::table)
            .values(entity)
            .returning(ImageEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(ImageModel::from_entity(&result))
    }
}
