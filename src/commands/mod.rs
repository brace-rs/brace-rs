use crate::exit_with_msg;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, ArgMatches};

pub mod init;
pub mod web;

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(init::cli())
        .subcommand(web::cli())
}

pub fn exec(matches: &ArgMatches) {
    match matches.subcommand() {
        ("init", Some(matches)) => init::exec(matches),
        ("web", Some(matches)) => web::exec(matches),
        (command, _) => exit_with_msg(1, &format!("Error: Invalid command: {}", command)),
    }
}

pub fn run() {
    exec(&cli().get_matches());
}
