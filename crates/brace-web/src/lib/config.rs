use std::fmt::{Display, Formatter, Result as FormatResult};
use std::net::Ipv4Addr;

use brace_db::DatabaseConfig;
use brace_theme::config::ThemeReferenceInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub web: WebConfig,
    pub database: DatabaseConfig,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<ThemeReferenceInfo>,
}

impl AppConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, failure::Error> {
        let string = std::fs::read_to_string(path)?;
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    pub fn from_json(json: Value) -> Result<Self, failure::Error> {
        let config = serde_json::from_value(json)?;

        Ok(config)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
            database: DatabaseConfig::default(),
            themes: vec![ThemeReferenceInfo {
                name: Some("default".to_string()),
                path: PathBuf::from("themes/default/theme.toml"),
            }],
        }
    }
}

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
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
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
