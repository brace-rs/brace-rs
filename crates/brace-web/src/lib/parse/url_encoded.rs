use std::str::FromStr;

use bytes::{Bytes, BytesMut};
use encoding_rs::{Encoding, UTF_8};
use futures::{Future, Stream};
use serde::de::DeserializeOwned;
use serde_qs::Config;

pub struct UrlEncoded {
    val: Bytes,
    cfg: UrlEncodedConfig,
}

impl UrlEncoded {
    pub fn to_value<T>(&self) -> Result<T, UrlEncodedError>
    where
        T: DeserializeOwned,
    {
        Config::new(self.cfg.max_depth, self.cfg.strict)
            .deserialize_bytes::<T>(&self.val)
            .map_err(|_| UrlEncodedError::Parse)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, UrlEncodedError> {
        Self::from_bytes_with(UrlEncodedConfig::default(), bytes)
    }

    pub fn from_bytes_with(cfg: UrlEncodedConfig, bytes: &[u8]) -> Result<Self, UrlEncodedError> {
        if bytes.len() > cfg.max_length {
            return Err(UrlEncodedError::Overflow);
        }

        Ok(Self {
            val: Bytes::from(bytes),
            cfg,
        })
    }

    pub fn from_str_with(cfg: UrlEncodedConfig, str: &str) -> Result<Self, UrlEncodedError> {
        Self::from_bytes_with(cfg, str.as_bytes())
    }

    pub fn from_stream<S, E>(stream: S) -> impl Future<Item = Self, Error = UrlEncodedError>
    where
        S: Stream<Item = Bytes, Error = E>,
    {
        Self::from_stream_with(UrlEncodedConfig::default(), stream)
    }

    pub fn from_stream_with<S, E>(
        cfg: UrlEncodedConfig,
        stream: S,
    ) -> impl Future<Item = Self, Error = UrlEncodedError>
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
                    UrlEncoded::from_bytes_with(cfg, &body)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlEncodedError::Parse)?;

                    UrlEncoded::from_str_with(cfg, &body)
                }
            })
    }
}

impl FromStr for UrlEncoded {
    type Err = UrlEncodedError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::from_bytes_with(UrlEncodedConfig::default(), str.as_bytes())
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
        let info = UrlEncoded::from_str_with(conf, data)
            .unwrap()
            .to_value::<Info>();

        assert!(info.is_ok());
    }

    #[test]
    fn test_from_bytes() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::from_bytes_with(conf, data)
            .unwrap()
            .to_value::<Info>();

        assert!(info.is_ok());
    }

    #[test]
    fn test_from_stream() {
        let (mut sender, payload) = Payload::create(false);

        sender.feed_data(Bytes::from("hello=world&world[hello]=universe"));
        sender.feed_eof();

        let conf = UrlEncodedConfig::default();
        let info = block_on(UrlEncoded::from_stream_with(conf, payload))
            .unwrap()
            .to_value::<Info>();

        assert!(info.is_ok());
    }

    #[test]
    fn test_nesting_flat() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default().max_depth(0);
        let info = UrlEncoded::from_bytes_with(conf, data)
            .unwrap()
            .to_value::<Info>();

        assert!(info.is_err());
    }

    #[test]
    fn test_nesting_shallow() {
        let data = b"hello=world&world[hello]=universe";
        let conf = UrlEncodedConfig::default().max_depth(1);
        let info = UrlEncoded::from_bytes_with(conf, data)
            .unwrap()
            .to_value::<Info>();

        assert_eq!(
            info.unwrap(),
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
        let info = UrlEncoded::from_bytes_with(conf, data)
            .unwrap()
            .to_value::<Info>();

        assert_eq!(
            info.unwrap(),
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );
    }
}
