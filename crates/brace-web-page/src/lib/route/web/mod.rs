use actix_web::error::PayloadError;
use actix_web::web::{self, Bytes};
use actix_web::Scope;
use futures::Stream;

pub mod create;
pub mod delete;
pub mod list;
pub mod retrieve;
pub mod update;

type PayloadStream = Box<dyn Stream<Item = Bytes, Error = PayloadError>>;

pub fn routes() -> Scope<PayloadStream> {
    web::scope("/pages")
        .service(web::resource("/").route(web::get().to_async(list::get)))
        .service(
            web::resource("/new")
                .route(web::get().to_async(create::get))
                .route(web::post().to_async(create::post)),
        )
        .service(web::resource("/{page}").route(web::get().to_async(retrieve::get)))
        .service(
            web::resource("/{page}/update")
                .route(web::get().to_async(update::get))
                .route(web::post().to_async(update::post)),
        )
        .service(
            web::resource("/{page}/delete")
                .route(web::get().to_async(delete::get))
                .route(web::post().to_async(delete::post)),
        )
}
