use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
}

impl LibraryInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn resources(&self) -> &Vec<String> {
        &self.resources
    }
}
