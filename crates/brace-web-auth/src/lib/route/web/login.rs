use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::middleware::identity::Identity;
use actix_web::web::{Data, Form as FormExtractor};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_form::{Form, FormData};
use futures::future::{err, ok, Either, Future};
use serde_json::json;

use crate::form::login::LoginForm;
use crate::model::UserAuth;
use crate::util::verify;

pub fn get(
    id: Identity,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match id.identity() {
        Some(_) => Either::A(ok(HttpRedirect::to("/").into_response())),
        None => Either::B(render(UserAuth::default(), renderer, None)),
    }
}

pub fn post(
    id: Identity,
    auth: FormExtractor<UserAuth>,
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
                        renderer,
                        Some("Invalid user credentials"),
                    )))
                }
            }
            Err(e) => Either::A(err(ErrorInternalServerError(e))),
        },
        Err(_) => Either::B(Box::new(render(
            auth.into_inner(),
            renderer,
            Some("Invalid user credentials"),
        ))),
    })
}

fn render(
    auth: UserAuth,
    renderer: Data<Renderer>,
    message: Option<&'static str>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match FormData::with(auth) {
        Ok(data) => Either::A(
            Form::build(LoginForm, data)
                .map_err(ErrorInternalServerError)
                .and_then(move |form| {
                    let template = Template::new(
                        "form-layout",
                        json!({
                            "title": "Log in",
                            "message": message,
                            "form": form,
                        }),
                    );

                    renderer
                        .send(template)
                        .map_err(ErrorInternalServerError)
                        .and_then(|res| match res {
                            Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
                            Err(err) => Err(ErrorInternalServerError(err)),
                        })
                }),
        ),
        Err(e) => Either::B(err(ErrorInternalServerError(e))),
    }
}
