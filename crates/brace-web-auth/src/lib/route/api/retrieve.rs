use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

pub fn retrieve(
    database: Data<Database>,
    path: Path<Info>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::retrieve::retrieve(&database, path.user)
        .map_err(ErrorInternalServerError)
        .and_then(|user| {
            HttpResponse::Ok().json(json!({
                "value": user,
            }))
        })
}

#[derive(Deserialize)]
pub struct Info {
    user: Uuid,
}
