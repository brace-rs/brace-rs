use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::App;
use failure::Error;
use log::info;

use crate::app::{AppConfig, AppState};

pub use self::config::{WebConfig, WebLogConfig};

pub mod config;
pub mod route;

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
