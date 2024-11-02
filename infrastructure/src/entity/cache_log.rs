//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cache_log")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub caption: String,
    pub description: String,
    pub project_id: i32,
    pub contractor_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cache_log_items::Entity")]
    CacheLogItems,
    #[sea_orm(
        belongs_to = "super::contractors::Entity",
        from = "Column::ProjectId",
        to = "super::contractors::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Contractors,
    #[sea_orm(
        belongs_to = "super::projects::Entity",
        from = "Column::ProjectId",
        to = "super::projects::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Projects,
}

impl Related<super::cache_log_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CacheLogItems.def()
    }
}

impl Related<super::contractors::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contractors.def()
    }
}

impl Related<super::projects::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Projects.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
