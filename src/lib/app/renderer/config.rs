use serde::{Deserialize, Serialize};

use crate::app::theme::config::ThemeReferenceInfo;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RendererConfig {
    pub themes: Vec<ThemeReferenceInfo>,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self { themes: Vec::new() }
    }
}
