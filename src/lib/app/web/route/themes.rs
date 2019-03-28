use std::path::PathBuf;

use actix_web::error::ErrorInternalServerError;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;
use serde_json::{json, to_value};

use crate::app::renderer::Template;
use crate::app::theme::config::{ThemeConfig, ThemeInfo};
use crate::app::theme::library::asset::AssetInfo;
use crate::app::AppState;

pub fn get(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let themes = req
        .state()
        .config()
        .theme
        .iter()
        .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
            Ok(conf) => Some(conf),
            Err(_) => None,
        })
        .collect::<Vec<ThemeConfig>>();

    let theme_info = themes
        .iter()
        .map(|theme| theme.theme.clone())
        .collect::<Vec<ThemeInfo>>();

    let asset_info = themes
        .iter()
        .map(|theme| {
            theme
                .libraries
                .iter()
                .map(|library| {
                    library
                        .assets
                        .iter()
                        .map(|asset| {
                            let mut asset = asset.clone();

                            match asset {
                                AssetInfo::StyleSheet(ref mut info) => {
                                    info.path = PathBuf::new()
                                        .join("/static/assets")
                                        .join(theme.theme.name.clone())
                                        .join(library.name.clone())
                                        .join("css")
                                        .join(&info.name);
                                }
                                AssetInfo::JavaScript(ref mut info) => {
                                    info.path = PathBuf::new()
                                        .join("/static/assets")
                                        .join(theme.theme.name.clone())
                                        .join(library.name.clone())
                                        .join("js")
                                        .join(&info.name);
                                }
                            }

                            asset
                        })
                        .collect::<Vec<AssetInfo>>()
                })
                .flatten()
                .collect::<Vec<AssetInfo>>()
        })
        .flatten()
        .collect::<Vec<AssetInfo>>();

    let template = Template::new(
        "themes",
        json!({
            "title": "Themes",
            "themes": to_value(theme_info).unwrap(),
            "library": {
                "assets": to_value(asset_info).unwrap(),
            }
        }),
    );

    req.state()
        .renderer()
        .send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
        .responder()
}
