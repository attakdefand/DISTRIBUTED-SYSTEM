use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("User not found")]
    UserNotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
        };

        (status, self.to_string()).into_response()
    }
}