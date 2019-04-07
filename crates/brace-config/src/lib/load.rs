use std::fs::read_to_string;
use std::path::Path;

use failure::{format_err, Error};
use serde::de::DeserializeOwned;

pub fn file<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            Some("toml") => toml(path),
            Some("yml") => yaml(path),
            Some("yaml") => yaml(path),
            Some("json") => json(path),
            _ => Err(format_err!(
                "The given path '{:?}' is not a recognized configuration format",
                path.as_ref()
            )),
        },
        None => Err(format_err!(
            "The given path '{:?}' is not a recognized configuration format",
            path.as_ref()
        )),
    }
}

pub fn toml<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = toml::from_str::<T>(&string)?;

    Ok(config)
}

pub fn yaml<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = serde_yaml::from_str::<T>(&string)?;

    Ok(config)
}

pub fn json<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = serde_json::from_str::<T>(&string)?;

    Ok(config)
}
