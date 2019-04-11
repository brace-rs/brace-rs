use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde_json::json;

use crate::model::User;

pub fn create(
    database: Data<Database>,
    user: Json<User>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::create::create(&database, user.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|user| {
            HttpResponse::Created()
                .header(header::LOCATION, format!("/api/users/{}", user.id))
                .json(json!({
                    "value": user,
                }))
        })
}
