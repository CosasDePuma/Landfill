use crate::prelude::*;

use sea_orm::{Database as DB, DatabaseConnection, DbBackend, Statement, ConnectionTrait, DatabaseBackend};
use url::Url;

/// Prepare the `SQLite in-memory` database. Return the URL if the database is temporary
fn check_if_temporary(temp: bool) -> Option<String> {
    if temp {
        log::warn!("You are using a temporary database, nothing will be saved!");
        Some("sqlite::memory:".to_owned())
    } else {
        None
    }
}

/// Prepare the `SQLite` database. Create the file if it does not exist and add the permissions to the URL
fn prepare_sqlite(url: &Url) -> Result<String> {
    // create the file if it does not exist
    let file = format!("{}{}", url.host().map(|host| host.to_string()).unwrap_or_default(), url.path());
    log::trace!("Creating SQLite database file {file}");
    std::fs::File::create(file)?;
    // check the mode
    url.query()
        .map_or_else(|| Ok(url.to_string()), // Some
            |_| { log::trace!("Adding SQLite permissions to the URL"); Ok(format!("{url}&mode=rwc")) }) // None
}

/// Prepare the `MySQL` database creating the database if it does not exist
async fn prepare_mysql(conn: &DatabaseConnection, url: &Url, default_db: &str) -> Result<String> {
    let stmt = format!("CREATE DATABASE IF NOT EXISTS `{default_db}`");
    log::trace!("Executing MySQL statement: {stmt}");
    let res = conn.execute(Statement::from_string(DbBackend::MySql, stmt)).await;
    res.map(|res| { log::trace!("Result: {:?}", res); Ok(format!("{url}/{default_db}")) })?
}

/// Prepare the `PostgreSQL` database creating the database if it does not exist
async fn prepare_postgres(conn: &DatabaseConnection, url: &Url, default_db: &str) -> Result<String> {
    let stmt = format!("CREATE DATABASE \"{default_db}\"");
    log::trace!("Executing PostgreSQL statement: {stmt}");
    let res = conn.execute(Statement::from_string(DbBackend::Postgres, stmt)).await;
    log::trace!("Result: {:?}", res);
    match res {
        Ok(res) => log::trace!("Result: {:?}", res),
        Err(err) => {
            if err.to_string().contains("already exists") {
                log::trace!("Database '{default_db}' already exists");
            } else {
                Err(err)?;
            }
        }
    };          
    Ok(format!("{url}/{default_db}"))
}

/// Create the database if it does not exist and connect to it
pub async fn start(app_name: &str, url: &str, temp: bool) -> Result<DatabaseConnection> {
    let dburl = if let Some(in_memory) = check_if_temporary(temp) { in_memory } else {
        let parsed_url = Url::parse(url)?;
        // If the scheme is SQLite, preapre the file.
        if parsed_url.scheme().to_lowercase().as_str() == "sqlite" {
            prepare_sqlite(&parsed_url)?
        } else {
            // Else, check if the URL contains a database name
            let mut path = parsed_url.path().trim();
            while path.starts_with('/') {
                path = &path[1..];
            }
            if path.is_empty() {
                let conn = DB::connect(url).await?;
                match &conn.get_database_backend() {
                    DatabaseBackend::MySql => prepare_mysql(&conn, &parsed_url, app_name).await?,
                    DatabaseBackend::Postgres => prepare_postgres(&conn, &parsed_url, app_name).await?,
                    DatabaseBackend::Sqlite => unreachable!("SQLite is already handled. This should never happen!"),
                }
            } else {
                parsed_url.to_string()
            }
        }
    };
    log::debug!("Connecting to database {dburl}");
    Ok(DB::connect(dburl).await?)
}