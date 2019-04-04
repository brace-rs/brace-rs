use std::path::Path;

use brace_web::config::LogOutput;
use chrono::Local;
use failure::Error;
use fern::{log_file, Dispatch};
use log::{warn, LevelFilter};
use path_absolutize::Absolutize;

use crate::config::AppConfig;

pub fn init(conf: &AppConfig, path: &Path) -> Result<(), Error> {
    let logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%SZ"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Off)
        .level_for("brace", conf.web.log.level.clone().into())
        .level_for("brace_web", conf.web.log.level.clone().into())
        .level_for("actix_web", conf.web.log.level.clone().into());

    match conf.web.log.output {
        LogOutput::Stdout => logger.chain(std::io::stdout()).apply()?,
        LogOutput::Stderr => logger.chain(std::io::stderr()).apply()?,
        LogOutput::File => match &conf.web.log.file {
            Some(file) => {
                let path = path.join("..").join(file).absolutize()?;
                logger.chain(log_file(path)?).apply()?;
            }
            None => {
                logger.chain(std::io::stderr()).apply()?;
                warn!("Invalid log file specified: {:?}", &conf.web.log.file);
            }
        },
    }

    Ok(())
}
