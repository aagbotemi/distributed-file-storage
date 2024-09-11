use std::{env::VarError, num::ParseIntError};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Migration error: {0}")]
    MigrationError(String),
    #[error("Variable error: {0}")]
    EnvVarError(String, VarError),
    #[error("ParseInt error: {0}")]
    ParseError(String, ParseIntError),
    #[error("File not found")]
    FileNotFound,
    #[error("Invalid Header")]
    InvalidHeader,
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::MigrationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Migration error"),
            AppError::EnvVarError(_, _) => (StatusCode::INTERNAL_SERVER_ERROR, "Variable error"),
            AppError::ParseError(_, _) => (StatusCode::INTERNAL_SERVER_ERROR, "ParseInt error"),
            AppError::FileNotFound => (StatusCode::NOT_FOUND, "File not found"),
            AppError::InvalidHeader => (StatusCode::INTERNAL_SERVER_ERROR, "Invalid Header"),
            AppError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, error_message).into_response()
    }
}
