use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::library::LibraryInfo;
use super::resource::ResourceInfo;
use super::template::TemplateInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ManifestConfig {
    pub manifest: ManifestInfo,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub libraries: Vec<LibraryInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub templates: Vec<TemplateInfo>,
}

impl Default for ManifestConfig {
    fn default() -> Self {
        Self {
            manifest: ManifestInfo::default(),
            libraries: Vec::new(),
            resources: Vec::new(),
            templates: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ManifestInfo {
    pub name: String,
    pub label: String,
    pub description: String,
}

impl Default for ManifestInfo {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            label: "Default".to_string(),
            description: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ManifestReferenceInfo {
    pub name: Option<String>,
    pub path: PathBuf,
}
