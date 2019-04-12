use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use brace_web::redirect::HttpRedirect;
use brace_web_auth::model::CurrentUser;
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::model::{Page, PageWithPath};

pub fn get(
    user: CurrentUser,
    database: Data<Database>,
    renderer: Data<Renderer>,
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

pub fn post(
    user: CurrentUser,
    page: Form<Page>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::create::create(&database, page.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|page| HttpRedirect::to(format!("/pages/{}", page.id))),
        ),
    }
}

fn render(
    pages: Vec<PageWithPath>,
    renderer: &Renderer,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "page-form",
        json!({
            "title": "Create page",
            "page": Page::default(),
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
