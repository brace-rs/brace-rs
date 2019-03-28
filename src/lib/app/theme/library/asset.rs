use std::path::{Path, PathBuf};

use actix_web::dev::{AsyncResult, Handler};
use actix_web::error::{Error as ActixError, ErrorNotFound};
use actix_web::fs::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Responder};
use failure::format_err;
use serde::{Deserialize, Serialize};

use crate::app::theme::config::ThemeConfig;
use crate::app::theme::library::LibraryInfo;
use crate::app::AppState;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AssetInfo {
    #[serde(rename = "css")]
    StyleSheet { name: String, path: PathBuf },
    #[serde(rename = "js")]
    JavaScript { name: String, path: PathBuf },
}

impl AssetInfo {
    pub fn name(&self) -> &str {
        match self {
            AssetInfo::StyleSheet { name, .. } => name,
            AssetInfo::JavaScript { name, .. } => name,
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            AssetInfo::StyleSheet { path, .. } => path,
            AssetInfo::JavaScript { path, .. } => path,
        }
    }

    pub fn is_stylesheet(&self) -> bool {
        match self {
            AssetInfo::StyleSheet { .. } => true,
            AssetInfo::JavaScript { .. } => false,
        }
    }

    pub fn is_javascript(&self) -> bool {
        match self {
            AssetInfo::StyleSheet { .. } => false,
            AssetInfo::JavaScript { .. } => true,
        }
    }
}

pub struct AssetLibraryRouter;

impl AssetLibraryRouter {
    pub fn load_themes(req: &HttpRequest<AppState>) -> Vec<(ThemeConfig, PathBuf)> {
        req.state()
            .config()
            .theme
            .iter()
            .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
                Ok(conf) => Some((conf, theme.path.clone())),
                Err(_) => None,
            })
            .collect()
    }

    pub fn find_theme(name: String, req: &HttpRequest<AppState>) -> Option<(ThemeConfig, PathBuf)> {
        Self::load_themes(req).iter().find_map(|(theme, path)| {
            if theme.theme.name == name {
                if let Some(path) = path.parent() {
                    Some((theme.clone(), path.to_path_buf()))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn find_library(name: String, theme: ThemeConfig) -> Option<LibraryInfo> {
        theme.libraries.iter().find_map(|library| {
            if library.name == name {
                Some(library.clone())
            } else {
                None
            }
        })
    }

    pub fn find_asset(path: String, library: LibraryInfo) -> Option<AssetInfo> {
        library.assets.iter().find_map(|asset| {
            if asset.path() == PathBuf::from(path.trim_start_matches('/')) {
                Some(asset.clone())
            } else {
                None
            }
        })
    }
}

impl Handler<AppState> for AssetLibraryRouter {
    type Result = Result<AsyncResult<HttpResponse>, ActixError>;

    fn handle(&self, req: &HttpRequest<AppState>) -> Self::Result {
        let info = req.match_info();
        let asset = info
            .get_decoded("theme")
            .and_then(|theme| Self::find_theme(theme, req))
            .and_then(|(theme, theme_path)| {
                info.get_decoded("library")
                    .and_then(|library| Self::find_library(library, theme))
                    .and_then(|library| {
                        info.get_decoded("tail")
                            .and_then(|asset| Self::find_asset(asset, library))
                            .and_then(|mut asset| {
                                match asset {
                                    AssetInfo::StyleSheet { ref mut path, .. } => {
                                        *path = theme_path.join(&path);
                                    }
                                    AssetInfo::JavaScript { ref mut path, .. } => {
                                        *path = theme_path.join(&path);
                                    }
                                }
                                Some(asset)
                            })
                    })
            });

        match asset {
            Some(asset) => NamedFile::open(asset.path())?
                .respond_to(&req)?
                .respond_to(&req),
            None => Err(ErrorNotFound(format_err!("Asset could not be found"))),
        }
    }
}
