use std::collections::HashMap;
use std::path::Path;

use failure::{format_err, Error};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use super::{load, save};

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
        if key.is_empty() {
            return Err(format_err!("Invalid key"));
        }

        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.len() == 1 {
            self.config.insert(key.into(), serde_json::to_value(value)?);
        } else {
            match self.config.get_mut(keys[0]) {
                Some(target) => match set(target, keys[1], serde_json::to_value(value)?) {
                    Ok(_) => (),
                    Err(err) => return Err(err),
                },
                None => {
                    let mut obj = Value::Object(Map::new());

                    match set(&mut obj, keys[1], serde_json::to_value(value)?) {
                        Ok(_) => {
                            self.config.insert(keys[0].into(), obj);
                        }
                        Err(err) => return Err(err),
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn get<T>(&self, key: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        if key.is_empty() {
            return Err(format_err!("Invalid key"));
        }

        let keys: Vec<&str> = key.splitn(2, '.').collect();

        if keys.len() == 1 {
            match self.config.get(key) {
                Some(value) => Ok(T::deserialize(value)?),
                None => Err(format_err!("Invalid key {}", key)),
            }
        } else {
            match self.config.get(keys[0]) {
                Some(value) => match get(value, keys[1]) {
                    Some(value) => Ok(T::deserialize(value)?),
                    None => Err(format_err!("Invalid key {}", key)),
                },
                None => Err(format_err!("Invalid key {}", key)),
            }
        }
    }

    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conf: HashMap<String, Value> = load::file(path)?;

        Ok(Self { config: conf })
    }

    pub fn save<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        save::file(path, &self.config)?;

        Ok(())
    }

    pub fn lock(self) -> ImmutableConfig {
        ImmutableConfig { config: self }
    }

    pub fn merge(&mut self, config: &Config) -> &mut Config {
        self.config
            .extend(config.config.iter().map(|(k, v)| (k.clone(), v.clone())));
        self
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

fn get<'a>(source: &'a Value, key: &str) -> Option<&'a Value> {
    let keys = key.split('.');
    let mut target = source;

    for key in keys {
        let target_opt = match *target {
            Value::Object(ref map) => map.get(key),
            Value::Array(ref arr) => key.parse::<usize>().ok().and_then(|x| arr.get(x)),
            _ => return None,
        };
        if let Some(t) = target_opt {
            target = t;
        } else {
            return None;
        }
    }

    Some(target)
}

fn set<'a>(source: &'a mut Value, key: &str, value: Value) -> Result<(), Error> {
    let keys: Vec<&str> = key.splitn(2, '.').collect();
    let key = keys[0];

    if keys.len() == 1 {
        match source {
            Value::Object(map) => {
                map.insert(key.into(), value);
            }
            Value::Array(arr) => {
                arr.insert(key.parse()?, value);
            }
            Value::Null => match key.parse::<usize>() {
                Ok(key) => {
                    let mut arr = Vec::new();
                    arr.insert(key, value);
                    *source = Value::Array(arr);
                }
                Err(_) => {
                    let mut map = Map::new();
                    map.insert(key.into(), value);
                    *source = Value::Object(map);
                }
            },
            _ => return Err(format_err!("Unsupported nesting")),
        }
    } else {
        let tail = keys[1];

        match source {
            Value::Object(map) => match map.get_mut(key) {
                Some(target) => return set(target, tail, value),
                None => {
                    let mut obj = Value::Object(Map::new());
                    match set(&mut obj, tail, value) {
                        Ok(_) => {
                            map.insert(key.into(), obj);
                        }
                        Err(err) => return Err(err),
                    }
                }
            },
            Value::Array(arr) => match arr.get_mut(key.parse::<usize>()?) {
                Some(target) => return set(target, tail, value),
                None => {
                    let mut obj = Value::Object(Map::new());
                    match set(&mut obj, tail, value) {
                        Ok(_) => {
                            arr.insert(key.parse()?, obj);
                        }
                        Err(err) => return Err(err),
                    }
                }
            },
            Value::Null => match key.parse::<usize>() {
                Ok(_) => {
                    *source = Value::Array(Vec::new());
                    return set(source, key, value);
                }
                Err(_) => {
                    *source = Value::Object(Map::new());
                    return set(source, key, value);
                }
            },
            _ => return Err(format_err!("Unsupported nesting")),
        }
    }

    Ok(())
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
    fn test_config_nested() {
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

        assert!(conf.set("web.address.host", "127.0.0.1").is_ok());
        assert!(conf.set("web.host.address", "127.0.0.1").is_err());
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
        conf1.merge(&conf2);

        assert_eq!(
            conf1.get::<String>("host").unwrap(),
            "127.0.0.1".to_string()
        );
        assert_eq!(conf1.get::<i32>("port").unwrap(), 80);
    }
}
