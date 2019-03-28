use std::path::PathBuf;

use actix_web::dev::AsyncResult;
use actix_web::error::{Error as ActixError, ErrorNotFound};
use actix_web::fs::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Path, Responder};
use failure::format_err;
use serde::Deserialize;

use crate::app::theme::config::ThemeConfig;
use crate::app::theme::library::asset::AssetInfo;
use crate::app::theme::library::LibraryInfo;
use crate::app::AppState;

#[derive(Deserialize)]
pub struct PathInfo {
    pub theme: String,
    pub library: String,
    pub kind: String,
    pub asset: String,
}

pub fn get(
    path_info: Path<PathInfo>,
    req: HttpRequest<AppState>,
) -> Result<AsyncResult<HttpResponse>, ActixError> {
    let asset = find_theme(&path_info.theme, &req).and_then(|(theme, theme_path)| {
        find_library(&path_info.library, theme).and_then(|library| {
            find_asset(&path_info.asset, library).and_then(|mut asset| match asset {
                AssetInfo::StyleSheet(ref mut info) => {
                    if &path_info.kind == "css" {
                        info.path = theme_path.join(&info.path);

                        Some(asset)
                    } else {
                        None
                    }
                }
                AssetInfo::JavaScript(ref mut info) => {
                    if &path_info.kind == "js" {
                        info.path = theme_path.join(&info.path);

                        Some(asset)
                    } else {
                        None
                    }
                }
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

fn load_themes(req: &HttpRequest<AppState>) -> Vec<(ThemeConfig, PathBuf)> {
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

fn find_theme(name: &str, req: &HttpRequest<AppState>) -> Option<(ThemeConfig, PathBuf)> {
    load_themes(req).iter().find_map(|(theme, path)| {
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

fn find_library(name: &str, theme: ThemeConfig) -> Option<LibraryInfo> {
    theme.libraries.iter().find_map(|library| {
        if library.name == name {
            Some(library.clone())
        } else {
            None
        }
    })
}

fn find_asset(name: &str, library: LibraryInfo) -> Option<AssetInfo> {
    library.assets.iter().find_map(|asset| {
        if asset.name() == name {
            Some(asset.clone())
        } else {
            None
        }
    })
}
