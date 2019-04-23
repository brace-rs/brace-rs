use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form as FormData};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_form::Form;
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::form::user::UserForm;
use crate::model::{CurrentUser, User};

pub fn get(
    user: CurrentUser,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(render(renderer)),
    }
}

pub fn post(
    user: CurrentUser,
    data: FormData<User>,
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

fn render(renderer: Data<Renderer>) -> impl Future<Item = HttpResponse, Error = Error> {
    Form::build(UserForm, User::default(), ())
        .map_err(ErrorInternalServerError)
        .and_then(move |form| {
            let template = Template::new(
                "form-layout",
                json!({
                    "title": "Create user",
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
        })
}
