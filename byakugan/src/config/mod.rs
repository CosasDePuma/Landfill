use crate::prelude::*;

mod logging;
mod arguments;

/// A struct to hold the configuration of the application
pub struct Config {
    /// The app name
    pub app_name: String,
    /// The app version
    pub app_version: String,
    /// The database wrapper
    pub database: Database,
    /// The mode of the application
    pub mode: arguments::Subcommands,
}
impl Config {
    /// Create a new configuration
    /// 
    /// # Errors
    /// 
    /// Will return an error if the database connection fails or if the logger fails to initialize
    pub async fn new() -> Result<Self> {
        let app_name = env!("CARGO_PKG_NAME").to_string();
        let app_version = env!("CARGO_PKG_VERSION").to_string();

        // Parse the command line arguments
        let args = arguments::get()?;

        // Initialize the logger
        _ = logging::init(&app_name, &args.global.log_file, args.global.verbose, args.global.quiet)?;

        // Log the application name and version
        log::info!("Starting {app_name} v{app_version}");

        // Initialize the application directory
        init_app_directory(&app_name)?;

        // Connect to the database
        let database = Database::new(&app_name, &args.global.db, args.global.temp).await?;

        // Save the database connection into the configuration
        Ok(Self {
            app_name,
            app_version,
            database,
            mode: args.subcommand,
        })
    }
    /// Check if the mode is console
    #[must_use] /// FIXME: WTF is this? Try to remove it or understand it
    pub fn is_console(&self) -> bool {
        self.mode == arguments::Subcommands::Console
    }
    /*
    /// Check if the mode is server
    pub fn is_server(&self) -> bool {
        self.mode == arguments::Subcommands::Server
    }
    */
}

/// Initialize the application directory, ensuring it exists
fn init_app_directory(app_name: &str) -> Result<()> {
    let data_dir = dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    let app_dir = data_dir.join(app_name);
    std::fs::create_dir_all(app_dir)?;
    Ok(())
}