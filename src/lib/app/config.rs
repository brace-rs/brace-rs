use super::database::DatabaseConfig;
use super::renderer::RendererConfig;
use super::web::WebConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub web: WebConfig,
    pub database: DatabaseConfig,
    pub renderer: RendererConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
            database: DatabaseConfig::default(),
            renderer: RendererConfig::default(),
        }
    }
}
