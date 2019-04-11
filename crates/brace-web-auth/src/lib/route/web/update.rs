use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Form, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use brace_web::redirect::HttpRedirect;
use futures::future::Future;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::User;

pub fn get(
    info: Path<Info>,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::retrieve::retrieve(&database, info.user)
        .map_err(ErrorInternalServerError)
        .and_then(move |user| {
            let template = Template::new(
                "user-form",
                json!({
                    "title": format!("Update user <em>{}</em>", user.email),
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
        })
}

pub fn post(
    user: Form<User>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    crate::action::update::update(&database, user.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|user| HttpRedirect::to(format!("/users/{}", user.id)))
}

#[derive(Deserialize)]
pub struct Info {
    user: Uuid,
}
