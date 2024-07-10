use super::super::model::{ipv4, port, service};

use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "service_ipv4")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub service: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ip: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub port: Uuid,
    pub first_seen: String,
    pub last_seen: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "ipv4::Entity",
        from = "Column::Ip",
        to = "ipv4::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Ipv4,
    #[sea_orm(
        belongs_to = "port::Entity",
        from = "Column::Port",
        to = "port::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Ports,
    #[sea_orm(
        belongs_to = "service::Entity",
        from = "Column::Service",
        to = "service::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Services,
}

impl Related<ipv4::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ipv4.def()
    }
}
impl Related<port::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ports.def()
    }
}
impl Related<service::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Services.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
