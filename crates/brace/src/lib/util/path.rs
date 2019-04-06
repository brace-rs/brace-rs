use std::path::{Path, PathBuf};

use failure::{format_err, Error};
use path_absolutize::Absolutize;

pub fn absolute(path: &Path) -> Result<PathBuf, Error> {
    let path = path.absolutize()?;

    Ok(path)
}

pub fn get_dir(path: &Path) -> Result<PathBuf, Error> {
    let path = absolute(path)?;

    if path.is_dir() {
        Ok(path)
    } else {
        Err(format_err!(
            "The path '{:?}' must be a valid directory.",
            path
        ))
    }
}

pub fn get_dir_with_name(path: &Path) -> Result<(String, PathBuf), Error> {
    let path = get_dir(path)?;

    match path.parent() {
        Some(parent) => match parent.file_stem() {
            Some(name) => Ok((name.to_string_lossy().to_string(), path)),
            None => Err(format_err!(
                "The path '{:?}' must be a valid directory.",
                path
            )),
        },
        None => Err(format_err!(
            "The path '{:?}' must be a valid directory.",
            path
        )),
    }
}
