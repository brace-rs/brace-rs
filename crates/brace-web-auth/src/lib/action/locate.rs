use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::User;

static QUERY: &str = r#"
    SELECT id, email, password, created, updated
    FROM users
    WHERE email = $1
"#;

pub fn locate<S: Into<String>>(
    database: &Database,
    user: S,
) -> impl Future<Item = User, Error = Error> {
    database
        .send(Locate(user.into()))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Locate(pub String);

impl Message for Locate {
    type Result = Result<User, Error>;
}

impl Handler<Locate> for DatabaseInner {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: Locate, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[&msg.0])?;

        if rows.is_empty() {
            return Err(format_err!("Row not found"));
        }

        let row = rows.get(0);

        Ok(User {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2),
            created: row.get(3),
            updated: row.get(4),
        })
    }
}
