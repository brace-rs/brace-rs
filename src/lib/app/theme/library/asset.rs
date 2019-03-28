use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AssetInfo {
    #[serde(rename = "css")]
    StyleSheet(StyleSheetInfo),
    #[serde(rename = "js")]
    JavaScript(JavaScriptInfo),
}

impl AssetInfo {
    pub fn name(&self) -> &str {
        match self {
            AssetInfo::StyleSheet(ref info) => &info.name,
            AssetInfo::JavaScript(ref info) => &info.name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            AssetInfo::StyleSheet(ref info) => &info.path,
            AssetInfo::JavaScript(ref info) => &info.path,
        }
    }

    pub fn as_stylesheet(&self) -> Option<&StyleSheetInfo> {
        match self {
            AssetInfo::StyleSheet(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn is_stylesheet(&self) -> bool {
        match self {
            AssetInfo::StyleSheet(_) => true,
            AssetInfo::JavaScript(_) => false,
        }
    }

    pub fn as_javascript(&self) -> Option<&JavaScriptInfo> {
        match self {
            AssetInfo::JavaScript(ref info) => Some(info),
            _ => None,
        }
    }

    pub fn is_javascript(&self) -> bool {
        match self {
            AssetInfo::StyleSheet(_) => false,
            AssetInfo::JavaScript(_) => true,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StyleSheetInfo {
    pub name: String,
    pub path: PathBuf,
}

impl StyleSheetInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JavaScriptInfo {
    pub name: String,
    pub path: PathBuf,
}

impl JavaScriptInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
