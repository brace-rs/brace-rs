use std::ops::Deref;

use actix::{Actor, Addr, SyncArbiter, SyncContext};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

pub use self::config::DatabaseConfig;

pub mod config;

#[derive(Clone)]
pub struct Database(pub Addr<DatabaseInner>);

impl Database {
    pub fn from_config(conf: DatabaseConfig) -> Result<Self, failure::Error> {
        let manager = PostgresConnectionManager::new(conf, TlsMode::None)?;
        let pool = Pool::new(manager)?;

        Ok(Self(SyncArbiter::start(3, move || {
            DatabaseInner(pool.clone())
        })))
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
