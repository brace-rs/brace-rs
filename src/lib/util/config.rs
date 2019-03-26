use crate::app::config::Config;
use std::error::Error;

pub fn load(path: &str) -> Result<Config, Box<dyn Error + 'static>> {
    let string = std::fs::read_to_string(path)?;
    let config = toml::from_str(&string)?;

    Ok(config)
}
