use crate::util::command::*;
use clap::{crate_authors, crate_description, crate_name, crate_version};

pub mod init;
pub mod web;

pub fn cmd() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Mutes command output but retains logging")
                .conflicts_with("verbose"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Makes command output verbose")
                .conflicts_with("quiet"),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .help("Sets command output coloring")
                .takes_value(true)
                .possible_values(&["auto", "always", "never"]),
        )
        .subcommand(init::cmd())
        .subcommand(web::cmd())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    if matches.is_present("verbose") {
        shell.set_verbosity(Verbosity::Verbose);
    }

    if matches.is_present("quiet") {
        shell.set_verbosity(Verbosity::Quiet);
    }

    if let Some(color) = matches.value_of("color") {
        shell.set_color_choice(color)?;
    }

    match matches.subcommand() {
        ("init", Some(matches)) => init::exec(shell, matches),
        ("web", Some(matches)) => web::exec(shell, matches),
        (command, _) => exit_command_invalid(command, shell, &matches.usage()),
    }
}

pub fn run() {
    let mut shell = Shell::new();

    if let Err(err) = exec(&mut shell, &cmd().get_matches()) {
        shell.error(err).unwrap();
        shell.exit(1);
    }
}
