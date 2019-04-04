use brace_cli::prelude::*;

pub mod init;
pub mod run;

pub fn cmd() -> Command {
    Command::new("web")
        .about("The built-in web server")
        .subcommand(init::cmd())
        .subcommand(run::cmd())
        .setting(AppSettings::AllowExternalSubcommands)
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    match matches.subcommand() {
        ("run", Some(matches)) => run::exec(shell, matches),
        ("init", Some(matches)) => init::exec(shell, matches),
        (command, _) => exit_command_invalid(command, shell, &matches.usage()),
    }
}
