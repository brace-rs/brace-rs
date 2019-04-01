pub mod shell;

pub mod prelude {
    pub use clap::{
        crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg,
        ArgMatches,
    };

    pub use super::shell::command::{exit_command_invalid, Command, ExecResult};
    pub use super::shell::{Shell, Verbosity};
}
