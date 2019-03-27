use self::database::Database;
use self::renderer::Renderer;

pub use self::config::AppConfig;

pub mod config;
pub mod database;
pub mod init;
pub mod renderer;
pub mod theme;
pub mod web;

#[derive(Clone)]
pub struct AppState {
    config: AppConfig,
    database: Database,
    renderer: Renderer,
}

impl AppState {
    pub fn from_config(config: AppConfig) -> Result<Self, failure::Error> {
        let database = Database::from_config(config.database.clone())?;
        let renderer = Renderer::from_config(config.renderer.clone())?;

        Ok(Self {
            database,
            renderer,
            config,
        })
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }
}
