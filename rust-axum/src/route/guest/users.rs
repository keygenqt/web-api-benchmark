use axum::extract::{Path, Query};
use serde::Deserialize;

use crate::{
    app::{app_error::AppError, app_json::AppJson, app_state::AppStateConn},
    db::{models::user_model::UserModel, services::user_service::UserService},
};

#[derive(Deserialize)]
pub struct SearchParams {
    fname: Option<String>,
    lname: Option<String>,
}

/// GET find by fname and lname
/// /api/users
pub async fn search(
    AppStateConn(_conf, conn): AppStateConn,
    Query(search): Query<SearchParams>,
) -> Result<AppJson<Vec<UserModel>>, AppError> {
    let result = UserService::search(conn, search.fname, search.lname).await?;
    Ok(AppJson(result))
}

/// GET find by fname and lname
/// /api/users/{id}
pub async fn get_by_id(
    AppStateConn(_conf, conn): AppStateConn,
    Path(id): Path<i32>,
) -> Result<AppJson<UserModel>, AppError> {
    let result = UserService::find_by_id(conn, id).await?;
    Ok(AppJson(result))
}
