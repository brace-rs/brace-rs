use std::collections::HashMap;
use std::path::Path;

use failure::Error;
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

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: Serialize,
    {
        self.config
            .insert(key.into(), serde_json::to_value(value).unwrap());
    }

    pub fn get<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        match self.config.get(key.into()) {
            Some(value) => match T::deserialize(value) {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conf: HashMap<String, Value> = load_from_file(path)?;

        Ok(Self { config: conf })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::Config;

    #[test]
    fn test_config_getters() {
        let mut conf = Config::new();

        conf.set("host", "127.0.0.1");

        assert_eq!(conf.get::<String>("host"), Some("127.0.0.1".to_string()));
        assert_eq!(
            conf.get::<Ipv4Addr>("host"),
            Some(Ipv4Addr::new(127, 0, 0, 1))
        );

        conf.set("host", Ipv4Addr::new(127, 0, 0, 1));

        assert_eq!(conf.get::<String>("host"), Some("127.0.0.1".to_string()));
        assert_eq!(
            conf.get::<Ipv4Addr>("host"),
            Some(Ipv4Addr::new(127, 0, 0, 1))
        );
    }
}
