use std::env::current_dir;
use std::net::Ipv4Addr;
use std::path::Path;

use brace_cli::prelude::*;
use brace_config::load;
use failure::format_err;
use path_absolutize::Absolutize;

use crate::config::AppConfig;

pub fn cmd() -> Command {
    Command::new("run")
        .about("Runs the built-in web server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("The configuration file to use"),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("The host address"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("The port number"),
        )
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    match matches.value_of("config") {
        Some(file) => match load::file(file) {
            Ok(config) => {
                let config = overload_file(file, config, shell, matches)?;

                shell.info(format!("Using configuration file: {}", file))?;
                crate::run(config, Path::new(file))?;

                Ok(())
            }
            Err(err) => {
                shell.error(format!("Invalid configuration: {}", err))?;
                shell.exit(1);
            }
        },
        None => {
            let config = overload_default(shell, matches)?;

            shell.warn("No configuration file specified")?;
            crate::run(config, &current_dir()?)?;

            Ok(())
        }
    }
}

pub fn overload(
    mut config: AppConfig,
    shell: &mut Shell,
    matches: &ArgMatches,
) -> Result<AppConfig, failure::Error> {
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

pub fn overload_file(
    path: &str,
    config: AppConfig,
    shell: &mut Shell,
    matches: &ArgMatches,
) -> Result<AppConfig, failure::Error> {
    let mut config = overload(config, shell, matches)?;

    match Path::new(path).parent() {
        Some(parent) => {
            for theme in config.themes.iter_mut() {
                theme.path = parent.join(&theme.path).absolutize()?;
            }

            Ok(config)
        }
        None => Err(format_err!("Invalid path {}", path)),
    }
}

pub fn overload_default(
    shell: &mut Shell,
    matches: &ArgMatches,
) -> Result<AppConfig, failure::Error> {
    let mut config = overload(AppConfig::default(), shell, matches)?;

    for theme in config.themes.iter_mut() {
        theme.path = theme.path.absolutize()?;
    }

    Ok(config)
}
