use serde::{Deserialize, Serialize};

pub fn submit<U>(url: U) -> Submit
where
    U: Into<String>,
{
    Submit::new(url)
}

pub fn cancel<U>(url: U) -> Cancel
where
    U: Into<String>,
{
    Cancel::new(url)
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Submit(Submit),
    Cancel(Cancel),
}

#[derive(Serialize, Deserialize)]
pub struct Submit {
    pub name: Option<String>,
    pub label: String,
    pub url: String,
    pub weight: i32,
}

impl Submit {
    pub fn new<U>(url: U) -> Self
    where
        U: Into<String>,
    {
        Self {
            name: None,
            label: "Submit".to_owned(),
            url: url.into(),
            weight: 0,
        }
    }

    pub fn name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = label.into();
        self
    }

    pub fn weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

impl From<Submit> for Action {
    fn from(action: Submit) -> Self {
        Action::Submit(action)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cancel {
    pub label: String,
    pub url: String,
    pub weight: i32,
}

impl Cancel {
    pub fn new<U>(url: U) -> Self
    where
        U: Into<String>,
    {
        Self {
            label: "Cancel".to_owned(),
            url: url.into(),
            weight: 0,
        }
    }

    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<String>,
    {
        self.label = label.into();
        self
    }

    pub fn weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

impl From<Cancel> for Action {
    fn from(action: Cancel) -> Self {
        Action::Cancel(action)
    }
}
