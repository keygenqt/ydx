use crate::{
    api::api_response::ApiResponse,
    db::{db_handle_error, entities::token_entity::TokenEntity, models::token_model::TokenModel},
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct TokenService {}

#[allow(unused)]
impl TokenService {
    /// Find entity by id
    pub async fn find_by_id(
        mut conn: PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<TokenModel, ApiResponse> {
        let result = TokenEntity::query()
            .find(id)
            .first(&mut conn)
            .await
            .map_err(db_handle_error)?;
        Ok(TokenModel::from_entity(&result))
    }
}
