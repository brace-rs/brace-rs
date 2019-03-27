use std::path::Path;

use path_absolutize::Absolutize;

use crate::app::AppConfig;

pub fn init(path: &str) -> Result<(), failure::Error> {
    let target_dir = Path::new(path).absolutize()?;
    let target_file = target_dir.join("Config.toml");

    if target_dir.is_dir() {
        if target_file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "A site already exists at the given directory.",
            )
            .into());
        } else {
            let config = AppConfig::default();
            let string = toml::to_string_pretty(&config)?;

            std::fs::write(target_file, string)?;
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "The given directory could not be found.",
        )
        .into());
    }

    Ok(())
}
