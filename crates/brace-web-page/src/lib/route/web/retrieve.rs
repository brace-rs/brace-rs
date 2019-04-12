use actix_web::error::{Error, ErrorForbidden, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use brace_db::Database;
use brace_web::redirect::HttpRedirect;
use brace_web_auth::model::CurrentUser;
use futures::future::{err, Either, Future};
use serde::Deserialize;
use uuid::Uuid;

pub fn get(
    user: CurrentUser,
    info: Path<Info>,
    database: Data<Database>,
) -> impl Future<Item = HttpRedirect, Error = Error> {
    match user {
        CurrentUser::Anonymous => Either::A(err(ErrorForbidden("Forbidden"))),
        CurrentUser::Authenticated(_) => Either::B(
            crate::action::retrieve_path::retrieve_path(&database, info.page)
                .map_err(ErrorInternalServerError)
                .and_then(HttpRedirect::found),
        ),
    }
}

#[derive(Deserialize)]
pub struct Info {
    page: Uuid,
}
