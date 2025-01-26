use app::models::user::error::UserError;
use sqlx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerUserError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    User(#[from] UserError),
}

impl From<ServerUserError> for UserError {
    fn from(error: ServerUserError) -> Self {
        match error {
            ServerUserError::Database(e) => Self::Internal(e.to_string()),
            ServerUserError::User(e) => e,
        }
    }
}
