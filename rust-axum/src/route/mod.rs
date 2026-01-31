use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Router, middleware::Next};
use axum::{extract::Request, middleware::from_fn, response::Response};
use axum_extra::extract::cookie::Key;

use crate::app::app_config::AppConfig;
use crate::app::app_response::AppResponse;
use crate::app::app_state::AppState;
use crate::db::configure_db;

#[path = "guest/mod.rs"]
mod guest;

#[path = "user/mod.rs"]
mod user;

/// Init Axum server
pub async fn configure_route(config: &AppConfig) -> Router {
    let state = AppState {
        jkey: Arc::new(Key::from(config.server.secret.as_bytes())),
        conf: Arc::new(config.clone()),
        auth: Arc::new(None),
        pool: Arc::new(configure_db(config).await),
    };
    Router::new()
        .route("/", get(guest::home::index))
        .route("/api/auth", post(guest::auth::basic))
        .route("/api/logout", delete(guest::auth::logout))
        .route("/api/users", get(user::users::search))
        .route("/api/users/{id}", get(user::users::get_by_id))
        .with_state(state)
        .layer(from_fn(log_app_errors))
        .fallback(handle_404)
}

/// Handle route 404
async fn handle_404() -> impl IntoResponse {
    AppResponse::new_404()
}

/// Handle any error for tracing
async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(err) = response.extensions().get::<Arc<AppResponse>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }
    response
}
