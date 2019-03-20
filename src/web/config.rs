use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,
    pub log: LogConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::new(0, 0, 0, 0),
            port: 80,
            log: LogConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "warn".to_string(),
            format: r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#.to_string(),
        }
    }
}
