use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

use crate::util::log::LogLevel;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct WebConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub log: WebLogConfig,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::new(127, 0, 0, 1),
            port: 8080,
            log: WebLogConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct WebLogConfig {
    pub level: LogLevel,
    pub format: String,
}

impl Default for WebLogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Warn,
            format: r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#.to_string(),
        }
    }
}
