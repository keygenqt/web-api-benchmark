use std::sync::Arc;

use axum::{Router, middleware::Next};
use axum::{extract::Request, middleware::from_fn, response::Response};

use crate::app::app_config::AppConfig;
use crate::app::app_error::AppError;
use crate::app::app_route::AppRoute;
use crate::db::configure_db;

#[path = "guest/mod.rs"]
mod guest;

pub async fn configure_route(config: AppConfig) -> Router {
    Router::new()
        .route("/", guest::get_users::get())
        .with_state(AppRoute::new(config.clone(), configure_db(config).await))
        .layer(from_fn(log_app_errors))
}

async fn log_app_errors(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    if let Some(err) = response.extensions().get::<Arc<AppError>>() {
        tracing::error!(?err, "an unexpected error occurred inside a handler");
    }
    response
}
