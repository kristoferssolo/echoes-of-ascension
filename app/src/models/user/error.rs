use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Username validation failed: {0}")]
    UsernameValidation(String),

    #[error("Code hashing failed: {0}")]
    HashingError(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Username already taken: {0}")]
    UsernameTaken(String),

    #[error("Invalid code format")]
    InvalidCode,

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Internal server error: {0}")]
    Internal(String),
}
