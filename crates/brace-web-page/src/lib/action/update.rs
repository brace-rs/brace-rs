use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::Page;

static QUERY: &'static str = r#"
    UPDATE pages
    SET parent = $2, slug = $3, title = $4, description = $5, document = $6, created = $7, updated = $8
    WHERE id = $1
    RETURNING id, parent, slug, title, description, document, created, updated
"#;

pub fn update(database: &Database, page: Page) -> impl Future<Item = Page, Error = Error> {
    database
        .send(Update(page))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Update(pub Page);

impl Message for Update {
    type Result = Result<Page, Error>;
}

impl Handler<Update> for DatabaseInner {
    type Result = Result<Page, Error>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(
            QUERY,
            &[
                &msg.0.id,
                &msg.0.parent,
                &msg.0.slug,
                &msg.0.title,
                &msg.0.description,
                &msg.0.document,
                &msg.0.created,
                &msg.0.updated,
            ],
        )?;

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
