use axum::extract::{Path, Query};
use serde::Deserialize;

use crate::{
    app::{
        app_json::AppJson,
        app_response::AppResponse,
        app_state::{AuthModel, Db},
    },
    db::{models::user_model::UserModel, services::user_service::UserService},
};

/// Query params
#[derive(Deserialize)]
pub struct SearchParams {
    fname: Option<String>,
    lname: Option<String>,
}

/// GET find by fname and lname
/// /api/users
pub async fn search(
    AuthModel(user): AuthModel,
    Db(conn): Db,
    Query(search): Query<SearchParams>,
) -> Result<AppJson<Vec<UserModel>>, AppResponse> {
    if user.is_some() {
        let result = UserService::search(conn, search.fname, search.lname).await?;
        Ok(AppJson(result))
    } else {
        Err(AppResponse::new_403())
    }
}

/// GET find by user ID
/// /api/users/{id}
pub async fn get_by_id(
    AuthModel(user): AuthModel,
    Db(conn): Db,
    Path(id): Path<i32>,
) -> Result<AppJson<UserModel>, AppResponse> {
    if user.is_some() {
        let result = UserService::find_by_id(conn, id).await?;
        Ok(AppJson(result))
    } else {
        Err(AppResponse::new_403())
    }
}
