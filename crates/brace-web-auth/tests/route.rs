use actix::System;
use actix_http::HttpService;
use actix_http_test::TestServer;
use actix_web::http::header::{self, HeaderValue};
use actix_web::http::{Method, StatusCode};
use actix_web::App;
use brace_db::{Database, DatabaseConfig};
use brace_web_auth::action::create::create;
use brace_web_auth::action::install::install;
use brace_web_auth::action::uninstall::uninstall;
use brace_web_auth::model::User;
use chrono::Utc;
use futures::future::Future;
use serde_json::{json, Value};
use uuid::Uuid;

#[test]
fn test_user_route_lifecycle() {
    let mut system = System::new("test");
    let database = Database::from_config(DatabaseConfig::default()).unwrap();
    let uuid = Uuid::new_v4();
    let path = format!("/api/users/{}", uuid);
    let user = User {
        id: uuid,
        email: "user1@domain.test".to_string(),
        password: "password1".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(install(&database)).unwrap();

    let mut srv = TestServer::new(|| {
        HttpService::new(
            App::new()
                .data(Database::from_config(DatabaseConfig::default()).unwrap())
                .configure(brace_web_auth::route::api::config),
        )
    });

    let req = srv.request(Method::GET, srv.url("/api/users/")).send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let header = res
        .headers()
        .get("WWW-Authenticate")
        .unwrap()
        .to_str()
        .unwrap();

    assert_eq!(header, r#"Bearer realm="localhost", charset="UTF-8""#);

    let req = srv
        .request(Method::GET, srv.url("/api/users/"))
        .header("Authorization", "Bearer invalidtoken")
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let req = srv
        .request(Method::POST, srv.url("/api/users/"))
        .send_json(&user);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let req = srv.request(Method::GET, srv.url(&path)).send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let req = srv.request(Method::PUT, srv.url(&path)).send_json(&user);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let req = srv.request(Method::DELETE, srv.url(&path)).send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    let admin = User {
        id: Uuid::new_v4(),
        email: "admin@domain.test".to_string(),
        password: "password".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(create(&database, admin)).unwrap();

    let auth = json!({
        "email": "admin@domain.test",
        "password": "password",
    });

    let req = srv
        .request(Method::POST, srv.url("/api/auth"))
        .send_json(&auth);
    let mut res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let json = res.json::<Value>().wait().unwrap();
    let token = json.get("token").unwrap();
    let header = format!("Bearer {}", token.as_str().unwrap());

    let uuid = Uuid::new_v4();
    let path = format!("/api/users/{}", uuid);
    let user = User {
        id: uuid,
        email: "user1@domain.test".to_string(),
        password: "password1".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    let req = srv
        .request(Method::GET, srv.url("/api/users/"))
        .header("Authorization", header.clone())
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::POST, srv.url("/api/users/"))
        .header("Authorization", header.clone())
        .send_json(&user);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    assert_eq!(
        res.headers().get(header::LOCATION),
        Some(&HeaderValue::from_str(&path).unwrap())
    );

    let req = srv
        .request(Method::GET, srv.url(&path))
        .header("Authorization", header.clone())
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::PUT, srv.url(&path))
        .header("Authorization", header.clone())
        .send_json(&user);
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let req = srv
        .request(Method::DELETE, srv.url(&path))
        .header("Authorization", header.clone())
        .send();
    let res = srv.block_on(req).unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    system.block_on(uninstall(&database)).unwrap();
}
