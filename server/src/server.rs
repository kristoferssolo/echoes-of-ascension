use leptos::prelude::*;
use tokio::{net::TcpListener, task::JoinHandle};

use crate::{
    config::Settings,
    error::ServerError,
    routes::route,
    startup::{get_connection_pool, App},
};

#[derive(Debug)]
pub struct Server(JoinHandle<Result<(), std::io::Error>>);

impl Server {
    pub async fn build(config: Settings) -> Result<Self, ServerError> {
        let pool = get_connection_pool(&config.database);

        // Get Leptos configuration but override the address
        let conf = get_configuration(None)?;

        // Use application's address configuration
        let addr = conf.leptos_options.site_addr;
        let listener = TcpListener::bind(addr).await?;

        let app_state = App {
            pool,
            leptos_options: conf.leptos_options,
        }
        .into();
        let server = tokio::spawn(async move { axum::serve(listener, route(app_state)).await });

        Ok(Self(server))
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.0.await?
    }
}
