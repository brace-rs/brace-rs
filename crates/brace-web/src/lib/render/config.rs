use brace_theme::config::ThemeReferenceInfo;
use serde::{Deserialize, Serialize};

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
