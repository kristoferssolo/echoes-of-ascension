use app::telemetry::{get_subscriber, init_subscriber};
use leptos::prelude::*;
use server::config::get_config;
use server::startup::{Application, ApplicationError};

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    // Generate the list of routes in your Leptos App
    let subscriber = get_subscriber("echoes-of-ascension-backend", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuation.");

    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
