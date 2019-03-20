use crate::{exit_with_msg, init};
use clap::{App, Arg, ArgMatches};

pub fn cli() -> App<'static, 'static> {
    App::new("init")
        .about("Creates a new site in an existing directory")
        .arg(
            Arg::with_name("directory")
                .value_name("DIR")
                .required(true)
                .index(1)
                .help("The target directory"),
        )
}

pub fn exec(matches: &ArgMatches) {
    if let Err(err) = init::init(matches.value_of("directory").unwrap()) {
        exit_with_msg(1, &format!("Error initializing site: {}", err));
    }
}
