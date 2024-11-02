use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241025_173018_create_table::Contractors;
use crate::m20241025_173018_create_table::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(CacheLog::Table)
            .col(pk_auto(CacheLog::Id))
            .col(string(CacheLog::Caption))
            .col(string(CacheLog::Description))
            .col(integer(CacheLog::ProjectId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_owner_projects_id")
                    .from(CacheLog::Table, CacheLog::ProjectId)
                    .to(Projects::Table, Projects::Id),
            )
            .col(integer(CacheLog::ContractorId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_owner_contractor_id")
                    .from(CacheLog::Table, CacheLog::ProjectId)
                    .to(Contractors::Table, Contractors::Id),
            )
            .to_owned();
        manager.create_table(table).await?;

        let table = table_auto(CacheLogItems::Table)
            .col(pk_auto(CacheLogItems::Id))
            .col(string(CacheLogItems::Caption))
            .col(float(CacheLogItems::Price))
            .col(integer(CacheLogItems::Count))
            .col(integer(CacheLogItems::CacheLogId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_owner_cache_log_id")
                    .from(CacheLogItems::Table, CacheLogItems::CacheLogId)
                    .to(CacheLog::Table, CacheLog::Id),
            )
            .to_owned();
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CacheLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CacheLog {
    Table,
    Id,
    Caption,
    Description,
    ProjectId,
    ContractorId,
}


#[derive(DeriveIden)]
enum CacheLogItems {
    Table,
    Id,
    CacheLogId,
    Caption,
    Price,
    Count,
}
