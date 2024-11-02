use sea_orm_migration::{prelude::*, schema::*};
use super::m20220101_000001_create_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(Projects::Table)
            .col(pk_auto(Projects::Id))
            .col(string(Projects::Caption))
            .col(string(Projects::Description))
            .col(integer(Projects::OwnerId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_owner_users_id")
                    .from(Projects::Table, Projects::OwnerId)
                    .to(Users::Table, Users::Id),
            )
            .to_owned();
        manager.create_table(table).await?;

        let table = table_auto(Contractors::Table)
            .col(pk_auto(Contractors::Id))
            .col(string(Contractors::Caption))
            .col(string(Contractors::Description))
            .col(integer(Contractors::OwnerId))
            .foreign_key(
                ForeignKey::create()
                    .name("FK_owner_users_id")
                    .from(Contractors::Table, Contractors::OwnerId)
                    .to(Users::Table, Users::Id),
            )
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Projects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Contractors::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Projects {
    Table,
    Id,
    Caption,
    Description,
    OwnerId,
}

#[derive(Iden)]
pub enum Contractors {
    Table,
    Id,
    Caption,
    Description,
    OwnerId,
}