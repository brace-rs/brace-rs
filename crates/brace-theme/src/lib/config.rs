use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::manifest::ManifestReferenceInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ThemeConfig {
    pub theme: ThemeInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub manifests: Vec<ManifestReferenceInfo>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            theme: ThemeInfo::default(),
            manifests: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ThemeInfo {
    pub name: String,
    pub label: String,
    pub description: String,
}

impl Default for ThemeInfo {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            label: "Default".to_string(),
            description: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ThemeReferenceInfo {
    pub name: Option<String>,
    pub path: PathBuf,
}
