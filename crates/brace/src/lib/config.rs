use std::path::{Path, PathBuf};

use brace_db::DatabaseConfig;
use brace_theme::config::ThemeReferenceInfo;
use brace_web::config::WebConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
