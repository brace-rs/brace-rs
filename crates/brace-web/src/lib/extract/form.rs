use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use actix_http::encoding::Decoder;
use actix_http::error::{
    ErrorBadRequest, ErrorInternalServerError, ErrorPayloadTooLarge, ErrorUnsupportedMediaType,
};
use actix_http::http::header::CONTENT_LENGTH;
use actix_http::{Error, HttpMessage, Payload};
use actix_web::{FromRequest, HttpRequest};
use encoding_rs::UTF_8;
use futures::future::{err, Future};
use serde::de::DeserializeOwned;

use crate::parse::{UrlEncoded, UrlEncodedConfig, UrlEncodedError};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Form<T>(pub T);

impl<T> Form<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for Form<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Display for Form<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> FromRequest for Form<T>
where
    T: DeserializeOwned + 'static,
{
    type Config = FormConfig;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self, Error = Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        if req.content_type().to_lowercase() != "application/x-www-form-urlencoded" {
            return Box::new(err(handle_err(req, FormError::UnsupportedContentType)));
        }

        let len = content_length(req);
        let enc = match req.encoding() {
            Ok(enc) => enc,
            Err(_) => return Box::new(err(handle_err(req, FormError::UnknownEncoding))),
        };
        let cfg = req
            .app_data::<Self::Config>()
            .map(|cfg| cfg.clone().into())
            .unwrap_or_else(UrlEncodedConfig::default)
            .encoding(enc);

        if let Some(len) = len {
            if len > cfg.max_length {
                return Box::new(err(handle_err(req, FormError::PayloadTooLarge)));
            }
        }

        let request = req.clone();
        let payload = Decoder::from_headers(payload.take(), req.headers());

        Box::new(
            UrlEncoded::from_stream_with(cfg, payload)
                .from_err()
                .map(|enc| enc.to_value::<T>())
                .flatten()
                .map_err(move |err| handle_err(&request, err.into()))
                .map(Self),
        )
    }
}

#[derive(Clone)]
pub struct FormConfig {
    max_length: usize,
    max_depth: usize,
    strict: bool,
    ehandler: Option<Rc<dyn Fn(FormError, &HttpRequest) -> Error>>,
}

impl FormConfig {
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(FormError, &HttpRequest) -> Error + 'static,
    {
        self.ehandler = Some(Rc::new(f));
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        Self {
            max_length: 16_384,
            max_depth: 5,
            strict: true,
            ehandler: None,
        }
    }
}

impl From<FormConfig> for UrlEncodedConfig {
    fn from(from: FormConfig) -> Self {
        Self {
            max_length: from.max_length,
            max_depth: from.max_depth,
            strict: from.strict,
            encoding: UTF_8,
        }
    }
}

#[derive(Debug)]
pub enum FormError {
    InternalServerError,
    MalformedSyntax,
    PayloadTooLarge,
    UnknownEncoding,
    UnsupportedContentType,
}

impl From<UrlEncodedError> for FormError {
    fn from(from: UrlEncodedError) -> Self {
        match from {
            UrlEncodedError::Stream => FormError::InternalServerError,
            UrlEncodedError::Overflow => FormError::PayloadTooLarge,
            UrlEncodedError::Parse => FormError::MalformedSyntax,
        }
    }
}

impl From<FormError> for Error {
    fn from(from: FormError) -> Self {
        match from {
            FormError::InternalServerError => ErrorInternalServerError("Internal server error"),
            FormError::MalformedSyntax => ErrorBadRequest("Malformed syntax"),
            FormError::PayloadTooLarge => ErrorPayloadTooLarge("Payload too large"),
            FormError::UnknownEncoding => ErrorUnsupportedMediaType("Unknown encoding"),
            FormError::UnsupportedContentType => {
                ErrorUnsupportedMediaType("Unsupported content type")
            }
        }
    }
}

fn content_length(req: &HttpRequest) -> Option<usize> {
    if let Some(len) = req.headers().get(CONTENT_LENGTH) {
        if let Ok(len) = len.to_str() {
            if let Ok(len) = len.parse::<usize>() {
                return Some(len);
            }
        }
    }

    None
}

fn handle_err(req: &HttpRequest, err: FormError) -> Error {
    let err_handler = req
        .app_data::<FormConfig>()
        .map(|cfg| cfg.ehandler.clone())
        .unwrap_or(None);

    match err_handler {
        Some(err_handler) => (*err_handler)(err, &req),
        None => err.into(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::header::{CONTENT_LENGTH, CONTENT_TYPE};
    use actix_web::test::{block_on, TestRequest};
    use actix_web::FromRequest;
    use bytes::Bytes;
    use serde::Deserialize;

    use super::Form;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Info {
        hello: String,
        world: NestedInfo,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct NestedInfo {
        hello: String,
    }

    #[test]
    fn test_request_form_url_encoded() {
        let (req, mut pl) = TestRequest::default()
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(CONTENT_LENGTH, "33")
            .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
            .to_http_parts();

        let form = block_on(Form::<Info>::from_request(&req, &mut pl)).unwrap();
        let info = form.into_inner();

        assert_eq!(
            info,
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );
    }
}
