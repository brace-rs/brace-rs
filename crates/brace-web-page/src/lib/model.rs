use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Page {
    pub id: Uuid,
    #[serde(deserialize_with = "serde_option_uuid::deserialize")]
    pub parent: Option<Uuid>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub document: Value,
    #[serde(with = "serde_datetime_utc")]
    pub created: DateTime<Utc>,
    #[serde(with = "serde_datetime_utc")]
    pub updated: DateTime<Utc>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            slug: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            document: json!({}),
            created: Utc::now(),
            updated: Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageWithPath {
    pub id: Uuid,
    #[serde(deserialize_with = "serde_option_uuid::deserialize")]
    pub parent: Option<Uuid>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub document: Value,
    #[serde(with = "serde_datetime_utc")]
    pub created: DateTime<Utc>,
    #[serde(with = "serde_datetime_utc")]
    pub updated: DateTime<Utc>,
    pub path: String,
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

mod serde_option_uuid {
    use serde::de::{Deserialize, Deserializer, Error};
    use uuid::Uuid;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Uuid>, D::Error> {
        let string: Option<String> = Deserialize::deserialize(deserializer)?;

        if let Some(string) = string {
            if string.is_empty() {
                Ok(None)
            } else {
                Uuid::parse_str(&string).map_err(Error::custom).map(Some)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::Page;

    #[test]
    fn test_serde_page_default() {
        let src = Page::default();
        let str = serde_json::to_string(&src).unwrap();
        let out: Page = serde_json::from_str(&str).unwrap();

        assert!(out.parent.is_none());
    }

    #[test]
    fn test_serde_page_custom() {
        let src = Page::default();
        let mut val = serde_json::to_value(&src).unwrap();

        val["parent"] = Value::String("".to_string());

        let str = serde_json::to_string(&val).unwrap();
        let out: Page = serde_json::from_str(&str).unwrap();

        assert!(out.parent.is_none());
    }
}
