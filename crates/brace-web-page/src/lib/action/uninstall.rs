use actix::{Handler, Message};
use brace_db::{Database, DatabaseInner};
use failure::{format_err, Error};
use futures::future::Future;

static QUERY: &str = r#"
    DROP TABLE pages
"#;

pub fn uninstall(database: &Database) -> impl Future<Item = (), Error = Error> {
    database
        .send(Uninstall)
        .map_err(|err| format_err!("{}", err))
        .and_then(|res| res)
}

pub struct Uninstall;

impl Message for Uninstall {
    type Result = Result<(), Error>;
}

impl Handler<Uninstall> for DatabaseInner {
    type Result = Result<(), Error>;

    fn handle(&mut self, _: Uninstall, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get()?;

        conn.execute(QUERY, &[])?;

        Ok(())
    }
}
