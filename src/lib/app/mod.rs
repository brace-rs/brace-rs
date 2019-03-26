use crate::util::db::Database;
use crate::util::render::Renderer;

pub mod config;
pub mod init;
pub mod web;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub renderer: Renderer,
}
