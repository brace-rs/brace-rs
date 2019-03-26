use crate::config::Config;
use path_absolutize::Absolutize;
use std::path::Path;

pub fn init(path: &str) -> Result<(), failure::Error> {
    let target_dir = Path::new(path).absolutize()?;
    let target_file = target_dir.join("Config.toml");
    let template_path = target_dir.join("templates");
    let template_file = template_path.join("index.html");

    if target_dir.is_dir() {
        if target_file.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "A site already exists at the given directory.",
            )
            .into());
        } else {
            let config = Config::default();
            let string = toml::to_string_pretty(&config)?;

            std::fs::write(target_file, string)?;
            std::fs::create_dir(template_path)?;
            std::fs::write(template_file, include_str!("../../../templates/index.html"))?;
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
