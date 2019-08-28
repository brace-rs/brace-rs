use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::User;

static QUERY: &str = r#"
    SELECT id, email, password, created, updated
    FROM users
"#;

pub fn list(database: &Database) -> impl Future<Item = Vec<User>, Error = Error> {
    database
        .send(List)
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct List;

impl Message for List {
    type Result = Result<Vec<User>, Error>;
}

impl Handler<List> for DatabaseInner {
    type Result = Result<Vec<User>, Error>;

    fn handle(&mut self, _: List, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[])?;

        Ok(rows
            .iter()
            .map(|row| User {
                id: row.get(0),
                email: row.get(1),
                password: row.get(2),
                created: row.get(3),
                updated: row.get(4),
            })
            .collect())
    }
}
