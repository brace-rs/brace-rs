use actix::System;
use brace_db::{Database, DatabaseConfig};
use brace_web_page::action::create::create;
use brace_web_page::action::delete::delete;
use brace_web_page::action::install::install;
use brace_web_page::action::list::list;
use brace_web_page::action::locate::locate;
use brace_web_page::action::retrieve::retrieve;
use brace_web_page::action::uninstall::uninstall;
use brace_web_page::action::update::update;
use brace_web_page::model::Page;
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

#[test]
fn test_page_lifecycle() {
    let mut system = System::new("test");
    let database = Database::from_config(DatabaseConfig::default()).unwrap();
    let uuid = Uuid::new_v4();
    let page = Page {
        id: uuid,
        parent: None,
        slug: "foo".to_string(),
        title: "Foo".to_string(),
        description: "FOO".to_string(),
        document: json!({}),
        created: Utc::now(),
        updated: Utc::now(),
    };

    let uuid2 = Uuid::new_v4();
    let page2 = Page {
        id: uuid2,
        parent: Some(uuid),
        slug: "bar".to_string(),
        title: "Bar".to_string(),
        description: "BAR".to_string(),
        document: json!({}),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(install(&database)).unwrap();

    assert!(system.block_on(create(&database, page.clone())).is_ok());
    assert!(system.block_on(create(&database, page.clone())).is_err());
    assert!(system.block_on(create(&database, page2.clone())).is_ok());
    assert!(system.block_on(update(&database, page.clone())).is_ok());
    assert!(system.block_on(retrieve(&database, uuid)).is_ok());
    assert!(system
        .block_on(retrieve(&database, Uuid::new_v4()))
        .is_err());
    assert!(system.block_on(locate(&database, "/foo")).is_ok());
    assert!(system.block_on(locate(&database, "/foo/bar")).is_ok());
    assert!(system.block_on(locate(&database, "/bar")).is_err());
    assert!(system.block_on(delete(&database, uuid)).is_err());
    assert!(system.block_on(delete(&database, uuid2)).is_ok());
    assert!(system.block_on(delete(&database, uuid)).is_ok());
    assert!(system.block_on(delete(&database, Uuid::new_v4())).is_err());
    assert!(system.block_on(retrieve(&database, uuid)).is_err());

    assert_eq!(
        system
            .block_on(create(&database, page.clone()))
            .unwrap()
            .title,
        "Foo"
    );
    assert_eq!(
        system.block_on(retrieve(&database, uuid)).unwrap().title,
        "Foo"
    );

    let page = Page {
        id: uuid,
        parent: None,
        slug: "b".to_string(),
        title: "B".to_string(),
        description: "B".to_string(),
        document: json!({}),
        created: Utc::now(),
        updated: Utc::now(),
    };

    assert_eq!(
        system
            .block_on(update(&database, page.clone()))
            .unwrap()
            .title,
        "B"
    );
    assert_eq!(
        system.block_on(retrieve(&database, uuid)).unwrap().title,
        "B"
    );

    let uuid = Uuid::new_v4();
    let page = Page {
        id: uuid,
        parent: None,
        slug: "c".to_string(),
        title: "C".to_string(),
        description: "C".to_string(),
        document: json!({}),
        created: Utc::now(),
        updated: Utc::now(),
    };

    assert!(system.block_on(create(&database, page.clone())).is_ok());
    assert_eq!(system.block_on(list(&database)).unwrap().len(), 2);
    assert!(system.block_on(uninstall(&database)).is_ok());
}
