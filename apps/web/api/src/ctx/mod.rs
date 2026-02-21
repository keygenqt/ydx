use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;
use std::sync::Arc;

use crate::app::app_config::AppConfig;
use crate::app::app_user::AppUser;

pub mod ctx_config;
pub mod ctx_cookie;
pub mod ctx_database;
pub mod ctx_user;

#[derive(Clone, FromRef)]
pub struct AppContext {
    pub jkey: Arc<Key>,
    pub conf: Arc<AppConfig>,
    pub auth: Arc<Option<AppUser>>,
    pub pool: Arc<bb8::Pool<AsyncPgConnection>>,
}
