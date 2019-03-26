use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct LogConfig {
    pub level: LogLevel,
    pub format: String,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Warn,
            format: r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::Off => write!(f, "off"),
            LogLevel::Error => write!(f, "error"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Trace => write!(f, "trace"),
        }
    }
}
