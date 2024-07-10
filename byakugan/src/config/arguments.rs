use crate::prelude::*;

use dotenvy::dotenv;
use clap::{Parser, Args, ArgAction, Subcommand};

/// Command line arguments and subcommands
#[derive(Parser)]
#[command(
    about           = None,
    name            = env!("CARGO_PKG_NAME"),
    version         = env!("CARGO_PKG_VERSION"),
    author          = env!("CARGO_PKG_AUTHORS"),
    long_about      = env!("CARGO_PKG_DESCRIPTION"),
    rename_all      = "kebab-case",
    rename_all_env  = "SCREAMING_SNAKE_CASE"
)]
pub struct Arguments {
    /// The subcommand to run
    #[clap(subcommand)]
    pub subcommand: Subcommands,

    /// The global arguments
    #[clap(flatten)]
    pub global: Globals,
}

/// The subcommands of the CLI
#[derive(Subcommand, Eq, PartialEq)]
pub enum Subcommands {
    /// Terminal user interface
    #[clap(short_flag(None))]
    Console,
    /// Server
    #[clap(short_flag(None))]
    Server,
}

/// The global arguments of the CLI
#[derive(Args)]
pub struct Globals {
    /// The database URL.
    #[clap(global(true), short('D'), long, env("BYAKUGAN_DB"), default_value(default_db()))]
    pub db: String,

    /// Path to the log file. By default, logs only to stdout
    #[clap(global(true), short(None), long, env("BYAKUGAN_LOGFILE"))]
    pub log_file: Option<String>,

    /// Quiet mode, do not print anything to stdout. Overrides verbose mode in stdout
    #[clap(global(true), short, long)]
    pub quiet: bool,

    /// Temporary mode, do not save anything to the database
    #[clap(global(true), short(None), long)]
    pub temp: bool,

    /// Verbose mode, use multiple times for more verbosity
    #[clap(global(true), short, long, action(ArgAction::Count))]
    pub verbose: u8,
}

fn default_db() -> String {
    let app_name = env!("CARGO_PKG_NAME");
    let db_name = format!("{app_name}.db");
    "sqlite://".to_string() + dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(app_name)
        .join(&db_name)
        .to_str()
        .unwrap_or(&db_name)
}

/// Get the command line arguments
pub fn get() -> Result<Arguments> {
    // Parse the environment variables and the command line arguments
    dotenv()?;
    Ok(Arguments::parse())
}