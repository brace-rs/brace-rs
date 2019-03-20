use crate::config::Config;
use std::error::Error;
use std::path::Path;

pub fn init(path: &str) -> Result<(), Box<dyn Error + 'static>> {
    let target_dir = Path::new(path);
    let target_file = target_dir.join("Config.toml");

    if target_dir.is_dir() {
        if target_file.exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "A site already exists at the given directory.",
            )));
        } else {
            let config = Config::default();
            let string = toml::to_string_pretty(&config)?;

            std::fs::write(target_file, string)?;
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The given directory could not be found.",
        )));
    }

    Ok(())
}
