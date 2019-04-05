use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Form, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use futures::future::Future;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::Page;

pub fn get(
    info: Path<Info>,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::retrieve::retrieve(&database, info.page)
        .map_err(ErrorInternalServerError)
        .and_then(move |page| {
            let template = Template::new(
                "page-form",
                json!({
                    "title": format!("Update page <em>{}</em>", page.title),
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
    page: Form<Page>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::update::update(&database, page.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|_| {
            Ok(HttpResponse::SeeOther()
                .header(header::LOCATION, "/")
                .finish())
        })
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
