use std::fs::read_to_string;
use std::path::Path;

use failure::{format_err, Error};
use serde::de::DeserializeOwned;

pub fn load_from_file<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            Some("toml") => load_from_toml(path),
            Some("yml") => load_from_yaml(path),
            Some("yaml") => load_from_yaml(path),
            Some("json") => load_from_json(path),
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

pub fn load_from_toml<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = toml::from_str::<T>(&string)?;

    Ok(config)
}

pub fn load_from_yaml<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = serde_yaml::from_str::<T>(&string)?;

    Ok(config)
}

pub fn load_from_json<T, P>(path: P) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let string = read_to_string(path)?;
    let config = serde_json::from_str::<T>(&string)?;

    Ok(config)
}
