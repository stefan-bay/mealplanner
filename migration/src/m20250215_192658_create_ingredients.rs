use sea_orm_migration::{prelude::*, schema::*};

use super::m20250215_190556_create_recipes::Recipe;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredient::Table)
                    .if_not_exists()
                    .col(pk_auto(Ingredient::Id))
                    .col(string(Ingredient::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredient::Table)
                    .if_not_exists()
                    .col(pk_auto(RecipeIngredient::Id))
                    .col(integer(RecipeIngredient::RecipeId))
                    .col(integer(RecipeIngredient::IngredientId))
                    .col(string(RecipeIngredient::Amount))
                    .col(string(RecipeIngredient::Unit))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipeingredient-recipe_id")
                            .from(RecipeIngredient::Table, RecipeIngredient::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipeingredient-ingredient_id")
                            .from(RecipeIngredient::Table, RecipeIngredient::IngredientId)
                            .to(Ingredient::Table, Ingredient::Id)
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingredient::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ingredient {
    Table,
    Id,

    Name,
}

#[derive(DeriveIden)]
enum RecipeIngredient {
    Table,
    Id,

    RecipeId,
    IngredientId,
    Amount,
    Unit,
}
