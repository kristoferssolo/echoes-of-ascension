use leptos::config::errors::LeptosConfigError;
use sqlx;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Leptos configuration error: {0}")]
    LeptosConfig(#[from] LeptosConfigError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}
