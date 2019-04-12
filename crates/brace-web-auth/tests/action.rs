use actix::System;
use brace_db::{Database, DatabaseConfig};
use brace_web_auth::action::create::create;
use brace_web_auth::action::delete::delete;
use brace_web_auth::action::install::install;
use brace_web_auth::action::list::list;
use brace_web_auth::action::locate::locate;
use brace_web_auth::action::retrieve::retrieve;
use brace_web_auth::action::uninstall::uninstall;
use brace_web_auth::action::update::update;
use brace_web_auth::model::User;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_user_lifecycle() {
    let mut system = System::new("test");
    let database = Database::from_config(DatabaseConfig::default()).unwrap();
    let uuid = Uuid::new_v4();
    let user = User {
        id: uuid,
        email: "user1@domain.test".to_string(),
        password: "password1".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    let uuid2 = Uuid::new_v4();
    let user2 = User {
        id: uuid2,
        email: "user2@domain.test".to_string(),
        password: "password2".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    system.block_on(install(&database)).unwrap();

    assert!(system.block_on(create(&database, user.clone())).is_ok());
    assert!(system.block_on(create(&database, user.clone())).is_err());
    assert!(system.block_on(create(&database, user2.clone())).is_ok());
    assert!(system.block_on(update(&database, user.clone())).is_ok());
    assert!(system.block_on(retrieve(&database, uuid)).is_ok());
    assert!(system
        .block_on(retrieve(&database, Uuid::new_v4()))
        .is_err());
    assert!(system
        .block_on(locate(&database, "user1@domain.test"))
        .is_ok());
    assert!(system
        .block_on(locate(&database, "user2@domain.test"))
        .is_ok());
    assert!(system
        .block_on(locate(&database, "user3@domain.test"))
        .is_err());
    assert!(system.block_on(delete(&database, uuid)).is_ok());
    assert!(system.block_on(delete(&database, uuid2)).is_ok());
    assert!(system.block_on(delete(&database, uuid2)).is_err());
    assert!(system.block_on(delete(&database, Uuid::new_v4())).is_err());
    assert!(system.block_on(retrieve(&database, uuid)).is_err());

    assert_eq!(
        system
            .block_on(create(&database, user.clone()))
            .unwrap()
            .email,
        "user1@domain.test"
    );
    assert_eq!(
        system.block_on(retrieve(&database, uuid)).unwrap().email,
        "user1@domain.test"
    );

    let user = User {
        id: uuid,
        email: "user3@domain.test".to_string(),
        password: "password3".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    assert_eq!(
        system
            .block_on(update(&database, user.clone()))
            .unwrap()
            .email,
        "user3@domain.test"
    );
    assert_eq!(
        system.block_on(retrieve(&database, uuid)).unwrap().email,
        "user3@domain.test"
    );

    let uuid = Uuid::new_v4();
    let user = User {
        id: uuid,
        email: "user4@domain.test".to_string(),
        password: "password4".to_string(),
        created: Utc::now(),
        updated: Utc::now(),
    };

    assert!(system.block_on(create(&database, user.clone())).is_ok());
    assert_eq!(system.block_on(list(&database)).unwrap().len(), 2);
    assert!(system.block_on(uninstall(&database)).is_ok());
}
