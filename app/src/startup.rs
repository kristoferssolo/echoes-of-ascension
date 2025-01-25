use std::sync::Arc;

use leptos::config::{errors::LeptosConfigError, LeptosOptions};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;

use crate::config::DatabaseSettings;

pub type AppState = Arc<App>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Leptos configuration error: {0}")]
    LeptosConfig(#[from] LeptosConfigError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Server error: {0}")]
    Server(String),
}

#[derive(Debug)]
pub struct App {
    pub pool: PgPool,
    pub leptos_options: LeptosOptions,
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
