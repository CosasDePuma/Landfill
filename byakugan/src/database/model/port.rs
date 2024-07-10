use super::super::relations::service_ipv4;

use sea_orm::{ActiveValue, entity::prelude::*};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "ports")]
pub struct Model {
    #[sea_orm(unique)] // FIXME: This should be primary key but sea-orm doesn't support composite primary keys
    pub id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub port: u16,
    #[sea_orm(primary_key, auto_increment = false)]
    pub protocol: String,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub first_seen: DateTime,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub last_seen: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "service_ipv4::Entity")]
    ServiceIpv4,
}

impl Related<service_ipv4::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServiceIpv4.def()
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