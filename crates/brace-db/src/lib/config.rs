use std::error::Error;
use std::net::Ipv4Addr;

use postgres::params::{ConnectParams, Host, IntoConnectParams};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DatabaseConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::new(127, 0, 0, 1),
            port: 5432,
            username: "postgres".into(),
            password: "postgres".into(),
            database: "postgres".into(),
        }
    }
}

impl IntoConnectParams for DatabaseConfig {
    fn into_connect_params(self) -> Result<ConnectParams, Box<dyn Error + Sync + Send>> {
        let mut builder = ConnectParams::builder();

        builder.user(&self.username, Some(&self.password));
        builder.database(&self.database);

        Ok(builder.build(Host::Tcp(self.host.to_string())))
    }
}
