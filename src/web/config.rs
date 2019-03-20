use serde::Deserialize;
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub log: LogConfig,
}

#[derive(Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

pub fn load(path: &str) -> Result<Config, Box<dyn Error + 'static>> {
    let string = std::fs::read_to_string(path)?;
    let config = toml::from_str(&string)?;

    Ok(config)
}
