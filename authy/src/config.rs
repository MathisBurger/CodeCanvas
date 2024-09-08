use std::collections::HashMap;
use figment::{Error, Figment};
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub login_uri: String,
    pub service_locations: HashMap<String, String>
}

impl AppConfig {

    pub fn parse() -> Result<Self, Error> {
        let config: AppConfig = Figment::new()
            .merge(Toml::file("config.toml"))
            .merge(Env::raw())
            .extract()?;
        Ok(config)
    }
}