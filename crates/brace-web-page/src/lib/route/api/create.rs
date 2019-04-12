use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::http::header;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web_auth::model::CurrentAuth;
use futures::future::{ok, Either, Future};
use serde_json::json;

use crate::model::Page;

pub fn create(
    auth: CurrentAuth,
    database: Data<Database>,
    page: Json<Page>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match auth {
        CurrentAuth::Unauthenticated => Either::A(ok(HttpResponse::Unauthorized()
            .header(
                "WWW-Authenticate",
                r#"Bearer realm="localhost", charset="UTF-8""#,
            )
            .finish())),
        CurrentAuth::Authenticated(_) => Either::B(
            crate::action::create::create(&database, page.into_inner())
                .map_err(ErrorInternalServerError)
                .and_then(|page| {
                    HttpResponse::Created()
                        .header(header::LOCATION, format!("/api/pages/{}", page.id))
                        .json(json!({
                            "value": page,
                        }))
                }),
        ),
    }
}
