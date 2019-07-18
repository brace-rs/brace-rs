use std::collections::HashMap;
use std::fmt::{Display, Error as FmtError, Formatter, Result as FmtResult};
use std::str::FromStr;

use brace_config::{from_value, Value};
use bytes::{Bytes, BytesMut};
use encoding_rs::{Encoding, UTF_8};
use futures::{Future, Stream};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_qs::{to_string, Config};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UrlEncoded(HashMap<String, Value>);

impl UrlEncoded {
    pub fn to_value<T>(&self) -> Result<T, UrlEncodedError>
    where
        T: DeserializeOwned,
    {
        from_value::<T>(Value::from(self.0.clone())).map_err(|_| UrlEncodedError::Parse)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, UrlEncodedError> {
        Self::from_bytes_with(UrlEncodedConfig::default(), bytes)
    }

    pub fn from_bytes_with(cfg: UrlEncodedConfig, bytes: &[u8]) -> Result<Self, UrlEncodedError> {
        if bytes.len() > cfg.max_length {
            return Err(UrlEncodedError::Overflow);
        }

        Config::new(cfg.max_depth, cfg.strict)
            .deserialize_bytes::<HashMap<String, Value>>(bytes)
            .map_err(|_| UrlEncodedError::Parse)
            .map(Self)
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

impl Display for UrlEncoded {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match to_string(&self.0) {
            Ok(str) => write!(f, "{}", str),
            Err(_) => Err(FmtError),
        }
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
    use std::collections::HashMap;

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

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestInt {
        one: usize,
        two: isize,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestList {
        list: Vec<isize>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestMap {
        map: HashMap<String, TestList>,
    }

    #[test]
    fn test_value_integer() {
        let data = "one=1&two=-5";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::from_str_with(conf, data)
            .unwrap()
            .to_value::<TestInt>();

        assert_eq!(info.unwrap(), TestInt { one: 1, two: -5 });
    }

    #[test]
    fn test_value_list() {
        let data = "list[0]=1&list[1]=-5";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::from_str_with(conf, data)
            .unwrap()
            .to_value::<TestList>();

        assert_eq!(info.unwrap(), TestList { list: vec![1, -5] });
    }

    #[test]
    fn test_value_map() {
        let data = "map[a][list][0]=7&map[a][list][1]=-55&map[b][list][0]=2563";
        let conf = UrlEncodedConfig::default();
        let info = UrlEncoded::from_str_with(conf, data)
            .unwrap()
            .to_value::<TestMap>();

        assert_eq!(
            info.unwrap(),
            TestMap {
                map: {
                    let mut map = HashMap::new();
                    map.insert("a".to_owned(), TestList { list: vec![7, -55] });
                    map.insert("b".to_owned(), TestList { list: vec![2563] });
                    map
                },
            },
        );
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

    #[test]
    fn test_parse() {
        let str = "hello=world&world[hello]=universe";
        let enc: UrlEncoded = str.parse().unwrap();
        let val = enc.to_value::<Info>().unwrap();

        assert_eq!(
            val,
            Info {
                hello: "world".to_owned(),
                world: NestedInfo {
                    hello: "universe".to_owned(),
                },
            }
        );

        assert_eq!(enc.to_string(), "hello=world&world[hello]=universe");
    }
}
