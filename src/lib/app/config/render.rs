use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
