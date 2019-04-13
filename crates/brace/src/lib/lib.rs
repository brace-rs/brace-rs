use std::path::{Path, PathBuf};

use actix::System;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::web::{get, resource};
use actix_web::App;
use actix_web::HttpServer;
use brace_config::{load, save};
use brace_db::Database;
use brace_theme::config::ThemeConfig;
use brace_web::render::{Renderer, RendererConfig};
use failure::Error;
use log::info;

use self::config::AppConfig;
use self::route::resources::ThemeResources;
use crate::util::path::get_dir;

pub mod cli;
pub mod config;
pub mod logger;
pub mod route;
pub mod util;

pub fn init(config: AppConfig, path: &Path) -> Result<(), Error> {
    let path = get_dir(path)?;

    std::fs::create_dir_all(path.join("themes/default")).unwrap();
    save::file(path.join("config.toml"), &config)?;
    brace_theme::init(ThemeConfig::default(), &path.join("themes/default")).unwrap();

    Ok(())
}

pub fn run(config: AppConfig, path: &Path) -> Result<(), Error> {
    logger::init(&config, path)?;

    let system = System::new("brace");
    let database = Database::from_config(config.database.clone())?;
    let renderer = Renderer::from_config(RendererConfig {
        themes: config.themes.clone(),
    })?;
    let format = config.web.log.format.clone();
    let themes = config
        .themes
        .iter()
        .filter_map(|theme| match load::file(&theme.path) {
            Ok(conf) => Some((conf, theme.path.clone())),
            Err(_) => None,
        })
        .collect::<Vec<(ThemeConfig, PathBuf)>>();

    let host = config.web.host;
    let port = config.web.port;

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(database.clone())
            .data(renderer.clone())
            .wrap(Logger::new(&format))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .secure(false),
            ))
            .service(resource("/").route(get().to_async(route::index::get)))
            .service(resource("/themes").route(get().to_async(route::themes::get)))
            .service(brace_web_auth::route::api::auth_route())
            .service(brace_web_auth::route::web::login_route())
            .service(brace_web_auth::route::web::logout_route())
            .service(brace_web_auth::route::api::routes())
            .service(brace_web_auth::route::web::routes())
            .service(brace_web_page::route::api::routes())
            .service(brace_web_page::route::web::routes())
            .service(ThemeResources::new("/static/resources", themes.clone()))
            .service(brace_web_page::router::PageRouter::new("/"))
    })
    .bind(format!("{}:{}", host, port))?
    .start();

    info!("Started http server on {}:{}", host, port);

    system.run()?;

    Ok(())
}
