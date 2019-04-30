use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form as FormExtractor, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_form::{Form, FormData};
use futures::future::{err, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::form::user::UserForm;
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
                .and_then(move |user| render(user, renderer)),
        ),
    }
}

pub fn post(
    user: CurrentUser,
    data: FormExtractor<User>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::update::update(&database, data.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|user| HttpRedirect::to(format!("/users/{}", user.id))),
        ),
    }
}

fn render(user: User, renderer: Data<Renderer>) -> impl Future<Item = HttpResponse, Error = Error> {
    let title = format!("Update user <em>{}</em>", user.email);

    match FormData::with(user) {
        Ok(data) => {
            let mut form = Form::new((), data);

            form.builder(UserForm);

            Either::A(
                form.build()
                    .map_err(ErrorInternalServerError)
                    .and_then(move |form| {
                        let template = Template::new(
                            "form-layout",
                            json!({
                                "title": title,
                                "form": form,
                            }),
                        );

                        renderer
                            .send(template)
                            .map_err(ErrorInternalServerError)
                            .and_then(|res| match res {
                                Ok(body) => {
                                    Ok(HttpResponse::Ok().content_type("text/html").body(body))
                                }
                                Err(err) => Err(ErrorInternalServerError(err)),
                            })
                    }),
            )
        }
        Err(e) => Either::B(err(ErrorInternalServerError(e))),
    }
}

#[derive(Deserialize)]
pub struct Info {
    user: Uuid,
}
