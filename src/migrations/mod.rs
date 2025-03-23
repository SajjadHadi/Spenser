pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20250323_095154_create_categories_table;
mod m20250323_101235_create_transactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20250323_095154_create_categories_table::Migration),
            Box::new(m20250323_101235_create_transactions_table::Migration),
        ]
    }
}
