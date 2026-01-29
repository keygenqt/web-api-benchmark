use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::extract::PrivateCookieJar;
use axum_extra::extract::cookie::Key;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::bb8;
use std::sync::Arc;

use crate::app::app_config::AppConfig;
use crate::app::app_response::AppResponse;
use crate::db::db_handle_internal_error;
use crate::db::models::user_auth::UserAuth;
use crate::db::services::user_service::UserService;
use crate::utils::constants;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub jkey: Arc<Key>,
    pub conf: Arc<AppConfig>,
    pub auth: Arc<Option<UserAuth>>,
    pub pool: Arc<bb8::Pool<AsyncPgConnection>>,
}

/// Impl PrivateCookieJar
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        (*state.jkey).clone()
    }
}

/// Impl for route AppConfig
#[allow(unused)]
pub struct Config(pub Arc<AppConfig>);

impl FromRequestParts<AppState> for Config {
    type Rejection = AppResponse;

    async fn from_request_parts(
        _parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        Ok(Config(state.conf))
    }
}

/// Impl for route UserModel
pub struct AuthModel(pub Arc<Option<UserAuth>>);

impl FromRequestParts<AppState> for AuthModel {
    type Rejection = AppResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let mut state = AppState::from_ref(state);
        // get secret cookies
        let key = (*state.jkey).clone();
        let jar = match PrivateCookieJar::<Key>::from_request_parts(parts, &key).await {
            Ok(jar) => jar,
            Err(err) => match err {},
        };
        // get auth model
        let user_auth = if let Some(data) = jar.get(constants::SESSION_NAME) {
            match serde_json::from_str::<UserAuth>(data.value()) {
                Ok(model) => Some(model),
                Err(_) => None,
            }
        } else {
            None
        };
        // database check
        if let Some(user_auth) = user_auth.clone() {
            // get connect
            let conn = state
                .pool
                .get_owned()
                .await
                .map_err(db_handle_internal_error)?;
            // get or error
            let _ = UserService::find_by_id(conn, user_auth.id).await?;
        };
        // save
        state.auth = Arc::new(user_auth);
        // result
        Ok(AuthModel(state.auth))
    }
}

/// Impl for route AsyncPgConnection
pub struct Db(pub bb8::PooledConnection<'static, AsyncPgConnection>);

impl FromRequestParts<AppState> for Db {
    type Rejection = AppResponse;

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
        Ok(Db(conn))
    }
}
