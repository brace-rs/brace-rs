use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::Page;

static QUERY: &'static str = r#"
    SELECT id, parent, slug, title, content, created, updated
    FROM pages
"#;

pub fn list(database: &Database) -> impl Future<Item = Vec<Page>, Error = Error> {
    database
        .send(List)
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct List;

impl Message for List {
    type Result = Result<Vec<Page>, Error>;
}

impl Handler<List> for DatabaseInner {
    type Result = Result<Vec<Page>, Error>;

    fn handle(&mut self, _: List, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[])?;

        Ok(rows
            .iter()
            .map(|row| Page {
                id: row.get(0),
                parent: row.get(1),
                slug: row.get(2),
                title: row.get(3),
                content: row.get(4),
                created: row.get(5),
                updated: row.get(6),
            })
            .collect())
    }
}
