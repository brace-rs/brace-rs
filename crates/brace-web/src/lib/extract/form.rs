use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use actix_http::{Error, Payload};
use actix_web::error::UrlencodedError;
use actix_web::{FromRequest, HttpRequest};
use futures::future::Future;
use serde::de::DeserializeOwned;

use crate::parse::UrlEncoded;

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

impl<T> FromRequest for Form<T>
where
    T: DeserializeOwned + 'static,
{
    type Config = FormConfig;
    type Error = Error;
    type Future = Box<Future<Item = Self, Error = Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req2 = req.clone();
        let (limit, depth, strict, err) = req
            .app_data::<FormConfig>()
            .map(|c| (c.limit, c.depth, c.strict, c.ehandler.clone()))
            .unwrap_or((16384, 5, true, None));

        Box::new(
            UrlEncoded::new(req, payload)
                .limit(limit)
                .depth(depth)
                .strict(strict)
                .map_err(move |e| {
                    if let Some(err) = err {
                        (*err)(e, &req2)
                    } else {
                        e.into()
                    }
                })
                .map(Form),
        )
    }
}

impl<T: Debug> Debug for Form<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Display> Display for Form<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub struct FormConfig {
    limit: usize,
    depth: usize,
    strict: bool,
    ehandler: Option<Rc<Fn(UrlencodedError, &HttpRequest) -> Error>>,
}

impl FormConfig {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(UrlencodedError, &HttpRequest) -> Error + 'static,
    {
        self.ehandler = Some(Rc::new(f));
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        FormConfig {
            limit: 16384,
            depth: 5,
            strict: true,
            ehandler: None,
        }
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
    fn test_form() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "33")
                .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
                .to_http_parts();

        let s = block_on(Form::<Info>::from_request(&req, &mut pl)).unwrap();
        assert_eq!(s.hello, "world");
        assert_eq!(s.world.hello, "universe")
    }
}
