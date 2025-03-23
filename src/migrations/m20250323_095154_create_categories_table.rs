use super::m20220101_000001_create_users_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(pk_auto(Category::Id))
                    .col(integer(Category::UserId).not_null())
                    .col(string(Category::Name).not_null())
                    .col(string(Category::Description).null())
                    .col(big_integer(Category::Balance).default(0))
                    .col(
                        timestamp(Category::CreatedAt)
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        timestamp(Category::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .extra("ON UPDATE CURRENT_TIMESTAMP")
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_user_id") // Optional: custom constraint name
                            .from(Category::Table, Category::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Balance,
    CreatedAt,
    UpdatedAt,
}
