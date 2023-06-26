use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::Display;
use serde_json::json;
use thiserror::Error;

/// Our app's top level error type.
#[derive(Error, Debug, Display)]
pub enum AppError {
    // Specific Error types
    User(UserError),
    Question(QuestionError),
    // Catchall Error route for anyhow bails
    Any(anyhow::Error),
}

// Allows `?` to automatically convert all anyhow Errors into AppErrors
impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        Self::Any(inner)
    }
}

// Axum's main error handler apparatus - we simply turn them into Responses!
// Note here that we're seeing a "Successful Failure" here, which HTTP considers a success
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Fish a Status Code and error message out of each error
        let (status, error_message) = match self {
            // User-related errors
            AppError::User(err) => match err {
                // We can simply err.to_string() here thanks to `thiserror`
                UserError::NotFound => (StatusCode::NOT_FOUND, err.to_string()),
                UserError::InvalidPassword => (StatusCode::UNAUTHORIZED, err.to_string()),
            },
            AppError::Question(err) => match err {
                QuestionError::InvalidId => (StatusCode::NOT_FOUND, err.to_string()),
            },
            // No touchy!  Handles *all* anyhow-originating errors
            AppError::Any(err) => {
                let message = format!("Internal Server Error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

/// Errors that can happen related to users.
#[derive(Debug, Error)]
pub enum UserError {
    // Thiserror utility!  This is what arrives in our error message's text field
    #[error("User Not Found")]
    #[allow(dead_code)]
    NotFound,
    #[error("Invalid Password")]
    #[allow(dead_code)]
    InvalidPassword,
}

/// Allows `?` to automatically convert a `UserError` into AppError
impl From<UserError> for AppError {
    fn from(inner: UserError) -> Self {
        Self::User(inner)
    }
}

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error("Question ID Not Found")]
    InvalidId,
}

impl From<QuestionError> for AppError {
    fn from(inner: QuestionError) -> Self {
        Self::Question(inner)
    }
}
