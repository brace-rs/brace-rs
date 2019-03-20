use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};
use config::Config;
use std::fmt::Display;
use std::net::Ipv4Addr;
use std::process::exit;

mod config;
mod init;
mod web;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new site")
                .arg(
                    Arg::with_name("directory")
                        .value_name("DIR")
                        .required(true)
                        .index(1)
                        .help("The target directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("web")
                .about("The built-in web server")
                .subcommand(
                    SubCommand::with_name("run")
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
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(matches)) => {
            if let Err(err) = init::init(matches.value_of("directory").unwrap()) {
                exit_with_msg(1, &format!("Error initializing site: {}", err));
            }
        }
        ("web", Some(matches)) => {
            if let Some(matches) = matches.subcommand_matches("run") {
                match matches.value_of("config") {
                    Some(file) => match config::load(file) {
                        Ok(config) => web::run(override_config(config, matches)),
                        Err(err) => {
                            exit_with_msg(1, &format!("Error: Invalid configuration: {}", err))
                        }
                    },
                    None => web::run(override_config(config::Config::default(), matches)),
                }
            }
        }
        _ => exit_with_msg(1, &format!("{}", matches.usage())),
    }
}

fn override_config(mut config: Config, matches: &ArgMatches) -> Config {
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

fn exit_with_msg(code: i32, err: &Display) -> ! {
    println!("{}", err);
    exit(code)
}
