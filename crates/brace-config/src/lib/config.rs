use std::collections::HashMap;
use std::path::Path;

use failure::{format_err, Error};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use super::file::load_from_file;

#[derive(Debug, Clone)]
pub struct Config {
    config: HashMap<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<&mut Config, Error>
    where
        T: Serialize,
    {
        let value = serde_json::to_value(value)?;

        self.config.insert(key.into(), value);

        Ok(self)
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        match self.config.get(key) {
            Some(value) => Ok(T::deserialize(value)?),
            None => Err(format_err!("Could not find key {}", key)),
        }
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conf: HashMap<String, Value> = load_from_file(path)?;

        Ok(Self { config: conf })
    }

    pub fn lock(self) -> ImmutableConfig {
        ImmutableConfig { config: self }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config: HashMap::new(),
        }
    }
}

pub struct ImmutableConfig {
    config: Config,
}

impl ImmutableConfig {
    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        self.config.get(key)
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conf = Config::load(path)?;

        Ok(Self { config: conf })
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::Config;

    #[test]
    fn test_config_getters() {
        let mut conf = Config::new();

        conf.set("host", "127.0.0.1").unwrap();

        assert_eq!(conf.get::<String>("host").unwrap(), "127.0.0.1".to_string());
        assert_eq!(
            conf.get::<Ipv4Addr>("host").unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );

        conf.set("host", Ipv4Addr::new(127, 0, 0, 1)).unwrap();

        assert_eq!(conf.get::<String>("host").unwrap(), "127.0.0.1".to_string());
        assert_eq!(
            conf.get::<Ipv4Addr>("host").unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );
    }

    #[test]
    fn test_config_chaining() {
        let mut conf = Config::new();

        conf.set("host", "127.0.0.1")
            .unwrap()
            .set("port", 80)
            .unwrap();

        assert_eq!(conf.get::<String>("host").unwrap(), "127.0.0.1".to_string());
        assert_eq!(conf.get::<i32>("port").unwrap(), 80);
    }

    #[test]
    fn test_immutable_config() {
        let mut conf = Config::new();

        conf.set("host", "127.0.0.1").unwrap();

        let conf = conf.lock();

        assert_eq!(conf.get::<String>("host").unwrap(), "127.0.0.1".to_string());
    }
}
