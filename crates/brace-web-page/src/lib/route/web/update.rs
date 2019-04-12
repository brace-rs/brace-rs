use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_auth::model::CurrentUser;
use futures::future::{err, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::{Page, PageWithPath};

pub fn get(
    user: CurrentUser,
    info: Path<Info>,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::retrieve::retrieve(&database, info.page)
                .map_err(ErrorInternalServerError)
                .and_then(move |page| {
                    crate::action::list::list(&database)
                        .map_err(ErrorInternalServerError)
                        .and_then(move |pages| render(page, pages, &renderer))
                }),
        ),
    }
}

pub fn post(
    user: CurrentUser,
    page: Form<Page>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::update::update(&database, page.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|page| HttpRedirect::to(format!("/pages/{}", page.id))),
        ),
    }
}

fn render(
    page: Page,
    pages: Vec<PageWithPath>,
    renderer: &Renderer,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "page-form",
        json!({
            "title": format!("Update page <em>{}</em>", page.title),
            "page": page,
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

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
