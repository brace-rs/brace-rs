use crate::config::Config;
use crate::util::db::Database;
use crate::util::render::Renderer;
use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};
use log::info;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub renderer: Renderer,
}

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello world!"
}

pub fn run(config: Config) {
    std::env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},brace={}",
            config.web.log.level, config.web.log.level
        ),
    );
    env_logger::init();

    let system = System::new("brace");
    let format = config.web.log.format;
    let state = AppState {
        database: Database::new(config.database),
        renderer: Renderer::new(config.renderer),
    };

    HttpServer::new(move || {
        App::with_state(state.clone())
            .middleware(Logger::new(&format))
            .resource("/", |r| r.f(index))
    })
    .bind(format!("{}:{}", config.web.host, config.web.port))
    .unwrap()
    .start();

    info!(
        "Started http server on {}:{}",
        config.web.host, config.web.port
    );

    system.run();
}
