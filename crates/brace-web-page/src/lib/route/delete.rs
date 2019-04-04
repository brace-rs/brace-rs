use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

pub fn delete(
    database: Data<Database>,
    path: Path<Info>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::delete::delete(&database, path.page)
        .map_err(ErrorInternalServerError)
        .and_then(|page| {
            HttpResponse::Ok().json(json!({
                "value": page,
            }))
        })
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
