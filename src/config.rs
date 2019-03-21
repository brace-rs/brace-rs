use crate::util::shell::Shell;
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

pub fn overload(
    mut config: Config,
    shell: &mut Shell,
    matches: &ArgMatches,
) -> Result<Config, failure::Error> {
    if let Some(host) = matches.value_of("host") {
        if let Ok(host) = host.parse::<Ipv4Addr>() {
            config.web.host = host;
        } else {
            shell.error(format!("Invalid host address: {}", host))?;
            shell.exit(1);
        }
    }

    if let Some(port) = matches.value_of("port") {
        if let Ok(port) = port.parse::<u16>() {
            config.web.port = port;
        } else {
            shell.error(format!("Invalid port number: {}", port))?;
            shell.exit(1);
        }
    }

    Ok(config)
}

pub fn overload_default(shell: &mut Shell, matches: &ArgMatches) -> Result<Config, failure::Error> {
    overload(Config::default(), shell, matches)
}
