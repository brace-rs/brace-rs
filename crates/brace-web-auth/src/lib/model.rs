use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    #[serde(with = "serde_datetime_utc")]
    pub created: DateTime<Utc>,
    #[serde(with = "serde_datetime_utc")]
    pub updated: DateTime<Utc>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            email: "".to_string(),
            password: "".to_string(),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuth {
    pub email: String,
    pub password: String,
}

impl Default for UserAuth {
    fn default() -> Self {
        Self {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

mod serde_datetime_utc {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::de::Error;
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

        NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M")
            .map_err(Error::custom)
            .map(|datetime| DateTime::from_utc(datetime, Utc))
    }
}
