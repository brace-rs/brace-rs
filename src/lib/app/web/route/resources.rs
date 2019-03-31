use std::path::PathBuf;

use actix_web::dev::AsyncResult;
use actix_web::error::{Error as ActixError, ErrorNotFound};
use actix_web::fs::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Path, Responder};
use failure::format_err;
use serde::Deserialize;

use crate::app::theme::config::ThemeConfig;
use crate::app::theme::library::resource::ResourceInfo;
use crate::app::theme::library::LibraryInfo;
use crate::app::AppState;

#[derive(Deserialize)]
pub struct PathInfo {
    pub theme: String,
    pub library: String,
    pub kind: String,
    pub resource: String,
}

pub fn get(
    path_info: Path<PathInfo>,
    req: HttpRequest<AppState>,
) -> Result<AsyncResult<HttpResponse>, ActixError> {
    let resource = find_theme(&path_info.theme, &req).and_then(|(theme, theme_path)| {
        find_library(&path_info.library, theme).and_then(|library| {
            find_resource(&path_info.resource, library).and_then(|mut resource| match resource {
                ResourceInfo::StyleSheet(ref mut info) => {
                    if &path_info.kind == "css" {
                        if info.location.is_internal() {
                            info.location =
                                theme_path.join(info.location.clone().into_inner()).into();

                            Some(resource)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                ResourceInfo::JavaScript(ref mut info) => {
                    if &path_info.kind == "js" {
                        if info.location.is_internal() {
                            info.location =
                                theme_path.join(info.location.clone().into_inner()).into();

                            Some(resource)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        })
    });

    match resource {
        Some(resource) => NamedFile::open(resource.location().clone().into_inner())?
            .respond_to(&req)?
            .respond_to(&req),
        None => Err(ErrorNotFound(format_err!("Resource could not be found"))),
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

fn find_resource(name: &str, library: LibraryInfo) -> Option<ResourceInfo> {
    library.resources.iter().find_map(|resource| {
        if resource.name() == name {
            Some(resource.clone())
        } else {
            None
        }
    })
}
