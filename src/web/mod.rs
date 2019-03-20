use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};
use config::Config;
use log::info;

pub mod config;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn run(config: Config) {
    std::env::set_var(
        "RUST_LOG",
        format!("actix_web={},brace={}", config.log.level, config.log.level),
    );
    env_logger::init();

    let system = System::new("brace");
    let format = config.log.format;

    HttpServer::new(move || {
        App::new()
            .middleware(Logger::new(&format))
            .resource("/", |r| r.f(index))
    })
    .bind(format!("{}:{}", config.host, config.port))
    .unwrap()
    .start();

    info!("Started http server on {}:{}", config.host, config.port);

    system.run();
}
