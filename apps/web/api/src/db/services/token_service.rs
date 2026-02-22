use crate::{
    api::api_response::ApiResponse,
    db::{
        db_handle_error,
        entities::token_entity::TokenEntity,
        models::{token_model::TokenModel, user_model::UserModel},
        schema,
    },
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct TokenService {}

#[allow(unused)]
impl TokenService {
    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        user: &UserModel,
        uuid: &String,
        secret: &String,
    ) -> Result<TokenModel, ApiResponse> {
        let user_id = user.id.ok_or_else(|| {
            ApiResponse::new_500().with_message("User ID is required to create token")
        })?;
        let mut model = TokenModel::new(user_id, uuid.clone(), secret.clone());
        let entity = model.to_new_entity();
        let result = diesel::insert_into(schema::tokens::table)
            .values(entity)
            .returning(TokenEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(model.with_entity(&result))
    }
}
