use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use brace_theme::renderer::{Renderer, Template};
use futures::future::Future;
use serde_json::json;

pub fn get(
    renderer: Data<Renderer>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::list::list(&database)
        .map_err(ErrorInternalServerError)
        .and_then(move |users| {
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
        })
}
