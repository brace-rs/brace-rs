use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use brace_web::redirect::HttpRedirect;
use futures::future::Future;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

pub fn get(
    info: Path<Info>,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::retrieve::retrieve(&database, info.page)
        .map_err(ErrorInternalServerError)
        .and_then(move |page| {
            let template = Template::new(
                "page-delete-form",
                json!({
                    "title": format!("Delete page <em>{}</em>?", page.title),
                    "page": page,
                }),
            );

            renderer
                .send(template)
                .map_err(ErrorInternalServerError)
                .and_then(|res| match res {
                    Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
                    Err(err) => Err(ErrorInternalServerError(err)),
                })
        })
}

pub fn post(
    info: Path<Info>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    crate::action::delete::delete(&database, info.page)
        .map_err(ErrorInternalServerError)
        .and_then(|_| HttpRedirect::to("/pages/"))
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
