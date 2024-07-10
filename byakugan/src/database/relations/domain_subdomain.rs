use super::super::model::domain;

use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "domain_subdomain")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub domain: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub subdomain: Uuid,
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
        belongs_to = "domain::Entity",
        from = "Column::Subdomain",
        to = "domain::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Subdomain,
}

impl ActiveModelBehavior for ActiveModel {}
