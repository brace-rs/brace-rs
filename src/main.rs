use clap::{crate_authors, crate_description, crate_name, crate_version, App, SubCommand};

mod web;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("web")
                .about("The built-in web server")
                .subcommand(SubCommand::with_name("run").about("Runs the built-in web server")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("web") {
        if let Some(_) = matches.subcommand_matches("run") {
            web::run();
        }
    }
}
