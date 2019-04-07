use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use failure::{format_err, Error};
use serde::Serialize;
use toml::Value;

pub fn file<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(ext) => match ext.to_str() {
            Some("toml") => toml(path, value),
            Some("yml") => yaml(path, value),
            Some("yaml") => yaml(path, value),
            Some("json") => json(path, value),
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

pub fn toml<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let value = Value::try_from(value)?;
    let string = toml::to_string_pretty(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(string.as_ref())?;

    Ok(())
}

pub fn yaml<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let string = serde_yaml::to_string(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(string.as_ref())?;

    Ok(())
}

pub fn json<T, P>(path: P, value: &T) -> Result<(), Error>
where
    T: Serialize,
    P: AsRef<Path>,
{
    let string = serde_json::to_string_pretty(&value)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(string.as_ref())?;

    Ok(())
}
