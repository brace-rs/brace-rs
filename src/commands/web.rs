use crate::exit_with_msg;
use clap::{App, ArgMatches};

pub mod run;

pub fn cli() -> App<'static, 'static> {
    App::new("web")
        .about("The built-in web server")
        .subcommand(run::cli())
}

pub fn exec(matches: &ArgMatches) {
    match matches.subcommand() {
        ("run", Some(matches)) => run::exec(matches),
        (command, _) => exit_with_msg(1, &format!("Error: Invalid command: {}", command)),
    }
}
