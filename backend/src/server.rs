use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    config::Settings,
    routes::route,
    startup::{get_connection_pool, App},
};

#[derive(Debug)]
pub struct Server {
    port: u16,
    server: JoinHandle<Result<(), std::io::Error>>,
}

impl Server {
    pub async fn build(config: Settings) -> Result<Self, std::io::Error> {
        let pool = get_connection_pool(&config.database);

        // Use application's address configuration
        let addr = format!("{}:{}", config.application.host, config.application.port);
        let listener = TcpListener::bind(addr).await?;
        let port = listener.local_addr()?.port();
        let app_state = App { pool }.into();
        let server = tokio::spawn(async move { axum::serve(listener, route(app_state)).await });

        Ok(Self { port, server })
    }

    #[must_use]
    #[inline]
    pub const fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await?
    }
}
