use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;

use crate::config::DatabaseSettings;

pub type AppState = Arc<App>;

#[derive(Debug)]
pub struct App {
    pub pool: PgPool,
}

#[must_use]
#[inline]
pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
