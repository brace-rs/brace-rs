use failure::Error;

use brace_db::Database;
use brace_theme::renderer::{Renderer, RendererConfig};

use super::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    config: AppConfig,
    database: Database,
    renderer: Renderer,
}

impl AppState {
    pub fn from_config(config: AppConfig) -> Result<Self, Error> {
        let database = Database::from_config(config.database.clone())?;
        let renderer = Renderer::from_config(RendererConfig {
            themes: config.themes.clone(),
        })?;

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
