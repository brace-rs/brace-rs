use crate::util::shell::Shell;
use clap::{App, AppSettings, ArgMatches};

pub mod run;

pub fn cli() -> App<'static, 'static> {
    App::new("web")
        .about("The built-in web server")
        .subcommand(run::cli())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> Result<(), failure::Error> {
    match matches.subcommand() {
        ("run", Some(matches)) => run::exec(shell, matches),
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
