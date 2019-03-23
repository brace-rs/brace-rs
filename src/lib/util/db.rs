use crate::config::db::DatabaseConfig;
use actix::{Actor, Addr, SyncArbiter, SyncContext};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::ops::Deref;

#[derive(Clone)]
pub struct Database(pub Addr<DatabaseInner>);

impl Database {
    pub fn new(conf: DatabaseConfig) -> Self {
        let manager = PostgresConnectionManager::new(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                conf.username, conf.password, conf.host, conf.port, conf.database
            ),
            TlsMode::None,
        )
        .unwrap();
        let pool = Pool::new(manager).unwrap();

        Self(SyncArbiter::start(3, move || DatabaseInner(pool.clone())))
    }
}

impl Deref for Database {
    type Target = Addr<DatabaseInner>;

    fn deref(&self) -> &Addr<DatabaseInner> {
        &self.0
    }
}

pub struct DatabaseInner(pub Pool<PostgresConnectionManager>);

impl Actor for DatabaseInner {
    type Context = SyncContext<Self>;
}
