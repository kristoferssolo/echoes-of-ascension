use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Username already taken")]
    UsernameTaken,
    #[error("User not found")]
    NotFound,
}

pub async fn create_user() {}
