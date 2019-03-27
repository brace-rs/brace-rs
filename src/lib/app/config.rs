use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::database::DatabaseConfig;
use super::theme::config::ThemeReferenceInfo;
use super::web::WebConfig;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub web: WebConfig,
    pub database: DatabaseConfig,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub theme: Vec<ThemeReferenceInfo>,
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
            theme: vec![ThemeReferenceInfo {
                name: Some("default".to_string()),
                path: PathBuf::from("themes/default/Theme.toml"),
            }],
        }
    }
}
