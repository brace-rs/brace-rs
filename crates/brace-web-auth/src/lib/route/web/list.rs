use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web::render::{Renderer, Template};
use futures::future::{err, Either, Future};
use serde_json::json;

use crate::model::{CurrentUser, User};

pub fn get(
    user: CurrentUser,
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::list::list(&database)
                .map_err(ErrorInternalServerError)
                .and_then(move |users| render(users, &renderer)),
        ),
    }
}

fn render(
    users: Vec<User>,
    renderer: &Renderer,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let template = Template::new(
        "user-list",
        json!({
            "title": "Users",
            "users": users,
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
