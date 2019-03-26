use crate::app::database::DatabaseConfig;
use render::RendererConfig;
use serde::{Deserialize, Serialize};
use web::WebConfig;

pub mod log;
pub mod render;
pub mod web;

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
