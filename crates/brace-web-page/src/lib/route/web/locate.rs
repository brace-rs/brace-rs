use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use brace_db::Database;
use brace_web::render::{Renderer, Template};
use futures::future::Future;
use serde_json::json;

pub fn get(
    req: HttpRequest,
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::locate::locate(&database, req.match_info().path().to_owned())
        .map_err(ErrorInternalServerError)
        .and_then(move |page| {
            let template = Template::new(
                "page",
                json!({
                    "title": page.title,
                    "page": page,
                }),
            );

            renderer
                .send(template)
                .map_err(ErrorInternalServerError)
                .and_then(move |res| match res {
                    Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
                    Err(err) => Err(ErrorInternalServerError(err)),
                })
        })
}
