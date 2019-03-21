use crate::util::shell::Shell;
use crate::{config, web};
use clap::{App, Arg, ArgMatches};

pub fn cli() -> App<'static, 'static> {
    App::new("run")
        .about("Runs the built-in web server")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("The configuration file to use"),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("The host address"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("The port number"),
        )
}

pub fn exec(shell: &mut Shell, matches: &ArgMatches) -> Result<(), failure::Error> {
    match matches.value_of("config") {
        Some(file) => match config::load(file) {
            Ok(config) => {
                let config = config::overload(config, shell, matches)?;

                shell.info(format!("Using configuration file: {}", file))?;
                web::run(config);

                Ok(())
            }
            Err(err) => {
                shell.error(format!("Invalid configuration: {}", err))?;
                shell.exit(1);
            }
        },
        None => {
            let config = config::overload_default(shell, matches)?;

            shell.warn("No configuration file specified")?;
            web::run(config);

            Ok(())
        }
    }
}
