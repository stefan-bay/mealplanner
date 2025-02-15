use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(pk_auto(Recipe::Id))
                    .col(date_time(Recipe::CreatedAt))
                    .col(date_time(Recipe::UpdatedAt))
                    .col(string(Recipe::Title))
                    .col(string(Recipe::Instructions))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Recipe {
    Table,
    Id,

    Title,
    Instructions,

    CreatedAt,
    UpdatedAt,
}
