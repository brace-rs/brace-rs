use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::{ok, Either, Future};
use serde_json::json;

use crate::model::{CurrentAuth, User};

pub fn create(
    auth: CurrentAuth,
    database: Data<Database>,
    user: Json<User>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match auth {
        CurrentAuth::Unauthenticated => Either::A(ok(HttpResponse::Unauthorized()
            .header(
                "WWW-Authenticate",
                r#"Bearer realm="localhost", charset="UTF-8""#,
            )
            .finish())),
        CurrentAuth::Authenticated(_) => Either::B(
            crate::action::create::create(&database, user.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|user| {
                    HttpResponse::Created()
                        .header(header::LOCATION, format!("/api/users/{}", user.id))
                        .json(json!({
                            "value": user,
                        }))
                }),
        ),
    }
}
