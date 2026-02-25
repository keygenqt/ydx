use crate::{
    api::api_response::ApiResponse,
    db::{
        db_handle_error,
        schema::{self, users::dsl},
    },
    domain::user::{entity::UserEntity, model::UserModel},
};

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::bb8::PooledConnection};

pub struct UserService {}

#[allow(unused)]
impl UserService {
    /// Search user by fname and lname
    pub async fn search(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        fname: Option<String>,
        lname: Option<String>,
    ) -> Result<Vec<UserModel>, ApiResponse> {
        if fname.is_none() && lname.is_none() {
            return Ok(vec![]);
        }
        let result: Vec<UserModel> = UserEntity::query()
            .filter(dsl::fname.ilike(format!("%{}%", fname.unwrap_or("".to_string()))))
            .filter(dsl::lname.ilike(format!("%{}%", lname.unwrap_or("".to_string()))))
            .load(conn)
            .await
            .map_err(db_handle_error)?
            .iter()
            .map(|e| UserModel::from_entity(e))
            .collect();
        Ok(result)
    }

    /// Find entity by id
    pub async fn find_by_id(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        id: i32,
    ) -> Result<UserModel, ApiResponse> {
        let result = UserEntity::query()
            .find(id)
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }

    /// Find entity by email
    pub async fn find_by_email(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        email: &String,
    ) -> Result<UserModel, ApiResponse> {
        let result: UserEntity = UserEntity::query()
            .filter(dsl::email.eq(email))
            .first(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }

    /// Create new entity
    pub async fn create(
        conn: &mut PooledConnection<'static, AsyncPgConnection>,
        user: &UserModel,
    ) -> Result<UserModel, ApiResponse> {
        let entity = user.to_new_entity();
        let result = diesel::insert_into(schema::users::table)
            .values(entity)
            .returning(UserEntity::as_returning())
            .get_result(conn)
            .await
            .map_err(db_handle_error)?;
        Ok(UserModel::from_entity(&result))
    }
}
