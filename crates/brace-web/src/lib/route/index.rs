use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_theme::renderer::Template;
use futures::future::Future;
use serde_json::json;

use crate::state::AppState;

pub fn get(data: Data<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "index",
        json!({
            "title": "Under Construction",
            "message": "This site is currently under construction, please come back later.",
        }),
    );

    data.renderer()
        .send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
}
