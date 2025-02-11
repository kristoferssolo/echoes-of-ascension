pub mod application;
pub mod database;
pub mod environment;

use application::ApplicationSettings;
pub use database::DatabaseSettings;
use environment::Environment;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

/// Get the configuration settings for the application.
///
/// # Panics
///
/// This function may panic in the following cases:
///
/// - If the current directory cannot be determined. This is highly unusual.
/// - If the `APP_ENVIRONMENT` environment variable is set to an invalid value
///   that cannot be converted to an `Environment` enum variant.
///
/// # Errors
///
/// This function returns an error if:
///
/// - Any of the configuration files (`base.toml`, `{environment}.toml`) cannot be read or parsed.
/// - Environment variables prefixed with `APP_` cannot be read or parsed.
/// - The resulting configuration cannot be deserialized into the `Settings` struct.
pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let config_directory = base_path.join("config");
    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let env_filename = format!("{}.toml", &env);

    let settings = config::Config::builder()
        .add_source(config::File::from(config_directory.join("base.toml")))
        .add_source(config::File::from(config_directory.join(env_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}
