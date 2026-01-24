use axum::http::StatusCode;
use deadpool_diesel::{
    Manager, Pool, Runtime,
    postgres::{self, Object},
};
use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::app::{app_config::AppConfig, app_error::AppError};

pub mod entities;
pub mod services;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn configure_db(config: AppConfig) -> Pool<Manager<PgConnection>> {
    // Create connect url
    let url = config.postgres.url.replace(
        "postgres://",
        &format!(
            "postgres://{}:{}@",
            config.postgres.user, config.postgres.password
        ),
    );
    // Connect
    let manager = postgres::Manager::new(url, Runtime::Tokio1);
    let pool = postgres::Pool::builder(manager).build().unwrap();
    // Migration
    pool.get()
        .await
        .unwrap()
        .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
        .await
        .unwrap()
        .unwrap();
    // Result connection pool
    pool
}

pub fn internal_error<E: std::error::Error>(err: E) -> AppError {
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn get_poll(pool: Pool<Manager<PgConnection>>) -> Result<Object, AppError> {
    Ok(pool.get().await.map_err(internal_error)?)
}

pub fn map_poll<R>(
    result: Result<Result<R, diesel::result::Error>, postgres::InteractError>,
) -> Result<R, AppError> {
    Ok(result.map_err(internal_error)?.map_err(internal_error)?)
}
