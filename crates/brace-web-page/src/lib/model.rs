use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
