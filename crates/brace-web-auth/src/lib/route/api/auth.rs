use actix_web::error::Error;
use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use brace_db::Database;
use futures::future::Future;
use serde_json::json;

use crate::model::UserAuth;
use crate::util::{create_token, verify};

pub fn post(
    data: Json<UserAuth>,
    database: Data<Database>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    crate::action::locate::locate(&database, data.email.clone()).then(move |res| {
        if let Ok(user) = res {
            if let Ok(is_match) = verify(&data.password, &user.password) {
                if is_match {
                    if let Ok(token) = create_token(user.into()) {
                        return HttpResponse::Ok().json(json!({ "token": token }));
                    }
                }
            }
        }

        HttpResponse::Unauthorized()
            .header(
                "WWW-Authenticate",
                r#"Bearer realm="localhost", charset="UTF-8""#,
            )
            .finish()
    })
}
