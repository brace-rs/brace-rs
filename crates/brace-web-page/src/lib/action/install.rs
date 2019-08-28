use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

static QUERY: &str = r#"
    CREATE TABLE pages (
        id uuid PRIMARY KEY,
        parent uuid REFERENCES pages(id),
        slug character varying(255) NOT NULL,
        title text NOT NULL CHECK (title <> ''),
        description text NOT NULL DEFAULT '',
        document jsonb NOT NULL DEFAULT '{}'::jsonb,
        created timestamp with time zone NOT NULL DEFAULT now(),
        updated timestamp with time zone NOT NULL DEFAULT now() CHECK (updated >= created)
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
