use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;
use uuid::Uuid;

static QUERY: &str = r#"
    WITH RECURSIVE cte AS (
        SELECT id, parent, slug, '/' || slug AS path
        FROM pages
        WHERE parent is null
        UNION ALL
        SELECT t.id, t.parent, t.slug, concat_ws('/', r.path, t.slug) AS path
        FROM pages t
        JOIN cte r ON t.parent = r.id
    )
    SELECT path
    FROM cte
    WHERE id = $1
"#;

pub fn retrieve_path(database: &Database, page: Uuid) -> impl Future<Item = String, Error = Error> {
    database
        .send(RetrievePath(page))
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct RetrievePath(pub Uuid);

impl Message for RetrievePath {
    type Result = Result<String, Error>;
}

impl Handler<RetrievePath> for DatabaseInner {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: RetrievePath, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;
        let rows = conn.query(QUERY, &[&msg.0])?;

        if rows.is_empty() {
            return Err(format_err!("Row not found"));
        }

        let row = rows.get(0);
        let path: String = row.get(0);

        Ok(path)
    }
}
