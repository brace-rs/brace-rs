use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form as FormExtractor};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_auth::model::CurrentUser;
use brace_web_form::{Form, FormData};
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::form::page::PageForm;
use crate::model::Page;

pub fn get(
    user: CurrentUser,
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(render(database, renderer)),
    }
}

pub fn post(
    user: CurrentUser,
    page: FormExtractor<Page>,
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
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match FormData::with(Page::default()) {
        Ok(data) => {
            let mut form = Form::new(()).with(data);

            form.builder(PageForm {
                database: (*database).clone(),
            });

            Either::A(
                form.build()
                    .map_err(ErrorInternalServerError)
                    .and_then(move |form| {
                        let template = Template::new(
                            "form-layout",
                            json!({
                                "title": "Create page",
                                "form": form,
                            }),
                        );

                        renderer
                            .send(template)
                            .map_err(ErrorInternalServerError)
                            .and_then(move |res| match res {
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
