use actix::System;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let system = System::new("brace");

    HttpServer::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/", |r| r.f(index))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:8080");
    system.run();
}