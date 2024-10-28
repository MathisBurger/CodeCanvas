use figment::providers::{Env, Format, Toml};
use figment::{Error, Figment};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub admin_restricted_services: Vec<String>,
    pub login_uri: String,
    pub service_locations: HashMap<String, String>,
    pub jwt_secret: String,
    pub allowed_origin: String,
    pub github_api_key: Option<String>,
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
