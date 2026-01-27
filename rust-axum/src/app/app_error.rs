use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::sync::Arc;

use crate::app::app_json::AppJson;

/// Application custom Error
#[derive(Debug, Clone)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn new_404() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "Not Found".to_string(),
        }
    }
}

// Implementation axum response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Response data
        #[derive(Serialize)]
        struct ErrorResponse {
            code: u16,
            message: String,
        }
        // Copy error
        let err = self.clone();
        // Create response
        let mut response = (
            self.code,
            AppJson(ErrorResponse {
                code: self.code.as_u16(),
                message: self.message,
            }),
        )
            .into_response();
        // Set error to response
        response.extensions_mut().insert(Arc::new(err));
        // Result
        response
    }
}
