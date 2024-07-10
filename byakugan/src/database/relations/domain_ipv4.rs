use super::super::model::{domain, ipv4};

use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "domain_ipv4")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub domain: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub ip: Uuid,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub first_seen: DateTime,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "domain::Entity",
        from = "Column::Domain",
        to = "domain::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Domain,
    #[sea_orm(
        belongs_to = "ipv4::Entity",
        from = "Column::Ip",
        to = "ipv4::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Ip,
}

impl Related<domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domain.def()
    }
}

impl Related<ipv4::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ip.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
