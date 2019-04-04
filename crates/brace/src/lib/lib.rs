use std::path::{Path, PathBuf};

use actix::System;
use actix_web::middleware::Logger;
use actix_web::web::{get, resource};
use actix_web::App;
use actix_web::HttpServer;
use brace_theme::config::ThemeConfig;
use failure::Error;
use log::info;

use self::config::AppConfig;
use self::route::resources::ThemeResources;
use self::state::AppState;
use crate::util::path::get_dir;

pub mod cli;
pub mod config;
pub mod logger;
pub mod route;
pub mod state;
pub mod util;

pub fn init(config: AppConfig, path: &Path) -> Result<(), Error> {
    let path = get_dir(path)?;

    std::fs::create_dir_all(path.join("themes/default")).unwrap();
    std::fs::write(path.join("config.toml"), toml::to_string_pretty(&config)?)?;
    brace_theme::init(ThemeConfig::default(), &path.join("themes/default")).unwrap();

    Ok(())
}

pub fn run(config: AppConfig, path: &Path) -> Result<(), Error> {
    logger::init(&config, path)?;

    let system = System::new("brace");
    let state = AppState::from_config(config.clone())?;
    let format = config.web.log.format;
    let themes = config
        .themes
        .iter()
        .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
            Ok(conf) => Some((conf, theme.path.clone())),
            Err(_) => None,
        })
        .collect::<Vec<(ThemeConfig, PathBuf)>>();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(Logger::new(&format))
            .service(resource("/").route(get().to_async(route::index::get)))
            .service(resource("/themes").route(get().to_async(route::themes::get)))
            .service(ThemeResources::new("/static/resources", themes.clone()))
    })
    .bind(format!("{}:{}", config.web.host, config.web.port))?
    .start();

    info!(
        "Started http server on {}:{}",
        config.web.host, config.web.port
    );

    system.run()?;

    Ok(())
}
