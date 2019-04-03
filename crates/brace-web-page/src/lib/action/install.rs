use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

static QUERY: &'static str = r#"
    CREATE TABLE pages (
        id uuid PRIMARY KEY,
        title text NOT NULL CHECK (title <> ''),
        content text NOT NULL DEFAULT '',
        created timestamp with time zone NOT NULL DEFAULT now(),
        updated timestamp with time zone NOT NULL DEFAULT now()
    )
"#;

pub fn install(database: &Database) -> impl Future<Item = (), Error = Error> {
    database
        .send(Install)
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Install;

impl Message for Install {
    type Result = Result<(), Error>;
}

impl Handler<Install> for DatabaseInner {
    type Result = Result<(), Error>;

    fn handle(&mut self, _: Install, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;

        conn.execute(QUERY, &[])?;

        Ok(())
    }
}
