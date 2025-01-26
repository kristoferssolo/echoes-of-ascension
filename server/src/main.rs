mod config;
mod db;
pub mod domain;
mod error;
mod routes;
mod server;
mod startup;

use app::telemetry::{get_subscriber, init_subscriber};
use config::get_config;
use error::ServerError;
use leptos::prelude::*;
use server::Server;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Generate the list of routes in your Leptos App
    let subscriber = get_subscriber("echoes-of-ascension-server", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuation.");

    let application = Server::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
