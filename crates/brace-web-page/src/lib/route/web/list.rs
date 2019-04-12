use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::render::{Renderer, Template};
use brace_web_auth::model::CurrentUser;
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::model::PageWithPath;

pub fn get(
    user: CurrentUser,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::list::list(&database)
                .map_err(ErrorInternalServerError)
                .and_then(move |pages| render(pages, &renderer)),
        ),
    }
}

fn render(
    pages: Vec<PageWithPath>,
    renderer: &Renderer,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "page-list",
        json!({
            "title": "Pages",
            "pages": pages,
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
