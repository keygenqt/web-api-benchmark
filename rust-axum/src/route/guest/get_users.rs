use axum::{
    extract::State,
    routing::{self, MethodRouter},
};

use crate::{
    app::{app_error::AppError, app_json::AppJson, app_route::AppRoute},
    db::{entities::user_entity::UserModel, services::user_service::UserService},
};

pub fn get() -> MethodRouter<AppRoute> {
    routing::get(
        async |State(app): State<AppRoute>| -> Result<AppJson<Vec<UserModel>>, AppError> {
            // Get users list
            let result = UserService::get_list(app.db).await?;
            // Result
            Ok(AppJson(result))
        },
    )
}
