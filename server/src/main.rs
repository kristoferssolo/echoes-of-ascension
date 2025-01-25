mod application;
mod routes;

use app::{
    config::get_config,
    startup::ApplicationError,
    telemetry::{get_subscriber, init_subscriber},
};
use application::Server;
use leptos::prelude::*;

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Generate the list of routes in your Leptos App
    let subscriber = get_subscriber("echoes-of-ascension-server", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuation.");

    let application = Server::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
