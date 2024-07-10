// --- | Linter | ---
#![cfg_attr(
    feature = "cargo-clippy",
    deny(clippy::correctness),
    warn(clippy::complexity, clippy::nursery, clippy::pedantic, clippy::perf, clippy::style, clippy::suspicious)
)]

// --- | Modules | ---
mod prelude;
pub use prelude::*;

mod cli;
mod config;
mod database;
mod error;

// --- | Entrypoint | ---

#[tokio::main]
async fn main() {
    if let Err(e) = entrypoint().await {
        if matches!(e, Error::Env(_)) || matches!(e, Error::Logging) {
            eprintln!("{e}");
        } else {
            log::error!("{e}");
        }
        std::process::exit(1);
    }
}

async fn entrypoint() -> Result<()> {
    let app = Config::new().await?;

    // Handle the modes
    if app.is_console() {
        cli::run()?;
    /*
    } else if app.is_server() {
        // server::run(app).await?
    */
    } else {
        unreachable!("Invalid mode. How did you get here?");
    }
    Ok(())
}