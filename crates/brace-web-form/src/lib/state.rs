use failure::{format_err, Error};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct FormState(Value);

impl FormState {
    pub fn new() -> Self {
        Self(Value::Null)
    }

    pub fn with<S>(state: S) -> Result<Self, Error>
    where
        S: Serialize,
    {
        Ok(Self(to_value(state)?))
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match &self.0 {
            Value::Object(map) => match map.get(key) {
                Some(value) => from_value(value.clone()).map_err(Error::from),
                None => Err(format_err!("form state does not contain key {}", key)),
            },
            _ => Err(format_err!("form state does not contain key {}", key)),
        }
    }
}

impl Default for FormState {
    fn default() -> Self {
        Self(Value::Null)
    }
}
