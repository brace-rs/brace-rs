use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod config;
mod init;
mod web;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new site")
                .arg(
                    Arg::with_name("directory")
                        .value_name("DIR")
                        .required(true)
                        .index(1)
                        .help("The target directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("web")
                .about("The built-in web server")
                .subcommand(
                    SubCommand::with_name("run")
                        .about("Runs the built-in web server")
                        .arg(
                            Arg::with_name("config")
                                .short("c")
                                .long("config")
                                .value_name("FILE")
                                .help("The configuration file to use"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(matches)) => {
            if let Err(err) = init::init(matches.value_of("directory").unwrap()) {
                println!("Error initializing site: {}", err);
            }
        }
        ("web", Some(matches)) => {
            if let Some(matches) = matches.subcommand_matches("run") {
                match matches.value_of("config") {
                    Some(file) => match config::load(file) {
                        Ok(config) => web::run(config),
                        Err(err) => println!("Error loading configuration: {}", err),
                    },
                    None => web::run(config::Config::default()),
                }
            }
        }
        _ => println!("{}", matches.usage()),
    }
}
