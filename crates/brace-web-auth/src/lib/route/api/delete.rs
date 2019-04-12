use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::{Data, Path};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::{ok, Either, Future};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::model::CurrentAuth;

pub fn delete(
    auth: CurrentAuth,
    database: Data<Database>,
    path: Path<Info>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match auth {
        CurrentAuth::Unauthenticated => Either::A(ok(HttpResponse::Unauthorized()
            .header(
                "WWW-Authenticate",
                r#"Bearer realm="localhost", charset="UTF-8""#,
            )
            .finish())),
        CurrentAuth::Authenticated(_) => Either::B(
            crate::action::delete::delete(&database, path.user)
                .map_err(ErrorInternalServerError)
                .and_then(|user| {
                    HttpResponse::Ok().json(json!({
                        "value": user,
                    }))
                }),
        ),
    }
}

#[derive(Deserialize)]
pub struct Info {
    user: Uuid,
}
