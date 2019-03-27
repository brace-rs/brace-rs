use std::path::Path;

use failure::Error;

use crate::app::theme::ThemeConfig;
use crate::util::path::get_dir;

pub use self::config::AppConfig;
pub use self::state::AppState;

pub mod config;
pub mod database;
pub mod renderer;
pub mod state;
pub mod theme;
pub mod web;

pub fn init(config: AppConfig, path: &Path) -> Result<(), Error> {
    let path = get_dir(path)?;

    std::fs::create_dir_all(path.join("themes/default")).unwrap();
    std::fs::write(path.join("Config.toml"), toml::to_string_pretty(&config)?)?;
    crate::app::theme::init(ThemeConfig::default(), &path.join("themes/default")).unwrap();

    Ok(())
}
