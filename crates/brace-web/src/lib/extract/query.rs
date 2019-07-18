use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use actix_http::error::{Error, ErrorBadRequest, ErrorInternalServerError, ErrorUriTooLong};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use encoding_rs::UTF_8;
use serde::de::DeserializeOwned;

use crate::parse::{UrlEncoded, UrlEncodedConfig, UrlEncodedError};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Query<T>(pub T);

impl<T> Query<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for Query<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Display for Query<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> FromRequest for Query<T>
where
    T: DeserializeOwned,
{
    type Config = QueryConfig;
    type Error = Error;
    type Future = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cfg = req
            .app_data::<Self::Config>()
            .map(|cfg| cfg.clone().into())
            .unwrap_or_else(UrlEncodedConfig::default);

        match UrlEncoded::from_str_with(cfg, req.query_string()) {
            Ok(enc) => match enc.to_value::<T>() {
                Ok(val) => Ok(Self(val)),
                Err(err) => Err(handle_err(&req, err.into())),
            },
            Err(err) => Err(handle_err(&req, err.into())),
        }
    }
}

#[derive(Clone)]
pub struct QueryConfig {
    max_length: usize,
    max_depth: usize,
    strict: bool,
    ehandler: Option<Rc<Fn(QueryError, &HttpRequest) -> Error>>,
}

impl QueryConfig {
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
        F: Fn(QueryError, &HttpRequest) -> Error + 'static,
    {
        self.ehandler = Some(Rc::new(f));
        self
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            max_length: 16_384,
            max_depth: 5,
            strict: true,
            ehandler: None,
        }
    }
}

impl From<QueryConfig> for UrlEncodedConfig {
    fn from(from: QueryConfig) -> Self {
        Self {
            max_length: from.max_length,
            max_depth: from.max_depth,
            strict: from.strict,
            encoding: UTF_8,
        }
    }
}

#[derive(Debug)]
pub enum QueryError {
    InternalServerError,
    MalformedSyntax,
    QueryTooLong,
}

impl From<UrlEncodedError> for QueryError {
    fn from(from: UrlEncodedError) -> Self {
        match from {
            UrlEncodedError::Stream => QueryError::InternalServerError,
            UrlEncodedError::Overflow => QueryError::QueryTooLong,
            UrlEncodedError::Parse => QueryError::MalformedSyntax,
        }
    }
}

impl From<QueryError> for Error {
    fn from(from: QueryError) -> Self {
        match from {
            QueryError::InternalServerError => ErrorInternalServerError("Internal server error"),
            QueryError::MalformedSyntax => ErrorBadRequest("Malformed syntax"),
            QueryError::QueryTooLong => ErrorUriTooLong("Query too long"),
        }
    }
}

fn handle_err(req: &HttpRequest, err: QueryError) -> Error {
    let err_handler = req
        .app_data::<QueryConfig>()
        .map(|cfg| cfg.ehandler.clone())
        .unwrap_or(None);

    match err_handler {
        Some(err_handler) => (*err_handler)(err, &req),
        None => err.into(),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::test::{block_on, TestRequest};
    use actix_web::FromRequest;
    use serde::Deserialize;

    use super::Query;

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
    fn test_request_query_string() {
        let (req, mut pl) =
            TestRequest::with_uri("/path?hello=world&world[hello]=universe").to_http_parts();

        let query = block_on(Query::<Info>::from_request(&req, &mut pl)).unwrap();
        let inner = query.into_inner();

        assert_eq!(
            inner,
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );
    }
}
