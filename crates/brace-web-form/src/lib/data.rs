use actix_web::dev::Payload;
use actix_web::error::{Error as WebError, PayloadError};
use actix_web::web::Form as FormExtractor;
use actix_web::{FromRequest, HttpRequest};
use bytes::Bytes;
use failure::{format_err, Error};
use futures::future::{ok, Future};
use futures::stream::Stream;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct FormData(Value);

impl FormData {
    pub fn new() -> Self {
        Self(Value::Null)
    }

    pub fn with<T>(data: T) -> Result<Self, Error>
    where
        T: Serialize,
    {
        Ok(Self(to_value(data)?))
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match &self.0 {
            Value::Object(map) => match map.get(key) {
                Some(value) => from_value(value.clone()).map_err(Error::from),
                None => Err(format_err!("form data does not contain key {}", key)),
            },
            _ => Err(format_err!("form data does not contain key {}", key)),
        }
    }
}

impl Default for FormData {
    fn default() -> Self {
        Self(Value::Null)
    }
}

impl<P> FromRequest<P> for FormData
where
    P: Stream<Item = Bytes, Error = PayloadError> + 'static,
{
    type Error = WebError;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload<P>) -> Self::Future {
        Box::new(
            FormExtractor::<Value>::from_request(req, payload)
                .and_then(|data| ok(Self(data.into_inner()))),
        )
    }
}
