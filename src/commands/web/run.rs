use crate::{config, exit_with_msg, web};
use clap::{App, Arg, ArgMatches};

pub fn cli() -> App<'static, 'static> {
    App::new("run")
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

pub fn exec(matches: &ArgMatches) {
    match matches.value_of("config") {
        Some(file) => match config::load(file) {
            Ok(config) => web::run(config::overload(config, matches)),
            Err(err) => exit_with_msg(1, &format!("Error: Invalid configuration: {}", err)),
        },
        None => web::run(config::overload_default(matches)),
    }
}
