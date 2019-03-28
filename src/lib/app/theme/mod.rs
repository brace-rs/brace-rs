use std::path::{Path, PathBuf};

use failure::Error;

use self::template::TemplateInfo;
use crate::util::path::get_dir_with_name;

pub use self::config::ThemeConfig;

pub mod config;
pub mod library;
pub mod template;

pub fn init(mut config: ThemeConfig, path: &Path) -> Result<(), Error> {
    let (name, path) = get_dir_with_name(path)?;

    config.theme.name = name;
    config.templates.push(TemplateInfo::Tera {
        name: "index".to_string(),
        path: PathBuf::from("templates/index.html"),
    });

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("Theme.toml"), toml::to_string_pretty(&config)?)?;
    std::fs::write(
        path.join("templates/index.html"),
        include_str!("../../../../themes/default/templates/index.html"),
    )?;

    Ok(())
}
