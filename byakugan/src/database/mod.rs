use crate::prelude::*;

mod connection;
mod schema;
pub mod model;
pub mod relations;

use sea_orm::{DatabaseConnection, Schema, EntityTrait, ConnectionTrait, DbBackend};

/// Wrapper around the database connection
pub struct Database {
    backend: DbBackend,
    conn: DatabaseConnection,
    schema: Schema
}

impl Database {
    /// Create a new database connection
    /// 
    /// # Errors
    /// 
    /// Will return an error if the database connection fails
    pub async fn new(app_name: &str, url: &str, temp: bool) -> Result<Self> {
        // connect to the database
        let conn: DatabaseConnection = connection::start(app_name, url, temp).await?;
        log::info!("Successfully connected to database");
        // get the backend and the schema
        let backend = conn.get_database_backend();
        let schema = Schema::new(backend);
        // create the database handler
        let handler = Self { backend, conn, schema };
        // create the schema
        schema::create(&handler).await?;
        Ok(handler)
    }

    /// Create a table from an entity
    async fn create_table(&self, entity: impl EntityTrait) -> Result<()> {
        let stmt = self.backend.build(&self.schema.create_table_from_entity(entity));
        log::trace!("Executing statement: {stmt}");
        let res = self.conn.execute(stmt).await;
        match res {
            Ok(res) => log::trace!("Result: {:?}", res),
            Err(err) => {
                if err.to_string().contains("already exists") {
                    log::trace!("Table already exists");
                } else {
                    Err(err)?;
                }
            }
        };
        Ok(())
    }
}