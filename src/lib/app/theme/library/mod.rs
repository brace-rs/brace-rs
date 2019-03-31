use serde::{Deserialize, Serialize};

use self::resource::ResourceInfo;

pub mod resource;

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryInfo {
    pub name: String,
    #[serde(rename = "resource", skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceInfo>,
}

impl LibraryInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn resources(&self) -> &Vec<ResourceInfo> {
        &self.resources
    }
}
