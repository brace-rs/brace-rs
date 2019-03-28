use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum TemplateInfo {
    Static { name: String, path: PathBuf },
    Tera { name: String, path: PathBuf },
    Text { name: String, text: String },
}

impl TemplateInfo {
    pub fn name(&self) -> &String {
        match self {
            TemplateInfo::Static { name, .. } => name,
            TemplateInfo::Tera { name, .. } => name,
            TemplateInfo::Text { name, .. } => name,
        }
    }
}
