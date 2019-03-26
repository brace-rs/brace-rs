use crate::app::AppState;
use crate::util::render::Template;
use actix_web::error::ErrorInternalServerError;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse};
use futures::future::Future;
use serde_json::json;

pub fn get(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let template = Template::new(
        "index.html",
        json!({
            "title": "Under Construction",
            "message": "This site is currently under construction, please come back later.",
        }),
    );

    req.state()
        .renderer
        .send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
        .responder()
}
