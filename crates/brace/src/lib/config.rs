use std::path::PathBuf;

use brace_db::DatabaseConfig;
use brace_theme::config::ThemeReferenceInfo;
use brace_web::config::WebConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AppConfig {
    pub web: WebConfig,
    pub database: DatabaseConfig,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<ThemeReferenceInfo>,
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
