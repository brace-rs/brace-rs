use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};
use serde::Deserialize;
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct Config {
    host: Ipv4Addr,
    port: u16,
    log_level: String,
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error + 'static>> {
    let string = std::fs::read_to_string(path)?;
    let config = toml::from_str(&string)?;

    Ok(config)
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
