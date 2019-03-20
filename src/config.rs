use crate::exit_with_msg;
use crate::web::config::Config as WebConfig;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub web: WebConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web: WebConfig::default(),
        }
    }
}

pub fn load(path: &str) -> Result<Config, Box<dyn Error + 'static>> {
    let string = std::fs::read_to_string(path)?;
    let config = toml::from_str(&string)?;

    Ok(config)
}

pub fn overload(mut config: Config, matches: &ArgMatches) -> Config {
    if let Some(host) = matches.value_of("host") {
        if let Ok(host) = host.parse::<Ipv4Addr>() {
            config.web.host = host;
        } else {
            exit_with_msg(1, &format!("Error: Invalid host address {}", host));
        }
    }

    if let Some(port) = matches.value_of("port") {
        if let Ok(port) = port.parse::<u16>() {
            config.web.port = port;
        } else {
            exit_with_msg(1, &format!("Error: Invalid port number {}", port));
        }
    }

    config
}

pub fn overload_default(matches: &ArgMatches) -> Config {
    overload(Config::default(), matches)
}
