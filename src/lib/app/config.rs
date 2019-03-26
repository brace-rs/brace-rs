use super::database::DatabaseConfig;
use super::renderer::RendererConfig;
use super::web::WebConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub web: WebConfig,
    pub database: DatabaseConfig,
    pub renderer: RendererConfig,
}

impl AppConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
        let string = std::fs::read_to_string(path)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn from_json(json: Value) -> Result<Self, failure::Error> {
        let config = serde_json::from_value(json)?;

        Ok(config)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
            database: DatabaseConfig::default(),
            renderer: RendererConfig::default(),
        }
    }
}
