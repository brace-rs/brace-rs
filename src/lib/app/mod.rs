use self::config::Config;
use crate::util::db::Database;
use crate::util::render::Renderer;

pub mod config;
pub mod init;
pub mod web;

#[derive(Clone)]
pub struct AppState {
    config: Config,
    database: Database,
    renderer: Renderer,
}

impl AppState {
    pub fn from_config(config: Config) -> Self {
        Self {
            database: Database::new(config.database.clone()),
            renderer: Renderer::new(config.renderer.clone()),
            config,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
