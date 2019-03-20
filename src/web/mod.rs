use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};
use config::Config;

pub mod config;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn run(config: Config) {
    std::env::set_var("RUST_LOG", format!("actix_web={}", config.log_level));
    env_logger::init();

    let system = System::new("brace");

    HttpServer::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/", |r| r.f(index))
    })
    .bind(format!("{}:{}", config.host, config.port))
    .unwrap()
    .start();

    println!("Started http server: {}:{}", config.host, config.port);
    system.run();
}
