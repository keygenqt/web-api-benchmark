use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;
use std::sync::Arc;

use crate::app::app_config::AppConfig;
use crate::app::app_error::AppError;
use crate::db::db_handle_internal_error;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub conf: Arc<AppConfig>,
    pub pool: Arc<bb8::Pool<AsyncPgConnection>>,
}

pub struct AppStateConn(
    pub Arc<AppConfig>,
    pub bb8::PooledConnection<'static, AsyncPgConnection>,
);

impl FromRequestParts<AppState> for AppStateConn {
    type Rejection = AppError;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let conn = state
            .pool
            .get_owned()
            .await
            .map_err(db_handle_internal_error)?;
        Ok(AppStateConn(state.conf, conn))
    }
}
