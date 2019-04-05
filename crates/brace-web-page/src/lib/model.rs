use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "".to_string(),
            content: "".to_string(),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}
