use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::User;
use crate::util::hash;

static QUERY: &'static str = r#"
    INSERT INTO users (id, email, password, created, updated)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, email, password, created, updated
"#;

pub fn create(database: &Database, user: User) -> impl Future<Item = User, Error = Error> {
    database
        .send(Create(user))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Create(pub User);

impl Message for Create {
    type Result = Result<User, Error>;
}

impl Handler<Create> for DatabaseInner {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(
            QUERY,
            &[
                &msg.0.id,
                &msg.0.email,
                &hash(&msg.0.password)?,
                &msg.0.created,
                &msg.0.updated,
            ],
        )?;

        if rows.is_empty() {
            return Err(format_err!("Row not returned"));
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
