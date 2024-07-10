use crate::prelude::*;

/// Create the database schema
pub async fn create(db: &super::Database) -> Result<()> {
    use super::{model::prelude::*, relations::prelude::*};
    log::debug!("Ensuring that the database schema is defined...");
    // create the entities
    db.create_table(Domain).await?;
    db.create_table(Ipv4).await?;
    db.create_table(Port).await?;
    db.create_table(Service).await?;
    // create the relations
    db.create_table(DomainIpv4).await?;
    db.create_table(DomainSubdomain).await?;
    db.create_table(ServiceIpv4).await?;
    Ok(())
}