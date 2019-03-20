use crate::web::config::Config as WebConfig;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

pub fn load(path: &str) -> Result<Config, Box<dyn Error + 'static>> {
    let string = std::fs::read_to_string(path)?;
    let config = toml::from_str(&string)?;

    Ok(config)
}
