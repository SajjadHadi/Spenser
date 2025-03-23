use sea_orm_migration::{prelude::*, schema::*};
use crate::migrations::m20250323_095154_create_categories_table::Category;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::FirstName).not_null())
                    .col(string(User::LastName).not_null())
                    .col(string(User::Email).not_null())
                    .col(string(User::Password).not_null())
                    .col(big_integer(User::Balance).default(0))
                    .col(
                        timestamp(User::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    Password,
    Balance,
    CreatedAt,
    UpdatedAt,
}
