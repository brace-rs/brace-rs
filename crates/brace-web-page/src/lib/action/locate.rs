use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::Page;

static QUERY: &'static str = r#"
    WITH RECURSIVE cte AS (
        SELECT id, parent, slug, title, description, created, updated, '/' || slug AS path
        FROM pages
        WHERE parent is null
        UNION ALL
        SELECT t.id, t.parent, t.slug, t.title, t.description, t.created, t.updated, concat_ws('/', r.path, t.slug) AS path
        FROM pages t
        JOIN cte r ON t.parent = r.id
    )
    SELECT id, parent, slug, title, description, created, updated
    FROM cte
    WHERE path = $1
"#;

pub fn locate<S: Into<String>>(
    database: &Database,
    page: S,
) -> impl Future<Item = Page, Error = Error> {
    database
        .send(Locate(page.into()))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Locate(pub String);

impl Message for Locate {
    type Result = Result<Page, Error>;
}

impl Handler<Locate> for DatabaseInner {
    type Result = Result<Page, Error>;

    fn handle(&mut self, msg: Locate, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[&msg.0])?;

        if rows.is_empty() {
            return Err(format_err!("Row not found"));
        }

        let row = rows.get(0);

        Ok(Page {
            id: row.get(0),
            parent: row.get(1),
            slug: row.get(2),
            title: row.get(3),
            description: row.get(4),
            created: row.get(5),
            updated: row.get(6),
        })
    }
}
