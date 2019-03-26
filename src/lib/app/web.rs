use crate::app::config::Config;
use crate::app::AppState;
use crate::util::db::Database;
use crate::util::render::{Renderer, Template};
use actix::System;
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::Logger;
use actix_web::server::HttpServer;
use actix_web::{App, AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;
use log::info;
use serde_json::json;

fn index(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let template = Template::new(
        "index.html",
        json!({
            "title": "Under Construction",
            "message": "This site is currently under construction, please come back later.",
        }),
    );

    req.state()
        .renderer
        .send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
        .responder()
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
