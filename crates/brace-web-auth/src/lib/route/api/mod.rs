use actix_web::error::PayloadError;
use actix_web::web::{self, Bytes, RouterConfig};
use futures::Stream;

pub mod auth;
pub mod create;
pub mod delete;
pub mod list;
pub mod retrieve;
pub mod update;

type PayloadStream = Box<dyn Stream<Item = Bytes, Error = PayloadError>>;

pub fn config(conf: &mut RouterConfig<PayloadStream>) {
    conf.service(web::resource("/api/auth").route(web::post().to_async(auth::post)))
        .service(
            web::scope("/api/users")
                .service(
                    web::resource("/")
                        .route(web::get().to_async(list::list))
                        .route(web::post().to_async(create::create)),
                )
                .service(
                    web::resource("/{user}")
                        .route(web::get().to_async(retrieve::retrieve))
                        .route(web::put().to_async(update::update))
                        .route(web::delete().to_async(delete::delete)),
                ),
        );
}
