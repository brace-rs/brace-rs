use crate::util::command::*;

pub mod run;

pub fn cmd() -> Command {
    Command::new("web")
        .about("The built-in web server")
        .subcommand(run::cmd())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
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
