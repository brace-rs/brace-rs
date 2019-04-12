use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::model::{CurrentUser, User};

pub fn get(
    user: CurrentUser,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(render(&renderer)),
    }
}

pub fn post(
    user: CurrentUser,
    data: Form<User>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::create::create(&database, data.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|user| HttpRedirect::to(format!("/users/{}", user.id))),
        ),
    }
}

fn render(renderer: &Renderer) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "user-form",
        json!({
            "title": "Create user",
            "user": User::default(),
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
