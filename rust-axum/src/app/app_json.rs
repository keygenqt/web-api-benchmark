use axum::Json;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};

use crate::app::app_error::AppError;

/// Application custom Json
#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
