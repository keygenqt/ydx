use crate::{
    api::api_response::ApiResponse,
    db::{
        db_handle_error,
        entities::token_entity::TokenEntity,
        models::token_model::TokenModel,
        schema::{self, tokens::dsl},
    },
    utils,
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct TokenService {}

#[allow(unused)]
impl TokenService {
    /// Find entity
    pub async fn find_by_id_uuid(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        user_id: i32,
        uuid: &String,
    ) -> Result<TokenModel, ApiResponse> {
        let result = TokenEntity::query()
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::device_uuid.eq(uuid))
            .order(dsl::created_at.desc())
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(TokenModel::from_entity(&result))
    }

    /// Clear token by uuid
    pub async fn clear_by_id_uuid(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        user_id: i32,
        uuid: &String,
    ) -> Result<usize, ApiResponse> {
        let deleted_count = diesel::delete(schema::tokens::table)
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::device_uuid.eq(uuid))
            .execute(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(deleted_count)
    }

    /// Clear all user tokens
    pub async fn clear_by_id(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        user_id: i32,
    ) -> Result<usize, ApiResponse> {
        let deleted_count = diesel::delete(schema::tokens::table)
            .filter(dsl::user_id.eq(user_id))
            .execute(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(deleted_count)
    }

    /// Clear all expired tokens
    pub async fn clear_expired_tokens(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
    ) -> Result<usize, ApiResponse> {
        let deleted_count = diesel::delete(schema::tokens::table)
            .filter(dsl::expires_at.lt(utils::local_time::chrono_time_now()))
            .execute(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(deleted_count)
    }

    /// Clear all old duplicate tokens
    pub async fn clear_duplicate_tokens(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
    ) -> Result<usize, ApiResponse> {
        let deleted_count = diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "WITH keep_ids AS (
                SELECT MAX(id) as id
                FROM tokens
                GROUP BY user_id, device_uuid
            )
            DELETE FROM tokens
            WHERE id NOT IN (SELECT id FROM keep_ids)"
        )
        .execute(conn)
        .await
        .map_err(db_handle_error)?;

        Ok(deleted_count)
    }

    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        model: &TokenModel,
    ) -> Result<TokenModel, ApiResponse> {
        let entity = model.to_new_entity();
        let result = diesel::insert_into(schema::tokens::table)
            .values(entity)
            .returning(TokenEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(TokenModel::from_entity_with_tokens(
            &result,
            &model.access_token,
            &model.refresh_token,
        ))
    }
}
