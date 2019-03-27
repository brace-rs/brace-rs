use actix_web::error::ErrorInternalServerError;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;
use serde_json::json;

use crate::app::renderer::Template;
use crate::app::theme::config::{ThemeConfig, ThemeInfo};
use crate::app::AppState;

pub fn get(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let themes: Vec<ThemeInfo> = req
        .state()
        .config()
        .theme
        .iter()
        .filter_map(|theme| match ThemeConfig::from_file(&theme.path) {
            Ok(conf) => Some(conf.theme),
            Err(_) => None,
        })
        .collect();

    let template = Template::new(
        "themes",
        json!({
            "title": "Themes",
            "themes": serde_json::to_value(themes).unwrap(),
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
