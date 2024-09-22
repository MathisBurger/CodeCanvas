use figment::providers::{Env, Format, Toml};
use figment::{Error, Figment};
use serde::Deserialize;

/// The generic app config
#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
    pub usernator_grpc: String,
}

impl AppConfig {
    /// Parses the app config
    pub fn parse() -> Result<Self, Error> {
        let config: AppConfig = Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::raw())
            .extract()?;
        Ok(config)
    }
}
