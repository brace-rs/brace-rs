use actix::System;
use brace_db::{Database, DatabaseConfig};
use brace_web_page::action::create::create;
use brace_web_page::action::delete::delete;
use brace_web_page::action::install::install;
use brace_web_page::action::list::list;
use brace_web_page::action::retrieve::retrieve;
use brace_web_page::action::uninstall::uninstall;
use brace_web_page::action::update::update;
use brace_web_page::model::Page;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_page_lifecycle() {
    let mut system = System::new("test");
    let database = Database::from_config(DatabaseConfig::default()).unwrap();
    let uuid = Uuid::new_v4();
    let page = Page {
        id: uuid,
        title: "A".to_string(),
        content: "A".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(install(&database)).unwrap();

    assert!(system.block_on(create(&database, page.clone())).is_ok());
    assert!(system.block_on(create(&database, page.clone())).is_err());
    assert!(system.block_on(update(&database, page.clone())).is_ok());
    assert!(system.block_on(retrieve(&database, uuid)).is_ok());
    assert!(system
        .block_on(retrieve(&database, Uuid::new_v4()))
        .is_err());
    assert!(system.block_on(delete(&database, uuid)).is_ok());
    assert!(system.block_on(delete(&database, Uuid::new_v4())).is_err());
    assert!(system.block_on(retrieve(&database, uuid)).is_err());

    assert_eq!(
        system
            .block_on(create(&database, page.clone()))
            .unwrap()
            .title,
        "A"
    );
    assert_eq!(
        system.block_on(retrieve(&database, uuid)).unwrap().title,
        "A"
    );

    let page = Page {
        id: uuid,
        title: "B".to_string(),
        content: "B".to_string(),
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
        title: "C".to_string(),
        content: "C".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    assert!(system.block_on(create(&database, page.clone())).is_ok());
    assert_eq!(system.block_on(list(&database)).unwrap().len(), 2);
    assert!(system.block_on(uninstall(&database)).is_ok());
}
