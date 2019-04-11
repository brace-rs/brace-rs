use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::middleware::identity::Identity;
use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use brace_web::redirect::HttpRedirect;
use futures::future::{err, ok, Either, Future};
use serde_json::json;

use crate::model::UserAuth;
use crate::util::verify;

pub fn get(
    id: Identity,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match id.identity() {
        Some(_) => Either::A(ok(HttpRedirect::to("/").into_response())),
        None => Either::B(render(UserAuth::default(), &renderer, None)),
    }
}

pub fn post(
    id: Identity,
    auth: Form<UserAuth>,
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::locate::locate(&database, auth.email.clone()).then(move |res| match res {
        Ok(user) => match verify(&auth.password, &user.password) {
            Ok(is_match) => {
                if is_match {
                    id.remember(user.id.to_string());
                    Either::A(ok(HttpRedirect::to("/").into_response()))
                } else {
                    Either::B(Box::new(render(
                        auth.into_inner(),
                        &renderer,
                        Some("Invalid user credentials"),
                    )))
                }
            }
            Err(e) => Either::A(err(ErrorInternalServerError(e))),
        },
        Err(_) => Either::B(Box::new(render(
            auth.into_inner(),
            &renderer,
            Some("Invalid user credentials"),
        ))),
    })
}

fn render(
    auth: UserAuth,
    renderer: &Renderer,
    message: Option<&str>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "login-form",
        json!({
            "title": "Log in",
            "message": message,
            "auth": auth,
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
