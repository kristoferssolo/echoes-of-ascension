use std::sync::Arc;

use leptos::config::{errors::LeptosConfigError, get_configuration, LeptosOptions};
use sqlx::{postgres::PgPoolOptions, PgPool};
use thiserror::Error;
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    config::{DatabaseSettings, Settings},
    routes::route,
};

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

#[derive(Debug)]
pub struct Application {
    port: u16,
    server: JoinHandle<Result<(), std::io::Error>>,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, ApplicationError> {
        let pool = get_connection_pool(&config.database);

        // Get Leptos configuration but override the address
        let conf = get_configuration(None)?;

        // Use application's address configuration
        let addr = conf.leptos_options.site_addr;
        let listener = TcpListener::bind(addr).await?;
        let port = listener.local_addr()?.port();

        let app_state = App {
            pool,
            leptos_options: conf.leptos_options,
        }
        .into();
        let server = tokio::spawn(async move { axum::serve(listener, route(app_state)).await });

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await?
    }
}

pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
