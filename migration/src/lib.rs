pub use sea_orm_migration::prelude::*;

mod m20250215_190556_create_recipes;
mod m20250215_192658_create_ingredients;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250215_190556_create_recipes::Migration),
            Box::new(m20250215_192658_create_ingredients::Migration),
        ]
    }
}
