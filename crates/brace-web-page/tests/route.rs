use actix::System;
use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_web::http::header::{self, HeaderValue};
use actix_web::http::{Method, StatusCode};
use actix_web::App;
use brace_db::{Database, DatabaseConfig};
use brace_web_page::action::install::install;
use brace_web_page::action::uninstall::uninstall;
use brace_web_page::model::Page;
use brace_web_page::route::routes;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_page_route_lifecycle() {
    let mut system = System::new("test");
    let database = Database::from_config(DatabaseConfig::default()).unwrap();
    let uuid = Uuid::new_v4();
    let path = format!("/pages/{}", uuid);
    let page = Page {
        id: uuid,
        title: "A".to_string(),
        content: "A".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(install(&database)).unwrap();

    let mut srv = TestServer::new(|| {
        HttpService::new(
            App::new()
                .data(Database::from_config(DatabaseConfig::default()).unwrap())
                .service(routes()),
        )
    });

    let req = srv.request(Method::GET, srv.url("/pages/")).send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::POST, srv.url("/pages/"))
        .send_json(&page);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_eq!(
        res.headers().get(header::LOCATION),
        Some(&HeaderValue::from_str(&path).unwrap())
    );

    let req = srv
        .request(Method::GET, srv.url(&format!("/pages/{}", uuid)))
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::PUT, srv.url(&format!("/pages/{}", uuid)))
        .send_json(&page);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::DELETE, srv.url(&format!("/pages/{}", uuid)))
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    system.block_on(uninstall(&database)).unwrap();
}
