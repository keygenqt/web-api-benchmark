use std::sync::Arc;

use axum::handler::HandlerWithoutStateExt;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, middleware::Next};
use axum::{extract::Request, middleware::from_fn, response::Response};
use tower_http::services::ServeDir;

use crate::app::app_config::AppConfig;
use crate::app::app_error::AppError;
use crate::app::app_state::AppState;
use crate::db::configure_db;

#[path = "guest/mod.rs"]
mod guest;

pub async fn configure_route(config: &AppConfig) -> Router {
    let state = AppState {
        conf: Arc::new(config.clone()),
        pool: Arc::new(configure_db(config).await),
    };
    Router::new()
        .route("/api/users", get(guest::users::search))
        .route("/api/users/{id}", get(guest::users::get_by_id))
        .with_state(state)
        .layer(from_fn(log_app_errors))
        .fallback_service(ServeDir::new("assets").not_found_service(handle_404.into_service()))
}

async fn handle_404() -> impl IntoResponse {
    AppError::new_404()
}

async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(err) = response.extensions().get::<Arc<AppError>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }
    response
}
