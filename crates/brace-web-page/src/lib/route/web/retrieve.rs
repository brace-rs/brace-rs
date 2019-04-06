use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use futures::future::Future;
use serde::Deserialize;
use uuid::Uuid;

pub fn get(
    info: Path<Info>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    crate::action::retrieve_path::retrieve_path(&database, info.page)
        .map_err(ErrorInternalServerError)
        .and_then(HttpRedirect::found)
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
