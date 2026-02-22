use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Router, middleware::Next};
use axum::{extract::Request, middleware::from_fn, response::Response};
use axum_extra::extract::cookie::Key;

use crate::api::api_response::ApiResponse;
use crate::app::app_config::AppConfig;
use crate::ctx::AppContext;
use crate::db::configure_db;

#[path = "guest/mod.rs"]
mod guest;

#[path = "user/mod.rs"]
mod user;

/// Init Axum server
pub async fn configure_route(config: &AppConfig) -> Router {
    // route state
    let state = AppContext {
        jkey: Arc::new(Key::from(config.server.secret.as_bytes())),
        conf: Arc::new(config.clone()),
        pool: Arc::new(configure_db(config).await),
    };
    // route config
    Router::new()
        .route("/", get(guest::home::app_info))
        .route("/api", get(guest::home::app_info))
        .route("/api/signin", post(guest::signin::form))
        .route("/api/signout", delete(guest::signout::clear_session))
        .route("/api/signup", post(guest::signup::reg_user))
        .route("/api/cards/{id}", get(user::cards::get_by_id))
        .route("/api/sets/{id}", get(user::sets::get_by_id))
        .route("/api/users/{id}", get(user::users::get_by_id))
        // configure
        .with_state(state)
        .layer(from_fn(log_app_responses))
        .fallback(handle_404)
}

/// Handle route 404
async fn handle_404() -> impl IntoResponse {
    ApiResponse::new_404()
}

/// Handle any error for tracing
async fn log_app_responses(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(response) = response.extensions().get::<Arc<ApiResponse>>() {
        if response.code != 200 {
            tracing::error!(?response, "an unexpected error occurred inside a handler");
        }
    }
    response
}
