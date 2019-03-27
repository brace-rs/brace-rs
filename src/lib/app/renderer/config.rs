use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RendererConfig {
    pub theme: PathBuf,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            theme: PathBuf::from("theme/Theme.toml"),
        }
    }
}
