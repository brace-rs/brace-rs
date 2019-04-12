use actix_web::error::{Error, ErrorInternalServerError};
use actix_web::web::Data;
use actix_web::HttpResponse;
use brace_db::Database;
use brace_web_auth::model::CurrentAuth;
use futures::future::{ok, Either, Future};
use serde_json::json;

pub fn list(
    auth: CurrentAuth,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    match auth {
        CurrentAuth::Unauthenticated => Either::A(ok(HttpResponse::Unauthorized()
            .header(
                "WWW-Authenticate",
                r#"Bearer realm="localhost", charset="UTF-8""#,
            )
            .finish())),
        CurrentAuth::Authenticated(_) => Either::B(
            crate::action::list::list(&database)
                .map_err(ErrorInternalServerError)
                .and_then(|pages| {
                    HttpResponse::Ok().json(json!({
                        "value": pages,
                    }))
                }),
        ),
    }
}
