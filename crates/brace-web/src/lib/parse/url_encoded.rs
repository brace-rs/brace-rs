use actix_http::{HttpMessage, Payload};
use actix_web::dev::Decompress;
use actix_web::error::UrlencodedError;
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::HttpRequest;
use bytes::BytesMut;
use encoding_rs::{Encoding, UTF_8};
use futures::{Future, Poll, Stream};
use serde::de::DeserializeOwned;
use serde_qs::Config;

pub struct UrlEncoded<U> {
    stream: Option<Decompress<Payload>>,
    limit: usize,
    depth: usize,
    strict: bool,
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
            depth: 5,
            strict: true,
            length: len,
            fut: None,
            err: None,
        }
    }

    fn err(e: UrlencodedError) -> Self {
        UrlEncoded {
            stream: None,
            limit: 32_768,
            depth: 5,
            strict: true,
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

    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
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

        let depth = self.depth;
        let strict = self.strict;
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
                    Config::new(depth, strict)
                        .deserialize_bytes::<U>(&body)
                        .map_err(|_| UrlencodedError::Parse)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlencodedError::Parse)?;

                    Config::new(depth, strict)
                        .deserialize_str::<U>(&body)
                        .map_err(|_| UrlencodedError::Parse)
                }
            });

        self.fut = Some(Box::new(fut));
        self.poll()
    }
}

#[cfg(test)]
mod tests {
    use actix_web::error::UrlencodedError;
    use actix_web::http::header::{CONTENT_LENGTH, CONTENT_TYPE};
    use actix_web::test::{block_on, TestRequest};
    use bytes::Bytes;
    use serde::Deserialize;

    use super::UrlEncoded;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Info {
        hello: String,
        world: NestedInfo,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct NestedInfo {
        hello: String,
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

    #[test]
    fn test_depth_flat() {
        let (req, mut pl) = TestRequest::default()
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(CONTENT_LENGTH, "33")
            .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
            .to_http_parts();

        let encoded = UrlEncoded::<Info>::new(&req, &mut pl).depth(0);
        let info = block_on(encoded);

        assert!(info.is_err());
    }

    #[test]
    fn test_depth_shallow() {
        let (req, mut pl) = TestRequest::default()
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(CONTENT_LENGTH, "33")
            .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
            .to_http_parts();

        let encoded = UrlEncoded::<Info>::new(&req, &mut pl).depth(1);
        let info = block_on(encoded);

        assert!(info.is_ok());
    }

    #[test]
    fn test_depth_deep() {
        let (req, mut pl) = TestRequest::default()
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(CONTENT_LENGTH, "33")
            .set_payload(Bytes::from_static(b"hello=world&world[hello]=universe"))
            .to_http_parts();

        let encoded = UrlEncoded::<Info>::new(&req, &mut pl).depth(2);
        let info = block_on(encoded);

        assert!(info.is_ok());
    }
}
