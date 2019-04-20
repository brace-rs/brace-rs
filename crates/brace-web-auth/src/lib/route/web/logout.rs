use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::middleware::identity::Identity;
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use futures::future::{err, ok, Either, Future};
use serde_json::json;
use uuid::Uuid;

use crate::model::User;

pub fn get(
    id: Identity,
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match id.identity() {
        Some(user) => match user.parse::<Uuid>() {
            Ok(uuid) => Either::B(
                crate::action::retrieve::retrieve(&database, uuid)
                    .map_err(ErrorInternalServerError)
                    .and_then(move |user| render(user, &renderer)),
            ),
            Err(e) => {
                id.forget();

                Either::A(err(ErrorInternalServerError(e)))
            }
        },
        None => Either::A(ok(HttpRedirect::to("/login").into_response())),
    }
}

pub fn post(id: Identity) -> impl Future<Item = HttpRedirect, Error = Error> {
    match id.identity() {
        Some(_) => {
            id.forget();

            ok(HttpRedirect::to("/"))
        }
        None => err(ErrorForbidden("Forbidden")),
    }
}

fn render(user: User, renderer: &Renderer) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "form-confirm",
        json!({
            "title": "Log out",
            "message": format!("Are you sure that you want to log out of user <em>{}</em>?", user.email),
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
