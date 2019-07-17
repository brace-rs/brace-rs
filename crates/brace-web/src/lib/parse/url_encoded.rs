use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Deref, DerefMut};

use bytes::{Bytes, BytesMut};
use encoding_rs::{Encoding, UTF_8};
use futures::{Future, Stream};
use serde::de::DeserializeOwned;
use serde_qs::Config;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct UrlEncoded<T>(pub T);

impl<T> UrlEncoded<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for UrlEncoded<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for UrlEncoded<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Debug for UrlEncoded<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Display for UrlEncoded<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> UrlEncoded<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(cfg: UrlEncodedConfig, bytes: &[u8]) -> Result<Self, UrlEncodedError> {
        if bytes.len() > cfg.max_length {
            return Err(UrlEncodedError::Overflow);
        }

        Config::new(cfg.max_depth, cfg.strict)
            .deserialize_bytes::<T>(bytes)
            .map_err(|_| UrlEncodedError::Parse)
            .map(UrlEncoded)
    }

    pub fn from_str(cfg: UrlEncodedConfig, str: &str) -> Result<Self, UrlEncodedError> {
        Self::from_bytes(cfg, str.as_bytes())
    }

    pub fn from_stream<S, E>(
        cfg: UrlEncodedConfig,
        stream: S,
    ) -> impl Future<Item = UrlEncoded<T>, Error = UrlEncodedError>
    where
        S: Stream<Item = Bytes, Error = E>,
    {
        let max_length = cfg.max_length;
        let encoding = cfg.encoding;

        stream
            .map_err(|_| UrlEncodedError::Stream)
            .fold(BytesMut::with_capacity(8192), move |mut body, chunk| {
                if (body.len() + chunk.len()) > max_length {
                    Err(UrlEncodedError::Overflow)
                } else {
                    body.extend_from_slice(&chunk);

                    Ok(body)
                }
            })
            .and_then(move |body| {
                if encoding == UTF_8 {
                    UrlEncoded::from_bytes(cfg, &body)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlEncodedError::Parse)?;

                    UrlEncoded::from_str(cfg, &body)
                }
            })
    }
}

#[derive(Clone)]
pub struct UrlEncodedConfig {
    pub(crate) max_length: usize,
    pub(crate) max_depth: usize,
    pub(crate) strict: bool,
    pub(crate) encoding: &'static Encoding,
}

impl UrlEncodedConfig {
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

    pub fn encoding(mut self, encoding: &'static Encoding) -> Self {
        self.encoding = encoding;
        self
    }
}

impl Default for UrlEncodedConfig {
    fn default() -> Self {
        Self {
            max_length: 32_768,
            max_depth: 5,
            strict: true,
            encoding: UTF_8,
        }
    }
}

#[derive(Debug)]
pub enum UrlEncodedError {
    Stream,
    Overflow,
    Parse,
}

#[cfg(test)]
mod tests {
    use actix_http::h1::Payload;
    use actix_web::test::block_on;
    use bytes::Bytes;
    use serde::Deserialize;

    use super::{UrlEncoded, UrlEncodedConfig};

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
    fn test_from_str() {
        let data = "hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::<Info>::from_str(conf, data);

        assert!(info.is_ok());
    }

    #[test]
    fn test_from_bytes() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::<Info>::from_bytes(conf, data);

        assert!(info.is_ok());
    }

    #[test]
    fn test_from_stream() {
        let (mut sender, payload) = Payload::create(false);

        sender.feed_data(Bytes::from("hello=world&world[hello]=universe"));
        sender.feed_eof();

        let conf = UrlEncodedConfig::default();
        let info = block_on(UrlEncoded::<Info>::from_stream(conf, payload));

        assert!(info.is_ok());
    }

    #[test]
    fn test_nesting_flat() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default().max_depth(0);
        let info = UrlEncoded::<Info>::from_bytes(conf, data);

        assert!(info.is_err());
    }

    #[test]
    fn test_nesting_shallow() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default().max_depth(1);
        let info = UrlEncoded::<Info>::from_bytes(conf, data);

        assert_eq!(
            info.unwrap().into_inner(),
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );
    }

    #[test]
    fn test_nesting_deep() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default().max_depth(2);
        let info = UrlEncoded::<Info>::from_bytes(conf, data);

        assert_eq!(
            info.unwrap().into_inner(),
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );
    }
}
