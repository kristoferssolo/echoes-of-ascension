use backend::{
    config::get_config,
    server::Server,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Generate the list of routes in your Leptos App
    let subscriber = get_subscriber("echoes-of-ascension-server", "info", std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read configuation.");
    //
    let application = Server::build(config).await?;
    application.run_until_stopped().await?;
    Ok(())
}
