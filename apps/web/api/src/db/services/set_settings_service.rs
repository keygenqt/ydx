use crate::{
    app::app_response::AppResponse,
    db::{
        db_handle_error, entities::set_settings_entity::SetSettingsEntity,
        models::set_settings_model::SetSettingsModel,
    },
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct SetSettingsService {}

#[allow(unused)]
impl SetSettingsService {
    /// Find entity by id
    pub async fn find_by_id(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<SetSettingsModel, AppResponse> {
        let result = SetSettingsEntity::query()
            .find(id)
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(SetSettingsModel::from_entity(&result))
    }
}
