use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(with = "datetime")]
    pub created: DateTime<Utc>,
    #[serde(with = "datetime")]
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

mod datetime {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(
        datetime: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        datetime
            .format("%Y-%m-%dT%H:%M")
            .to_string()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        let datetime: String = Deserialize::deserialize(deserializer)?;

        Ok(NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M")
            .map_err(serde::de::Error::custom)
            .map(|datetime| DateTime::from_utc(datetime, Utc))?)
    }
}
