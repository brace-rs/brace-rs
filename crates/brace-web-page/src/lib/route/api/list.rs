use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde_json::json;

pub fn list(database: Data<Database>) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::list::list(&database)
        .map_err(ErrorInternalServerError)
        .and_then(|pages| {
            HttpResponse::Ok().json(json!({
                "value": pages,
            }))
        })
}
