use crate::prelude::*;

use log::LevelFilter;
use log4rs::{
    Handle,
    append::console::ConsoleAppender,
    config::{Appender, Config, Root, Logger},
    encode::pattern::PatternEncoder
};

/// Initialize logging with the given level and quiet flag. The quiet flag only applies to the stdout.
pub fn init(name: &str, path: &Option<String>, verbosity: u8, quiet: bool) -> Result<Handle> {
    let logger = Logger::builder();
    let mut root = Root::builder();
    let mut config = Config::builder();

    let level = match verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let pattern = "| {date(%Y-%m-%d %H:%M:%S)(local)} | {highlight({level:<5})} | {message}{n}";
    
    // If not quiet, logs to stdout
    if !quiet {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();
        config = config.appender(Appender::builder().build("console", Box::new(console)));
        root = root.appender("console");
    }

    // If a path is given, logs to file
    // TODO: Syslog if not path provided and not windows
    if let Some(path) = path {
        let file = log4rs::append::file::FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build(path)?;
        config = config.appender(Appender::builder().build("file", Box::new(file)));
        root = root.appender("file");
    }

    Ok(log4rs::init_config(config
        .logger(logger.build(name, level))
        .build(root.build(LevelFilter::Off))?
    )?)
}