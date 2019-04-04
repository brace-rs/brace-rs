use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde_json::json;

use crate::model::Page;

pub fn create(
    database: Data<Database>,
    page: Json<Page>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::create::create(&database, page.into_inner())
        .map_err(ErrorInternalServerError)
        .and_then(|page| {
            HttpResponse::Created()
                .header(header::LOCATION, format!("/pages/{}", page.id))
                .json(json!({
                    "value": page,
                }))
        })
}
