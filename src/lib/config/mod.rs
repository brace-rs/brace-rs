use db::DatabaseConfig;
use serde::{Deserialize, Serialize};
use web::WebConfig;

pub mod db;
pub mod log;
pub mod web;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub web: WebConfig,
    pub database: DatabaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
            database: DatabaseConfig::default(),
        }
    }
}
