use std::path::Path;

use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::App;
use brace_theme::config::ThemeConfig;
use failure::Error;
use log::info;

use self::config::AppConfig;
use self::state::AppState;
use crate::util::path::get_dir;

pub mod cli;
pub mod config;
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

pub fn run(config: AppConfig) -> Result<(), Error> {
    std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},brace={}",
            config.web.log.level, config.web.log.level
        ),
    );
    env_logger::init();

    let system = System::new("brace");
    let state = AppState::from_config(config.clone())?;
    let format = config.web.log.format;

    HttpServer::new(move || {
        App::with_state(state.clone())
            .middleware(Logger::new(&format))
            .resource("/", |r| r.get().with(route::index::get))
            .resource("/themes", |r| r.get().with(route::themes::get))
            .resource("/static/resources/{theme}/{kind}/{resource}", |r| {
                r.get().with(route::resources::get)
            })
    })
    .bind(format!("{}:{}", config.web.host, config.web.port))?
    .start();

    info!(
        "Started http server on {}:{}",
        config.web.host, config.web.port
    );

    system.run();

    Ok(())
}
