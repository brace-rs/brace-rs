use actix::System;
use actix::{Handler, Message};
use futures::future::lazy;

use brace::app::database::{Database, DatabaseConfig, DatabaseInner};

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
            Database::from_config(DatabaseConfig::default())
                .unwrap()
                .send(Msg(5))
        }))
        .unwrap()
        .unwrap();

    assert_eq!(res.0, 5);
}
