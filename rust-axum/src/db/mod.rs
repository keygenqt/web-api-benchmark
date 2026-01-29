use axum::http::StatusCode;
use diesel::result::Error;
use diesel_async::{
    AsyncMigrationHarness, AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::app::{app_config::AppConfig, app_response::AppResponse};

pub mod entities;
pub mod models;
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
pub fn db_handle_internal_error<E: std::error::Error>(err: E) -> AppResponse {
    AppResponse::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// Handle any error db to response
pub fn db_handle_error(err: Error) -> AppResponse {
    match err {
        Error::NotFound => AppResponse::new(StatusCode::NOT_FOUND, err.to_string()),
        _ => AppResponse::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}
