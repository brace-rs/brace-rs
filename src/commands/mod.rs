use crate::util::shell::*;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, ArgMatches,
};

pub mod init;
pub mod web;

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
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
        .subcommand(init::cli())
        .subcommand(web::cli())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> Result<(), failure::Error> {
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
        ("", _) => {
            shell.error("Expected a valid subcommand")?;
            shell.print("")?;
            shell.print(matches.usage())?;
            shell.exit(1);
        }
        (command, _) => {
            shell.error(format!("Invalid subcommand: {}", command))?;
            shell.print("")?;
            shell.print(matches.usage())?;
            shell.exit(1);
        }
    }
}

pub fn run() {
    let mut shell = Shell::new();

    if let Err(err) = exec(&mut shell, &cli().get_matches()) {
        shell.error(err).unwrap();
        shell.exit(1);
    }
}
