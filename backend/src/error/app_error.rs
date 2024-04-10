use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt;

use crate::json::extractor::Extractor;
use crate::storage::error::Error;

pub enum AppErrorCode {
    InvalidInput,
    InternalError,
}

impl fmt::Display for AppErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppErrorCode::InvalidInput => write!(f, "InvalidInput"),
            AppErrorCode::InternalError => write!(f, "InternalError"),
        }
    }
}

// The kinds of errors we can hit in our application.
pub enum AppError {
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // Undefined database error
    DatabaseError(Error),
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        Self::DatabaseError(error)
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
                AppErrorCode::InvalidInput,
                "Invalid input".to_string(),
            ),
            AppError::DatabaseError(_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                AppErrorCode::InternalError,
                "We made a mistake. Sorry".to_string(),
            ),
        };

        (
            response_status,
            Extractor(ErrorResponse {
                code: error_code.to_string(),
                message: error_message,
            }),
        )
            .into_response()
    }
}
