use std::path::{Path, PathBuf};

use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_theme::config::{ThemeConfig, ThemeInfo};
use brace_theme::manifest::ManifestConfig;
use brace_theme::renderer::{Renderer, Template};
use brace_theme::resource::ResourceInfo;
use futures::future::Future;
use serde_json::{json, to_value};

use crate::config::AppConfig;

pub fn get(
    conf: Data<AppConfig>,
    rend: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let themes = conf
        .themes
        .iter()
        .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
            Ok(conf) => Some((conf, theme.path.as_path())),
            Err(_) => None,
        })
        .collect::<Vec<(ThemeConfig, &Path)>>();

    let theme_info = themes
        .iter()
        .map(|(theme, _)| theme.theme.clone())
        .collect::<Vec<ThemeInfo>>();

    let resource_info = themes
        .iter()
        .map(|(theme, theme_path)| {
            theme
                .manifests
                .iter()
                .filter_map(|manifest| match theme_path.parent() {
                    Some(parent) => match ManifestConfig::from_file(parent.join(&manifest.path)) {
                        Ok(manifest) => Some(
                            manifest
                                .resources
                                .iter()
                                .map(|resource| {
                                    let mut resource = resource.clone();

                                    match resource {
                                        ResourceInfo::StyleSheet(ref mut info) => {
                                            if info.location.is_internal() {
                                                info.location = PathBuf::new()
                                                    .join("/static/resources")
                                                    .join(theme.theme.name.clone())
                                                    .join("css")
                                                    .join(&info.name)
                                                    .into();
                                            }
                                        }
                                        ResourceInfo::JavaScript(ref mut info) => {
                                            if info.location.is_internal() {
                                                info.location = PathBuf::new()
                                                    .join("/static/resources")
                                                    .join(theme.theme.name.clone())
                                                    .join("js")
                                                    .join(&info.name)
                                                    .into();
                                            }
                                        }
                                    }

                                    resource
                                })
                                .collect::<Vec<ResourceInfo>>(),
                        ),
                        Err(_) => None,
                    },
                    None => None,
                })
                .flatten()
                .collect::<Vec<ResourceInfo>>()
        })
        .flatten()
        .collect::<Vec<ResourceInfo>>();

    let template = Template::new(
        "themes",
        json!({
            "title": "Themes",
            "themes": to_value(theme_info).unwrap(),
            "resources": to_value(resource_info).unwrap(),
        }),
    );

    rend.send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
}
