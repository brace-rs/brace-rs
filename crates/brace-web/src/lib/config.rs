use std::fmt::{Display, Formatter, Result as FormatResult};
use std::net::Ipv4Addr;
use std::path::PathBuf;

use log::LevelFilter;
use serde::{Deserialize, Serialize};

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
    pub output: LogOutput,
    pub file: Option<PathBuf>,
}

impl Default for WebLogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Warn,
            format: r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#.to_string(),
            output: LogOutput::Stderr,
            file: None,
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

impl Into<LevelFilter> for LogLevel {
    fn into(self) -> LevelFilter {
        match self {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    Stderr,
    File,
}

impl Display for LogOutput {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        match self {
            LogOutput::Stdout => write!(f, "stdout"),
            LogOutput::Stderr => write!(f, "stderr"),
            LogOutput::File => write!(f, "file"),
        }
    }
}
