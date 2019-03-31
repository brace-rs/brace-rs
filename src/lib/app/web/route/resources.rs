use std::path::{Path, PathBuf};

use actix_web::dev::AsyncResult;
use actix_web::error::{Error as ActixError, ErrorNotFound};
use actix_web::fs::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Path as RoutePath, Responder};
use failure::format_err;
use serde::Deserialize;

use crate::app::theme::config::ThemeConfig;
use crate::app::theme::manifest::ManifestConfig;
use crate::app::theme::resource::ResourceInfo;
use crate::app::AppState;

#[derive(Deserialize)]
pub struct PathInfo {
    pub theme: String,
    pub kind: String,
    pub resource: String,
}

pub fn get(
    path_info: RoutePath<PathInfo>,
    req: HttpRequest<AppState>,
) -> Result<AsyncResult<HttpResponse>, ActixError> {
    let resource = find_theme(&path_info.theme, &req).and_then(|(theme, theme_path)| {
        find_resource(&path_info.resource, theme, &theme_path).and_then(|mut resource| {
            match resource {
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
            }
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
        .themes
        .iter()
        .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
            Ok(conf) => Some((conf, theme.path.clone())),
            Err(_) => None,
        })
        .collect()
}

fn load_manifests(theme: ThemeConfig, path: &Path) -> Vec<ManifestConfig> {
    theme
        .manifests
        .iter()
        .filter_map(
            |manifest| match ManifestConfig::from_file(path.join(&manifest.path)) {
                Ok(conf) => Some(conf),
                Err(_) => None,
            },
        )
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

fn find_resource(name: &str, theme: ThemeConfig, path: &Path) -> Option<ResourceInfo> {
    load_manifests(theme, path)
        .iter()
        .map(|manifest| manifest.resources.clone())
        .flatten()
        .find_map(|resource| {
            if resource.name() == name {
                Some(resource.clone())
            } else {
                None
            }
        })
}
