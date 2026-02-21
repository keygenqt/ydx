use diesel::result::Error;
use diesel_async::{
    AsyncMigrationHarness, AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::{api::api_response::ApiResponse, app::app_config::AppConfig};

pub mod entities;
pub mod models;
pub mod responses;
pub mod schema;
pub mod services;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// Init database connect
pub async fn configure_db(config: &AppConfig) -> bb8::Pool<AsyncPgConnection> {
    // create connect url
    let url = config.postgres.url.replace(
        "postgres://",
        &format!(
            "postgres://{}:{}@",
            config.postgres.user, config.postgres.password
        ),
    );
    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();
    // migration
    let mut harness = AsyncMigrationHarness::new(pool.get_owned().await.unwrap());
    harness.run_pending_migrations(MIGRATIONS).unwrap();
    // result
    pool
}

// Handle INTERNAL_SERVER_ERROR db to response
pub fn db_handle_internal_error<E: std::error::Error>(err: E) -> ApiResponse {
    ApiResponse::new_500().with_message(&err.to_string())
}

// Handle any error db to response
pub fn db_handle_error(err: Error) -> ApiResponse {
    match err {
        Error::NotFound => ApiResponse::new_404().with_message(&err.to_string()),
        _ => ApiResponse::new_500().with_message(&err.to_string()),
    }
}
