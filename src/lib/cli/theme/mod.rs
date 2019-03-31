use crate::util::shell::prelude::*;

pub mod init;

pub fn cmd() -> Command {
    Command::new("theme")
        .about("The theme system")
        .subcommand(init::cmd())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    match matches.subcommand() {
        ("init", Some(matches)) => init::exec(shell, matches),
        (command, _) => exit_command_invalid(command, shell, &matches.usage()),
    }
}
