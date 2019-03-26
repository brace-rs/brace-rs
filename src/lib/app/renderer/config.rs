use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RendererConfig {
    pub templates: String,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            templates: "./templates".to_string(),
        }
    }
}
