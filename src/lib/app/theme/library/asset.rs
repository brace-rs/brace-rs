use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AssetInfo {
    #[serde(rename = "css")]
    StyleSheet { name: String, path: PathBuf },
    #[serde(rename = "js")]
    JavaScript { name: String, path: PathBuf },
}

impl AssetInfo {
    pub fn name(&self) -> &str {
        match self {
            AssetInfo::StyleSheet { name, .. } => name,
            AssetInfo::JavaScript { name, .. } => name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            AssetInfo::StyleSheet { path, .. } => path,
            AssetInfo::JavaScript { path, .. } => path,
        }
    }

    pub fn is_stylesheet(&self) -> bool {
        match self {
            AssetInfo::StyleSheet { .. } => true,
            AssetInfo::JavaScript { .. } => false,
        }
    }

    pub fn is_javascript(&self) -> bool {
        match self {
            AssetInfo::StyleSheet { .. } => false,
            AssetInfo::JavaScript { .. } => true,
        }
    }
}
