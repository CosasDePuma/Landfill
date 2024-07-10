use super::{domain, super::relations::{domain_ipv4, service_ipv4}};

use sea_orm::{ActiveValue, entity::prelude::*};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ipv4")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub ip: String,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub first_seen: DateTime,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "domain_ipv4::Entity")]
    DomainIpv4,
    #[sea_orm(has_many = "service_ipv4::Entity")]
    ServiceIpv4,
}

impl Related<domain_ipv4::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DomainIpv4.def()
    }
}
impl Related<service_ipv4::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceIpv4.def()
    }
}
impl Related<domain::Entity> for Entity {
    fn to() -> RelationDef {
        domain_ipv4::Relation::Domain.def()
    }
    fn via() -> Option<RelationDef> {
        Some(domain_ipv4::Relation::Ip.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}