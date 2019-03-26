use std::net::Ipv4Addr;

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
