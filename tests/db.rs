use actix::System;
use actix::{Handler, Message};
use brace::app::config::db::DatabaseConfig;
use brace::util::db::Database;
use brace::util::db::DatabaseInner;
use futures::future::lazy;

struct Msg(i32);

impl Message for Msg {
    type Result = Result<Msg, failure::Error>;
}

impl Handler<Msg> for DatabaseInner {
    type Result = Result<Msg, failure::Error>;

    fn handle(&mut self, msg: Msg, _: &mut Self::Context) -> Self::Result {
        let pool = self.0.get()?;
        let rows = pool.query("SELECT $1::INT4", &[&msg.0])?;
        let row = rows.get(0);
        let num: i32 = row.get(0);

        Ok(Msg(num))
    }
}

#[test]
fn test_database_postgres() {
    let mut system = System::new("brace_test");

    let res = system
        .block_on(lazy(|| {
            Database::new(DatabaseConfig::default()).send(Msg(5))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res.0, 5);
}
