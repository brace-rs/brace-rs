use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RendererConfig {
    pub templates: PathBuf,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            templates: PathBuf::from("templates"),
        }
    }
}
