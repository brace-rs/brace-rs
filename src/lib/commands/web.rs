use crate::config::Config;
use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};
use log::info;

fn index(_req: &HttpRequest) -> &'static str {
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

    HttpServer::new(move || {
        App::new()
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
