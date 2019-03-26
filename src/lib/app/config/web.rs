use super::log::LogConfig;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct WebConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub log: LogConfig,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::new(127, 0, 0, 1),
            port: 8080,
            log: LogConfig::default(),
        }
    }
}
