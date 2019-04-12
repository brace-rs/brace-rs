use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use futures::future::{err, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::{CurrentUser, User};

pub fn get(
    user: CurrentUser,
    info: Path<Info>,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::retrieve::retrieve(&database, info.user)
                .map_err(ErrorInternalServerError)
                .and_then(move |user| render(user, &renderer)),
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
            crate::action::delete::delete(&database, info.user)
                .map_err(ErrorInternalServerError)
                .and_then(|_| HttpRedirect::to("/users/")),
        ),
    }
}

fn render(user: User, renderer: &Renderer) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "user-delete-form",
        json!({
            "title": format!("Delete user <em>{}</em>?", user.email),
            "user": user,
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
    user: Uuid,
}
