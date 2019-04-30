use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Form as FormExtractor, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web::render::{Renderer, Template};
use brace_web_auth::model::CurrentUser;
use brace_web_form::{Form, FormData};
use futures::future::{err, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::form::page::PageForm;
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
                .and_then(move |page| render(page, database, renderer)),
        ),
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
            crate::action::update::update(&database, page.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|page| HttpRedirect::to(format!("/pages/{}", page.id))),
        ),
    }
}

fn render(
    page: Page,
    database: Data<Database>,
    renderer: Data<Renderer>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let title = format!("Update page <em>{}</em>", page.title);

    match FormData::with(page) {
        Ok(data) => Either::A(
            Form::build(
                PageForm {
                    database: (*database).clone(),
                },
                (),
                data,
            )
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
                        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
                        Err(err) => Err(ErrorInternalServerError(err)),
                    })
            }),
        ),
        Err(e) => Either::B(err(ErrorInternalServerError(e))),
    }
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
