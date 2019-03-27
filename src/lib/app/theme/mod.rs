use std::path::{Path, PathBuf};

use failure::Error;

use crate::util::path::get_dir;

pub use self::config::{TemplateInfo, ThemeConfig};

pub mod config;

pub fn init(mut config: ThemeConfig, path: &Path) -> Result<(), Error> {
    let path = get_dir(path)?;

    config.templates.insert(
        "index".to_string(),
        TemplateInfo {
            name: None,
            path: PathBuf::from("templates/index.html"),
        },
    );

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("Theme.toml"), toml::to_string_pretty(&config)?)?;
    std::fs::write(
        path.join("templates/index.html"),
        include_str!("../../../../theme/templates/index.html"),
    )?;

    Ok(())
}
