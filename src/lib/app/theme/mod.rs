use std::path::{Path, PathBuf};

use failure::Error;

use self::manifest::{ManifestConfig, ManifestReferenceInfo};
use self::template::TemplateInfo;
use crate::util::path::get_dir_with_name;

pub use self::config::ThemeConfig;

pub mod config;
pub mod library;
pub mod manifest;
pub mod resource;
pub mod template;

pub fn init(mut config: ThemeConfig, path: &Path) -> Result<(), Error> {
    let (name, path) = get_dir_with_name(path)?;

    config.theme.name = name;
    config.manifests.push(ManifestReferenceInfo {
        name: None,
        path: path.join("manifest.toml"),
    });

    let mut manifest = ManifestConfig::default();

    manifest.templates.push(TemplateInfo::Tera {
        name: "index".to_string(),
        path: PathBuf::from("templates/index.html"),
    });

    std::fs::create_dir(path.join("templates")).unwrap();
    std::fs::write(path.join("theme.toml"), toml::to_string_pretty(&config)?)?;
    std::fs::write(
        path.join("manifest.toml"),
        toml::to_string_pretty(&manifest)?,
    )?;
    std::fs::write(
        path.join("templates/index.html"),
        include_str!("../../../../themes/default/templates/index.html"),
    )?;

    Ok(())
}
