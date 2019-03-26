use std::fmt::Display;

pub use clap::{App, AppSettings, Arg, ArgMatches};

pub use super::shell::{Shell, Verbosity};

pub type Command = App<'static, 'static>;

pub type ExecResult = Result<(), failure::Error>;

pub fn exit_command_invalid(cmd: &str, shell: &mut Shell, message: &dyn Display) -> ExecResult {
    match cmd {
        "" => {
            shell.error("Expected a valid subcommand")?;
            shell.print("")?;
            shell.print(message)?;
            shell.exit(1);
        }
        cmd => {
            shell.error(format!("Invalid subcommand: {}", cmd))?;
            shell.print("")?;
            shell.print(message)?;
            shell.exit(1);
        }
    }
}
