use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Authentication/Authorization errors
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    // Resource errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("{resource} already exists: {id}")]
    AlreadyExists { resource: &'static str, id: String },

    // Database errors
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    // Internal errors
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message, details) = match self {
            // Auth errors
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string(), None),
            // Validation errors
            Self::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                Some(msg),
            ),
            // Resource errors
            Self::NotFound(resource) => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                Some(resource),
            ),
            Self::AlreadyExists { resource, id } => (
                StatusCode::CONFLICT,
                format!("{resource} already exists"),
                Some(id),
            ),
            // Database/Internal errors
            Self::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
            Self::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
        };

        let body = Json(ErrorResponse {
            error: error_message,
            details,
        });
        (status, body).into_response()
    }
}
