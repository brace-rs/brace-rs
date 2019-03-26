use brace::util::command::*;

pub fn cmd() -> Command {
    Command::new("init")
        .about("Creates a new site in an existing directory")
        .arg(
            Arg::with_name("directory")
                .value_name("DIR")
                .required(true)
                .index(1)
                .help("The target directory"),
        )
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> ExecResult {
    let directory = matches.value_of("directory").unwrap();
    match brace::app::init::init(directory) {
        Ok(()) => {
            shell.info(format!("Created new site at {}", directory))?;
            shell.exit(0);
        }
        Err(err) => {
            shell.error(err)?;
            shell.exit(1);
        }
    }
}
