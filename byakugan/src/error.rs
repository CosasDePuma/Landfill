/// Standard Result type for this crate. The Err variant is always an Error.
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for this crate.
pub enum Error {
    /// Error from the database
    Database(String),
    /// Error from environment variables
    Env(String),
    /// Error interacting with the filesystem
    IO(String),
    /// Error setting up logging
    Logging,
    /// Error parsing URLs
    Url(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Database(ref e) => write!(f, "Database error: {e}"),
            Self::Env(ref e) => write!(f, "Environment error: {e}"),
            Self::IO(ref e) => write!(f, "IO error: {e}"),
            Self::Logging => write!(f, "Could not set up logging"),
            Self::Url(ref e) => write!(f, "URL error: {e}"),
        }
    }
}

impl From<dotenvy::Error> for Error {
    fn from(e: dotenvy::Error) -> Self {
        Self::Env(e.to_string())
    }
}
impl From<log::SetLoggerError> for Error {
    fn from(_: log::SetLoggerError) -> Self {
        Self::Logging
    }
}
impl From<log4rs::config::runtime::ConfigErrors> for Error {
    fn from(_: log4rs::config::runtime::ConfigErrors) -> Self {
        Self::Logging
    }
}
impl From<sea_orm::error::DbErr> for Error {
    fn from(e: sea_orm::error::DbErr) -> Self {
        Self::Database(e.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.to_string())
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::Url(e.to_string())
    }
}
