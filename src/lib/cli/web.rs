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
        (command, _) => exit_command_invalid(command, shell, &matches.usage()),
    }
}
