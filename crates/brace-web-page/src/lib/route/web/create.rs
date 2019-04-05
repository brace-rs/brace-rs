use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use futures::future::Future;
use serde_json::json;

use crate::model::Page;

pub fn get(renderer: Data<Renderer>) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "page-form",
        json!({
            "title": "Create page",
            "page": Page::default(),
        }),
    );

    renderer
        .send(template)
        .map_err(ErrorInternalServerError)
        .and_then(|res| match res {
            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
            Err(err) => Err(ErrorInternalServerError(err)),
        })
}

pub fn post(
    page: Form<Page>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::create::create(&database, page.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|page| {
            Ok(HttpResponse::SeeOther()
                .header(header::LOCATION, format!("/pages/{}", page.id))
                .finish())
        })
}
