use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::value::{table::Table, Error};
use crate::{load, save};

#[derive(Debug, Clone)]
pub struct Config(Table);

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<'de, T>(&'de self, key: &str) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        self.0.get(key)
    }

    pub fn set<T>(&mut self, key: &str, value: T) -> Result<&mut Config, Error>
    where
        T: Serialize,
    {
        self.0.set(key, value)?;

        Ok(self)
    }

    pub fn set_default<T>(&mut self, key: &str, value: T) -> Result<&mut Config, Error>
    where
        T: Serialize,
    {
        self.0.set_default(key, value)?;

        Ok(self)
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        match load::file::<Table, _>(path.as_ref()) {
            Ok(conf) => Ok(Self(conf)),
            Err(err) => Err(Error::custom(err)),
        }
    }

    pub fn save<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        match save::file(path.as_ref(), &self.0) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::custom(err)),
        }
    }

    pub fn lock(self) -> ImmutableConfig {
        ImmutableConfig(self)
    }

    pub fn merge(&mut self, config: &Config) -> Result<&mut Config, Error> {
        self.0.merge(&config.0)?;

        Ok(self)
    }

    pub fn merge_default(&mut self, config: &Config) -> Result<&mut Config, Error> {
        self.0.merge_default(&config.0)?;

        Ok(self)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self(Table::new())
    }
}

pub struct ImmutableConfig(Config);

impl ImmutableConfig {
    pub fn get<'de, T>(&'de self, key: &str) -> Result<T, Error>
    where
        T: 'de + Deserialize<'de>,
    {
        self.0.get(key)
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Self(Config::load(path)?))
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::Config;

    #[test]
    fn test_config() {
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
    fn test_config_nested_table() {
        let mut conf = Config::new();

        conf.set("web.host", "127.0.0.1").unwrap();

        assert_eq!(
            conf.get::<String>("web.host").unwrap(),
            "127.0.0.1".to_string()
        );
        assert_eq!(
            conf.get::<Ipv4Addr>("web.host").unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );

        conf.set("web.host", Ipv4Addr::new(127, 0, 0, 1)).unwrap();

        assert_eq!(
            conf.get::<String>("web.host").unwrap(),
            "127.0.0.1".to_string()
        );
        assert_eq!(
            conf.get::<Ipv4Addr>("web.host").unwrap(),
            Ipv4Addr::new(127, 0, 0, 1)
        );

        conf.set("web.port", "8080").unwrap();

        assert_eq!(conf.get::<String>("web.port").unwrap(), "8080".to_string());
        assert_eq!(conf.get::<i32>("web.port").unwrap(), 8080);

        assert!(conf.set("web.address.host", "127.0.0.1").is_ok());
        assert!(conf.set("web.host.address", "127.0.0.1").is_err());
    }

    #[test]
    fn test_config_nested_array() {
        use std::collections::HashMap;

        let mut conf = Config::new();

        conf.set("list.0.0.item", "1").unwrap();
        conf.set("list.1.0.item", "2").unwrap();

        assert_eq!(
            conf.get::<String>("list.0.0.item").unwrap(),
            "1".to_string()
        );
        assert_eq!(conf.get::<usize>("list.0.0.item").unwrap(), 1,);
        assert!(conf.get::<Vec<HashMap<String, String>>>("list.0").is_ok());
        assert!(conf.get::<Vec<HashMap<String, String>>>("list.1").is_ok());
        assert!(conf.get::<Vec<HashMap<String, String>>>("list.2").is_err());
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

    #[test]
    fn test_config_merging() {
        let mut conf1 = Config::new();
        let mut conf2 = Config::new();

        conf1.set("host", "127.0.0.1").unwrap();
        conf2.set("port", 80).unwrap();
        conf1.merge(&conf2).unwrap();

        assert_eq!(
            conf1.get::<String>("host").unwrap(),
            "127.0.0.1".to_string()
        );
        assert_eq!(conf1.get::<i32>("port").unwrap(), 80);
    }
}
