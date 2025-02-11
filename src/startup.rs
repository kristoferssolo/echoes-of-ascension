use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    config::{DatabaseSettings, Settings},
    routes::route,
};

pub struct App {
    pub pool: PgPool,
}

pub type AppState = Arc<App>;

pub struct Application {
    port: u16,
    server: JoinHandle<Result<(), std::io::Error>>,
}

impl Application {
    /// Builds and starts the application server.
    ///
    /// # Errors
    ///
    /// - Returns `std::io::Error` if:
    ///   - It fails to bind to the specified address.
    ///
    /// # Panics
    ///
    /// - Panics if `listener.local_addr()` returns `None`. This should only occur if the
    ///   listener is not properly bound to an address, which is considered a critical
    ///   failure during application startup.
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&config.database);

        let addr = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(addr).await?;
        let port = listener
            .local_addr()
            .expect("Listener should have a local address")
            .port();
        let app_state = App { pool }.into();
        let server = tokio::spawn(async move { axum::serve(listener, route(app_state)).await });

        Ok(Self { port, server })
    }

    #[must_use]
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Runs the application until it is stopped.
    ///
    /// # Errors
    ///
    /// - Returns `std::io::Error` if the server task encounters an error.
    #[inline]
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await?
    }
}

#[must_use]
pub fn get_connection_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}
