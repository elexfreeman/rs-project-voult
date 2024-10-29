use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(Users::Table)
            .col(integer(Users::Id))
            .col(pk_auto(Users::UserId))
            .col(string(Users::FirstName))
            .col(string(Users::LastName))
            .col(string(Users::Username))
            .col(string(Users::LanguageCode))
            .index(Index::create().unique().name("idx-id-id").col(Users::Id))
            .to_owned();
        manager.create_table(table).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
    Username,
    LanguageCode
}