use axum::Json;
use axum::extract::FromRequest;
use axum::response::{IntoResponse, Response};

use crate::app::app_response::AppResponse;

/// Application custom Json
#[derive(FromRequest)]
#[from_request(via(Json), rejection(AppResponse))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        Json(self.0).into_response()
    }
}
