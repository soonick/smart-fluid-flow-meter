use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::json::extractor::Extractor;

// The kinds of errors we can hit in our application.
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

// Tell axum how `AppError` should be converted into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: String,
            message: String,
        }

        let (response_status, error_code, error_message) = match self {
            // This error is caused by bad user input
            AppError::JsonRejection(rejection) => (
                rejection.status(),
                "INVALID_INPUT".to_string(),
                "Invalid input".to_string(),
            ),
        };

        (
            response_status,
            Extractor(ErrorResponse {
                code: error_code,
                message: error_message,
            }),
        )
            .into_response()
    }
}
