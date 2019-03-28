use serde::{Deserialize, Serialize};

use self::asset::AssetInfo;

pub mod asset;

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryInfo {
    pub name: String,
    #[serde(rename = "asset", skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<AssetInfo>,
}

impl LibraryInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assets(&self) -> &Vec<AssetInfo> {
        &self.assets
    }
}
