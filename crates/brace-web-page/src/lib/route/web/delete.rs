use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_auth::model::CurrentUser;
use futures::future::{err, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::Page;

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
                .and_then(move |page| render(page, &renderer)),
        ),
    }
}

pub fn post(
    user: CurrentUser,
    info: Path<Info>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::delete::delete(&database, info.page)
                .map_err(ErrorInternalServerError)
                .and_then(|_| HttpRedirect::to("/pages/")),
        ),
    }
}

fn render(page: Page, renderer: &Renderer) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "form-confirm",
        json!({
            "title": format!("Delete page <em>{}</em>?", page.title),
            "message": format!("Are you sure that you want to delete the page <em>{}</em>?", page.title),
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
