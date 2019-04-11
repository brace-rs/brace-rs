use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Form};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use brace_web::redirect::HttpRedirect;
use futures::future::Future;
use serde_json::json;

use crate::model::User;

pub fn get(renderer: Data<Renderer>) -> impl Future<Item = HttpResponse, Error = Error> {
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

pub fn post(
    user: Form<User>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    crate::action::create::create(&database, user.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|user| HttpRedirect::to(format!("/users/{}", user.id)))
}
