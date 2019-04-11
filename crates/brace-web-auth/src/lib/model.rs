use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
