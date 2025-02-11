use echoes_of_ascension::{
    config::get_config,
    middleware::telemetry::{get_subscriber, init_subscriber},
    startup::Application,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("echoes_of_ascension", "info", std::io::stdout);
    init_subscriber(subscriber);
    let config = get_config().expect("Failed to read configuation.");
    let application = Application::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
