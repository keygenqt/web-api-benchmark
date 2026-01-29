use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::sync::Arc;

use crate::app::app_json::AppJson;

/// Application common response
#[derive(Debug, Clone)]
pub struct AppResponse {
    pub code: StatusCode,
    pub message: String,
}

impl AppResponse {
    pub fn new(code: StatusCode, message: String) -> Self {
        Self { code, message }
    }

    pub fn new_200() -> Self {
        Self {
            code: StatusCode::OK,
            message: "Success".to_string(),
        }
    }

    pub fn new_404() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "Not Found".to_string(),
        }
    }

    pub fn new_403() -> Self {
        Self {
            code: StatusCode::FORBIDDEN,
            message: "Access denied".to_string(),
        }
    }
}

/// Impl axum response
impl IntoResponse for AppResponse {
    fn into_response(self) -> Response {
        // response data
        #[derive(Serialize)]
        struct ErrorResponse {
            code: u16,
            message: String,
        }
        // copy app response
        let app_response = self.clone();
        // create response
        let mut response = (
            self.code,
            AppJson(ErrorResponse {
                code: self.code.as_u16(),
                message: self.message,
            }),
        )
            .into_response();
        // set error to response
        response.extensions_mut().insert(Arc::new(app_response));
        // result
        response
    }
}
