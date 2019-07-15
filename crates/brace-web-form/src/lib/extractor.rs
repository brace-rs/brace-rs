use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use actix_http::{Error, HttpMessage, Payload};
use actix_web::dev::Decompress;
use actix_web::error::UrlencodedError;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{FromRequest, HttpRequest};
use bytes::BytesMut;
use encoding_rs::{Encoding, UTF_8};
use futures::{Future, Poll, Stream};
use serde::de::DeserializeOwned;

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
        let (limit, err) = req
            .app_data::<FormConfig>()
            .map(|c| (c.limit, c.ehandler.clone()))
            .unwrap_or((16384, None));

        Box::new(
            UrlEncoded::new(req, payload)
                .limit(limit)
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
    ehandler: Option<Rc<Fn(UrlencodedError, &HttpRequest) -> Error>>,
}

impl FormConfig {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
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
            ehandler: None,
        }
    }
}

pub struct UrlEncoded<U> {
    stream: Option<Decompress<Payload>>,
    limit: usize,
    length: Option<usize>,
    encoding: &'static Encoding,
    err: Option<UrlencodedError>,
    fut: Option<Box<Future<Item = U, Error = UrlencodedError>>>,
}

impl<U> UrlEncoded<U> {
    pub fn new(req: &HttpRequest, payload: &mut Payload) -> UrlEncoded<U> {
        if req.content_type().to_lowercase() != "application/x-www-form-urlencoded" {
            return Self::err(UrlencodedError::ContentType);
        }

        let encoding = match req.encoding() {
            Ok(enc) => enc,
            Err(_) => return Self::err(UrlencodedError::ContentType),
        };

        let mut len = None;

        if let Some(l) = req.headers().get(CONTENT_LENGTH) {
            if let Ok(s) = l.to_str() {
                if let Ok(l) = s.parse::<usize>() {
                    len = Some(l)
                } else {
                    return Self::err(UrlencodedError::UnknownLength);
                }
            } else {
                return Self::err(UrlencodedError::UnknownLength);
            }
        };

        let payload = Decompress::from_headers(payload.take(), req.headers());

        UrlEncoded {
            encoding,
            stream: Some(payload),
            limit: 32_768,
            length: len,
            fut: None,
            err: None,
        }
    }

    fn err(e: UrlencodedError) -> Self {
        UrlEncoded {
            stream: None,
            limit: 32_768,
            fut: None,
            err: Some(e),
            length: None,
            encoding: UTF_8,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl<U> Future for UrlEncoded<U>
where
    U: DeserializeOwned + 'static,
{
    type Item = U;
    type Error = UrlencodedError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(ref mut fut) = self.fut {
            return fut.poll();
        }

        if let Some(err) = self.err.take() {
            return Err(err);
        }

        let limit = self.limit;
        if let Some(len) = self.length.take() {
            if len > limit {
                return Err(UrlencodedError::Overflow);
            }
        }

        let encoding = self.encoding;
        let fut = self
            .stream
            .take()
            .unwrap()
            .from_err()
            .fold(BytesMut::with_capacity(8192), move |mut body, chunk| {
                if (body.len() + chunk.len()) > limit {
                    Err(UrlencodedError::Overflow)
                } else {
                    body.extend_from_slice(&chunk);
                    Ok(body)
                }
            })
            .and_then(move |body| {
                if encoding == UTF_8 {
                    serde_qs::from_bytes::<U>(&body).map_err(|_| UrlencodedError::Parse)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlencodedError::Parse)?;

                    serde_qs::from_str::<U>(&body).map_err(|_| UrlencodedError::Parse)
                }
            });

        self.fut = Some(Box::new(fut));
        self.poll()
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::header::CONTENT_TYPE;
    use actix_web::test::{block_on, TestRequest};
    use bytes::Bytes;
    use serde::Deserialize;

    use super::*;

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

    fn eq(err: UrlencodedError, other: UrlencodedError) -> bool {
        match err {
            UrlencodedError::Overflow => match other {
                UrlencodedError::Overflow => true,
                _ => false,
            },
            UrlencodedError::UnknownLength => match other {
                UrlencodedError::UnknownLength => true,
                _ => false,
            },
            UrlencodedError::ContentType => match other {
                UrlencodedError::ContentType => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[test]
    fn test_urlencoded_error() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "xxxx")
                .to_http_parts();
        let info = block_on(UrlEncoded::<Info>::new(&req, &mut pl));
        assert!(eq(info.err().unwrap(), UrlencodedError::UnknownLength));

        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "1000000")
                .to_http_parts();
        let info = block_on(UrlEncoded::<Info>::new(&req, &mut pl));
        assert!(eq(info.err().unwrap(), UrlencodedError::Overflow));

        let (req, mut pl) = TestRequest::with_header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, "10")
            .to_http_parts();
        let info = block_on(UrlEncoded::<Info>::new(&req, &mut pl));
        assert!(eq(info.err().unwrap(), UrlencodedError::ContentType));
    }

    #[test]
    fn test_urlencoded() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "33")
                .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
                .to_http_parts();

        let info = block_on(UrlEncoded::<Info>::new(&req, &mut pl)).unwrap();
        assert_eq!(
            info,
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );

        let (req, mut pl) = TestRequest::with_header(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .header(CONTENT_LENGTH, "33")
        .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
        .to_http_parts();

        let info = block_on(UrlEncoded::<Info>::new(&req, &mut pl)).unwrap();
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
