pub use super::shell::{Shell, Verbosity};
pub use clap::{App, AppSettings, Arg, ArgMatches};

pub type Command = App<'static, 'static>;

pub type ExecResult = Result<(), failure::Error>;
