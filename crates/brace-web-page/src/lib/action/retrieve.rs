use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;
use uuid::Uuid;

use crate::model::Page;

static QUERY: &'static str = r#"
    SELECT id, title, content, created, updated
    FROM pages
    WHERE id = $1
"#;

pub fn retrieve(database: &Database, page: Uuid) -> impl Future<Item = Page, Error = Error> {
    database
        .send(Retrieve(page))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Retrieve(pub Uuid);

impl Message for Retrieve {
    type Result = Result<Page, Error>;
}

impl Handler<Retrieve> for DatabaseInner {
    type Result = Result<Page, Error>;

    fn handle(&mut self, msg: Retrieve, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[&msg.0])?;

        if rows.is_empty() {
            return Err(format_err!("Row not found"));
        }

        let row = rows.get(0);

        Ok(Page {
            id: row.get(0),
            title: row.get(1),
            content: row.get(2),
            created: row.get(3),
            updated: row.get(4),
        })
    }
}
