use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn hidden<S>(name: S) -> Hidden
where
    S: Into<String>,
{
    Hidden::new(name)
}

pub fn text<S>(name: S) -> Text
where
    S: Into<String>,
{
    Text::new(name)
}

pub fn textarea<S>(name: S) -> Textarea
where
    S: Into<String>,
{
    Textarea::new(name)
}

pub fn select<S>(name: S) -> Select
where
    S: Into<String>,
{
    Select::new(name)
}

pub fn datetime<S>(name: S) -> Datetime
where
    S: Into<String>,
{
    Datetime::new(name)
}

pub fn email<S>(name: S) -> Email
where
    S: Into<String>,
{
    Email::new(name)
}

pub fn password<S>(name: S) -> Password
where
    S: Into<String>,
{
    Password::new(name)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Field {
    Text(Text),
    Textarea(Textarea),
    Hidden(Hidden),
    Select(Select),
    Datetime(Datetime),
    Email(Email),
    Password(Password),
}

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub name: String,
    pub value: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}

impl Text {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
            label: None,
            description: None,
            placeholder: None,
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn placeholder<T>(mut self, placeholder: T) -> Self
    where
        T: Into<String>,
    {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl From<Text> for Field {
    fn from(field: Text) -> Self {
        Field::Text(field)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Textarea {
    pub name: String,
    pub value: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}

impl Textarea {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
            label: None,
            description: None,
            placeholder: None,
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn placeholder<T>(mut self, placeholder: T) -> Self
    where
        T: Into<String>,
    {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl From<Textarea> for Field {
    fn from(field: Textarea) -> Self {
        Field::Textarea(field)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Hidden {
    pub name: String,
    pub value: String,
}

impl Hidden {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }
}

impl From<Hidden> for Field {
    fn from(field: Hidden) -> Self {
        Field::Hidden(field)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Select {
    pub name: String,
    pub value: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub options: HashMap<String, String>,
}

impl Select {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
            label: None,
            description: None,
            options: HashMap::new(),
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn options(mut self, options: HashMap<String, String>) -> Self {
        self.options = options;
        self
    }
}

impl From<Select> for Field {
    fn from(field: Select) -> Self {
        Field::Select(field)
    }
}
#[derive(Serialize, Deserialize)]
pub struct Datetime {
    pub name: String,
    #[serde(with = "serde_datetime_utc")]
    pub value: DateTime<Utc>,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl Datetime {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: Utc::now(),
            label: None,
            description: None,
        }
    }

    pub fn value(mut self, value: DateTime<Utc>) -> Self {
        self.value = value;
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }
}

impl From<Datetime> for Field {
    fn from(field: Datetime) -> Self {
        Field::Datetime(field)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub name: String,
    pub value: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}

impl Email {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
            label: None,
            description: None,
            placeholder: None,
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn placeholder<T>(mut self, placeholder: T) -> Self
    where
        T: Into<String>,
    {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl From<Email> for Field {
    fn from(field: Email) -> Self {
        Field::Email(field)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub value: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub placeholder: Option<String>,
}

impl Password {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            value: "".to_owned(),
            label: None,
            description: None,
            placeholder: None,
        }
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.value = value.into();
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    pub fn placeholder<T>(mut self, placeholder: T) -> Self
    where
        T: Into<String>,
    {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl From<Password> for Field {
    fn from(field: Password) -> Self {
        Field::Password(field)
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
