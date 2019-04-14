use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

use crate::model::PageWithPath;

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
    SELECT id, parent, slug, title, description, created, updated, path
    FROM cte
    ORDER BY path
"#;

pub fn list(database: &Database) -> impl Future<Item = Vec<PageWithPath>, Error = Error> {
    database
        .send(List)
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct List;

impl Message for List {
    type Result = Result<Vec<PageWithPath>, Error>;
}

impl Handler<List> for DatabaseInner {
    type Result = Result<Vec<PageWithPath>, Error>;

    fn handle(&mut self, _: List, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[])?;

        Ok(rows
            .iter()
            .map(|row| PageWithPath {
                id: row.get(0),
                parent: row.get(1),
                slug: row.get(2),
                title: row.get(3),
                description: row.get(4),
                created: row.get(5),
                updated: row.get(6),
                path: row.get(7),
            })
            .collect())
    }
}
