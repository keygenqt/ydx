use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;
use std::sync::Arc;

use crate::app::app_config::AppConfig;

pub mod ctx_config;
pub mod ctx_cookie;
pub mod ctx_database;
pub mod ctx_user;
pub mod ctx_uuid;

#[derive(Clone, FromRef)]
pub struct AppContext {
    pub jkey: Arc<Key>,
    pub conf: Arc<AppConfig>,
    pub pool: Arc<bb8::Pool<AsyncPgConnection>>,
}
