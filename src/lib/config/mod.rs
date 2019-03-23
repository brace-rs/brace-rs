use serde::{Deserialize, Serialize};
use web::WebConfig;

pub mod log;
pub mod web;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub web: WebConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
        }
    }
}
