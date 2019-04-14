use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;
use uuid::Uuid;

use crate::model::Page;

static QUERY: &'static str = r#"
    DELETE FROM pages *
    WHERE id = $1
    RETURNING id, parent, slug, title, description, document, created, updated
"#;

pub fn delete(database: &Database, page: Uuid) -> impl Future<Item = Page, Error = Error> {
    database
        .send(Delete(page))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Delete(pub Uuid);

impl Message for Delete {
    type Result = Result<Page, Error>;
}

impl Handler<Delete> for DatabaseInner {
    type Result = Result<Page, Error>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[&msg.0])?;

        if rows.is_empty() {
            return Err(format_err!("Row not returned"));
        }

        let row = rows.get(0);

        Ok(Page {
            id: row.get(0),
            parent: row.get(1),
            slug: row.get(2),
            title: row.get(3),
            description: row.get(4),
            document: row.get(5),
            created: row.get(6),
            updated: row.get(7),
        })
    }
}
